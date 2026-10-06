use crate::model::project::Project;

use super::utils::escape_html;


pub(super) fn render_projects(projects: &[Project])->String{
    if projects.is_empty(){
        return  String::new();
    }

    let mut projects_html = String::new();

    for project in projects{
        let formatted_project = render_project(project);
        projects_html.push_str(&formatted_project)
    }
    format!(
        r#"
<section class="projects" id="projects">
    <div class="section-header">
        <p class="section-label">Selected work</p>
        <h2>Projects</h2>
    </div>

    <div class="project-grid">
        {projects_html}
    </div>
</section>
"#
    )
}

 fn render_project(project: &Project) -> String {

    let name = escape_html(&project.name);


    let version  = escape_html(&project.version);

    let category_html = render_category(project);

    let description_html =render_description(project);

    let technologies_html = render_technologies(project);

    let repository_html = render_repository(project);


    format!(
        r#"
<article class="project-item">

    <div class="project-top">
        <div class="project-identity">
            {category_html}

            <h3 class="project-title">{name}</h3>
        </div>

        <span class="project-arrow">↗</span>
    </div>

    {description_html}

    <div class="project-footer">
        <div class="project-technologies">
            {technologies_html}
        </div>

        {repository_html}
    </div>

</article>
"#
    )
}


fn render_technologies(project: &Project)->String{
    let mut technologies_html = String::new();

    for technology in &project.technologies{
        let escaped_technology = escape_html(technology);

        let formated_technology= format!(
            r#"<span class="project-technology">{}</span>"#,
            escaped_technology);

        technologies_html.push_str(&formated_technology);
    }

    technologies_html
}

fn render_category(project: &Project)->String{
    match &project.category {
        Some(category) => {
            let escaped_category = escape_html(category);

            format!(
                r#"<span class="project-category">{}</span>"#,
                escaped_category
            )
        }
        None=>String::new()
    }
}

fn render_description(project: &Project)->String{
    match &project.description {
        Some(description) => {
            let escaped_description = escape_html(description);

            format!(
                r#"<p class="project-description">{}</p>"#,
                escaped_description
            )
        }
        None=>String::new()
    }
}

fn render_repository(project: &Project)->String{
    match &project.repository {
        Some(repository) => {
            let escaped_repository = escape_html(repository);

            format!(
                r#"<a
            class="project-repository"
            href="{}"
            target="_blank"
            rel="noopener noreferrer">
            Repository ↗
        </a>"#,
                escaped_repository
            )
        }
        None=>String::new()
    }
}