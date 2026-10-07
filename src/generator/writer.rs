use std::fs;
use std::path::Path;

pub fn write_site(output_dir: &Path, html: &str, ) -> Result<(), std::io::Error> {

    if output_dir.exists(){
        fs::remove_dir_all(output_dir)?;
    }

    fs::create_dir_all(output_dir)?;

    let html_path = output_dir.join("index.html");
    fs::write(html_path, html)?;

    let assets_dir = output_dir.join("assets");

    copy_directory(
        Path::new("assets"),
        &assets_dir,
    )?;

    Ok(())
}

fn copy_directory(source: &Path, destination: &Path, ) -> Result<(), std::io::Error> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;

        let file_type = entry.file_type()?;

        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if file_type.is_file(){
            fs::copy(&source_path, &destination_path)?;
        } else if file_type.is_dir() {
            copy_directory(&source_path, &destination_path)?;
        }
    }

    Ok(())
}