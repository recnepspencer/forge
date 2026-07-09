#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationAspectPosture {
    None,
    RequiredContract,
    RetainedContractAndCoverage,
    AspectSensitiveReadmission,
    CategoryScopedAspectComposition,
}

impl WorthQueryOrchestrationAspectPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RequiredContract => "required_contract",
            Self::RetainedContractAndCoverage => "retained_contract_and_coverage",
            Self::AspectSensitiveReadmission => "aspect_sensitive_readmission",
            Self::CategoryScopedAspectComposition => "category_scoped_aspect_composition",
        }
    }
}
