use std::path:: PathBuf;
use crate::generator::html::{render_portfolio, render_project};
use crate::generator::page::GeneratedPage;
use crate::model::portfolio::Portfolio;

pub fn generate_site(portfolio: &Portfolio)->Vec<GeneratedPage>{

    let mut pages = Vec::new();

    let _portfolio =   render_portfolio(portfolio);

   let generated_portfolio = GeneratedPage {
        path : PathBuf::from("index.html"),
        content : _portfolio
    };
    pages.push(generated_portfolio);

    for project in &portfolio.projects{
        for project in &portfolio.projects {
            pages.push(GeneratedPage {
                path: PathBuf::from("projects")
                    .join(&project.name)
                    .join("index.html"),
                content: render_project(project),
            });
        }
    }
    pages


}