use crate::model::project::Project;

use super::utils::{escape_html, render_markdown};

pub fn render_project(project: &Project) -> String {
    let name = escape_html(&project.name);
    let version = escape_html(&project.version);

    let featured_html = if project.featured {
        r#"<span class="project-featured">Featured</span>"#.to_string()
    } else {
        String::new()
    };

    let description_html = project
        .description
        .as_deref()
        .map(escape_html)
        .unwrap_or_default();

    let repository_html = project
        .repository
        .as_deref()
        .map(|url| {
            let url = escape_html(url);
            format!(
                r#"<a class="hero-cta" href="{url}" target="_blank" rel="noopener noreferrer">View repository ↗</a>"#
            )
        })
        .unwrap_or_default();

    let category_html = project
        .category
        .as_deref()
        .map(|category| {
            format!(
                r#"<span class="project-category">{}</span>"#,
                escape_html(category)
            )
        })
        .unwrap_or_default();

    let technologies_html = project
        .technologies
        .iter()
        .map(|technology| {
            format!(
                r#"<span class="project-technology">{}</span>"#,
                escape_html(technology)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let readme_html = render_markdown(&project.readme);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="color-scheme" content="light dark">

    <link rel="stylesheet" href="../../assets/style.css">

    <title>{name} — Project</title>
</head>
<body>
    <div class="page project-page">

        <header class="site-header">
            <a class="brand" href="../../"
               onclick="window.location.replace(this.href); return false;">
                Rustfolio
            </a>

            <nav class="site-nav">
                <a
                    class="nav-link"
                    href="../../#projects"
                    onclick="window.location.replace(this.href); return false;">
                    All Projects ↗
                </a>
            </nav>
        </header>

        <main>
            <section class="project-hero">

                <a
                    class="back-link"
                    href="../../#projects"
                    onclick="window.location.replace(this.href); return false;">
                    ← Back to projects
                </a>

                <p class="hero-eyebrow">Project Details</p>

                <h1 class="hero-name">{name}</h1>

                <div class="project-meta">
                    <span class="project-version">v{version}</span>
                    {featured_html}
                    {category_html}
                </div>

                <p class="hero-bio">
                    {description_html}
                </p>

                <div class="project-technologies">
                    {technologies_html}
                </div>

                <div class="project-actions">
                    {repository_html}
                </div>

            </section>

            <section class="project-readme">
                <h2 class="section-title">About this project</h2>

                <article class="readme-content">
                    {readme_html}
                </article>
            </section>
        </main>

        <footer class="site-footer">
            <p>Built with Rustfolio.</p>
            <a
                href="../../#projects"
                onclick="window.location.replace(this.href); return false;">
                Explore other projects ↑
            </a>
        </footer>

    </div>

    <script src="../../assets/main.js"></script>
</body>
</html>"#
    )
}
