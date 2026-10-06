use crate::model::portfolio::Portfolio;

use super::diagnostics::render_diagnostics;
use super::projects::render_project;
use super::utils::escape_html;
use super::hero::render_hero;
use super::experience::render_experience;

pub fn render_portfolio(portfolio: &Portfolio) -> String {
    let name = escape_html(&portfolio.profile.name);

    let mut projects_html = String::new();

    for project in &portfolio.projects {
        let html = render_project(project);
        projects_html.push_str(&html);
    }
    let diagnostic_html = render_diagnostics(&portfolio);

    let hero_html = render_hero(&portfolio.profile);
    let experience_html = render_experience(&portfolio.experience);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="color-scheme" content="light dark">
    <link rel="stylesheet" href="assets/style.css">

    <title>{name}</title>
</head>

<body>
    <div class="page">
        {hero_html}

        <section class="projects" id="projects">
            <div class="section-header">
                <p class="section-label">Selected work</p>
                <h2>Projects</h2>
            </div>

            <div class="project-grid">
                {projects_html}
            </div>

            {diagnostic_html}
        </section>
        {experience_html}
    </div>
</body>
</html>"#,
    )
}
