use crate::model::portfolio::Portfolio;

#[derive(Debug)]
pub struct BuildDiagnostic{
    pub source: String,
    pub message: String,
    pub detail: String,
}

#[derive(Debug)]
pub struct BuildReport{
    pub diagnostics: Vec<BuildDiagnostic>,
}


#[derive(Debug)]
pub struct PortfolioBuild{
    pub portfolio: Portfolio,
    pub report: BuildReport,
}