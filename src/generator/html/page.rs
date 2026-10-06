use crate::model::portfolio::Portfolio;

use super::diagnostics::render_diagnostics;
use super::projects::render_project;
use super::utils::escape_html;

pub fn render_portfolio(portfolio: &Portfolio) -> String {
    let name = escape_html(&portfolio.profile.name);
    let headline = escape_html(&portfolio.profile.headline);
    let bio = escape_html(&portfolio.profile.bio);

    let mut projects_html = String::new();

    for project in &portfolio.projects {
        let html = render_project(project);
        projects_html.push_str(&html);
    }
    let diagnostic_html = render_diagnostics(&portfolio);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="color-scheme" content="light dark">
    <link rel="stylesheet" href="assets/style.css">

    <title>{}</title>
</head>

<body>
    <div class="page">

        <main class="hero">
            <h1>{}</h1>
            <h2>{}</h2>
            <p>{}</p>
        </main>

        <section class="projects">
            <div class="section-header">
                <p class="section-label">Selected work</p>
                <h2>Projects</h2>
            </div>

            <div class="project-grid">
                {}
            </div>
            {}
        </section>

    </div>
</body>
</html>"#,
        name,
        name,
        headline,
        bio,
        projects_html,
        diagnostic_html
    )
}
