use super::vocabulary::{
    FoundationalTransitionCertifiedSurface, FoundationalTransitionCertifiedSurfaceEvidence,
    FoundationalTransitionCompileFailBoundary, FoundationalTransitionCompileFailEvidence,
    FoundationalTransitionForgeProofApi, FoundationalTransitionForgeProofApiEvidence,
    FoundationalTransitionForgeProofForbiddenSurface, FoundationalTransitionForgeProofSurface,
    FoundationalTransitionMilestone5PhaseGate, FoundationalTransitionPhaseGateEvidence,
    FoundationalTransitionResidualDebt, FoundationalTransitionRuntimeAssumption,
    FoundationalTransitionRuntimeNonAssumption, FoundationalTransitionSyntheticPressureEvidence,
    FoundationalTransitionSyntheticRuntimePressure,
};

pub(super) fn certified_surfaces() -> Vec<FoundationalTransitionCertifiedSurface> {
    vec![
        FoundationalTransitionCertifiedSurface::ScopedMergeRequestVocabulary,
        FoundationalTransitionCertifiedSurface::ScopedMergeAdmissionEvidence,
        FoundationalTransitionCertifiedSurface::ScopedMergeDenialUnavailableTopology,
        FoundationalTransitionCertifiedSurface::ScopedMergeCanonicalLocatorDiagnostics,
        FoundationalTransitionCertifiedSurface::ScopedMergeAdoptionContract,
    ]
}

pub(super) fn certified_surface_evidence() -> Vec<FoundationalTransitionCertifiedSurfaceEvidence> {
    vec![
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::ScopedMergeRequestVocabulary,
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeCategorySubstitutionHostility,
            FoundationalTransitionCompileFailBoundary::ScopedMergeScopeRequiresTypedLoci,
            "tests/certification/transitions/scoped_merge.rs",
            "tests/ui/transitions/merge_admission/raw_string_cannot_satisfy_merge_scope_api.rs",
            "tests/certification/transitions/scoped_merge.rs",
        ),
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::ScopedMergeAdmissionEvidence,
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeProducerDiversityHostility,
            FoundationalTransitionCompileFailBoundary::SelectedNodeAndAspectRequestsAreNotSubstitutable,
            "tests/certification/transitions/scoped_merge_evidence.rs",
            "tests/ui/transitions/merge_admission/selected_node_cannot_satisfy_selected_aspect_entry_api.rs",
            "tests/certification/transitions/scoped_merge_evidence.rs",
        ),
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::ScopedMergeDenialUnavailableTopology,
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeUnavailableDenialHonesty,
            FoundationalTransitionCompileFailBoundary::ScopedMergeScopeRequiresTypedLoci,
            "tests/certification/transitions/scoped_merge_denials.rs",
            "tests/ui/transitions/merge_admission/raw_string_cannot_satisfy_merge_scope_api.rs",
            "tests/certification/transitions/scoped_merge_posture/mod.rs",
        ),
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::ScopedMergeCanonicalLocatorDiagnostics,
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeCanonicalLocatorStability,
            FoundationalTransitionCompileFailBoundary::SelectedScopeLocatorRequiresTypedLoci,
            "tests/certification/transitions/scoped_merge_canonical/mod.rs",
            "tests/ui/transitions/merge_admission/raw_string_cannot_satisfy_selected_scope_locator_api.rs",
            "tests/certification/transitions/scoped_merge_diagnostics/mod.rs",
        ),
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::ScopedMergeAdoptionContract,
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeRuntimeBoundaryHonesty,
            FoundationalTransitionCompileFailBoundary::TransitionReadinessRequiresCertifiedArtifact,
            "tests/certification/transitions/readiness.rs",
            "tests/ui/transitions/readiness_boundaries/plain_transition_bundle_cannot_satisfy_transition_production_readiness.rs",
            "docs/scoped-merge-adoption.md",
        ),
    ]
}

