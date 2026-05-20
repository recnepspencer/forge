mod eligibility_evidence;
mod envelope;
mod evidence;
mod row;
mod stages;

pub(crate) const INTENT_ADMISSION_TRACE_MODULE_ROOT: &str = "intent_admission/trace/mod.rs";
pub(crate) const INTENT_ADMISSION_TRACE_CHILD_MODULES: &[&str] = &[
    "eligibility_evidence",
    "envelope",
    "evidence",
    "row",
    "stages",
];
pub(crate) const INTENT_ADMISSION_TRACE_EXPORTED_SURFACE: &[&str] = &[
    "ForgeQueryIntentEligibilityTraceEvidence",
    "ForgeQueryIntentDecisionTraceEnvelope",
    "ForgeQueryIntentDecisionTraceEvidence",
    "ForgeQueryIntentDecisionTraceEvidenceOwner",
    "ForgeQueryIntentDecisionTraceRow",
    "ForgeQueryIntentDecisionTraceEnvelopeKind",
    "ForgeQueryIntentDecisionTraceStage",
];

pub use eligibility_evidence::ForgeQueryIntentEligibilityTraceEvidence;
pub use envelope::ForgeQueryIntentDecisionTraceEnvelope;
pub use evidence::{
    ForgeQueryIntentDecisionTraceEvidence, ForgeQueryIntentDecisionTraceEvidenceOwner,
};
pub use row::ForgeQueryIntentDecisionTraceRow;
pub use stages::{ForgeQueryIntentDecisionTraceEnvelopeKind, ForgeQueryIntentDecisionTraceStage};
