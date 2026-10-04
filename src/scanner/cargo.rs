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


    let manifest = toml::from_str::<CargoManifest>(&content)?;

    Ok(manifest)
}

