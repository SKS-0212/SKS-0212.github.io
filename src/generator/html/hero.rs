use super::utils::escape_html;
use crate::model::profile::Profile;

pub(super) fn render_hero(profile: &Profile) -> String {
    // Escape:
    // profile.name
    let name = escape_html(&profile.name);
    // profile.headline
    let headline = escape_html(&profile.headline);
    // profile.bio
    let bio = escape_html(&profile.bio);

    // Render:
    // links
    let links_html = render_links(&profile);
    // skills
    let skills_html = render_skills(&profile);
    // location
    let location_html = render_location(&profile);

    format!(
        r##"
<header class="site-header">
    <a class="brand" href="/">{}</a>

    <nav class="site-nav">
        {}
  <button
    type="button"
    class="theme-toggle"
    data-theme-toggle
    aria-label="Toggle color theme"
    aria-pressed="false"
></button>
    </nav>
</header>

<main class="hero">
    <div class="hero-intro">
        <p class="hero-eyebrow">Software Engineer</p>

        <h1 class="hero-name">{}</h1>

        <h2 class="hero-headline">{}</h2>

        <p class="hero-bio">{}</p>
    </div>

    <div class="hero-meta">
        <a class="hero-cta" href="#projects">
            Explore work ↓
        </a>

        {}
    </div>

    <div class="hero-skills">
        {}
    </div>
</main>
"##,
        // figure out argument order
        name,
        links_html,
        name,
        headline,
        bio,
        location_html,
        skills_html
    )
}

fn render_skills(profile: &Profile) -> String {
    let mut skills_html = String::new();

    for skill in &profile.skills {
        let escaped_skill = escape_html(skill);

        let formatted_skill = format!(r#"<span class="hero-skill">{}</span>"#, escaped_skill);

        skills_html.push_str(&formatted_skill);
    }

    skills_html
}

fn render_links(profile: &Profile) -> String {
    let mut links_html = String::new();

    for link in &profile.links {
        let escaped_label = escape_html(&link.label);
        let escaped_url = escape_html(&link.url);

        let formatted_link = format!(
            r#"<a class="nav-link" href="{}" target="_blank" rel="noopener noreferrer">{} ↗</a>"#,
            escaped_url, escaped_label
        );

        links_html.push_str(&formatted_link);
    }

    links_html
}

fn render_location(profile: &Profile) -> String {
    match &profile.location {
        Some(location) => {
            let escaped_location = escape_html(location);
            format!(
                r#"<span class="hero-location">Based in {}</span>"#,
                escaped_location
            )
        }
        None => String::new(),
    }
}
