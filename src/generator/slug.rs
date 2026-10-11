
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
}