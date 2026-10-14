use reqwest::blocking::Client;
use reqwest::header::{ACCEPT,LINK, USER_AGENT};
use reqwest::StatusCode;
use crate::github::model::{GithubRepository,GithubRepositoryPage};
use crate::github::error::GithubError;

pub struct GithubClient {
    client: Client,
}

impl GithubClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }


    pub fn fetch_repositories_page(
        &self,
        username: &str,
        page: u32,
    ) -> Result<GithubRepositoryPage, GithubError> {
        let url = format!(
            "https://api.github.com/users/{username}/repos"
        );

        let response = self.client
            .get(&url)
            .header(USER_AGENT, "rustfolio")
            .header(ACCEPT, "application/vnd.github+json")
            .query(&[
                ("per_page", "100"),
                ("type", "owner"),
                ("page", &page.to_string()),
            ])
            .send()
            .map_err(GithubError::Network)?;

        let status = response.status();

        // Handle nonexistent GitHub users.
        if status == StatusCode::NOT_FOUND {
            return Err(GithubError::UserNotFound(
                username.to_string()
            ));
        }

        // Handle HTTP 429.
        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(GithubError::RateLimited);
        }

        // Handle GitHub primary rate-limit exhaustion.
        if status == StatusCode::FORBIDDEN {
            let remaining = response
                .headers()
                .get("x-ratelimit-remaining")
                .and_then(|value| value.to_str().ok());

            if remaining == Some("0") {
                return Err(GithubError::RateLimited);
            }
        }

        // Handle other HTTP errors.
        let response = response
            .error_for_status()
            .map_err(GithubError::Api)?;

        let has_next = response
            .headers()
            .get(LINK)
            .and_then(|value| value.to_str().ok())
            .map(has_next_page)
            .unwrap_or(false);

        // Deserialize successful JSON responses.
        let repositories = response
            .json::<Vec<GithubRepository>>()
            .map_err(GithubError::Decode)?;

        Ok(GithubRepositoryPage {
            repositories,
            has_next_page: has_next,
        })
    }

    pub fn fetch_all_repositories(
        &self,
        username: &str,
    ) -> Result<Vec<GithubRepository>, GithubError> {
        collect_repository_pages(|page| {
            self.fetch_repositories_page(username, page)
        })
    }


}


fn has_next_page(link_header: &str) -> bool {
    // Step 1: Split the header into individual links.
    for link in link_header.split(',') {

        // Step 2: Split each link into its parameters.
        for parameter in link.split(';') {

            // Step 3: Remove leading and trailing whitespace.
            let parameter = parameter.trim();

            // Step 4: Check for the next-page relationship.
            if parameter == r#"rel="next""# {
                return true;
            }
        }
    }

    // Step 5: No next-page relationship was found.
    false
}


fn collect_repository_pages<F>(
    mut fetch_page: F,
) -> Result<Vec<GithubRepository>, GithubError>
where
    F: FnMut(u32) -> Result<GithubRepositoryPage, GithubError>,
{
    let mut all_repositories = Vec::new();
    let mut page: u32 = 1;

    loop {
        let result = fetch_page(page)?;

        let has_next = result.has_next_page;

        all_repositories.extend(result.repositories);

        if !has_next {
            break;
        }

        page = page
            .checked_add(1)
            .ok_or(GithubError::PaginationOverflow)?;
    }

    Ok(all_repositories)
}


#[cfg(test)]
mod tests {
    use super::*;


    fn mock_repository(name: &str) -> GithubRepository {
        serde_json::from_value(serde_json::json!({
        "name": name,
        "full_name": format!("test/{name}"),
        "html_url": format!("https://github.com/test/{name}"),
        "description": null,
        "language": "Rust",
        "private": false,
        "fork": false
    }))
            .expect("Valid mock GitHub repository")
    }

    #[test]
    #[ignore = "Requires network access"]
    fn fetch_live_github_repositories() {
        let client = GithubClient::new();

        let page = client
            .fetch_repositories_page("octocat", 1)
            .expect("Failed to fetch repositories");

        println!("Total repositories: {}", page.repositories.len());
        println!("Has next page: {}", page.has_next_page);

        for repository in &page.repositories {
            println!(
                "{} | {:?} | {}",
                repository.name,
                repository.language,
                repository.html_url
            );
        }

        let rust_repositories: Vec<&GithubRepository> = page
            .repositories
            .iter()
            .filter(|repository| repository.is_rust_project())
            .collect();

        println!("Rust repositories: {}", rust_repositories.len());

        assert!(page.repositories.len() <= 100);
    }

