use crate::model::portfolio::Portfolio;

use super::about::render_about;
use super::diagnostics::render_diagnostics;
use super::projects::render_projects;
use super::utils::escape_html;
use super::hero::render_hero;
use super::experience::render_experience;
use super::footer::render_footer;

pub fn render_portfolio(portfolio: &Portfolio) -> String {
    let name = escape_html(&portfolio.profile.name);

    let hero_html = render_hero(&portfolio.profile);

    let projects_html = render_projects(&portfolio.projects);

    let experience_html = render_experience(&portfolio.experience);

    let about_html = render_about(&portfolio.profile);

    let diagnostic_html = render_diagnostics(&portfolio.diagnostics);

    let footer_html = render_footer(&portfolio.profile);

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
        {projects_html}
        {experience_html}
        {about_html}
        {diagnostic_html}
        {footer_html}
    </div>
    <script src="assets/main.js"></script>
</body>
</html>"#,
    )
}
