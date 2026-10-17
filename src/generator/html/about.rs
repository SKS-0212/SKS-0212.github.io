use super::utils::escape_html;
use crate::model::profile::Profile;

fn render_location(profile: &Profile) -> String {
    match &profile.location {
        Some(location) => {
            let escaped_location = escape_html(location);

            format!(
                r#"
<div class="about-detail">
    <span class="about-detail-label">Location</span>
    <span class="about-detail-value">{escaped_location}</span>
</div>
"#
            )
        }
        None => String::new(),
    }
}
fn render_focus(profile: &Profile) -> String {
    if profile.skills.is_empty() {
        return String::new();
    }

    let mut skills_html = String::new();

    for skill in &profile.skills {
        let escaped_focus = escape_html(skill);
        let formatted_focus = format!(r#"<span>{escaped_focus}</span>"#);

        skills_html.push_str(&formatted_focus);
    }

    format!(
        r#"
        <div class="about-detail">
            <span class="about-detail-label">Focus</span>
            <div class="about-focus">
                {skills_html}
            </div>
        </div>
        "#
    )
}

pub(super) fn render_about(profile: &Profile) -> String {
    let bio = escape_html(&profile.bio);
    let location_html = render_location(profile);
    let focus_html = render_focus(profile);

    format!(
        r#"
        <section class="about" id="about">
            <div class="section-header">
                <p class="section-label">About</p>
                <h2>A little about me</h2>
            </div>

            <div class="about-layout">
                 <div class="about-intro">
                    <p class="about-text">{bio}</p>
                </div>

            <div class="about-details">
            {location_html}
            {focus_html}
            </div>
        </div>
    </section>
    "#
    )
}
