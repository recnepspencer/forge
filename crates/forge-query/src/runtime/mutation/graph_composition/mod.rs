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

pub use builder::ForgeQueryGraphCompositionBuilder;
pub(crate) use capability_families::{
    GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES, GRAPH_COMPOSITION_LIFECYCLE_FAMILIES,
    GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES,
};
pub(crate) use denial::graph_composition_error;
pub use denial::{ForgeQueryGraphCompositionDenial, ForgeQueryGraphCompositionDenialKind};
pub use domain_invariant_denial::ForgeQueryGraphCompositionDomainInvariantDenial;
pub use hooks::{
    ForgeQueryGraphCompositionInvariantPackContext,
    ForgeQueryGraphCompositionInvariantPackViolation,
};
pub(crate) use obligation::registrations_from_relational_invariant_catalog;
pub use obligation::{
    ForgeQueryAuthoritativeMutationObligationDispatch,
    ForgeQueryAuthoritativeMutationObligationDispatchProjection,
    ForgeQueryAuthoritativeMutationObligationDispatchProjectionRow,
    ForgeQueryGraphMutationPolicyGateEvidence, ForgeQueryGraphMutationPolicyGateVerdict,
    ForgeQueryGraphObligationArtifactPolicy, ForgeQueryGraphObligationAttachmentEvidence,
    ForgeQueryGraphObligationBudgetExceededPolicy,
    ForgeQueryGraphObligationDenialAttachmentProjection,
    ForgeQueryGraphObligationDenialAttachmentProjectionRow,
    ForgeQueryGraphObligationDenialProjection, ForgeQueryGraphObligationDenialProjectionRow,
    ForgeQueryGraphObligationDiagnosticMaterialization, ForgeQueryGraphObligationDispatchContext,
    ForgeQueryGraphObligationDispatchContextKind, ForgeQueryGraphObligationDispatchEnvelope,
    ForgeQueryGraphObligationDispatchEnvelopeBuilder, ForgeQueryGraphObligationDispatchError,
    ForgeQueryGraphObligationDispatchPlan, ForgeQueryGraphObligationDispatchPlanDraft,
    ForgeQueryGraphObligationExecutionBudget, ForgeQueryGraphObligationExecutionContext,
    ForgeQueryGraphObligationExecutionCostClass, ForgeQueryGraphObligationExecutionInput,
    ForgeQueryGraphObligationExecutionResultEnvelope, ForgeQueryGraphObligationExecutionResultRow,
    ForgeQueryGraphObligationExecutionScope, ForgeQueryGraphObligationExecutionStatus,
    ForgeQueryGraphObligationExecutorContract, ForgeQueryGraphObligationIndex,
    ForgeQueryGraphObligationIndexBuildCounters, ForgeQueryGraphObligationIndexComplexityContract,
    ForgeQueryGraphObligationIndexComplexityContractStatus, ForgeQueryGraphObligationIndexEntry,
    ForgeQueryGraphObligationIndexSupportRow, ForgeQueryGraphObligationIndexSupportStatus,
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationMaterializedDispatch,
    ForgeQueryGraphObligationMatrixCertificationCase,
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldDescriptorKind,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationPreflightWitness,
    ForgeQueryGraphObligationReduction, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphObligationRegistrationDenial,
    ForgeQueryGraphObligationRegistrationDenialKind, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphObligationSelection, ForgeQueryGraphObligationSelectionCounters,
    ForgeQueryGraphObligationSelectorPerturbationCase, ForgeQueryGraphObligationStateAccessPolicy,
    ForgeQueryGraphObligationStateLoadCounters, ForgeQueryGraphObligationStateLoadPlan,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportMatrix,
    ForgeQueryGraphObligationSupportMatrixRow, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphObligationSupportStatus, ForgeQueryGraphObligationVerdict,
    ForgeQueryGraphScopedCustomInvariantRegistration, ForgeQueryGraphTouchSelector,
    FORGE_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME,
};
pub use relation_builder::ForgeQueryGraphRelationMutationBuilder;
pub use symbols::{ForgeQueryGraphEntitySymbol, ForgeQueryGraphRelationSymbol};
pub use touch_descriptor::{
    ForgeQueryGraphReadTouchShape, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchDescriptorDenialKind,
    ForgeQueryGraphTouchDescriptorKind, ForgeQueryGraphTouchDescriptorRow,
    ForgeQueryGraphTouchLifecycleFamily, ForgeQueryGraphTouchReadVerb,
};
