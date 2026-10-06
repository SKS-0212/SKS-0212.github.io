use super::utils::escape_html;
use crate::model::link::Link;
use crate::model::profile::Profile;


fn render_footer_links(links: &[Link]) ->String{
    if links.is_empty(){
        return String::new();
    }

    let mut link_html = String::new();

    for link in links  {
        let escaped_label = escape_html(&link.label);
        let escaped_url = escape_html(&link.url);

        let formatted_link = format!(
            r#"<a class="footer-link" href="{escaped_url}" target="_blank" rel="noopener noreferrer">{escaped_label} ↗</a>"#
        );

        link_html.push_str(&formatted_link);

    }

    link_html
}

pub(super) fn render_footer(profile: &Profile)->String{
    let name = escape_html(&profile.name);
    let links_html = render_footer_links(&profile.links);

    format!(
        r#"
<footer class="site-footer">
    <div class="footer-main">
        <p class="footer-name">{name}</p>

        <nav class="footer-links">
            {links_html}
        </nav>
    </div>

    <div class="footer-meta">
        <span>Built with Rust.</span>
    </div>
</footer>
"#
    )
}

