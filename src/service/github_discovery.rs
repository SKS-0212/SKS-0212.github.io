use crate::github::client::GithubClient;
use crate::github::error::GithubError;
use crate::github::model::GithubRepository;
use crate::model::project::Project;
use crate::service::github_project::{GithubProjectError, load_github_project};

#[derive(Debug)]
pub struct GithubProjectDiagnostic {
    pub repository: String,
    pub message: String,
}

#[derive(Debug)]
pub struct GithubDiscoveryReport {
    pub projects: Vec<Project>,
    pub diagnostics: Vec<GithubProjectDiagnostic>,
}

/// Keeps only public, non-fork Rust repositories.
pub fn filter_rust_repositories(repositories: Vec<GithubRepository>) -> Vec<GithubRepository> {
    repositories
        .into_iter()
        .filter(|repository| repository.is_rust_project())
        .collect()
}

/// Processes repositories using a supplied project loader.
///
/// This function is independent of HTTP, making it easy
/// to test with a fake loader.
fn collect_github_projects<F>(
    repositories: Vec<GithubRepository>,
    mut load: F,
) -> Result<GithubDiscoveryReport, GithubError>
where
    F: FnMut(&GithubRepository) -> Result<Option<Project>, GithubProjectError>,
{
    let mut report = GithubDiscoveryReport {
        projects: Vec::new(),
        diagnostics: Vec::new(),
    };

    for repository in repositories {
        match load(&repository) {
            Ok(Some(project)) => {
                report.projects.push(project);
            }

            Ok(None) => {
                report.diagnostics.push(GithubProjectDiagnostic {
                    repository: repository.full_name.clone(),
                    message: "Cargo.toml or README.md is missing".to_string(),
                });
            }

            Err(GithubProjectError::GithubRequestError(error)) => {
                return Err(error);
            }

            Err(error) => {
                report.diagnostics.push(GithubProjectDiagnostic {
                    repository: repository.full_name.clone(),
                    message: error.to_string(),
                });
            }
        }
    }

    Ok(report)
}

/// Fetches repositories from GitHub, filters Rust projects,
/// and loads their metadata.
pub fn discover_github_projects(
    client: &GithubClient,
    username: &str,
) -> Result<GithubDiscoveryReport, GithubError> {
    let repositories = client.fetch_all_repositories(username)?;

    let rust_repositories = filter_rust_repositories(repositories);

    collect_github_projects(rust_repositories, |repository| {
        load_github_project(client, repository)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_repository(
        name: &str,
        language: Option<&str>,
        private: bool,
        fork: bool,
    ) -> GithubRepository {
        GithubRepository {
            name: name.to_string(),
            full_name: format!("example/{name}"),
            html_url: format!("https://github.com/example/{name}"),
            description: None,
            language: language.map(str::to_string),
            private,
            fork,
        }
    }

    fn make_project(name: &str) -> Project {
        Project {
            name: name.to_string(),
            description: None,
            version: "0.1.0".to_string(),
            repository: Some(format!("https://github.com/example/{name}")),
            category: Some("Systems Programming".to_string()),
            featured: false,
            technologies: vec![],
            readme: format!("# {name}"),
        }
    }

    #[test]
    fn filters_only_public_non_fork_rust_repositories() {
        let repositories = vec![
            make_repository("rust-shell", Some("Rust"), false, false),
            make_repository("website", Some("TypeScript"), false, false),
            make_repository("private-tool", Some("Rust"), true, false),
            make_repository("forked-project", Some("Rust"), false, true),
            make_repository("rustfolio", Some("Rust"), false, false),
        ];

        let filtered = filter_rust_repositories(repositories);

        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].name, "rust-shell");
        assert_eq!(filtered[1].name, "rustfolio");
    }

    #[test]
    fn collecting_empty_repositories_returns_empty_report() {
        let result = collect_github_projects(vec![], |_repository| {
            panic!("Loader should not be called");
        });

        let report = result.expect("Discovery should succeed");

        assert!(report.projects.is_empty());
        assert!(report.diagnostics.is_empty());
    }

    #[test]
    fn collects_successfully_loaded_projects() {
        let repositories = vec![
            make_repository("rust-shell", Some("Rust"), false, false),
            make_repository("rustfolio", Some("Rust"), false, false),
        ];

        let result = collect_github_projects(repositories, |repository| {
            Ok(Some(make_project(&repository.name)))
        });

        let report = result.expect("Discovery should succeed");

        assert_eq!(report.projects.len(), 2);
        assert!(report.diagnostics.is_empty());

        assert_eq!(report.projects[0].name, "rust-shell");
        assert_eq!(report.projects[1].name, "rustfolio");
    }

    #[test]
    fn missing_project_files_create_diagnostic() {
        let repositories = vec![
            make_repository("incomplete-project", Some("Rust"), false, false),
            make_repository("rustfolio", Some("Rust"), false, false),
        ];

        let result = collect_github_projects(repositories, |repository| {
            if repository.name == "incomplete-project" {
                Ok(None)
            } else {
                Ok(Some(make_project(&repository.name)))
            }
        });

        let report = result.expect("Discovery should continue");

        assert_eq!(report.projects.len(), 1);
        assert_eq!(report.projects[0].name, "rustfolio");

        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(
            report.diagnostics[0].repository,
            "example/incomplete-project"
        );
        assert!(report.diagnostics[0].message.contains("missing"));
    }

    #[test]
    fn invalid_repository_metadata_creates_diagnostic() {
        let repositories = vec![
            make_repository("invalid-project", Some("Rust"), false, false),
            make_repository("rustfolio", Some("Rust"), false, false),
        ];

        let result = collect_github_projects(repositories, |repository| {
            if repository.name == "invalid-project" {
                Err(GithubProjectError::InvalidRepositoryName(
                    repository.full_name.clone(),
                ))
            } else {
                Ok(Some(make_project(&repository.name)))
            }
        });

        let report = result.expect("Discovery should continue");

        assert_eq!(report.projects.len(), 1);
        assert_eq!(report.projects[0].name, "rustfolio");

        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(report.diagnostics[0].repository, "example/invalid-project");
    }

    #[test]
    fn github_request_failure_stops_discovery() {
        let repositories = vec![
            make_repository("rust-shell", Some("Rust"), false, false),
            make_repository("rustfolio", Some("Rust"), false, false),
        ];

        let mut calls = 0;

        let result = collect_github_projects(repositories, |_repository| {
            calls += 1;

            Err(GithubProjectError::GithubRequestError(
                GithubError::RateLimited,
            ))
        });

        assert!(matches!(result, Err(GithubError::RateLimited)));

        assert_eq!(calls, 1);
    }
}