pub(super) fn synthetic_pressures() -> Vec<FoundationalTransitionSyntheticRuntimePressure> {
    vec![
        FoundationalTransitionSyntheticRuntimePressure::ScopedMergeCategorySubstitutionHostility,
        FoundationalTransitionSyntheticRuntimePressure::ScopedMergeProducerDiversityHostility,
        FoundationalTransitionSyntheticRuntimePressure::ScopedMergeUnavailableDenialHonesty,
        FoundationalTransitionSyntheticRuntimePressure::ScopedMergeCanonicalLocatorStability,
        FoundationalTransitionSyntheticRuntimePressure::ScopedMergeRuntimeBoundaryHonesty,
    ]
}

pub(super) fn synthetic_pressure_evidence() -> Vec<FoundationalTransitionSyntheticPressureEvidence>
{
    vec![
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeCategorySubstitutionHostility,
            "tests/certification/transitions/scoped_merge.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeProducerDiversityHostility,
            "tests/certification/transitions/scoped_merge_evidence.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeUnavailableDenialHonesty,
            "tests/certification/transitions/scoped_merge_posture/mod.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeCanonicalLocatorStability,
            "tests/certification/transitions/scoped_merge_canonical/mod.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeRuntimeBoundaryHonesty,
            "docs/scoped-merge-adoption.md",
        ),
    ]
}

pub(super) fn compile_fail_boundaries() -> Vec<FoundationalTransitionCompileFailBoundary> {
    vec![
        FoundationalTransitionCompileFailBoundary::ScopedMergeScopeRequiresTypedLoci,
        FoundationalTransitionCompileFailBoundary::SelectedScopeLocatorRequiresTypedLoci,
        FoundationalTransitionCompileFailBoundary::SelectedNodeAndAspectRequestsAreNotSubstitutable,
        FoundationalTransitionCompileFailBoundary::TransitionReadinessRequiresCertifiedArtifact,
        FoundationalTransitionCompileFailBoundary::TransitionReadinessAuthorityCannotBeMinted,
    ]
}

pub(super) fn compile_fail_evidence() -> Vec<FoundationalTransitionCompileFailEvidence> {
    vec![
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::ScopedMergeScopeRequiresTypedLoci,
            "tests/ui/transitions/merge_admission/raw_string_cannot_satisfy_merge_scope_api.rs",
        ),
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::SelectedScopeLocatorRequiresTypedLoci,
            "tests/ui/transitions/merge_admission/raw_string_cannot_satisfy_selected_scope_locator_api.rs",
        ),
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::SelectedNodeAndAspectRequestsAreNotSubstitutable,
            "tests/ui/transitions/merge_admission/selected_node_cannot_satisfy_selected_aspect_entry_api.rs",
        ),
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::TransitionReadinessRequiresCertifiedArtifact,
            "tests/ui/transitions/readiness_boundaries/plain_transition_bundle_cannot_satisfy_transition_production_readiness.rs",
        ),
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::TransitionReadinessAuthorityCannotBeMinted,
            "tests/ui/transitions/readiness_boundaries/transition_readiness_authority_cannot_be_minted.rs",
        ),
    ]
}

pub(super) fn forge_proof_required_surfaces() -> Vec<FoundationalTransitionForgeProofSurface> {
    vec![
        FoundationalTransitionForgeProofSurface::TransitionOutcomeAdmissionLane,
        FoundationalTransitionForgeProofSurface::ProofBearingCommittedAuthorityArtifact,
        FoundationalTransitionForgeProofSurface::ProofBearingCommitReceiptArtifact,
        FoundationalTransitionForgeProofSurface::CurrentBasisArtifactConstructor,
        FoundationalTransitionForgeProofSurface::BoundaryBridgeTrustBoundary,
        FoundationalTransitionForgeProofSurface::BoundaryReadmitWithAuthority,
        FoundationalTransitionForgeProofSurface::ProductionReadinessCertificationArtifact,
    ]
}

pub(super) fn forge_proof_api_appendix() -> Vec<FoundationalTransitionForgeProofApi> {
    vec![
        FoundationalTransitionForgeProofApi::TransitionOutcomeStructuredCategories,
        FoundationalTransitionForgeProofApi::ProofFromAuthorityWitness,
        FoundationalTransitionForgeProofApi::ArtifactWithProofsAndCurrentBasis,
        FoundationalTransitionForgeProofApi::ArtifactBridgeTrustBoundary,
        FoundationalTransitionForgeProofApi::ArtifactReadmitWithAuthority,
    ]
}

