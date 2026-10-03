use crate::model::portfolio::Portfolio;

pub fn render_portfolio(portfolio: &Portfolio) -> String {
    let name = escape_html(&portfolio.profile.name);
    let headline = escape_html(&portfolio.profile.headline);
    let bio = escape_html(&portfolio.profile.bio);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body>
    <main>
        <h1>{}</h1>
        <h2>{}</h2>
        <p>{}</p>
    </main>
</body>
</html>"#,
        name,
        name,
        headline,
        bio,
    )
}

fn escape_html(input: &str) ->String{
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")

}