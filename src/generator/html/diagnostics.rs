use crate::model::portfolio::Portfolio;

use super::utils::escape_html;


pub(super)  fn render_diagnostics(portfolio: &Portfolio) -> String {
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
