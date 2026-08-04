pub(crate) mod domain_capability;
mod scoped;
#[cfg(test)]
mod scoped_tests;
mod workflow_context_identity;

mod binding;
mod comparison;
mod evaluation;
mod execution;
mod live;
mod session_context;
#[cfg(test)]
mod tests;
mod workflow_foundation;

pub(crate) use binding::bind_preflight_to_preview_session;
pub use binding::{
    PreviewBindingCounters, PreviewBindingError, PreviewBindingFailureClass, PreviewBindingReport,
    PreviewComplexityContract, PreviewLifecycleMetadata, PreviewPerformanceStatusMarker,
    PreviewSessionBasis, PreviewSessionBindingTuple, PreviewSessionPlanBinding,
    PromotionEligiblePreviewSessionPlanBinding, ReadOnlyPreviewSessionPlanBinding,
};
pub use evaluation::{
    PreviewEvaluationClass, PromotionEligiblePreviewEvaluation, ReadOnlyPreviewEvaluation,
};

#[cfg(test)]
pub(crate) use live::{
    admit_preview_live_session_plan, execute_preview_live_session_plan,
    preview_live_execution_counters,
};
pub(crate) use live::{admit_preview_live_session_plan_component, PreviewLiveSessionPlanBinding};
pub use live::{
    assess_preview_live_drift, PreviewLiveAdmissionReport, PreviewLiveCounters,
    PreviewLiveDriftDenied, PreviewLiveDriftOutcome, PreviewLiveError,
    PreviewLiveExecutionEnvelope, PreviewLiveFailureClass, PreviewLiveMaintained,
    PreviewLiveRebindArtifact,
};

#[cfg(test)]
pub(crate) use execution::{
    admit_promotion_eligible_preview_session_plan_binding,
    admit_read_only_preview_session_plan_binding, execute_preview_session_plan,
    execute_promotion_eligible_preview_session_plan, execute_read_only_preview_session_plan,
};
#[allow(unused_imports)]
pub use execution::{
    PreviewComparisonCounters, PreviewExecutionCounters, PreviewExecutionEnvelope,
    PreviewExecutionError, PreviewExecutionFailureClass, PreviewExecutionReport,
    PromotionEligiblePreviewExecutionEnvelope, ReadOnlyPreviewExecutionEnvelope,
};

pub use session_context::PreviewSessionQueryContext;

#[cfg(test)]
pub(crate) use comparison::{
    admit_authoritative_preview_comparison_candidate, admit_preview_promotion_parity_comparison,
    derive_preview_comparison_eligibility,
};
#[allow(unused_imports)]
pub use comparison::{
    AuthoritativePreviewComparisonCandidate, PreviewComparisonCandidateArtifact,
    PreviewComparisonEligibilityArtifact, PreviewComparisonError, PreviewComparisonFailureClass,
    PreviewExecutionComparisonAdmission, PromotionParityPreviewComparisonAdmission,
};

#[cfg(test)]
pub(crate) use workflow_foundation::{
    admit_preview_workflow_foundation, admit_preview_workflow_foundation_request,
};
pub use workflow_foundation::{
    AdmittedPreviewWorkflowFoundation, PreviewWorkflowFoundationArtifact,
    PreviewWorkflowFoundationError, PreviewWorkflowFoundationFailureClass,
    PreviewWorkflowFoundationRequest,
};

pub(crate) use domain_capability::{
    admit_contributed_preview_workflow_foundation,
    materialize_contributed_preview_workflow_foundation_artifact,
};
pub(crate) use scoped::{
    admit_scoped_preview_live_session_plan,
    admit_scoped_preview_session_plan_binding_from_preview_binding,
};
#[cfg(test)]
pub(crate) use scoped::{
    admit_scoped_preview_session_plan_binding, execute_scoped_preview_live_session_plan,
    scoped_observation_basis_for_preview_binding,
};
pub use scoped::{ScopedPreviewLiveSessionPlanBinding, ScopedPreviewSessionPlanBinding};
pub(crate) use workflow_context_identity::preview_lifecycle_state_label;
