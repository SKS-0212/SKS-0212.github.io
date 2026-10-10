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

#[cfg(test)]
mod tests{
    use crate::model::profile::Profile;
    use crate::model::project::Project;
    use super::*;

    fn sample_portfolio()-> Portfolio{

        let profile = Profile{
            name: String::from("Sachin Kumar Singh"),
            links: Vec::new(),
            bio: String::new(),
            skills: Vec::new(),
            headline: String::new(),
            location: None
        };

        Portfolio{
            projects : Vec::new(),
            profile,
            experience: Vec::new(),

        }
    }

    fn sample_project(name: &str)->Project{
        Project{
            name:name.to_string(),
            description: None,
            version: String::new(),
            repository: None,
            category: None,
            featured: false,
            technologies: Vec::new(),
            readme: String::new(),
        }
    }

    #[test]
    fn empty_portfolio_generation_homepage(){
        let portfolio = sample_portfolio();

        let pages = generate_site(&portfolio);

        assert_eq!(pages.len(), 1);

        assert_eq!(
            pages[0].path,
            PathBuf::from("index.html")
        );
    }

    #[test]
    fn multiple_projects_generate_multiple_pages(){
        let mut portfolio = sample_portfolio();

        portfolio.projects.push(sample_project("rsh"));
        portfolio.projects.push(sample_project("hexview"));

        let pages = generate_site(&portfolio);

        assert_eq!(pages.len(),3);
        assert_eq!(pages[0].path, PathBuf::from("index.html"));
        assert_eq!(pages[1].path, PathBuf::from("projects/rsh/index.html"));
        assert_eq!(pages[2].path, PathBuf::from("projects/hexview/index.html"));


    }

    #[test]
    fn home_page_contains_profile_name(){
        let portfolio = sample_portfolio();
        let page = generate_site(&portfolio);
        let homepage =&page[0];

        assert!(
            homepage.content.contains("Sachin Kumar Singh")
        );
    }

    #[test]
    fn project_page_uses_correct_assert_paths(){
        let mut portfolio = sample_portfolio();
        portfolio.projects.push(sample_project("rsh"));
        let pages = generate_site(&portfolio);
        let project_page = &pages[1];
        println!("{}", project_page.content);
        assert!(
            project_page.content.contains("../../assets/style.css")
        );
        assert!(
            project_page.content.contains("../../assets/main.js")
        );
    }

    #[test]
    fn project_page_contains_project_name() {
        let mut portfolio = sample_portfolio();

        portfolio.projects.push(sample_project("rsh"));

        let pages = generate_site(&portfolio);
        let project_page = &pages[1];

        // Verify the project name appears in the generated HTML.
        assert!(
            project_page.content.contains("rsh"),
            "Project page does not contain the project name"
        );

        // Verify the page contains a main heading.
        assert!(
            project_page.content.contains("<h1"),
            "Project page does not contain an H1 heading"
        );
    }


}