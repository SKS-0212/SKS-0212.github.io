

#[derive(Debug)]
pub struct Project{
    pub name:String,
    pub description: Option<String>,
    pub version: String,
    pub repository: Option<String>,
    
    pub category: Option<String>,
    pub featured: bool,
    pub readme: String,
}