pub(super) fn forge_proof_api_evidence() -> Vec<FoundationalTransitionForgeProofApiEvidence> {
    vec![
        FoundationalTransitionForgeProofApiEvidence::new(
            FoundationalTransitionForgeProofApi::TransitionOutcomeStructuredCategories,
            "src/transitions/merges/admission.rs",
            "TransitionOutcome::",
        ),
        FoundationalTransitionForgeProofApiEvidence::new(
            FoundationalTransitionForgeProofApi::ProofFromAuthorityWitness,
            "src/transitions/receipts/issuance.rs",
            "Proof::from_authority_witness",
        ),
        FoundationalTransitionForgeProofApiEvidence::new(
            FoundationalTransitionForgeProofApi::ArtifactWithProofsAndCurrentBasis,
            "src/transitions/commits/authority.rs",
            "Artifact::with_proofs_and_current_basis",
        ),
        FoundationalTransitionForgeProofApiEvidence::new(
            FoundationalTransitionForgeProofApi::ArtifactBridgeTrustBoundary,
            "src/transitions/basis/current_basis.rs",
            ".bridge_trust_boundary()",
        ),
        FoundationalTransitionForgeProofApiEvidence::new(
            FoundationalTransitionForgeProofApi::ArtifactReadmitWithAuthority,
            "src/transitions/basis/current_basis.rs",
            ".readmit_with_authority(",
        ),
    ]
}

pub(super) fn forge_proof_forbidden_surfaces(
) -> Vec<FoundationalTransitionForgeProofForbiddenSurface> {
    vec![
        FoundationalTransitionForgeProofForbiddenSurface::PlainScopedMergeStrings,
        FoundationalTransitionForgeProofForbiddenSurface::PlainMergeVerdictVocabulary,
    ]
}

pub(super) fn runtime_assumptions() -> Vec<FoundationalTransitionRuntimeAssumption> {
    vec![
        FoundationalTransitionRuntimeAssumption::Milestone2CanonicalizationRemainsAuthorityForTransitionBasisReadiness,
        FoundationalTransitionRuntimeAssumption::Milestone3ProfilesGovernTransitionAttachmentAndElision,
        FoundationalTransitionRuntimeAssumption::ScopedMergeVocabularyMustPrecedeRuntimeExecution,
        FoundationalTransitionRuntimeAssumption::TransitionMeaningRemainsFacadeControlled,
    ]
}

pub(super) fn runtime_non_assumptions() -> Vec<FoundationalTransitionRuntimeNonAssumption> {
    vec![
        FoundationalTransitionRuntimeNonAssumption::FoundationalExecutesScopedMergeOrCherryPick,
        FoundationalTransitionRuntimeNonAssumption::AdoptingCratesMayInventScopedMergeDialect,
        FoundationalTransitionRuntimeNonAssumption::GenericBranchOrMergeEngineExistsInFoundational,
    ]
}

pub(super) fn residual_debt() -> Vec<FoundationalTransitionResidualDebt> {
    vec![
        FoundationalTransitionResidualDebt::AdoptingCrateScopedMergeExecutionDeferred,
        FoundationalTransitionResidualDebt::NativeCherryPickExecutionDeferred,
        FoundationalTransitionResidualDebt::RuntimeConflictMaterializationDeferred,
    ]
}

pub(super) fn phase_gates() -> Vec<FoundationalTransitionPhaseGateEvidence> {
    vec![
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::ScopedMergeRequestVocabulary,
            "tests/certification/transitions/scoped_merge.rs",
        ),
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::ScopedMergeAdmissionEvidence,
            "tests/certification/transitions/scoped_merge_evidence.rs",
        ),
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::ScopedMergeDenialUnavailableTopology,
            "tests/certification/transitions/scoped_merge_denials.rs",
        ),
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::ScopedMergeCanonicalLocatorDiagnostics,
            "tests/certification/transitions/scoped_merge_diagnostics/mod.rs",
        ),
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::ScopedMergeProductionReadiness,
            "tests/certification/transitions/readiness.rs",
        ),
    ]
}
