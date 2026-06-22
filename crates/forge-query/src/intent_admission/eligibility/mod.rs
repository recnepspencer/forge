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
    "ForgeQueryIntentAdmissionEligibility",
    "ForgeQueryIntentAdmissionPreDecisionPosture",
    "ForgeQueryIntentAdmissionAuthorityLaneEligibility",
    "ForgeQueryIntentAdmissionBasisEligibility",
    "ForgeQueryIntentAdmissionCapabilityEligibility",
    "ForgeQueryIntentAdmissionInvariantEligibility",
    "ForgeQueryIntentAdmissionPolicyEligibility",
    "ForgeQueryIntentAdmissionProjectionSourceEligibility",
    "ForgeQueryIntentAdmissionRoutingSupportEligibility",
    "ForgeQueryIntentAdmissionSourceLaneEligibility",
    "ForgeQueryIntentAdmissionSupportEligibility",
    "ForgeQueryRawIntentAdmissionRequest",
    "ForgeQueryAuthoritativeMutationBatchIntentSeed",
    "ForgeQueryAuthoritativeMutationIntentSeed",
    "ForgeQueryAuthoritativeMutationPreflight",
    "ForgeQueryDerivedViewIntentSeed",
    "ForgeQueryExistingTruthProbeIntentSeed",
    "ForgeQueryExistingTruthProbeRoutingPreflight",
    "ForgeQueryGenericInspectionIntentSeed",
    "ForgeQueryGenericInspectionIntentTarget",
    "ForgeQueryGenericInspectionIntentTargetSeed",
    "ForgeQueryGenericInspectionRequestLabel",
    "ForgeQueryLiveReadIntentSeed",
    "ForgeQueryReadExecutionIntentSeed",
];

pub use artifact::{
    ForgeQueryIntentAdmissionEligibility, ForgeQueryIntentAdmissionPreDecisionPosture,
};
pub use facts::{
    ForgeQueryIntentAdmissionAuthorityLaneEligibility, ForgeQueryIntentAdmissionBasisEligibility,
    ForgeQueryIntentAdmissionCapabilityEligibility, ForgeQueryIntentAdmissionInvariantEligibility,
    ForgeQueryIntentAdmissionPolicyEligibility,
    ForgeQueryIntentAdmissionProjectionSourceEligibility,
    ForgeQueryIntentAdmissionRoutingSupportEligibility,
    ForgeQueryIntentAdmissionSourceLaneEligibility, ForgeQueryIntentAdmissionSupportEligibility,
};
pub use request::ForgeQueryRawIntentAdmissionRequest;
pub use seeds::{
    ForgeQueryAuthoritativeMutationBatchIntentSeed, ForgeQueryAuthoritativeMutationIntentSeed,
    ForgeQueryAuthoritativeMutationPreflight, ForgeQueryDerivedViewIntentSeed,
    ForgeQueryExistingTruthProbeIntentSeed, ForgeQueryExistingTruthProbeRoutingPreflight,
    ForgeQueryGenericInspectionIntentSeed, ForgeQueryGenericInspectionIntentTarget,
    ForgeQueryGenericInspectionIntentTargetSeed, ForgeQueryGenericInspectionRequestLabel,
    ForgeQueryLiveReadIntentSeed, ForgeQueryReadExecutionIntentSeed,
};
