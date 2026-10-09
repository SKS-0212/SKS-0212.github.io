use std::fs;
use std::path::Path;
use crate::generator::page::GeneratedPage;

pub fn write_site(output_dir: &Path, pages: &[GeneratedPage] ) -> Result<(), std::io::Error> {

    if output_dir.exists(){
        fs::remove_dir_all(output_dir)?;
    }

    fs::create_dir_all(output_dir)?;


    for page in pages{
        let destination = output_dir.join(&page.path);

        if let Some(dir ) = destination.parent(){
            fs::create_dir_all(dir)?;
        }
        fs::write(&destination, &page.content)?;
    }
    
    copy_directory(
        Path::new("assets"),
        &output_dir.join("assets"),
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