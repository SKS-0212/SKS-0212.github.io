use serde::Deserialize;
use std::fmt::Formatter;
use std::fs;
use std::io::Error;
use std::path::Path;

#[derive(Debug)]
pub struct ReadmeData {
    pub metadata: ReadmeMetadata,
    pub content: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct ReadmeMetadata {
    pub category: Option<String>,

    #[serde(default)]
    pub featured: bool,

    #[serde(default)]
    pub technologies: Vec<String>,
}
#[derive(Debug)]
pub enum ReadmeScanError {
    Io(std::io::Error),
    Yaml(serde_yaml_ng::Error),
    MissingFrontMatter,
    MissingClosingDelimiter,
}

impl From<std::io::Error> for ReadmeScanError {
    fn from(value: Error) -> Self {
        ReadmeScanError::Io(value)
    }
}

impl From<serde_yaml_ng::Error> for ReadmeScanError {
    fn from(value: serde_yaml_ng::Error) -> Self {
        ReadmeScanError::Yaml(value)
    }
}

impl std::fmt::Display for ReadmeScanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadmeScanError::Io(error) => write!(f, "Failed to read README: {}", error),
            ReadmeScanError::Yaml(error) => write!(f, "Failed to parse README YAML: {}", error),
            ReadmeScanError::MissingFrontMatter => write!(f, "README frontmatter is missing"),
            ReadmeScanError::MissingClosingDelimiter => {
                write!(f, "README closing delimiter is missing")
            }
        }
    }
}

impl std::error::Error for ReadmeScanError {}

pub fn read_readme(project_path: &Path) -> Result<ReadmeData, ReadmeScanError> {
    let path = project_path.join("README.md");
    let contents = fs::read_to_string(path)?;
    parse_readme(&contents)
}

pub fn parse_readme(content: &str) -> Result<ReadmeData, ReadmeScanError> {
    if !content.starts_with("---") {
        return Ok(ReadmeData {
            metadata: ReadmeMetadata::default(),
            content: content.to_string(),
        });
    }
    let remaining = content
        .strip_prefix("---")
        .ok_or(ReadmeScanError::MissingFrontMatter)?;
    let (metadata, markdown) = remaining
        .split_once("---")
        .ok_or(ReadmeScanError::MissingClosingDelimiter)?;
    let data = serde_yaml_ng::from_str::<ReadmeMetadata>(metadata)?;
    let readme_data = ReadmeData {
        metadata: data,
        content: markdown.to_string(),
    };
    Ok(readme_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_readme_from_string() {
        let content = r#"---
category: Systems Programming
featured: true
---
# Rust Shell

A shell written in Rust.
"#;

        let result = parse_readme(content).expect("README should parse successfully");

        assert_eq!(
            result.metadata.category.as_deref(),
            Some("Systems Programming")
        );

        assert!(result.metadata.featured);
        assert!(result.content.contains("# Rust Shell"));
    }

    #[test]
    fn parses_readme_without_frontmatter() {
        let input = "# Hexview\n\nA hexadecimal file viewer.";

        let result = parse_readme(input).expect("Normal README should be accepted");

        assert_eq!(result.content, input);
        assert_eq!(result.metadata.category, None);
        assert!(!result.metadata.featured);
        assert!(result.metadata.technologies.is_empty());
    }

    #[test]
    fn rejects_unclosed_frontmatter() {
        let input = "---\ncategory: Systems Programming\n";

        let result = parse_readme(input);

        assert!(result.is_err());
    }
}
