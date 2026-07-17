mod composition;
mod family;
mod invalidation;
mod policy;
mod resolution;
pub(crate) use composition::{replacement_activation_policy, UiAllocationResolvedCommitLane};
#[cfg(test)]
pub(crate) use composition::{resolve_stream_families, UiAllocationStreamCommitDecision};
pub use composition::{
    UiAllocationFamilyPairOutcome, UiAllocationIntermediatePolicyVerdict,
    UiAllocationStreamCompositionCounters, UiAllocationStreamCompositionDenial,
    UiResolvedAllocationPolicyBranch, UiResolvedAllocationStreamPolicy,
};
pub use family::UiAllocationStreamFamily;
pub(crate) use invalidation::UiAllocationFrameIngressRef;
pub use invalidation::{
    UiAllocationInvalidationFamily, UiAllocationInvalidationIntent,
    UiAllocationInvalidationReferenceDenial,
};
pub(crate) use policy::UiAllocationStreamPolicy;
pub use policy::{
    UiAllocationCadenceBudget, UiAllocationCadenceKind, UiAllocationCommitTarget,
    UiAllocationEvidenceCadence, UiAllocationPartialSettlementLaw, UiAllocationStreamCollapseLaw,
};
pub(crate) use resolution::{
    consume_pending_frame, UiAllocationFrameConsumptionDisposition, UiAllocationSourceOrderLedger,
};
pub use resolution::{
    UiAllocationDuplicatePosture, UiAllocationFrameCadenceVerdict, UiAllocationFramePlanIdentity,
    UiAllocationFrameRejection, UiAllocationFrameResolutionCounters,
    UiAllocationFrameResolutionDenial, UiAllocationIngressPolicyVerdict,
    UiAllocationSourceOrderVerdict, UiResolvedAllocationFramePlan,
};
