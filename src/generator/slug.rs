
pub fn is_valid_slug(slug: &str)->bool{
    if slug.is_empty() {
        return false;
    }


    if slug.starts_with('-') || slug.ends_with('-'){
        return false;
    }

     slug.chars().all(
        |ch|
            ch.is_ascii_lowercase()
            || ch.is_ascii_digit()
            || ch == '-'
        )

}

pub fn slugify(name: &str)-> String{
    name.trim()
        .to_ascii_lowercase()
        .chars()
        .map(|ch|{
            if ch.is_ascii_lowercase() || ch.is_ascii_digit(){
                ch
            }else{
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part|!part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn accepts_valid_slugs(){
        assert!(is_valid_slug("rsh"));
        assert!(is_valid_slug("hex-view"));
        assert!(is_valid_slug("rust2026"));
    }

    #[test]
    fn rejects_unsafe_slugs(){
        assert!(!is_valid_slug("../outside"));
        assert!(!is_valid_slug("hello/world"));
        assert!(!is_valid_slug("my project"));
        assert!(!is_valid_slug(""));
        assert!(!is_valid_slug("-rust"));
    }

    #[test]
    fn converts_project_names_to_slugs() {
        assert_eq!(slugify("Rust Shell"), "rust-shell");
        assert_eq!(slugify("HexView"), "hexview");
        assert_eq!(slugify("my---project"), "my-project");
        assert_eq!(slugify("  Rust 2026  "), "rust-2026");
        assert_eq!(slugify("../outside"), "outside");
        assert_eq!(slugify("---"), "");
    }
}