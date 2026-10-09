use std::path:: PathBuf;
use crate::generator::html::{render_portfolio, render_project};
use crate::generator::page::GeneratedPage;
use crate::generator::paths::SitePaths;
use crate::model::portfolio::Portfolio;

pub fn generate_site(portfolio: &Portfolio)->Vec<GeneratedPage>{

    let mut pages = Vec::new();

    let _portfolio =   render_portfolio(portfolio);

   let generated_portfolio = GeneratedPage {
        path : PathBuf::from("index.html"),
        content : _portfolio
    };
    pages.push(generated_portfolio);


        for project in &portfolio.projects {
            let page_path = PathBuf::from("projects")
                .join(&project.name)
                .join("index.html");

            let paths = SitePaths::from_page_path(&page_path);

            pages.push(GeneratedPage {
                path: page_path,
                content: render_project(project, &paths),
            });
        }

    pages


}