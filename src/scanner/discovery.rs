use std::path::{Path, PathBuf};
use std::fs;
use std::fs::read;

pub fn discover_projects(root: &Path) ->Result<Vec<PathBuf>, std::io::Error>{
    let mut paths  = Vec::new();
    let read_dir = fs::read_dir(root)?;

    for dir_entry in read_dir{
        let entry = dir_entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir(){
            let project_path = entry.path();
            let cargo_path = project_path.join("Cargo.toml");

            if cargo_path.exists(){
                paths.push(project_path);
            }
        }

    }

    Ok(paths)
}