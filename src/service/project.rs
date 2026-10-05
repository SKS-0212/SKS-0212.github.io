use std::error::Error;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};
use crate::scanner::cargo::{read_cargo, CargoScanError};
use crate::scanner::readme::{read_readme,ReadmeScanError};
use crate::model::project::Project;

#[derive(Debug)]
pub enum ProjectLoadError{
    Cargo(CargoScanError),
    Readme(ReadmeScanError),
}

impl From<CargoScanError> for ProjectLoadError{
    fn from(value: CargoScanError) -> Self {
        ProjectLoadError::Cargo(value)
    }
}

impl From <ReadmeScanError> for ProjectLoadError{
    fn from(value: ReadmeScanError) -> Self {
        ProjectLoadError::Readme(value)
    }
}

impl std::fmt::Display for ProjectLoadError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectLoadError::Cargo(error)=>write!(f,"Failed to load Cargo metadata: {}",error),
            ProjectLoadError::Readme(error)=>write!(f,"Failed to load project README: {}",error)
        }
    }
}

impl std::error::Error for ProjectLoadError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProjectLoadError::Cargo(error)=> Some(error),
            ProjectLoadError::Readme(error)=>Some(error),
        }
    }
}

#[derive(Debug)]
pub struct ProjectFailure{
    pub path: PathBuf,
    pub error: ProjectLoadError,
}

#[derive(Debug)]
pub struct ProjectLoadReport{
    pub projects: Vec<Project>,
    pub failures: Vec<ProjectFailure>,
}

pub fn load_project(project_path: &Path)->Result<Project, ProjectLoadError>{
    let cargo_manifest = read_cargo(project_path)?;
    let readme_data = read_readme(project_path)?;

    let project = Project{
        name: cargo_manifest.package.name,
        version: cargo_manifest.package.version,
        description: cargo_manifest.package.description,
        repository: cargo_manifest.package.repository,
        category: readme_data.metadata.category,
        featured: readme_data.metadata.featured,
        technologies: readme_data.metadata.technologies,
        readme: readme_data.content,

    };

    Ok(project)

}

pub fn load_projects(projects_paths: &[PathBuf]) -> ProjectLoadReport{

    // projects_paths
    //     .iter()
    //     .map(|path| load_project(path))
    //     .collect()

    let mut projects = Vec::new();
    let mut failures = Vec::new();

    for project_path in projects_paths {
        match load_project(project_path){
            Ok(project)=>{
                projects.push(project)
            },
            Err(error)=>{
                let project_failure =  ProjectFailure {
                    path: project_path.clone(),
                    error,
                };
                failures.push(project_failure)
            }
        }
    }

    ProjectLoadReport {
        projects,
        failures,
    }


}