    #[test]
    #[ignore = "Requires GitHub API access"]
    fn fetch_nonexistent_github_user() {
        let client = GithubClient::new();

        let result = client.fetch_repositories_page(
            "rustfolio-this-user-should-not-exist-987654321",
            1,
        );

        assert!(
            matches!(result, Err(GithubError::UserNotFound(_))),
            "Expected UserNotFound, got {:?}",
            result
        );
    }

    #[test]
    #[ignore = "Requires GitHub API access"]
    fn fetch_multiple_repository_pages() {
        let client = GithubClient::new();

        let page_1 = client
            .fetch_repositories_page("octocat", 1)
            .expect("Failed to fetch page 1");

        let page_2 = client
            .fetch_repositories_page("octocat", 2)
            .expect("Failed to fetch page 2");

        println!(
            "Page 1 repositories: {}",
            page_1.repositories.len()
        );

        println!(
            "Page 2 repositories: {}",
            page_2.repositories.len()
        );

        println!("Page 1 has next: {}", page_1.has_next_page);
        println!("Page 2 has next: {}", page_2.has_next_page);

        assert!(page_1.repositories.len() <= 100);
        assert!(page_2.repositories.len() <= 100);
    }


    #[test]
    #[ignore = "Requires GitHub API access"]
    fn fetch_all_github_repositories() {
        let client = GithubClient::new();

        let repositories = client
            .fetch_all_repositories("octocat")
            .expect("Failed to fetch all repositories");

        println!("Total repositories: {}", repositories.len());

        for repository in &repositories {
            println!("{}", repository.name);
        }

        assert!(repositories.len() >= 1);
    }


    #[test]
    fn detects_next_page() {
        let header = r#"<https://api.github.com/users/test/repos?page=2>; rel="next""#;

        assert!(has_next_page(header));
    }

    #[test]
    fn detects_last_page() {
        let header = r#"<https://api.github.com/users/test/repos?page=1>; rel="prev""#;

        assert!(!has_next_page(header));
    }

    #[test]
    fn handles_empty_link_header() {
        assert!(!has_next_page(""));
    }

    #[test]
    #[ignore = "Requires GitHub API access"]
    fn fetch_repository_page_with_metadata() {
        let client = GithubClient::new();

        let result = client
            .fetch_repositories_page("octocat", 1)
            .expect("Failed to fetch GitHub repositories");

        println!("Repositories: {}", result.repositories.len());
        println!("Has next page: {}", result.has_next_page);

        assert!(result.repositories.len() <= 100);
    }


    #[test]
    fn collects_multiple_repository_pages() {
        let mut requested_pages = Vec::new();

        let repositories = collect_repository_pages(|page| {
            requested_pages.push(page);

            let result = match page {
                1 => GithubRepositoryPage {
                    repositories: vec![
                        mock_repository("repo-1"),
                        mock_repository("repo-2"),
                    ],
                    has_next_page: true,
                },
                2 => GithubRepositoryPage {
                    repositories: vec![
                        mock_repository("repo-3"),
                        mock_repository("repo-4"),
                    ],
                    has_next_page: true,
                },
                3 => GithubRepositoryPage {
                    repositories: vec![
                        mock_repository("repo-5"),
                    ],
                    has_next_page: false,
                },
                _ => panic!("Unexpected page requested: {page}"),
            };

            Ok(result)
        })
            .expect("Pagination should succeed");

        assert_eq!(repositories.len(), 5);
        assert_eq!(requested_pages, vec![1, 2, 3]);

        let names: Vec<&str> = repositories
            .iter()
            .map(|repository| repository.name.as_str())
            .collect();

        assert_eq!(
            names,
            vec!["repo-1", "repo-2", "repo-3", "repo-4", "repo-5"]
        );
    }

    #[test]
    fn stops_pagination_when_a_page_fails() {
        let mut requested_pages = Vec::new();

        let result = collect_repository_pages(|page| {
            requested_pages.push(page);

            match page {
                1 => Ok(GithubRepositoryPage {
                    repositories: vec![
                        mock_repository("repo-1"),
                    ],
                    has_next_page: true,
                }),

                2 => Err(GithubError::RateLimited),

                _ => panic!("Unexpected page requested: {page}"),
            }
        });

        assert!(matches!(result, Err(GithubError::RateLimited)));

        assert_eq!(requested_pages, vec![1, 2]);
    }
}