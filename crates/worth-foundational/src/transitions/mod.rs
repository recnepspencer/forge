mod basis;
mod branches;
mod commits;
mod merges;
mod readiness;
mod receipts;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionProductionTestReady;
impl worth_proof::PhaseMarker for FoundationalTransitionProductionTestReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionProductionReadinessCertified;
impl worth_proof::ProofMarker for FoundationalTransitionProductionReadinessCertified {}

pub use basis::{
    admit_current_basis_commit_receipt, admit_current_basis_committed_authority,
    attach_boundary_profiled_branch_candidate, attach_boundary_profiled_staged_branch,
    attach_proof_bearing_profiled_commit_receipt,
    attach_proof_bearing_profiled_committed_authority, attach_support_profiled_merge_verdict,
    bridge_current_basis_commit_receipt_trust_boundary,
    bridge_current_basis_committed_authority_trust_boundary,
    foundational_transition_canonical_basis_entries,
    foundational_transition_current_basis_authority,
    foundational_transition_current_basis_readmission_authority,
    prepare_admitted_merge_scope_for_canonical_basis, prepare_branch_candidate_for_canonical_basis,
    prepare_commit_receipt_for_canonical_basis, prepare_committed_authority_for_canonical_basis,
    prepare_merge_scope_for_canonical_basis, prepare_merge_verdict_for_canonical_basis,
    prepare_scoped_merge_denial_for_canonical_basis,
    prepare_scoped_merge_unavailable_for_canonical_basis,
    prepare_staged_branch_for_canonical_basis, readmit_current_basis_commit_receipt_after_boundary,
    readmit_current_basis_committed_authority_after_boundary,
    BoundaryBridgedCurrentBasisCommitReceiptArtifact,
    BoundaryBridgedCurrentBasisCommittedAuthorityArtifact, CurrentBasisCommitReceiptArtifact,
    CurrentBasisCommittedAuthorityArtifact, CurrentBasisTransitionPhase,
    FoundationalTransitionCurrentBasisAuthority,
    FoundationalTransitionCurrentBasisReadmissionAuthority,
};
pub use branches::{
    foundational_branch_candidate, foundational_branch_local_state_definitions,
    FoundationalBranchCandidateArtifact, FoundationalBranchCandidateBuilder,
    FoundationalBranchCandidateComparisonBasis, FoundationalBranchCandidateForkBasis,
    FoundationalBranchCandidateForkObservationBasis, FoundationalBranchCandidateId,
    FoundationalBranchCandidateObservationBasis, FoundationalBranchComparisonBasis,
    FoundationalBranchForkBasis, FoundationalBranchId, FoundationalBranchIdConstructionDenial,
    FoundationalBranchLocalConstructionDenial, FoundationalBranchLocalStateDefinition,
    FoundationalBranchLocalStateKind, FoundationalBranchReferenceGeneration,
    FoundationalBranchReferenceGenerationAdvanceDenial, FoundationalBranchReferenceMismatch,
    FoundationalBranchReferenceMismatchAxis, FoundationalBranchReferenceMovement,
    FoundationalBranchReferenceMovementKind, FoundationalBranchReferenceObservation,
    FoundationalBranchTarget, FoundationalBranchTargetBasis, FoundationalBranchTargetEncoding,
    FoundationalBranchTargetEncodingConstructionDenial, FoundationalStagedBranchArtifact,
};
pub use commits::{
    foundational_committed_authority_admission, FoundationalAuthorityTransitionClass,
    FoundationalAuthorityTransitionDenial, FoundationalAuthorityTransitionOutcomeKind,
    FoundationalCommitDeltaSummary, FoundationalCommitParentBasis, FoundationalCommitParentage,
    FoundationalCommittedAuthorityAdmission, FoundationalCommittedAuthorityAdmissionBasis,
    FoundationalCommittedAuthorityArtifact, FoundationalCommittedAuthorityConstructionDenial,
    FoundationalCommittedAuthorityInput, FoundationalCommittedAuthorityPhase,
    FoundationalCommittedDeltaLocus, FoundationalMergeAncestryBasis, FoundationalNoOpCause,
};
pub use merges::{
    foundational_merge, prepare_scoped_merge_diagnostic_explanation,
    FoundationalAdmittedMergeScopeEvidence, FoundationalBranchBasisDrift,
    FoundationalBranchBasisDriftKind, FoundationalDeniedScopeLocus,
    FoundationalMergeAdmissionDeferred, FoundationalMergeAdmissionDenial,
    FoundationalMergeAdmissionFailure, FoundationalMergeAdmissionOutcome,
    FoundationalMergeAdmissionRebindRequired, FoundationalMergeBaseSelectionBasis,
    FoundationalMergeBasis, FoundationalMergeBuilder, FoundationalMergeCandidate,
    FoundationalMergeConflictLocus, FoundationalMergeConstructionDenial, FoundationalMergeIntent,
    FoundationalMergeScope, FoundationalMergeScopeFamily, FoundationalMergeStructuralSummary,
    FoundationalMergeVerdict, FoundationalMergeVerdictKind, FoundationalScopeAdmissionBasis,
    FoundationalScopeBreadthSummary, FoundationalScopedMergeDenialEvidence,
    FoundationalScopedMergeDenialKind, FoundationalScopedMergeDiagnosticInput,
    FoundationalScopedMergeUnavailableOutcomeCategory, FoundationalScopedMergeUnavailablePosture,
    FoundationalScopedMergeUnavailableReason, FoundationalSelectedAspectLocus,
    FoundationalSelectedAspectRequestEntry, FoundationalSelectedNodeLocus,
    FoundationalSelectedScopeLocus, FoundationalSelectedScopeNoOpCause,
    FoundationalSelectedScopeNoOpEvidence, FoundationalSkippedOutOfScopeEvidence,
    FoundationalStrategyBasis, FoundationalTransitionBasisFamily,
    FoundationalTransitionBasisIdentity, FoundationalTransitionBasisVersion,
    FoundationalTransitionCorrespondenceBasis, FoundationalTransitionRemapBasis,
    FoundationalTransitionStrategyContractBasis, FoundationalTransitionStrategyDescriptorDigest,
    FoundationalTransitionStrategyFamily, FoundationalTransitionStrategyId,
    FoundationalTransitionStrategyIdentity, FoundationalTransitionStrategyOwnershipClass,
    FoundationalTransitionStrategySemanticName, FoundationalTransitionStrategyVersion,
};
pub use readiness::{
    certify_foundational_transition_milestone5_production_test_readiness,
    certify_foundational_transition_milestone9_scoped_merge_production_test_readiness,
    foundational_transition_milestone5_readiness_report,
    foundational_transition_milestone9_scoped_merge_readiness_report,
    require_foundational_transition_milestone5_production_test_readiness,
    require_foundational_transition_milestone9_scoped_merge_production_test_readiness,
    FoundationalTransitionCertifiedSurface, FoundationalTransitionCertifiedSurfaceEvidence,
    FoundationalTransitionCompileFailBoundary, FoundationalTransitionCompileFailEvidence,
    FoundationalTransitionMilestone5PhaseGate, FoundationalTransitionPhaseGateEvidence,
    FoundationalTransitionProductionReadinessAuthority,
    FoundationalTransitionProductionReadinessReport,
    FoundationalTransitionProductionReadinessScope,
    FoundationalTransitionProductionTestReadyArtifact, FoundationalTransitionResidualDebt,
    FoundationalTransitionRuntimeAssumption, FoundationalTransitionRuntimeNonAssumption,
    FoundationalTransitionSyntheticPressureEvidence,
    FoundationalTransitionSyntheticRuntimePressure, FoundationalTransitionWORTHProofApi,
    FoundationalTransitionWORTHProofApiEvidence, FoundationalTransitionWORTHProofForbiddenSurface,
    FoundationalTransitionWORTHProofSurface,
};
pub use receipts::{
    foundational_commit_receipt_issuance, FoundationalBranchCloseoutCause,
    FoundationalBranchDiscardReceipt, FoundationalCommitId, FoundationalCommitReceiptArtifact,
    FoundationalCommitReceiptIdentity, FoundationalCommitReceiptIssuance,
    FoundationalCommitReceiptIssuanceBasis, FoundationalCommitReceiptIssuanceDenial,
    FoundationalCommitReceiptPhase, FoundationalNonAuthoritativeResidueReport,
    FoundationalTransitionBundle, FoundationalTransitionBundleBuilder,
    FoundationalTransitionBundleMaterializationCost, FoundationalTransitionIssuanceCause,
    FoundationalTransitionProvenanceRow,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "transitions",
        "branch-local transition identity, merge candidate/verdict law, proof-bearing committed-authority transition vocabulary, commit receipt/bundle emission, and transition basis/current-basis/profile integration",
        "boundary-artifact category law, runtime branch graphs, or one merge/commit execution engine",
    )
}
