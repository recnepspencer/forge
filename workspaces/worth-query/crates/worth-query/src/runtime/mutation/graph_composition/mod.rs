mod builder;
mod capability_families;
mod denial;
mod domain_invariant_denial;
mod existing_lifecycle;
mod hooks;
mod obligation;
mod relation_builder;
mod symbols;
mod touch_descriptor;

pub use builder::WorthQueryGraphCompositionBuilder;
pub(crate) use capability_families::{
    GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES, GRAPH_COMPOSITION_LIFECYCLE_FAMILIES,
    GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES,
};
pub(crate) use denial::graph_composition_error;
pub use denial::{WorthQueryGraphCompositionDenial, WorthQueryGraphCompositionDenialKind};
pub use domain_invariant_denial::WorthQueryGraphCompositionDomainInvariantDenial;
pub use hooks::{
    WorthQueryGraphCompositionInvariantPackContext,
    WorthQueryGraphCompositionInvariantPackViolation,
};
#[cfg(test)]
pub(crate) use obligation::registrations_from_relational_invariant_catalog;
pub use obligation::{
    WorthQueryAuthoritativeMutationObligationDispatch,
    WorthQueryAuthoritativeMutationObligationDispatchProjection,
    WorthQueryAuthoritativeMutationObligationDispatchProjectionRow,
    WorthQueryGraphMutationPolicyGateEvidence, WorthQueryGraphMutationPolicyGateVerdict,
    WorthQueryGraphObligationArtifactPolicy, WorthQueryGraphObligationAttachmentEvidence,
    WorthQueryGraphObligationBudgetExceededPolicy,
    WorthQueryGraphObligationDenialAttachmentProjection,
    WorthQueryGraphObligationDenialAttachmentProjectionRow,
    WorthQueryGraphObligationDenialProjection, WorthQueryGraphObligationDenialProjectionRow,
    WorthQueryGraphObligationDiagnosticMaterialization, WorthQueryGraphObligationDispatchContext,
    WorthQueryGraphObligationDispatchContextKind, WorthQueryGraphObligationDispatchEnvelope,
    WorthQueryGraphObligationDispatchEnvelopeBuilder, WorthQueryGraphObligationDispatchError,
    WorthQueryGraphObligationDispatchPlan, WorthQueryGraphObligationDispatchPlanDraft,
    WorthQueryGraphObligationExecutionBudget, WorthQueryGraphObligationExecutionContext,
    WorthQueryGraphObligationExecutionCostClass, WorthQueryGraphObligationExecutionInput,
    WorthQueryGraphObligationExecutionResultEnvelope, WorthQueryGraphObligationExecutionResultRow,
    WorthQueryGraphObligationExecutionScope, WorthQueryGraphObligationExecutionStatus,
    WorthQueryGraphObligationExecutorContract, WorthQueryGraphObligationIndex,
    WorthQueryGraphObligationIndexBuildCounters, WorthQueryGraphObligationIndexComplexityContract,
    WorthQueryGraphObligationIndexComplexityContractStatus, WorthQueryGraphObligationIndexEntry,
    WorthQueryGraphObligationIndexSupportRow, WorthQueryGraphObligationIndexSupportStatus,
    WorthQueryGraphObligationKind, WorthQueryGraphObligationMaterializedDispatch,
    WorthQueryGraphObligationMatrixCertificationCase,
    WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldDescriptorKind,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationPreflightWitness,
    WorthQueryGraphObligationReduction, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRegistrationCatalog, WorthQueryGraphObligationRegistrationDenial,
    WorthQueryGraphObligationRegistrationDenialKind, WorthQueryGraphObligationRuleIdentity,
    WorthQueryGraphObligationSelection, WorthQueryGraphObligationSelectionCounters,
    WorthQueryGraphObligationSelectorPerturbationCase, WorthQueryGraphObligationStateAccessPolicy,
    WorthQueryGraphObligationStateLoadCounters, WorthQueryGraphObligationStateLoadPlan,
    WorthQueryGraphObligationSupportLane, WorthQueryGraphObligationSupportMatrix,
    WorthQueryGraphObligationSupportMatrixRow, WorthQueryGraphObligationSupportPosture,
    WorthQueryGraphObligationSupportStatus, WorthQueryGraphObligationVerdict,
    WorthQueryGraphScopedCustomInvariantRegistration, WorthQueryGraphTouchSelector,
    WORTH_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME,
};
pub use relation_builder::WorthQueryGraphRelationMutationBuilder;
pub use symbols::{WorthQueryGraphEntitySymbol, WorthQueryGraphRelationSymbol};
pub use touch_descriptor::{
    WorthQueryGraphReadTouchShape, WorthQueryGraphTouchDescriptor,
    WorthQueryGraphTouchDescriptorDenial, WorthQueryGraphTouchDescriptorDenialKind,
    WorthQueryGraphTouchDescriptorKind, WorthQueryGraphTouchDescriptorRow,
    WorthQueryGraphTouchLifecycleFamily, WorthQueryGraphTouchReadVerb,
};
