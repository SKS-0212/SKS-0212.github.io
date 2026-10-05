use crate::model::experience::Experience;
use crate::model::profile::Profile;
use crate::model::project::Project;

#[derive(Debug)]
pub struct Portfolio{
    pub profile: Profile,
    pub experience: Vec<Experience>,
    pub projects: Vec<Project>,
    pub diagnostics: Vec<ProjectDiagnostic>,
}

#[derive(Debug)]
pub struct ProjectDiagnostic{
    pub project: String,
    pub message: String,
}

