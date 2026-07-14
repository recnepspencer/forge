mod artifact;
mod facts;
mod request;
mod resolution;
mod seeds;

pub(crate) const INTENT_ADMISSION_ELIGIBILITY_MODULE_ROOT: &str =
    "intent_admission/eligibility/mod.rs";
pub(crate) const INTENT_ADMISSION_ELIGIBILITY_CHILD_MODULES: &[&str] =
    &["artifact", "facts", "request", "resolution", "seeds"];
pub(crate) const INTENT_ADMISSION_ELIGIBILITY_EXPORTED_SURFACE: &[&str] = &[
    "WorthQueryIntentAdmissionEligibility",
    "WorthQueryIntentAdmissionPreDecisionPosture",
    "WorthQueryIntentAdmissionAuthorityLaneEligibility",
    "WorthQueryIntentAdmissionBasisEligibility",
    "WorthQueryIntentAdmissionCapabilityEligibility",
    "WorthQueryIntentAdmissionInvariantEligibility",
    "WorthQueryIntentAdmissionPolicyEligibility",
    "WorthQueryIntentAdmissionProjectionSourceEligibility",
    "WorthQueryIntentAdmissionRoutingSupportEligibility",
    "WorthQueryIntentAdmissionSourceLaneEligibility",
    "WorthQueryIntentAdmissionSupportEligibility",
    "WorthQueryRawIntentAdmissionRequest",
    "WorthQueryAuthoritativeMutationBatchIntentSeed",
    "WorthQueryAuthoritativeMutationIntentSeed",
    "WorthQueryAuthoritativeMutationPreflight",
    "WorthQueryDerivedViewIntentSeed",
    "WorthQueryExistingTruthProbeIntentSeed",
    "WorthQueryExistingTruthProbeRoutingPreflight",
    "WorthQueryGenericInspectionIntentSeed",
    "WorthQueryGenericInspectionIntentTarget",
    "WorthQueryGenericInspectionIntentTargetSeed",
    "WorthQueryGenericInspectionRequestLabel",
    "WorthQueryLiveReadIntentSeed",
    "WorthQueryReadExecutionIntentSeed",
];

pub use artifact::{
    WorthQueryIntentAdmissionEligibility, WorthQueryIntentAdmissionPreDecisionPosture,
};
pub use facts::{
    WorthQueryIntentAdmissionAuthorityLaneEligibility, WorthQueryIntentAdmissionBasisEligibility,
    WorthQueryIntentAdmissionCapabilityEligibility, WorthQueryIntentAdmissionInvariantEligibility,
    WorthQueryIntentAdmissionPolicyEligibility,
    WorthQueryIntentAdmissionProjectionSourceEligibility,
    WorthQueryIntentAdmissionRoutingSupportEligibility,
    WorthQueryIntentAdmissionSourceLaneEligibility, WorthQueryIntentAdmissionSupportEligibility,
};
pub use request::WorthQueryRawIntentAdmissionRequest;
pub(crate) use seeds::authoritative_mutation_input_identity;
pub use seeds::{
    WorthQueryAuthoritativeMutationBatchIntentSeed, WorthQueryAuthoritativeMutationIntentSeed,
    WorthQueryAuthoritativeMutationPreflight, WorthQueryDerivedViewIntentSeed,
    WorthQueryExistingTruthProbeIntentSeed, WorthQueryExistingTruthProbeRoutingPreflight,
    WorthQueryGenericInspectionIntentSeed, WorthQueryGenericInspectionIntentTarget,
    WorthQueryGenericInspectionIntentTargetSeed, WorthQueryGenericInspectionRequestLabel,
    WorthQueryLiveReadIntentSeed, WorthQueryReadExecutionIntentSeed,
};
