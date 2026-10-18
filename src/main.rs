extern crate core;

mod config;
mod generator;
mod github;
mod model;
mod scanner;
mod service;

use crate::config::loader::{ConfigError, load_config};
use crate::generator::error::SiteGenerationError;
use crate::generator::site::generate_site;
use crate::generator::writer::write_site;
use crate::github::client::GithubClient;
use crate::github::error::GithubError;
use crate::service::github_discovery::discover_github_projects;
use crate::service::portfolio::{build_portfolio, combine_project_reports};
use crate::service::project::load_projects;
use std::fmt::Formatter;
use std::io::Error;
use std::path::Path;

fn main() {
    if let Err(error) = run() {
        eprintln!("Rustfolio build failed: {}", error);
        std::process::exit(1);
    }
}

#[derive(Debug)]
enum AppError {
    Config(ConfigError),
    Io(std::io::Error),
    SiteGenerationError(SiteGenerationError),
    Github(GithubError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Config(error) => write!(f, "{}", error),
            AppError::Io(error) => write!(f, "{}", error),
            AppError::SiteGenerationError(error) => write!(f, "{}", error),
            AppError::Github(error) => write!(f, "GitHub error: {}", error),
        }
    }
}

impl From<SiteGenerationError> for AppError {
    fn from(value: SiteGenerationError) -> Self {
        AppError::SiteGenerationError(value)
    }
}

impl From<ConfigError> for AppError {
    fn from(value: ConfigError) -> Self {
        AppError::Config(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Io(value)
    }
}

impl From<GithubError> for AppError {
    fn from(error: GithubError) -> Self {
        AppError::Github(error)
    }
}
impl std::error::Error for AppError {}

fn run() -> Result<(), AppError> {
    let config = load_config(Path::new("rustfolio.toml"))?;

    // Step 1: Load local projects.
    let local_report = load_projects(&config.projects);

    // Step 2: Start with local projects only.
    let mut report = local_report;
    let mut github_diagnostics = Vec::new();

    // Step 3: Discover GitHub projects when configured.
    if let Some(github_config) = &config.github {
        let client = GithubClient::new();

        let github_report = discover_github_projects(&client, &github_config.username)?;

        let (combined, diagnostics) = combine_project_reports(report, github_report);

        report = combined;
        github_diagnostics = diagnostics;
    }

    // Step 4: Build the portfolio.
    let mut build = build_portfolio(config, report);

    // Step 5: Preserve GitHub diagnostics.
    build.report.diagnostics.extend(github_diagnostics);

    // Step 6: Display warnings.
    for diagnostic in &build.report.diagnostics {
        eprintln!(
            "Build warning [{}]: {}\n  {}",
            diagnostic.source, diagnostic.message, diagnostic.detail
        );
    }

    // Step 7: Generate and write the website.
    let pages = generate_site(&build.portfolio)?;
    write_site(Path::new("dist"), &pages)?;

    Ok(())
}
