use std::path::Path;
use std::fs;

pub fn write_site(output_dir: &Path, html: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(output_dir)?;
    let html_path = output_dir.join("index.html");
    fs::write(html_path, html)?;
    Ok(())
}