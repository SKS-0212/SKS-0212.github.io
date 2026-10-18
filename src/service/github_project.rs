use crate::github::client::GithubClient;
use crate::github::error::GithubError;
use crate::github::model::GithubRepository;
use crate::model::project::Project;
use crate::scanner::cargo::CargoScanError;
use crate::scanner::cargo::parse_cargo;
use crate::scanner::readme::ReadmeScanError;
use crate::scanner::readme::parse_readme;

#[derive(Debug)]
pub enum GithubProjectError {
    GithubRequestError(GithubError),
    CargoParsingError(CargoScanError),
    ReadmeParsingError(ReadmeScanError),
    InvalidRepositoryName(String),
}

impl From<GithubError> for GithubProjectError {
    fn from(value: GithubError) -> Self {
        GithubProjectError::GithubRequestError(value)
    }
}
impl From<CargoScanError> for GithubProjectError {
    fn from(value: CargoScanError) -> Self {
        GithubProjectError::CargoParsingError(value)
    }
}

impl From<ReadmeScanError> for GithubProjectError {
    fn from(value: ReadmeScanError) -> Self {
        GithubProjectError::ReadmeParsingError(value)
    }
}

impl std::fmt::Display for GithubProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GithubRequestError(error) => write!(f, "GitHub request failed: {error}"),
            Self::CargoParsingError(error) => write!(f, "Cargo parsing failed: {error}"),
            Self::ReadmeParsingError(error) => write!(f, "README parsing failed: {error}"),
            Self::InvalidRepositoryName(name) => {
                write!(f, "Invalid GitHub repository name: {name}")
            }
        }
    }
}

impl std::error::Error for GithubProjectError {}

pub fn load_github_project(
    client: &GithubClient,
    repository: &GithubRepository,
) -> Result<Option<Project>, GithubProjectError> {
    let (owner, repo) = repository
        .full_name
        .split_once('/')
        .filter(|(owner, repo)| !owner.is_empty() && !repo.is_empty() && !repo.contains('/'))
        .ok_or_else(|| GithubProjectError::InvalidRepositoryName(repository.full_name.clone()))?;

    let cargo_content = client.fetch_file(owner, repo, "Cargo.toml")?;

    let Some(cargo_content) = cargo_content else {
        return Ok(None);
    };

    let readme_content = client.fetch_file(owner, repo, "README.md")?;

    let Some(readme_content) = readme_content else {
        return Ok(None);
    };

    let project = build_github_project(repository, &cargo_content, &readme_content)?;

    Ok(Some(project))
}

pub fn build_github_project(
    repository: &GithubRepository,
    cargo_content: &str,
    readme_content: &str,
) -> Result<Project, GithubProjectError> {
    let cargo = parse_cargo(cargo_content)?;
    let readme = parse_readme(readme_content)?;

    let project = Project {
        name: cargo.package.name,
        description: cargo.package.description,
        version: cargo.package.version,
        repository: Some(repository.html_url.clone()),
        category: readme.metadata.category,
        featured: readme.metadata.featured,
        technologies: vec![],
        readme: readme.content,
    };

    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_project_from_github_content() {
        // Arrange: Create sample GitHub repository metadata
        let repository = GithubRepository {
            name: "rust-shell".to_string(),
            full_name: "example/rust-shell".to_string(),
            html_url: "https://github.com/example/rust-shell".to_string(),
            description: Some("A Rust shell project".to_string()),
            language: Some("Rust".to_string()),
            private: false,
            fork: false,
        };

        // Arrange: Sample Cargo.toml
        let cargo_content = r#"
[package]
name = "rust-shell"
version = "0.1.0"
edition = "2024"
description = "A Unix-like shell written in Rust"
"#;

        // Arrange: Sample README.md
        let readme_content = r#"---
category: Systems Programming
featured: true
---
# Rust Shell

A shell built with Rust.
"#;

        // Act: Build a Project from the provided content
        let project = build_github_project(&repository, cargo_content, readme_content)
            .expect("Project should be constructed");

        // Assert: Verify the generated Project
        assert_eq!(project.name, "rust-shell");
        assert_eq!(project.version, "0.1.0");

        assert_eq!(
            project.description.as_deref(),
            Some("A Unix-like shell written in Rust")
        );

        assert_eq!(project.category.as_deref(), Some("Systems Programming"));

        assert!(project.featured);

        assert_eq!(
            project.repository.as_deref(),
            Some("https://github.com/example/rust-shell")
        );

        assert!(project.readme.contains("# Rust Shell"));
    }

    #[test]
    fn rejects_invalid_github_cargo() {
        let repository = GithubRepository {
            name: "rust-shell".to_string(),
            full_name: "example/rust-shell".to_string(),
            html_url: "https://github.com/example/rust-shell".to_string(),
            description: None,
            language: Some("Rust".to_string()),
            private: false,
            fork: false,
        };

        let cargo_content = r#"
[package
name = "rust-shell"
version = "0.1.0"
"#;

        let readme_content = r#"---
category: Systems Programming
featured: true
---
# Rust Shell
"#;

        let result =
            build_github_project(&repository, cargo_content, readme_content);

        assert!(matches!(
        result,
        Err(GithubProjectError::CargoParsingError(_))
    ));
    }

    #[test]
    fn rejects_invalid_github_readme() {
        let repository = GithubRepository {
            name: "rust-shell".to_string(),
            full_name: "example/rust-shell".to_string(),
            html_url: "https://github.com/example/rust-shell".to_string(),
            description: None,
            language: Some("Rust".to_string()),
            private: false,
            fork: false,
        };

        let cargo_content = r#"
[package]
name = "rust-shell"
version = "0.1.0"
edition = "2024"
"#;

        // Invalid README: opening frontmatter without a closing delimiter.
        let readme_content = r#"---
category: Systems Programming
featured: true
"#;

        let result =
            build_github_project(&repository, cargo_content, readme_content);

        assert!(matches!(
        result,
        Err(GithubProjectError::ReadmeParsingError(
            ReadmeScanError::MissingClosingDelimiter
        ))
    ));
    }



    #[test]
    fn builds_github_project_without_readme_frontmatter() {
        let repository = GithubRepository {
            name: "hexview".to_string(),
            full_name: "example/hexview".to_string(),
            html_url: "https://github.com/example/hexview".to_string(),
            description: Some("A hexadecimal file viewer".to_string()),
            language: Some("Rust".to_string()),
            private: false,
            fork: false,
        };

        let cargo_content = r#"
[package]
name = "hexview"
version = "0.1.0"
edition = "2024"
description = "A hexadecimal file viewer"
"#;

        let readme_content = "# Hexview\n\nA hexadecimal file viewer written in Rust.";

        let result = build_github_project(
            &repository,
            cargo_content,
            readme_content,
        )
            .expect("GitHub project should load without README frontmatter");

        assert_eq!(result.name, "hexview");
        assert_eq!(result.version, "0.1.0");
        assert_eq!(result.readme, readme_content);
        assert_eq!(result.category, None);
        assert!(!result.featured);
        assert!(result.technologies.is_empty());

        assert_eq!(
            result.repository.as_deref(),
            Some("https://github.com/example/hexview")
        );
    }
}
