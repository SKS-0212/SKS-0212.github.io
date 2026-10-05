use crate::model::link::Link;


#[derive(Debug)]
pub struct Profile{
    pub name: String,
    pub headline: String,
    pub bio: String,
    pub location: Option<String>,
    pub skills: Vec<String>,
    pub links: Vec<Link>
}