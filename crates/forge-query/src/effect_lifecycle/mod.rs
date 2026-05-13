mod admission;
mod authoring_basis;
mod batch;
mod batch_admission;
mod batch_execution;
mod certification;
mod counters;
mod diagnostics;
mod eligibility;
mod envelope;
mod execution;
mod execution_bridge;
mod execution_relational_batch;
mod execution_relational_scalar;
mod intent;
mod inventory;
mod inventory_rows;
mod lowering;
mod normalized;
mod oracle;
mod planning;
mod receipt;
mod support_contract;
mod support_matrix;
mod support_matrix_rows;
mod taxonomy;

pub use admission::{admit_effect_intent, evaluate_effect_eligibility};
pub use authoring_basis::EffectAuthoringBasis;
pub use batch::{
    LoweredEffectBatchExecutionArtifact, LoweredEffectBatchExecutionPlan,
    LoweredRelationalMutationBatchExecutionArtifact,
};
pub use batch_admission::{
    admit_effect_batch_components, effect_batch, AdmittedEffectBatch, EffectBatchAdmissionDenial,
    EffectBatchAdmissionDenialKind, EffectBatchIntentDraft, EffectBatchIntentDraftWithBasis,
};
pub use batch_execution::{
    EffectBatchExecutionDenial, EffectBatchExecutionDenialKind, ExecutedEffectBatchPlan,
};
#[allow(unused_imports)]
pub use certification::{
    certify_effect_lifecycle_phase4, certify_effect_lifecycle_seeded,
    EffectLifecyclePhase4CertificationBundle, EffectLifecyclePhase4CertificationRow,
    EffectLifecyclePhase4LaneKind, EffectLifecyclePhase4LaneOutcome,
    EffectLifecycleSeededCertificationBundle, EffectLifecycleSeededCertificationRow,
    EffectLifecycleSeededOutcomeClass,
};
pub use counters::EffectLifecycleCounters;
pub use diagnostics::{EffectDiagnosticsMaterialization, EffectDiagnosticsRequest};
pub use eligibility::{
    AdmittedEffectIntent, AdvisoryEffectEligibility, DeferredEffectEligibility,
    DeniedEffectEligibility, EffectEligibility, EffectEligibilityDecisionTrace,
    EffectEligibilityOutcome, RebindRequiredEffectEligibility,
};
pub use envelope::{EffectEnvelopePrimaryResult, SelfDescribingEffectEnvelope};
pub use execution::{
    execute_lowered_effect_plan, EffectExecutionAuthority, EffectExecutionDenial,
    EffectExecutionDenialKind, ExecutedEffectAuthorityArtifact, ExecutedEffectPlan,
};
pub use intent::{normalize_raw_effect_intent, RawEffectIntent};
pub use inventory::{
    effect_lifecycle_family_inventory, effect_lifecycle_family_row_for_key,
    effect_lifecycle_public_surface_inventory, effect_lifecycle_support_row_matches_inventory,
    effect_lifecycle_supported_basis_families, EffectLifecycleFamilyInventory,
    EffectLifecycleFamilyInventoryRow, EffectLifecycleFamilyKey,
    EffectLifecyclePublicSurfaceInventory, EffectLifecyclePublicSurfaceRow,
    EffectLoweredArtifactKind, EffectPublicSurfaceAvailability, EffectPublicSurfaceKind,
    EffectReceiptArtifactKind,
};
pub use lowering::{
    lower_authority_scoped_effect_plan, EffectLoweringDenial, EffectLoweringDenialKind,
    LoweredEffectExecutionArtifact, LoweredEffectExecutionPlan,
};
pub use normalized::{EffectIntentDenial, EffectOperationInput, NormalizedEffectIntent};
pub use oracle::{
    BridgeExecutionOracle, EffectExecutionOracleError, EffectExecutionOracleErrorKind,
    EffectExecutionOracleVerification, EffectExecutionOracleVerificationKind,
    RelationalExecutionOracle,
};
pub use planning::{
    scope_admitted_effect_plan, AuthorityScopedEffectPlan, EffectArtifactPolicy,
    EffectAuthorityOwner, EffectConflictFootprint, EffectInvariantScope,
    EffectPermittedLoweringFamily, EffectPolicyPosture, EffectPreviewPosture,
    EffectStrategyIdentityTarget,
};
pub use receipt::{
    EffectExecutionReceipt, EffectReceiptDecisionTrace, EffectReceiptIntegrityMarkers,
    EffectReceiptTargetEvidence,
};
pub use support_contract::{
    EffectDeferredNeighborFamily, EffectDeferredResiduePosture, EffectDeferredSupportContract,
};
pub use support_matrix::{
    discover_effect_lifecycle_support, effect_lifecycle_support_matrix,
    EffectLifecycleSupportDiscovery, EffectLifecycleSupportMatrix, EffectLifecycleSupportRow,
    EffectSupportCause, EffectSupportPosture,
};
pub use taxonomy::{
    DeniedEffectEligibilityKind, EffectAuthorityLane, EffectFamily, EffectIntentDenialKind,
};

#[cfg(test)]
mod tests;
