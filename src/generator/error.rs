use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SiteGenerationError {
    InvalidSlug(String),
    DuplicateSlug(String),
}

impl Display for SiteGenerationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SiteGenerationError::InvalidSlug(slug) => {
                write!(f, "Invalid project slug: '{}'", slug)
            }
            SiteGenerationError::DuplicateSlug(slug) => {
                write!(f, "Duplicate project slug: '{}'", slug)
            }
        }
    }
}

impl std::error::Error for SiteGenerationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_slug_has_readable_message() {
        let error = SiteGenerationError::InvalidSlug("../outside".to_string());

        assert_eq!(error.to_string(), "Invalid project slug: '../outside'");
    }

    #[test]
    fn duplicate_slug_has_readable_message() {
        let error = SiteGenerationError::DuplicateSlug("rsh".to_string());

        assert_eq!(error.to_string(), "Duplicate project slug: 'rsh'");
    }
}
