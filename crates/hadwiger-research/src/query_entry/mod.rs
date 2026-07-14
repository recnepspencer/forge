mod admitted_handle;
mod domain_marker;
mod domain_package;
mod operating_context;
mod ordinary_query;

pub use admitted_handle::HadwigerResearchHandle;
pub use domain_marker::HadwigerResearchDomainEntry;
pub use domain_package::hadwiger_research_domain_package;
pub use operating_context::{
    HadwigerResearchAssumptionRegime, HadwigerResearchCheckerSupportRegime,
    HadwigerResearchInvalidationRegime, HadwigerResearchOperatingContext,
};
pub use ordinary_query::{HadwigerCandidateContribution, HadwigerResearchQueryExt};

#[cfg(test)]
mod ordinary_query_tests;
