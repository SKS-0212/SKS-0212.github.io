use crate::generator::page::GeneratedPage;
use std::fs;
use std::path::Path;

pub fn write_site(output_dir: &Path, pages: &[GeneratedPage]) -> Result<(), std::io::Error> {
    write_site_with_assets(output_dir, pages, Path::new("assets"))
}
fn write_site_with_assets(
    output_dir: &Path,
    pages: &[GeneratedPage],
    assets_dir: &Path,
) -> Result<(), std::io::Error> {
    write_site_with_publisher(
        output_dir,
        pages,
        assets_dir,
        |staging, output| fs::rename(staging, output),
    )
}

fn write_site_with_publisher<F>(
    output_dir: &Path,
    pages: &[GeneratedPage],
    assets_dir: &Path,
    publish: F,
) -> Result<(), std::io::Error>
where
    F: FnOnce(&Path, &Path) -> Result<(), std::io::Error>,
{
    let staging_dir = output_dir.with_extension("staging");
    let backup_dir = output_dir.with_extension("backup");

    // Do not overwrite an existing recovery backup.
    if backup_dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Recovery backup already exists: {}", backup_dir.display()),
        ));
    }

    // Clean up an incomplete staging directory.
    if staging_dir.exists() {
        fs::remove_dir_all(&staging_dir)?;
    }

    // STEP 1: Prepare the new website.
    fs::create_dir_all(&staging_dir)?;

    for page in pages {
        let destination = staging_dir.join(&page.path);

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&destination, &page.content)?;
    }

    copy_directory(assets_dir, &staging_dir.join("assets"))?;

    // STEP 2: Preserve the existing website.
    let had_previous_site = output_dir.exists();

    if had_previous_site {
        fs::rename(output_dir, &backup_dir)?;
    }

    // STEP 3: Publish the new website.
    match publish(&staging_dir, output_dir) {
        Ok(()) => {
            // STEP 4: Remove the backup after publication.
            if had_previous_site {
                fs::remove_dir_all(&backup_dir)?;
            }

            Ok(())
        }

        Err(publish_error) => {
            // STEP 4: Attempt rollback.
            if had_previous_site {
                match fs::rename(&backup_dir, output_dir) {
                    Ok(()) => Err(publish_error),

                    Err(rollback_error) => Err(std::io::Error::other(format!(
                        "Publication failed: {}. \
                             Rollback also failed: {}. \
                             Previous website may be recoverable at {}",
                        publish_error,
                        rollback_error,
                        backup_dir.display()
                    ))),
                }
            } else {
                Err(publish_error)
            }
        }
    }
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

        let result = write_site_with_assets(&output_dir, &pages, &missing_assets);

        assert!(result.is_err());

        assert_eq!(
            fs::read_to_string(&old_index).unwrap(),
            "Previous working website"
        );
    }

    #[test]
    fn successful_build_replaces_existing_site() {
        let temp = tempfile::tempdir().unwrap();

        let output_dir = temp.path().join("dist");
        let assets_dir = temp.path().join("source-assets");

        // Simulate an existing website.
        let old_project_dir = output_dir.join("projects/old-project");

        fs::create_dir_all(&old_project_dir).unwrap();

        fs::write(output_dir.join("index.html"), "Old homepage").unwrap();

        fs::write(old_project_dir.join("index.html"), "Old project page").unwrap();

        // Create source assets.
        fs::create_dir_all(&assets_dir).unwrap();

        fs::write(assets_dir.join("style.css"), "body { color: blue; }").unwrap();

        fs::write(assets_dir.join("main.js"), "console.log('Rustfolio');").unwrap();

        // New website pages.
        let pages = vec![
            GeneratedPage {
                path: PathBuf::from("index.html"),
                content: "New homepage".to_string(),
            },
            GeneratedPage {
                path: PathBuf::from("projects/hexview/index.html"),
                content: "Hexview project".to_string(),
            },
        ];

        let result = write_site_with_assets(&output_dir, &pages, &assets_dir);

        assert!(result.is_ok());

        // Verify the homepage was replaced.
        assert_eq!(
            fs::read_to_string(output_dir.join("index.html")).unwrap(),
            "New homepage"
        );

        // Verify the new project exists.
        assert_eq!(
            fs::read_to_string(output_dir.join("projects/hexview/index.html")).unwrap(),
            "Hexview project"
        );

        // Verify the old project was removed.
        assert!(!output_dir.join("projects/old-project").exists());

        // Verify assets were copied.
        assert_eq!(
            fs::read_to_string(output_dir.join("assets/style.css")).unwrap(),
            "body { color: blue; }"
        );

        assert_eq!(
            fs::read_to_string(output_dir.join("assets/main.js")).unwrap(),
            "console.log('Rustfolio');"
        );
    }

    #[test]
    fn publication_failure_restores_previous_site() {
        let temp = tempfile::tempdir().unwrap();

        let output_dir = temp.path().join("dist");
        let assets_dir = temp.path().join("source-assets");

        fs::create_dir_all(&output_dir).unwrap();
        fs::create_dir_all(&assets_dir).unwrap();

        fs::write(output_dir.join("index.html"), "Previous working website").unwrap();

        fs::write(assets_dir.join("style.css"), "body {}").unwrap();

        let pages = vec![GeneratedPage {
            path: PathBuf::from("index.html"),
            content: "New website".to_string(),
        }];

        let result =
            write_site_with_publisher(&output_dir, &pages, &assets_dir, |_staging, _output| {
                Err(std::io::Error::other("Simulated publication failure"))
            });

        assert!(result.is_err());

        assert_eq!(
            fs::read_to_string(output_dir.join("index.html")).unwrap(),
            "Previous working website"
        );

        assert!(!output_dir.with_extension("backup").exists());
    }


    #[test]
    fn existing_backup_prevents_build_and_preserves_data() {
        let temp = tempfile::tempdir().unwrap();

        let output_dir = temp.path().join("dist");
        let backup_dir = temp.path().join("dist.backup");
        let assets_dir = temp.path().join("source-assets");

        fs::create_dir_all(&output_dir).unwrap();
        fs::create_dir_all(&backup_dir).unwrap();
        fs::create_dir_all(&assets_dir).unwrap();

        fs::write(
            output_dir.join("index.html"),
            "Current website",
        )
            .unwrap();

        fs::write(
            backup_dir.join("index.html"),
            "Recovery website",
        )
            .unwrap();

        let pages = vec![GeneratedPage {
            path: PathBuf::from("index.html"),
            content: "New website".to_string(),
        }];

        let result = write_site_with_assets(
            &output_dir,
            &pages,
            &assets_dir,
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            std::io::ErrorKind::AlreadyExists
        );

        assert_eq!(
            fs::read_to_string(
                output_dir.join("index.html")
            )
                .unwrap(),
            "Current website"
        );

        assert_eq!(
            fs::read_to_string(
                backup_dir.join("index.html")
            )
                .unwrap(),
            "Recovery website"
        );
    }
}
