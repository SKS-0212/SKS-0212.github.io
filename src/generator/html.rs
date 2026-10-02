
use crate::model::portfolio::Portfolio;

pub fn render_portfolio(portfolio: &Portfolio) -> String {
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
        portfolio.profile.name,
        portfolio.profile.name,
        portfolio.profile.headline,
        portfolio.profile.bio,
    )
}