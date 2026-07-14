mod admission;
mod drift;
mod relevance;

pub(crate) use admission::admit_policy_aware_live_plan;
pub use admission::{PolicyAwareLiveAdmissionReport, PolicyAwareLivePlan};
pub use drift::{
    certify_policy_live_drift_evidence, PolicyDriftDisposition, PolicyLiveDensityEvidence,
    PolicyLiveDensityPosture, PolicyLiveDriftEvidenceReport, PolicyLiveEpochEvidence,
};
pub use relevance::PolicyAwareLiveRelevanceContract;

#[cfg(test)]
mod tests;
