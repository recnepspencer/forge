mod admission;
mod confidence;
mod denial;
mod evidence_basis;
mod media_assumptions;
mod rebind_triggers;
mod request;
mod support;
mod vocabulary;
mod witness;

pub use admission::PhysicalBackendCapabilityAdmissionAuthority;
pub use confidence::CapabilityConfidenceLimits;
pub use denial::{
    reject_certification_only_evidence, reject_copied_qualification_row,
    reject_environment_variable, reject_raw_backend_label, reject_raw_config_string,
    reject_raw_os_name, reject_raw_probe_observation, reject_same_process_metric_projection,
    reject_terminal_projection, BackendCapabilityAdmissionDenial,
};
pub use evidence_basis::BackendCapabilityEvidenceBasis;
pub use media_assumptions::BackendMediaAssumptionSet;
pub use rebind_triggers::BackendRebindTriggers;
pub use request::BackendCapabilityAdmissionRequest;
pub use support::BackendCapabilitySupportSet;
pub use vocabulary::{
    BackendCapabilityKind, BackendCapabilitySupportPosture, BackendTargetProfile,
    CapabilityConfidenceScope, CapabilityEvidenceClass, CapabilityResidualRisk,
};
pub use witness::{
    AdmittedBackendCapabilityWitness, BackendCapabilityClaimOutcome, BackendCapabilityClaimWitness,
    BackendCapabilityRebindRequired, BackendCapabilityStale,
};

#[cfg(test)]
mod tests;
