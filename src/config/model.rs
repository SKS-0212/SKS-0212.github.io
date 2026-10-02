use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct RustfolioConfig{
    pub projects: Vec<PathBuf>,
    pub profile: ProfileConfig,
}

#[derive(Debug,Deserialize)]
pub struct  ProfileConfig{
    pub name: String,
    pub headline: String,
    pub bio: String,
}
