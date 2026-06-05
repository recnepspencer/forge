mod admitted_handle;
mod domain_marker;
mod operating_context;

pub use admitted_handle::{
    admit_hadwiger_research_handle, HadwigerResearchAdmissionError, HadwigerResearchHandle,
};
pub use domain_marker::HadwigerResearchDomainEntry;
pub use operating_context::{
    HadwigerResearchAssumptionRegime, HadwigerResearchCheckerSupportRegime,
    HadwigerResearchInvalidationRegime, HadwigerResearchOperatingContext,
};
