use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct RustfolioConfig{
    pub projects: Vec<PathBuf>,
    pub profile: ProfileConfig,

    #[serde(default)]
    pub experience: Vec<ExperienceConfig>,
}

#[derive(Debug,Deserialize)]
pub struct  ProfileConfig{
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
pub struct LinkConfig{
    pub label: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ExperienceConfig{
    pub company: String,
    pub role: String,
    pub start: String,
    pub end: Option<String>,
    pub description: String,
}
