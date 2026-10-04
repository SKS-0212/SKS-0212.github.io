use crate::model::portfolio::Portfolio;
use crate::model::project::Project;
use pulldown_cmark::{Parser,html};

pub fn render_portfolio(portfolio: &Portfolio) -> String {
    let name = escape_html(&portfolio.profile.name);
    let headline = escape_html(&portfolio.profile.headline);
    let bio = escape_html(&portfolio.profile.bio);

    let mut project_html = String::new();
    for project in &portfolio.projects{
        let html =  render_project(project);
        project_html.push_str(&html);
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="assets/style.css">
    <title>{}</title>
</head>
<body>
    <main>
        <h1>{}</h1>
        <h2>{}</h2>
        <p>{}</p>
    </main>
    <section>{}</section>
</body>
</html>"#,
        name,
        name,
        headline,
        bio,
        project_html
    )
}



fn render_project(project: &Project) -> String {
    let name = escape_html(&project.name);
    let version  = escape_html(&project.version);
    let category_html = match &project.category {
        Some(category) => {
            let category = escape_html(category);
            format!("<p>{}</p>", category)
        }
        None => String::new(),
    };

    let description_html = match &project.description{
        Some(description)=>{
            let description = escape_html(description);
            format!("<p>{}</p>",description)
        },
        None => String::new(),
    };

    let repository_html = match &project.repository {
        Some(repository)=>{
            let repository = escape_html(repository);
            format!(
                "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">View Repository</a>",
                repository
            )
        },
        None => String::new()
    };

    let readme_html = render_markdown(&project.readme);

    format!(
        r#"<article>
        <h3>{}</h3>
    <p>{}</p>
        {}
        {}
        {}
    <div>{}</div>
    </article>"#,
        name,
        version,
        category_html,
        description_html,
        repository_html,
        readme_html

    )
}

fn render_markdown(markdown: &str)->String{
    let mut html_output = String::new();
    let parser = Parser::new(markdown);
    html::push_html(&mut html_output, parser);
    html_output
}



fn escape_html(input: &str) ->String{
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")

}