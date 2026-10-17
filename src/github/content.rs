use crate::github::error::GithubError;
use crate::github::model::GithubFileContent;
use base64::Engine;

pub fn decode_github_content(content: &str) -> Result<String, GithubError> {
    let cleared: String = content
        .chars()
        .filter(|character| *character != '\n' && *character != '\r')
        .collect();

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(cleared)
        .map_err(GithubError::Base64Decode)?;

    let decoded = String::from_utf8(bytes).map_err(GithubError::InvalidUtf8)?;

    Ok(decoded)
}

pub fn decode_github_file(file: GithubFileContent) -> Result<String, GithubError> {
    if file.encoding != "base64" {
        return Err(GithubError::UnsupportedEncoding(file.encoding));
    }
    decode_github_content(&file.content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_base64_github_content() {
        let encoded = "W3BhY2thZ2VdCg==";

        let decoded = decode_github_content(encoded).expect("Should decode valid Base64");

        assert_eq!(decoded, "[package]\n");
    }

    #[test]
    fn decodes_base64_with_line_breaks() {
        let encoded = "W3BhY2th\nZ2VdCg==\n";

        let decoded =
            decode_github_content(encoded).expect("Should decode Base64 containing line breaks");

        assert_eq!(decoded, "[package]\n");
    }

    #[test]
    fn rejects_invalid_base64() {
        let result = decode_github_content("%%%");
        assert!(matches!(result, Err(GithubError::Base64Decode(_))));
    }

    #[test]
    fn rejects_invalid_utf8() {
        let result = decode_github_content("/w==");

        assert!(matches!(result, Err(GithubError::InvalidUtf8(_))));
    }
}
