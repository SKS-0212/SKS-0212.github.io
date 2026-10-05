use crate::model::portfolio::Portfolio;
use crate::model::project::Project;
use pulldown_cmark::{html, HeadingLevel, Parser, Event, Tag, TagEnd};

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



fn render_project(project: &Project) -> String {
    let card_class = if project.featured {
        "project-card featured"
    } else {
        "project-card"
    };
    let name = escape_html(&project.name);

    let featured_html = if project.featured {
        r#"<span class="featured-badge">Featured</span>"#.to_string()
    } else {
        String::new()
    };
    let version  = escape_html(&project.version);
    let version_html =  format!(
        r#"<span class="project-badge">v{}</span>"#,
        version
    );
    let category_html = match &project.category {
        Some(category) => {
            let category = escape_html(category);
            format!(
                r#"<span class="project-badge">{}</span>"#,
                category
            )
        }
        None => String::new(),
    };

    let description_html = match &project.description{
        Some(description)=>{
            let description = escape_html(description);
            format!(
                r#"<p class="project-description">{}</p>"#,
                description
            )
        },
        None => String::new(),
    };

    let repository_html = match &project.repository {
        Some(repository)=>{
            let repository = escape_html(repository);
            format!(
                r#"
                <div class="project-actions">
                    <a href="{}" target="_blank" rel="noopener noreferrer">
                         View Repository →
                    </a>
                </div>"#,
                repository
            )
        },
        None => String::new()
    };

    let readme_html = render_markdown(&project.readme);

    format!(
        r#"<article class="{}">
    <div class="project-header">
        <div class="project-heading">
            <h3 class="project-title">{}</h3>
            {}
        </div>

        <div class="project-meta">
            {}
            {}
        </div>
    </div>

    {}

    {}

    <div class="project-content">
        {}
    </div>
</article>"#,
        card_class,
        name,
        featured_html,
        category_html,
        version_html,
        description_html,
        repository_html,
        readme_html,
    )
}

fn render_markdown(markdown: &str)->String{
    let mut html_output = String::new();
    let parser = Parser::new(markdown);

    let transformed = parser.map(|event| {
        match event {
            Event::Start(Tag::Heading {
                             level, id, classes, attrs })
            => Event::Start(Tag::Heading {
                level: shift_heading_level(level), id, classes, attrs, }),
            Event::End(TagEnd::Heading(level)) => {
                Event::End(TagEnd::Heading(shift_heading_level(level)))
            }
            event => event,
        }
    });

    html::push_html(&mut html_output, transformed);
    html_output
}

fn render_diagnostics(portfolio: &Portfolio) -> String {
    if portfolio.diagnostics.is_empty() {
        return String::new();
    }

    let mut items = String::new();

    for diagnostic in &portfolio.diagnostics {
        let project = escape_html(&diagnostic.project);
        let message = escape_html(&diagnostic.message);

        items.push_str(
            &format!(
                r#"<div class="diagnostic">
    <strong>{}</strong>
    <span>{}</span>
</div>"#,
                project,
                message,
            )
        );
    }

    format!(
        r#"<section class="diagnostics">
    <p class="diagnostics-label">Build notices</p>
    {}
</section>"#,
        items
    )
}

fn shift_heading_level(level: HeadingLevel)-> HeadingLevel{
    match level {
        HeadingLevel::H1 => HeadingLevel::H4,
        HeadingLevel::H2 => HeadingLevel::H5,
        HeadingLevel::H3 => HeadingLevel::H6,

        HeadingLevel::H4 | HeadingLevel::H5 | HeadingLevel::H6 => HeadingLevel::H6
    }
}

fn escape_html(input: &str) ->String{
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")

}