use std::fmt::Formatter;
use std::fs;
use std::path::Path;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct CargoManifest{
    pub package: CargoPackage,
}

#[derive(Deserialize,Debug)]
pub struct CargoPackage{
   pub name: String,
   pub  version: String,
   pub description: Option<String>,
    pub repository: Option<String>,
}

#[derive(Debug)]
pub enum CargoScanError{
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl From<std::io::Error> for CargoScanError {
    fn from(value:  std::io::Error) -> Self {
        CargoScanError::Io(value)
    }
}

impl From<toml::de::Error> for CargoScanError {
    fn from(value: toml::de::Error) -> Self {
        CargoScanError::Toml(value)
    }
}

impl std::fmt::Display for CargoScanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CargoScanError::Io(error)=>write!(f,"Failed to read file: {}",error),
            CargoScanError::Toml(error)=>write!(f,"Failed to parse Cargo.toml: {}",error),
        }
    }
}

impl  std::error::Error for CargoScanError {}

pub fn read_cargo(project_path: &Path) -> Result<CargoManifest, CargoScanError> {
    let path = project_path.join("Cargo.toml");

    let content = fs::read_to_string(path)?;

    parse_cargo(&content)
}

pub fn parse_cargo(content: &str)->Result<CargoManifest, CargoScanError>{
    let manifest = toml::from_str::<CargoManifest>(content)
        .map_err(CargoScanError::Toml)?;

    Ok(manifest)
}


#[cfg(test)]
mod tests{
    use super::*;


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parses_cargo_manifest_from_string() {
            let content = r#"
            [package]
            name = "rustfolio"
            version = "0.1.0"
            edition = "2024"
            description = "Static portfolio generator"
        "#;

            let manifest = parse_cargo(content)
                .expect("Valid Cargo manifest");

            assert_eq!(manifest.package.name, "rustfolio");
            assert_eq!(manifest.package.version, "0.1.0");
            assert_eq!(
                manifest.package.description.as_deref(),
                Some("Static portfolio generator")
            );
        }
    }
}

