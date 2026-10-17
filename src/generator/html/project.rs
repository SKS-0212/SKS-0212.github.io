use crate::generator::paths::SitePaths;
use crate::model::project::Project;

use super::utils::{escape_html, render_markdown};

pub fn render_project(project: &Project, paths: &SitePaths) -> String {
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

    let home_url = escape_html(&paths.home_url());
    let projects_url = escape_html(&paths.projects_url());
    let stylesheet_url = escape_html(&paths.asset_url("style.css"));
    let script_url = escape_html(&paths.asset_url("main.js"));

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="color-scheme" content="light dark">

    <title>{name} — Project</title>

    <!-- Initialize theme before rendering the page -->
    <script>
        try {{
            const savedTheme = localStorage.getItem("rustfolio-theme");

            if (savedTheme === "light" || savedTheme === "dark") {{
                document.documentElement.dataset.theme = savedTheme;
            }} else if (window.matchMedia("(prefers-color-scheme: dark)").matches) {{
                document.documentElement.dataset.theme = "dark";
            }}
        }} catch (_) {{
            // Continue using the default theme if storage is unavailable.
        }}
    </script>

    <link rel="stylesheet" href="{stylesheet_url}">
</head>

<body>
    <div class="page project-page">

        <!-- Header -->
   <header class="site-header">
    <div class="project-nav-start">
        <a
            href="{projects_url}"
            class="project-back-button"
            aria-label="Back to projects"
            title="Back to projects"
            onclick="window.location.replace(this.href); return false;"
        >
        <svg
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
    >
        <path d="m15 18-6-6 6-6" />
    </svg>
        </a>

        <a class="brand" href="{home_url}">
            Rustfolio
        </a>
    </div>

    <nav class="site-nav">
        <button
            type="button"
            class="theme-toggle"
            data-theme-toggle
            aria-label="Toggle color theme"
            aria-pressed="false"
        ></button>
    </nav>
</header>

        <main>

            <!-- Project hero -->
            <section class="project-hero">

                <p class="hero-eyebrow">
                    Project Details
                </p>

                <h1 class="hero-name">
                    {name}
                </h1>

                <div class="project-meta">
                    <span class="project-version">
                        v{version}
                    </span>

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

            <!-- Project documentation -->
            <section class="project-readme">

                <h2 class="section-title">
                    About this project
                </h2>

                <article class="readme-content">
                    {readme_html}
                </article>

            </section>

        </main>

        <!-- Footer -->
        <footer class="site-footer">

            <p>Built with Rust.</p>


        </footer>

    </div>

    <script src="{script_url}"></script>
</body>
</html>"##
    )
}
