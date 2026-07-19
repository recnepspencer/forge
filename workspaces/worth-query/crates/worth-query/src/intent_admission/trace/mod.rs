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
    "WorthQueryIntentEligibilityTraceEvidence",
    "WorthQueryIntentDecisionTraceEnvelope",
    "WorthQueryIntentDecisionTraceEvidence",
    "WorthQueryIntentDecisionTraceEvidenceOwner",
    "WorthQueryIntentDecisionTraceRow",
    "WorthQueryIntentDecisionTraceEnvelopeKind",
    "WorthQueryIntentDecisionTraceStage",
];

pub use eligibility_evidence::WorthQueryIntentEligibilityTraceEvidence;
pub use envelope::WorthQueryIntentDecisionTraceEnvelope;
pub use evidence::{
    WorthQueryIntentDecisionTraceEvidence, WorthQueryIntentDecisionTraceEvidenceOwner,
};
pub use row::WorthQueryIntentDecisionTraceRow;
pub use stages::{WorthQueryIntentDecisionTraceEnvelopeKind, WorthQueryIntentDecisionTraceStage};
