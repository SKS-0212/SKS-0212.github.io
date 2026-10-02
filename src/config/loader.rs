use std::error::Error;
use std::fmt::Formatter;
use std::path::Path;
use std::fs;
use crate::config::model::RustfolioConfig;

#[derive(Debug)]
pub enum ConfigError{
    Io(std::io::Error),
    Toml(toml::de::Error)
}

impl From<std::io::Error> for ConfigError{
    fn from(value: std::io::Error) -> Self {
        ConfigError::Io(value)
    }
}

impl From<toml::de::Error> for ConfigError{
    fn from(value: toml::de::Error) -> Self {
        ConfigError::Toml(value)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            ConfigError::Io(error)=> write!(f,"Failed to read Rustfolio configuration: {error}"),
            ConfigError::Toml(error)=> write!(f,"Failed to parse Rustfolio configuration: {error}"),
        }
    }
}

impl std::error::Error for ConfigError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::Io(error)=>Some(error),
            ConfigError::Toml(error)=>Some(error),
        }
    }
}

pub fn load_config(path: &Path) -> Result<RustfolioConfig, ConfigError>{
    let content = fs::read_to_string(path)?;
    let config = toml::from_str::<RustfolioConfig>(&content)?;

    Ok(config)
}