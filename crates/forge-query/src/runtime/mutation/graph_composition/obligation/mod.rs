mod admission;
mod attachment;
mod budget;
mod dispatch;
mod error;
mod execution;
mod execution_status;
mod index;
mod kind;
mod policy_gate;
mod registration;
mod rule_identity;
mod support_matrix;
mod verdict;

pub use admission::{
    ForgeQueryAuthoritativeMutationObligationDispatch,
    ForgeQueryAuthoritativeMutationObligationDispatchProjection,
    ForgeQueryAuthoritativeMutationObligationDispatchProjectionRow,
};
pub use attachment::{
    ForgeQueryGraphObligationAttachmentEvidence,
    ForgeQueryGraphObligationDenialAttachmentProjection,
    ForgeQueryGraphObligationDenialAttachmentProjectionRow,
};
pub use budget::{
    ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationExecutionBudget,
    ForgeQueryGraphObligationExecutionCostClass, ForgeQueryGraphObligationExecutionScope,
};
pub use dispatch::{
    ForgeQueryGraphObligationDispatchContext, ForgeQueryGraphObligationDispatchContextKind,
    ForgeQueryGraphObligationDispatchEnvelope, ForgeQueryGraphObligationDispatchEnvelopeBuilder,
    ForgeQueryGraphObligationDispatchPlan, ForgeQueryGraphObligationDispatchPlanDraft,
    ForgeQueryGraphObligationMaterializedDispatch,
    FORGE_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME,
};
pub use error::ForgeQueryGraphObligationDispatchError;
pub use execution::{
    ForgeQueryGraphObligationArtifactPolicy, ForgeQueryGraphObligationDenialProjection,
    ForgeQueryGraphObligationDenialProjectionRow,
    ForgeQueryGraphObligationDiagnosticMaterialization, ForgeQueryGraphObligationExecutionContext,
    ForgeQueryGraphObligationExecutionInput, ForgeQueryGraphObligationExecutionResultEnvelope,
    ForgeQueryGraphObligationExecutionResultRow, ForgeQueryGraphObligationExecutorContract,
    ForgeQueryGraphObligationPreflightWitness, ForgeQueryGraphObligationReduction,
    ForgeQueryGraphObligationStateAccessPolicy, ForgeQueryGraphObligationStateLoadCounters,
    ForgeQueryGraphObligationStateLoadPlan,
};
pub use execution_status::ForgeQueryGraphObligationExecutionStatus;
pub use index::{
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationIndexBuildCounters,
    ForgeQueryGraphObligationIndexComplexityContract,
    ForgeQueryGraphObligationIndexComplexityContractStatus, ForgeQueryGraphObligationIndexEntry,
    ForgeQueryGraphObligationIndexSupportRow, ForgeQueryGraphObligationIndexSupportStatus,
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldDescriptorKind, ForgeQueryGraphObligationSelection,
    ForgeQueryGraphObligationSelectionCounters,
};
pub use kind::ForgeQueryGraphObligationKind;
pub use policy_gate::{
    ForgeQueryGraphMutationPolicyGateEvidence, ForgeQueryGraphMutationPolicyGateVerdict,
};
pub(crate) use registration::registrations_from_relational_invariant_catalog;
pub use registration::{
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphObligationRegistrationDenial,
    ForgeQueryGraphObligationRegistrationDenialKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportPosture, ForgeQueryGraphObligationSupportStatus,
    ForgeQueryGraphScopedCustomInvariantRegistration, ForgeQueryGraphTouchSelector,
};
pub use rule_identity::ForgeQueryGraphObligationRuleIdentity;
pub use support_matrix::{
    ForgeQueryGraphObligationMatrixCertificationCase,
    ForgeQueryGraphObligationSelectorPerturbationCase, ForgeQueryGraphObligationSupportMatrix,
    ForgeQueryGraphObligationSupportMatrixRow,
};
pub use verdict::ForgeQueryGraphObligationVerdict;

#[cfg(test)]
mod tests;
