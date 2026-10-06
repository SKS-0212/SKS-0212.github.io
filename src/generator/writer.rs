use std::path::Path;
use std::fs;

pub fn write_site(output_dir: &Path, html: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(output_dir)?;
    let html_path = output_dir.join("index.html");
    fs::write(html_path, html)?;

    let assets_dir = output_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;

    let source_css = Path::new("assets/style.css");
    let destination_css = assets_dir.join("style.css");

    fs::copy(source_css,destination_css)?;
    fs::copy(
        Path::new("assets/main.js"),
        assets_dir.join("main.js"),
    )?;

    Ok(())
}