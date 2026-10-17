use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct RustfolioConfig {
    pub projects: Vec<PathBuf>,
    pub profile: ProfileConfig,

    #[serde(default)]
    pub experience: Vec<ExperienceConfig>,

    #[serde(default)]
    pub github: Option<GithubConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub headline: String,
    pub bio: String,

    pub location: Option<String>,

    #[serde(default)]
    pub skills: Vec<String>,

    #[serde(default)]
    pub links: Vec<LinkConfig>,
}

#[derive(Debug, Deserialize)]
pub struct LinkConfig {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ExperienceConfig {
    pub company: String,
    pub role: String,
    pub start: String,
    pub end: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubConfig {
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_config_without_github() {
        let content = r#"
projects = ["../rsh"]

[profile]
name = "Developer"
headline = "Rust Developer"
bio = "Building Rust projects."
"#;

        let config: RustfolioConfig = toml::from_str(content).expect("Valid configuration");

        assert!(config.github.is_none());
        assert_eq!(config.projects.len(), 1);
        assert!(config.experience.is_empty());
    }

    #[test]
    fn parses_config_with_github() {
        let content = r#"
projects = ["../rsh"]

[profile]
name = "Developer"
headline = "Rust Developer"
bio = "Building Rust projects."

[github]
username = "rust-developer"
"#;

        let config: RustfolioConfig = toml::from_str(content).expect("Valid configuration");

        assert_eq!(config.github.as_ref().unwrap().username, "rust-developer");
    }

    #[test]
    fn rejects_github_config_without_username() {
        let content = r#"
projects = ["../rsh"]

[profile]
name = "Developer"
headline = "Rust Developer"
bio = "Building Rust projects."

[github]
"#;

        let result = toml::from_str::<RustfolioConfig>(content);

        assert!(result.is_err());
    }
}
