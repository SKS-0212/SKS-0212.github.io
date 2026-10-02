mod model;
mod scanner;
mod service;
mod config;

use std::io;
use crate::service::project::{load_project, load_projects};
use crate::scanner::discovery::discover_projects;
use crate::config::loader::load_config;
use std::path::Path;
use crate::service::portfolio::build_portfolio;
fn main() {

  let config =  match load_config(Path::new("rustfolio.toml")) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}",error);
            return;
        },
    };
    let report = load_projects(&config.projects);

    let portfolio = build_portfolio(config,report);

    println!("{:#?}",portfolio);


}
