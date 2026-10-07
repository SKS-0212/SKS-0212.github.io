use crate::config::model::{LinkConfig, RustfolioConfig,ExperienceConfig};
use crate::model::experience::Experience;
use crate::model::link::Link;
use crate::model::portfolio::Portfolio;
use crate::model::profile::Profile;
use crate::service::project::ProjectLoadReport;
use super::build::{
    BuildDiagnostic,
    BuildReport,
    PortfolioBuild,
};

impl From<LinkConfig> for Link{
    fn from(link_config: LinkConfig) -> Self {
        Self{
            label: link_config.label,
            url: link_config.url
        }
    }
}

impl From<ExperienceConfig> for Experience{
    fn from(config: ExperienceConfig) -> Self {
        Self{
            company: config.company,
            role: config.role,
            start: config.start,
            end: config.end,
            description: config.description
        }
    }
}

pub fn build_portfolio(config: RustfolioConfig, project_report: ProjectLoadReport) -> PortfolioBuild{

    let links = config.profile.links
        .into_iter()
        .map(Link::from).collect();

    let profile = Profile{
        name: config.profile.name,
        headline: config.profile.headline,
        bio: config.profile.bio,
        location: config.profile.location,
        links,
        skills: config.profile.skills,
    };

    let mut diagnostics = Vec::new();

    for failure in project_report.failures{

        let project_name = failure.path
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

    let experience = config.experience.into_iter()
        .map(Experience::from).collect();

    let portfolio = Portfolio {
        profile,
        projects: project_report.projects,
        experience
    };

    let report = BuildReport{
        diagnostics
    };

    PortfolioBuild{
        portfolio,
        report
    }
}