mod admission;
mod admitted_capability;
mod canonical_basis;
mod counters;
mod decision_trace;
mod declarative;
mod dx;
mod intent;
mod lanes;
mod lower_runtime;
mod normalized_intent;
mod proofs;
mod receipts;
mod scoping;
mod support;
mod taxonomy;

pub use admission::{
    admit_basis_capability, evaluate_basis_certification_eligibility,
    evaluate_basis_effect_authoring_deferred_eligibility,
    evaluate_basis_inspection_advisory_eligibility, evaluate_basis_inspection_eligibility,
    evaluate_basis_materialization_eligibility, evaluate_basis_mutation_preparation_eligibility,
    evaluate_basis_observation_eligibility, evaluate_basis_preview_closeout_eligibility,
    evaluate_basis_replay_eligibility, evaluate_basis_subscription_activation_eligibility,
    evaluate_basis_subscription_declaration_eligibility,
};
pub use admitted_capability::AdmittedBasisCapability;
pub use counters::BasisEligibilityCounters;
pub use decision_trace::BasisEligibilityDecisionTrace;
pub use declarative::*;
pub use dx::{
    basis_lifecycle, basis_lifecycle_dx_certification_digest, BasisLifecycleIntentBuilder,
    BasisLifecycleIntentDraft, BasisLifecyclePolicyIntentDraft, InspectionAdvisoryBasisPath,
    MutationPreparationBasisAdmissionPath, ObservationBasisAdmissionPath,
    ObservationBasisReceiptPath, ObservationBasisUsePath,
};
pub use intent::{normalize_raw_basis_intent, RawBasisIntent};
pub use lanes::{
    BasisOperationLane, CertificationLaneWitness, EffectAuthoringLaneWitness,
    InspectionLaneWitness, MaterializationLaneWitness, MutationPreparationLaneWitness,
    ObservationLaneWitness, PreviewCloseoutLaneWitness, ReplayLaneWitness,
    SubscriptionActivationLaneWitness, SubscriptionDeclarationLaneWitness,
};
pub use lower_runtime::{
    readmit_lower_runtime_evidence, LowerRuntimeBasisEvidence, LowerRuntimeBoundBasis,
    LowerRuntimeEvidenceAuthority,
};
pub use normalized_intent::NormalizedBasisIntent;
pub use proofs::{
    AdvisoryBasisEligibility, BasisEligibility, BasisIntentDenial, DeferredBasisEligibility,
    DeniedBasisCapability,
};
pub use receipts::{
    emit_certification_basis_receipt, emit_inspection_basis_receipt,
    emit_materialization_basis_receipt, emit_mutation_preparation_basis_receipt,
    emit_observation_basis_receipt, emit_preview_closeout_basis_receipt, emit_replay_basis_receipt,
    emit_subscription_activation_basis_receipt, emit_subscription_declaration_basis_receipt,
    envelope_basis_use, BasisNextTransition, BasisUseReceipt, BasisUseReceiptKind,
    SelfDescribingBasisEnvelope,
};
pub use scoping::{
    activate_subscription_basis, scope_basis_for_certification, scope_basis_for_inspection,
    scope_basis_for_materialization, scope_basis_for_mutation_preparation,
    scope_basis_for_observation, scope_basis_for_preview_closeout, scope_basis_for_replay,
    scope_basis_for_subscription_activation, scope_basis_for_subscription_declaration,
    ScopedBasisProof, ScopedCertificationBasis, ScopedInspectionBasis, ScopedMaterializationBasis,
    ScopedMutationPreparationBasis, ScopedObservationBasis, ScopedPreviewCloseoutBasis,
    ScopedReplayBasis, ScopedSubscriptionActivationBasis, ScopedSubscriptionDeclarationBasis,
};
pub use support::{
    basis_lifecycle_support_matrix, discover_basis_lifecycle_support,
    BasisLifecycleSupportDiscovery, BasisLifecycleSupportMatrix, BasisLifecycleSupportRow,
    BasisSupportPosture,
};
pub use taxonomy::{
    BasisAuthorityPosture, BasisEligibilityDenialCause, BasisFamily, BasisIntentDenialKind,
    BasisLifecyclePosture, BasisScopePosture, BasisVisibilityPosture, DeniedBasisCapabilityKind,
};

#[cfg(test)]
mod tests;
