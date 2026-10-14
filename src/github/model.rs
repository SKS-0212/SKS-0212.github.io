use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubRepository{
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub private: bool,
    pub fork: bool,
}


impl GithubRepository {
    pub fn is_rust_project(&self)->bool{
        !self.private &&
            !self.fork &&
            self.language.as_deref() == Some("Rust")
    }
}

#[derive(Debug)]
pub struct GithubRepositoryPage {
    pub repositories: Vec<GithubRepository>,
    pub has_next_page: bool,
}

#[cfg(test)]
mod tests{
    use super::*;



    #[test]
    fn github_data_deserialization(){
        let json = r#"{
            "name": "rsh",
            "full_name": "example/rsh",
            "html_url": "https://github.com/example/rsh",
            "description": null,
            "language": "Rust",
            "private": false,
            "fork": false
            }"#;
        let repository: GithubRepository =
            serde_json::from_str(json).expect("Valid GitHub repository JSON");

        assert_eq!(repository.name, "rsh");
        assert_eq!(repository.full_name, "example/rsh");
        assert_eq!(repository.html_url, "https://github.com/example/rsh");
        assert_eq!(repository.description, None);
        assert_eq!(repository.language, Some("Rust".to_string()));
        assert_eq!(repository.private, false);
        assert_eq!(repository.fork, false);
    }
    
    #[test]
    fn github_nullable_fields_deserialization() {
        let json = r#"
    {
        "name": "hexview",
        "full_name": "example/hexview",
        "html_url": "https://github.com/example/hexview",
        "description": null,
        "language": null,
        "private": false,
        "fork": false
    }
    "#;

        let repository: GithubRepository =
            serde_json::from_str(json)
                .expect("Valid GitHub repository JSON");

        assert_eq!(repository.name, "hexview");
        assert_eq!(repository.description, None);
        assert_eq!(repository.language, None);
    }


    #[test]
    fn filters_public_rust_repositories() {
        let mut repository = GithubRepository {
            name: "rsh".to_string(),
            full_name: "example/rsh".to_string(),
            html_url: "https://github.com/example/rsh".to_string(),
            description: None,
            language: Some("Rust".to_string()),
            private: false,
            fork: false,
        };

        // Case 1: Public Rust repository
        assert!(repository.is_rust_project());

        // Case 2: Python repository
        repository.language = Some("Python".to_string());
        assert!(!repository.is_rust_project());

        // Case 3: Language is unknown
        repository.language = None;
        assert!(!repository.is_rust_project());

        // Case 4: Private Rust repository
        repository.language = Some("Rust".to_string());
        repository.private = true;
        assert!(!repository.is_rust_project());

        // Case 5: Forked Rust repository
        repository.private = false;
        repository.fork = true;
        assert!(!repository.is_rust_project());
    }


}