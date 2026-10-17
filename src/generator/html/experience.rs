use crate::generator::html::utils::escape_html;
use crate::model::experience::Experience;

pub(super) fn render_experience(experiences: &[Experience]) -> String {
    if experiences.is_empty() {
        return String::new();
    }

    let mut experience_html = String::new();

    for experience in experiences {
        let formatted_experience = render_experience_item(experience);

        experience_html.push_str(&formatted_experience);
    }
    format!(
        r#"
<section class="experience" id="experience">
    <div class="section-header">
        <p class="section-label">Experience</p>
        <h2>Where I've worked</h2>
    </div>

    <div class="experience-list">
        {experience_html}
    </div>
</section>
"#
    )
}

fn render_experience_item(experience: &Experience) -> String {
    let role = escape_html(&experience.role);
    let company = escape_html(&experience.company);
    let description = escape_html(&experience.description);
    let period_html = render_period(experience);

    format!(
        r#"
<article class="experience-item">
    {period_html}

    <div class="experience-content">
        <h3 class="experience-role">{role}</h3>

        <p class="experience-company">{company}</p>

        <p class="experience-description">{description}</p>
    </div>
</article>
"#
    )
}

fn render_period(experience: &Experience) -> String {
    let start = escape_html(&experience.start);

    let end = match &experience.end {
        Some(end) => escape_html(end),
        None => "Present".to_string(),
    };

    format!(
        r#"<div class="experience-period">{} — {}</div>"#,
        start, end
    )
}
