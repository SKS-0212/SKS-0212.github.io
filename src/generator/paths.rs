use std::path::Path;

#[derive(Debug)]
pub struct SitePaths {
    current_depth: usize,
}

impl SitePaths {
    pub fn new(current_depth: usize) -> Self {
        Self { current_depth }
    }

    pub fn root_prefix(&self) -> String {
        if self.current_depth == 0 {
            return "./".to_string();
        }

        "../".repeat(self.current_depth)
    }

    pub fn home_url(&self) -> String {
        self.root_prefix()
    }

    pub fn projects_url(&self) -> String {
        format!("{}#projects", self.root_prefix())
    }

    pub fn asset_url(&self, asset: &str) -> String {
        format!("{}assets/{asset}", self.root_prefix())
    }

    pub fn from_page_path(page_path: &Path)-> Self {
        let parent = page_path.parent().unwrap_or_else(|| Path::new(""));
        let depth = parent.components().count();
        Self::new(depth)
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::path::Path;

    #[test]
    fn home_has_zero_depth(){
        let paths = SitePaths::from_page_path(
            Path::new("index.html")
        );
        assert_eq!(paths.root_prefix(),"./")
    }

    #[test]
    fn project_depth(){
        let paths = SitePaths::from_page_path(
            Path::new("projects/rsh/index.html")
        );
        assert_eq!(paths.root_prefix(),"../../")
    }

    #[test]
    fn nested_project_path(){
        let paths = SitePaths::from_page_path(
            Path::new("projects/system/rsh/index.html")
        );
        assert_eq!(paths.root_prefix(),"../../../");
    }

    #[test]
    fn assert_url(){
        let paths = SitePaths::from_page_path(
            Path::new("projects/rsh/index.html")
        );

        assert_eq!(
            paths.asset_url("style.css"),
            "../../assets/style.css"
        );
    }
}
