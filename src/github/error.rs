use core::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum GithubError {
    UserNotFound(String),
    RateLimited,
    Network(reqwest::Error),
    Api(reqwest::Error),
    Decode(reqwest::Error),
    PaginationOverflow,
    Base64Decode(base64::DecodeError),
    InvalidUtf8(std::string::FromUtf8Error),
    UnsupportedEncoding(String),
}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubError::UserNotFound(username) => {
                write!(f, "GitHub user '{}' was not found", username)
            }
            GithubError::RateLimited => {
                write!(f, "GitHub API rate limit exceeded")
            }
            GithubError::Network(error) => {
                write!(f, "GitHub network error: {}", error)
            }
            GithubError::Api(error) => {
                write!(f, "GitHub API error: {}", error)
            }
            GithubError::Decode(error) => {
                write!(f, "Failed to decode GitHub response: {}", error)
            }
            GithubError::PaginationOverflow => {
                write!(f, "GitHub pagination page number overflow")
            }
            GithubError::Base64Decode(error) => {
                write!(f, "Failed to decode Github Base64 content: {}", error)
            }
            GithubError::InvalidUtf8(error) => {
                write!(f, "Github file contains invalid UTF-8: {}", error)
            }
            GithubError::UnsupportedEncoding(encoding) => {
                write!(f, "Unsupported Github file encoding: {}", encoding)
            }
        }
    }
}

impl std::error::Error for GithubError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_not_found_has_readable_message() {
        let error = GithubError::UserNotFound("unknown-user".to_string());

        assert_eq!(
            error.to_string(),
            "GitHub user 'unknown-user' was not found"
        );
    }

    #[test]
    fn rate_limit_has_readable_message() {
        let error = GithubError::RateLimited;

        assert_eq!(error.to_string(), "GitHub API rate limit exceeded");
    }
}
