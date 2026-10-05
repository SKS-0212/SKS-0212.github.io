use crate::config::model::RustfolioConfig;
use crate::model::portfolio::{Portfolio, ProjectDiagnostic};
use crate::model::profile::Profile;
use crate::service::project::ProjectLoadReport;

pub fn build_portfolio(
    config: RustfolioConfig,
    report: ProjectLoadReport
) -> Portfolio{

    let profile = Profile{
        name: config.profile.name,
        headline: config.profile.headline,
        bio: config.profile.bio,
    };

    let mut diagnostics = Vec::new();

    for failure in report.failures{

        let project_name = failure.path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown project".to_string());

            eprintln!(
                "Failed to load project '{}': {}", project_name,failure.error
            );

        let diagnostic = ProjectDiagnostic{
            project: project_name,
            message: "Project information could not be loaded.".to_string(),
        };

        diagnostics.push(diagnostic);
    }

    Portfolio {
        profile,
        projects: report.projects,
        diagnostics
    }
}