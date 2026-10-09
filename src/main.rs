mod model;
mod scanner;
mod service;
mod config;
mod generator;

use std::fmt:: Formatter;
use std::io::Error;
use crate::service::project::load_projects;
use crate::config::loader::{load_config, ConfigError};
use std::path::Path;
use crate::service::portfolio::build_portfolio;
use crate::generator::site::generate_site;
use crate::generator::writer::write_site;


fn main() {

    match run() {
        Ok(_)=>{},
        Err(error)=>eprintln!("{}",error)
    }
}





#[derive(Debug)]
enum AppError{
    Config(ConfigError),
    Io(std::io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Config(error)=> write!(f,"{}",error),
            AppError::Io(error)=>write!(f,"{}",error),
        }
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
impl std::error::Error for AppError {}


fn run ()->Result<(), AppError>{
    let config =   load_config(Path::new("rustfolio.toml"))?;
    let report = load_projects(&config.projects);

    let build = build_portfolio(config, report);

    for diagnostic in &build.report.diagnostics{
        eprintln!(
            "Build warning [{}]: {}\n  {}",
            diagnostic.source,
            diagnostic.message,
            diagnostic.detail
        );    }

    let pages = generate_site(&build.portfolio);
    write_site(Path::new("dist"), &pages)?;

    Ok(())
}
