use super::build::{BuildDiagnostic, BuildReport, PortfolioBuild};
use crate::config::model::{ExperienceConfig, LinkConfig, RustfolioConfig};
use crate::generator::slug::slugify;
use crate::model::experience::Experience;
use crate::model::link::Link;
use crate::model::portfolio::Portfolio;
use crate::model::profile::Profile;
use crate::model::project::Project;
use crate::service::github_discovery::GithubDiscoveryReport;
use crate::service::project::ProjectLoadReport;
use crate::service::project::{ProjectFailure, ProjectLoadError};
use std::collections::HashSet;

impl From<LinkConfig> for Link {
    fn from(link_config: LinkConfig) -> Self {
        Self {
            label: link_config.label,
            url: link_config.url,
        }
    }
}

impl From<ExperienceConfig> for Experience {
    fn from(config: ExperienceConfig) -> Self {
        Self {
            company: config.company,
            role: config.role,
            start: config.start,
            end: config.end,
            description: config.description,
        }
    }
}

pub fn build_portfolio(
    config: RustfolioConfig,
    project_report: ProjectLoadReport,
) -> PortfolioBuild {
    let links = config.profile.links.into_iter().map(Link::from).collect();

    let profile = Profile {
        name: config.profile.name,
        headline: config.profile.headline,
        bio: config.profile.bio,
        location: config.profile.location,
        links,
        skills: config.profile.skills,
    };

    let mut diagnostics = Vec::new();

    for failure in project_report.failures {
        let project_name = failure
            .path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown project".to_string());

        let diagnostic = BuildDiagnostic {
            source: project_name,
            message: "Project information could not be loaded.".to_string(),
            detail: failure.error.to_string(),
        };

        diagnostics.push(diagnostic);
    }

    let experience = config
        .experience
        .into_iter()
        .map(Experience::from)
        .collect();

    let portfolio = Portfolio {
        profile,
        projects: project_report.projects,
        experience,
    };

    let report = BuildReport { diagnostics };

    PortfolioBuild { portfolio, report }
}

pub fn merge_projects(local: Vec<Project>, remote: Vec<Project>) -> Vec<Project> {
    let mut seen = HashSet::new();
    let mut merged = Vec::new();

    // Local projects always take priority.
    for project in local {
        let slug = slugify(&project.name);

        seen.insert(slug);
        merged.push(project);
    }

    // Only add remote projects with new slugs.
    for project in remote {
        let slug = slugify(&project.name);

        if seen.insert(slug) {
            merged.push(project);
        }
    }

    merged
}

pub fn combine_project_reports(
    local: ProjectLoadReport,
    github: GithubDiscoveryReport,
) -> (ProjectLoadReport, Vec<BuildDiagnostic>) {
    let projects = merge_projects(local.projects, github.projects);

    let diagnostics: Vec<BuildDiagnostic> = github
        .diagnostics
        .into_iter()
        .map(|diagnostic| BuildDiagnostic {
            source: diagnostic.repository,
            message: "GitHub project could not be loaded.".to_string(),
            detail: diagnostic.message,
        })
        .collect();

    let combined = ProjectLoadReport {
        projects,
        failures: local.failures,
    };

    (combined, diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_project(name: &str) -> Project {
        Project {
            name: name.to_string(),
            description: None,
            version: "0.1.0".to_string(),
            repository: None,
            category: None,
            featured: false,
            technologies: vec![],
            readme: String::new(),
        }
    }

    #[test]
    fn merges_unique_local_and_remote_projects() {
        let local = vec![make_project("rsh"), make_project("hexview")];

        let remote = vec![make_project("rustfolio")];

        let merged = merge_projects(local, remote);

        assert_eq!(merged.len(), 3);
        assert_eq!(merged[0].name, "rsh");
        assert_eq!(merged[1].name, "hexview");
        assert_eq!(merged[2].name, "rustfolio");
    }

    #[test]
    fn local_project_takes_priority_over_remote() {
        let mut local_project = make_project("rsh");
        local_project.description = Some("Local version".to_string());

        let mut remote_project = make_project("rsh");
        remote_project.description = Some("GitHub version".to_string());

        let merged = merge_projects(vec![local_project], vec![remote_project]);

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].description.as_deref(), Some("Local version"));
    }

    #[test]
    fn detects_duplicates_using_normalized_slugs() {
        let local = vec![make_project("Rust Shell")];

        let remote = vec![make_project("rust-shell")];

        let merged = merge_projects(local, remote);

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].name, "Rust Shell");
    }

    #[test]
    fn preserves_duplicate_local_projects() {
        let local = vec![make_project("Rust Shell"), make_project("rust-shell")];

        let merged = merge_projects(local, vec![]);

        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn combines_local_and_github_project_reports() {
        let local = ProjectLoadReport {
            projects: vec![make_project("rsh"), make_project("hexview")],
            failures: vec![],
        };

        let github = GithubDiscoveryReport {
            projects: vec![make_project("rsh"), make_project("rustfolio")],
            diagnostics: vec![],
        };

        let (combined, diagnostics) = combine_project_reports(local, github);

        assert_eq!(combined.projects.len(), 3);

        assert_eq!(combined.projects[0].name, "rsh");
        assert_eq!(combined.projects[1].name, "hexview");
        assert_eq!(combined.projects[2].name, "rustfolio");

        assert!(combined.failures.is_empty());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn preserves_github_diagnostics() {
        use crate::service::github_discovery::GithubProjectDiagnostic;

        let local = ProjectLoadReport {
            projects: vec![],
            failures: vec![],
        };

        let github = GithubDiscoveryReport {
            projects: vec![],
            diagnostics: vec![GithubProjectDiagnostic {
                repository: "developer/broken-project".to_string(),
                message: "Invalid Cargo.toml".to_string(),
            }],
        };

        let (combined, diagnostics) = combine_project_reports(local, github);

        assert!(combined.projects.is_empty());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].source, "developer/broken-project");
        assert_eq!(diagnostics[0].detail, "Invalid Cargo.toml");
    }

    #[test]
    fn preserves_local_project_failures() {
        use crate::scanner::cargo::CargoScanError;
        use crate::service::project::{ProjectFailure, ProjectLoadError};
        use std::io::{Error, ErrorKind};
        use std::path::PathBuf;

        let local = ProjectLoadReport {
            projects: vec![make_project("rsh")],
            failures: vec![ProjectFailure {
                path: PathBuf::from("../broken-project"),
                error: ProjectLoadError::Cargo(CargoScanError::Io(Error::new(
                    ErrorKind::NotFound,
                    "Cargo.toml not found",
                ))),
            }],
        };

        let github = GithubDiscoveryReport {
            projects: vec![make_project("rustfolio")],
            diagnostics: vec![],
        };

        let (combined, diagnostics) = combine_project_reports(local, github);

        assert_eq!(combined.projects.len(), 2);
        assert_eq!(combined.projects[0].name, "rsh");
        assert_eq!(combined.projects[1].name, "rustfolio");

        assert_eq!(combined.failures.len(), 1);
        assert_eq!(
            combined.failures[0].path,
            PathBuf::from("../broken-project")
        );

        assert!(diagnostics.is_empty());
    }
}
