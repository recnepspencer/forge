mod admitted_handle;
mod domain_marker;
mod operating_context;
mod ordinary_query;

pub use admitted_handle::{
    admit_hadwiger_research_handle, HadwigerResearchAdmissionError, HadwigerResearchHandle,
};
pub use domain_marker::HadwigerResearchDomainEntry;
pub use operating_context::{
    HadwigerResearchAssumptionRegime, HadwigerResearchCheckerSupportRegime,
    HadwigerResearchInvalidationRegime, HadwigerResearchOperatingContext,
};
pub use ordinary_query::{
    declare_candidate_promotion, declare_candidate_search, HadwigerCandidateContribution,
};

#[cfg(test)]
mod ordinary_query_tests;
