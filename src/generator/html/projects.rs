use crate::model::project::Project;

use super::utils::{
    escape_html,
    render_markdown,
};

pub(super) fn render_project(project: &Project) -> String {
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
