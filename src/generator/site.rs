use std::path:: PathBuf;
use crate::generator::html::render_portfolio;
use crate::generator::page::GeneratedPage;
use crate::model::portfolio::Portfolio;

pub fn generate_site(portfolio: &Portfolio)->Vec<GeneratedPage>{

    let mut pages = Vec::new();

    let portfolio =   render_portfolio(portfolio);

   let generated_portfolio = GeneratedPage {
        path : PathBuf::from("index.html"),
        content : portfolio
    };

    pages.push(generated_portfolio);

    pages


}