mod admitted;
mod admitted_dispatch;
mod bindings;
mod execution_admission;
mod inspection;
mod mutation;
mod plan_conversion;
mod read;
mod routing;
mod runtime_intents;
mod unified_inspection;

pub(crate) const INTENT_ADMISSION_HANDOFFS_MODULE_ROOT: &str = "intent_admission/handoffs/mod.rs";
pub(crate) const INTENT_ADMISSION_HANDOFFS_CHILD_MODULES: &[&str] = &[
    "admitted",
    "admitted_dispatch",
    "bindings",
    "execution_admission",
    "inspection",
    "mutation",
    "plan_conversion",
    "read",
    "routing",
    "runtime_intents",
    "unified_inspection",
];
pub(crate) const INTENT_ADMISSION_HANDOFFS_EXPORTED_SURFACE: &[&str] = &[
    "WorthQueryAuthoritativeIntentExecutionBinding",
    "WorthQueryAuthoritativeMutationBatchExecutionBinding",
    "WorthQueryAuthoritativeMutationExecutionBinding",
    "WorthQueryDerivedInspectionExecutionBinding",
    "WorthQueryDerivedMaterializationExecutionBinding",
    "WorthQueryEffectTriggeredIntentExecutionBinding",
    "WorthQueryExistingTruthProbeExecutionBinding",
    "WorthQueryLiveReadExecutionBinding",
    "WorthQueryReadExecutionBinding",
    "WorthQueryUnifiedInspectionExecutionBinding",
    "WorthQueryDerivedInspectionExecutionHandoff",
    "WorthQueryDerivedMaterializationExecutionHandoff",
    "WorthQueryAuthoritativeMutationBatchExecutionHandoff",
    "WorthQueryAuthoritativeMutationExecutionHandoff",
    "WorthQueryLiveReadExecutionHandoff",
    "WorthQueryReadExecutionHandoff",
    "WorthQueryExistingTruthProbeExecutionHandoff",
    "WorthQueryUnifiedInspectionExecutionHandoff",
    "WorthQueryAuthoritativeIntentExecutionHandoff",
    "WorthQueryEffectTriggeredIntentExecutionHandoff",
    "WorthQueryAdmittedIntentExecutionHandoff",
];

use super::{
    WorthQueryAdmittedIntentPlan, WorthQueryAuthoritativeIntentExecutionPlan,
    WorthQueryEffectTriggeredIntentExecutionPlan, WorthQueryIntentAdmissionCoveredEntrypoint,
    WorthQueryIntentAdmissionExecutionSeam, WorthQueryIntentAdmissionFamily,
    WorthQueryIntentEligibilityTraceEvidence, WorthQueryIntentViolationDecision,
};

pub use admitted::WorthQueryAdmittedIntentExecutionHandoff;
pub use bindings::{
    WorthQueryAuthoritativeIntentExecutionBinding,
    WorthQueryAuthoritativeMutationBatchExecutionBinding,
    WorthQueryAuthoritativeMutationExecutionBinding, WorthQueryDerivedInspectionExecutionBinding,
    WorthQueryDerivedMaterializationExecutionBinding,
    WorthQueryEffectTriggeredIntentExecutionBinding, WorthQueryExistingTruthProbeExecutionBinding,
    WorthQueryLiveReadExecutionBinding, WorthQueryReadExecutionBinding,
    WorthQueryUnifiedInspectionExecutionBinding,
};
pub use inspection::{
    WorthQueryDerivedInspectionExecutionHandoff, WorthQueryDerivedMaterializationExecutionHandoff,
};
pub use mutation::{
    WorthQueryAuthoritativeMutationBatchExecutionHandoff,
    WorthQueryAuthoritativeMutationExecutionHandoff,
};
pub use read::{WorthQueryLiveReadExecutionHandoff, WorthQueryReadExecutionHandoff};
pub use routing::WorthQueryExistingTruthProbeExecutionHandoff;
pub use runtime_intents::{
    WorthQueryAuthoritativeIntentExecutionHandoff, WorthQueryEffectTriggeredIntentExecutionHandoff,
};
pub use unified_inspection::WorthQueryUnifiedInspectionExecutionHandoff;

pub(crate) use execution_admission::{admit_authoritative_execution, admit_effect_execution};
