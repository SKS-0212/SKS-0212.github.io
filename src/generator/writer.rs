use crate::generator::page::GeneratedPage;
use std::fs;
use std::path::Path;

pub fn write_site(
    output_dir: &Path,
    pages: &[GeneratedPage],
) -> Result<(), std::io::Error> {
    write_site_with_assets(
        output_dir,
        pages,
        Path::new("assets"),
    )
}

fn write_site_with_assets(
    output_dir: &Path,
    pages: &[GeneratedPage],
    assets_dir: &Path,
) -> Result<(), std::io::Error> {
    // Create a staging directory next to the output directory.
    let staging_dir = output_dir.with_extension("staging");

    // Remove any staging directory left by a previous build.
    if staging_dir.exists() {
        fs::remove_dir_all(&staging_dir)?;
    }

    fs::create_dir_all(&staging_dir)?;

    // Generate all HTML pages inside staging.
    for page in pages {
        let destination = staging_dir.join(&page.path);

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&destination, &page.content)?;
    }

    // Copy assets before modifying the existing website.
    copy_directory(
        assets_dir,
        &staging_dir.join("assets"),
    )?;

    // Only replace the output after staging succeeds.
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }

    fs::rename(&staging_dir, output_dir)?;

    Ok(())
}
fn copy_directory(source: &Path, destination: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;

        let file_type = entry.file_type()?;

        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if file_type.is_file() {
            fs::copy(&source_path, &destination_path)?;
        } else if file_type.is_dir() {
            copy_directory(&source_path, &destination_path)?;
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn failed_build_preserves_existing_site() {
        let temp = tempfile::tempdir().unwrap();

        let output_dir = temp.path().join("dist");
        let missing_assets = temp.path().join("missing-assets");

        fs::create_dir_all(&output_dir).unwrap();

        let old_index = output_dir.join("index.html");
        fs::write(&old_index, "Previous working website").unwrap();

        let pages = vec![GeneratedPage {
            path: PathBuf::from("index.html"),
            content: "New website".to_string(),
        }];

        let result = write_site_with_assets(
            &output_dir,
            &pages,
            &missing_assets,
        );

        assert!(result.is_err());

        assert_eq!(
            fs::read_to_string(&old_index).unwrap(),
            "Previous working website"
        );
    }
}
