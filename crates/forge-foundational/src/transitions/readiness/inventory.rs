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
        FoundationalTransitionCertifiedSurface::BranchLocalSeparation,
        FoundationalTransitionCertifiedSurface::MergeVerdictLaw,
        FoundationalTransitionCertifiedSurface::CommittedAuthorityTransitions,
        FoundationalTransitionCertifiedSurface::CommitReceiptsAndBundles,
        FoundationalTransitionCertifiedSurface::CanonicalBasisAndLocatorIntegration,
        FoundationalTransitionCertifiedSurface::ProfileRichnessAndCurrentBasisBehavior,
    ]
}

pub(super) fn synthetic_pressures() -> Vec<FoundationalTransitionSyntheticRuntimePressure> {
    vec![
        FoundationalTransitionSyntheticRuntimePressure::AuthoritySeparation,
        FoundationalTransitionSyntheticRuntimePressure::MergeTopologyHonesty,
        FoundationalTransitionSyntheticRuntimePressure::NoOpVersusCommitClassification,
        FoundationalTransitionSyntheticRuntimePressure::ReceiptIssuanceBoundary,
        FoundationalTransitionSyntheticRuntimePressure::ReplayInterpretationBoundary,
        FoundationalTransitionSyntheticRuntimePressure::ReducedRichnessPreservation,
        FoundationalTransitionSyntheticRuntimePressure::AmbientBasisChoiceHostility,
        FoundationalTransitionSyntheticRuntimePressure::HiddenStrategyInfluenceHostility,
        FoundationalTransitionSyntheticRuntimePressure::ThinReceiptRejection,
        FoundationalTransitionSyntheticRuntimePressure::GenericTransitionResultBagRejection,
        FoundationalTransitionSyntheticRuntimePressure::CheapConvenienceBypassRejection,
    ]
}

pub(super) fn certified_surface_evidence() -> Vec<FoundationalTransitionCertifiedSurfaceEvidence> {
    vec![
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::BranchLocalSeparation,
            FoundationalTransitionSyntheticRuntimePressure::AuthoritySeparation,
            FoundationalTransitionCompileFailBoundary::BranchLocalSurfacesCannotSatisfyAuthorityApis,
            "tests/certification/transitions/branch_local.rs",
            "tests/ui/transitions/branch_local/branch_candidate_cannot_satisfy_authoritative_boundary_api.rs",
            "tests/certification/transitions/branch_local.rs",
        ),
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::MergeVerdictLaw,
            FoundationalTransitionSyntheticRuntimePressure::MergeTopologyHonesty,
            FoundationalTransitionCompileFailBoundary::MergeAdmissionSurfacesRemainNonAuthoritative,
            "tests/certification/transitions/merge_verdicts.rs",
            "tests/ui/transitions/merge_admission/merge_verdict_has_no_receipt_api.rs",
            "tests/certification/transitions/merge_verdicts.rs",
        ),
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::CommittedAuthorityTransitions,
            FoundationalTransitionSyntheticRuntimePressure::NoOpVersusCommitClassification,
            FoundationalTransitionCompileFailBoundary::CommittedAuthorityRequiresProofBearingAdmission,
            "tests/certification/transitions/committed_authority.rs",
            "tests/ui/transitions/committed_authority/merge_verdict_has_no_committed_authority_api.rs",
            "tests/certification/transitions/committed_authority.rs",
        ),
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::CommitReceiptsAndBundles,
            FoundationalTransitionSyntheticRuntimePressure::ReceiptIssuanceBoundary,
            FoundationalTransitionCompileFailBoundary::ReceiptAndCloseoutPreserveAuthoritySeparation,
            "tests/certification/transitions/receipts_and_bundles.rs",
            "tests/ui/transitions/receipt_boundaries/plain_boundary_receipt_surface_has_no_transition_receipt_api.rs",
            "tests/certification/transitions/receipts_and_bundles.rs",
        ),
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::CanonicalBasisAndLocatorIntegration,
            FoundationalTransitionSyntheticRuntimePressure::AmbientBasisChoiceHostility,
            FoundationalTransitionCompileFailBoundary::Phase5BasisAndCurrentBasisRequireStrengthenedArtifacts,
            "tests/certification/transitions/phase5_basis.rs",
            "tests/ui/transitions/phase5_boundaries/raw_committed_authority_cannot_satisfy_current_basis_transition_api.rs",
            "tests/certification/transitions/phase5_basis.rs",
        ),
        FoundationalTransitionCertifiedSurfaceEvidence::new(
            FoundationalTransitionCertifiedSurface::ProfileRichnessAndCurrentBasisBehavior,
            FoundationalTransitionSyntheticRuntimePressure::ReducedRichnessPreservation,
            FoundationalTransitionCompileFailBoundary::Phase5BasisAndCurrentBasisRequireStrengthenedArtifacts,
            "tests/certification/transitions/phase5_basis.rs",
            "tests/ui/transitions/phase5_boundaries/raw_commit_receipt_cannot_satisfy_current_basis_transition_api.rs",
            "tests/certification/transitions/phase5_basis.rs",
        ),
    ]
}

pub(super) fn compile_fail_boundaries() -> Vec<FoundationalTransitionCompileFailBoundary> {
    vec![
        FoundationalTransitionCompileFailBoundary::BranchLocalSurfacesCannotSatisfyAuthorityApis,
        FoundationalTransitionCompileFailBoundary::MergeAdmissionSurfacesRemainNonAuthoritative,
        FoundationalTransitionCompileFailBoundary::CommittedAuthorityRequiresProofBearingAdmission,
        FoundationalTransitionCompileFailBoundary::ReceiptAndCloseoutPreserveAuthoritySeparation,
        FoundationalTransitionCompileFailBoundary::Phase5BasisAndCurrentBasisRequireStrengthenedArtifacts,
        FoundationalTransitionCompileFailBoundary::TransitionReadinessRequiresCertifiedArtifact,
        FoundationalTransitionCompileFailBoundary::TransitionReadinessAuthorityCannotBeMinted,
    ]
}

pub(super) fn forge_proof_required_surfaces() -> Vec<FoundationalTransitionForgeProofSurface> {
    vec![
        FoundationalTransitionForgeProofSurface::TransitionOutcomeAdmissionLane,
        FoundationalTransitionForgeProofSurface::AuthorityWitnessScopedAdmission,
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
        FoundationalTransitionForgeProofApi::AuthorityWitnessFromAuthorityMarker,
        FoundationalTransitionForgeProofApi::ProofFromAuthorityWitness,
        FoundationalTransitionForgeProofApi::ArtifactWithProofsAndCurrentBasis,
        FoundationalTransitionForgeProofApi::ArtifactWithCurrentBasis,
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
            FoundationalTransitionForgeProofApi::AuthorityWitnessFromAuthorityMarker,
            "src/transitions/commits/authority.rs",
            "AuthorityWitness::from_authority_marker",
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
            FoundationalTransitionForgeProofApi::ArtifactWithCurrentBasis,
            "src/transitions/basis/current_basis.rs",
            "Artifact::with_current_basis",
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
        FoundationalTransitionForgeProofForbiddenSurface::PlainBranchLocalVocabulary,
        FoundationalTransitionForgeProofForbiddenSurface::PlainMergeVerdictVocabulary,
        FoundationalTransitionForgeProofForbiddenSurface::PlainReceiptAndBundleVocabulary,
        FoundationalTransitionForgeProofForbiddenSurface::PlainCanonicalBasisAndLocatorVocabulary,
    ]
}

pub(super) fn runtime_assumptions() -> Vec<FoundationalTransitionRuntimeAssumption> {
    vec![
        FoundationalTransitionRuntimeAssumption::Milestone2CanonicalizationRemainsAuthorityForTransitionBasisReadiness,
        FoundationalTransitionRuntimeAssumption::Milestone3ProfilesGovernTransitionAttachmentAndElision,
        FoundationalTransitionRuntimeAssumption::StrongerCommittedAuthorityAndReceiptClaimsUseForgeProof,
        FoundationalTransitionRuntimeAssumption::TransitionMeaningRemainsFacadeControlled,
    ]
}

pub(super) fn synthetic_pressure_evidence() -> Vec<FoundationalTransitionSyntheticPressureEvidence>
{
    vec![
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::AuthoritySeparation,
            "tests/certification/transitions/branch_local.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::MergeTopologyHonesty,
            "tests/certification/transitions/merge_verdicts.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::NoOpVersusCommitClassification,
            "tests/certification/transitions/committed_authority.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::ReceiptIssuanceBoundary,
            "tests/certification/transitions/receipts_and_bundles.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::ReplayInterpretationBoundary,
            "tests/certification/transitions/phase5_basis.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::ReducedRichnessPreservation,
            "tests/certification/transitions/phase5_basis.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::AmbientBasisChoiceHostility,
            "tests/certification/transitions/phase5_basis.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::HiddenStrategyInfluenceHostility,
            "tests/certification/transitions/merge_verdicts.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::ThinReceiptRejection,
            "tests/certification/transitions/receipts_and_bundles.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::GenericTransitionResultBagRejection,
            "tests/certification/transitions/receipts_and_bundles.rs",
        ),
        FoundationalTransitionSyntheticPressureEvidence::new(
            FoundationalTransitionSyntheticRuntimePressure::CheapConvenienceBypassRejection,
            "tests/certification/transitions/phase5_basis.rs",
        ),
    ]
}

pub(super) fn compile_fail_evidence() -> Vec<FoundationalTransitionCompileFailEvidence> {
    vec![
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::BranchLocalSurfacesCannotSatisfyAuthorityApis,
            "tests/ui/transitions/branch_local/branch_candidate_cannot_satisfy_authoritative_boundary_api.rs",
        ),
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::MergeAdmissionSurfacesRemainNonAuthoritative,
            "tests/ui/transitions/merge_admission/merge_verdict_has_no_receipt_api.rs",
        ),
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::CommittedAuthorityRequiresProofBearingAdmission,
            "tests/ui/transitions/committed_authority/caller_cannot_mint_committed_authority_admission.rs",
        ),
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::ReceiptAndCloseoutPreserveAuthoritySeparation,
            "tests/ui/transitions/receipt_boundaries/plain_boundary_receipt_surface_has_no_transition_receipt_api.rs",
        ),
        FoundationalTransitionCompileFailEvidence::new(
            FoundationalTransitionCompileFailBoundary::Phase5BasisAndCurrentBasisRequireStrengthenedArtifacts,
            "tests/ui/transitions/phase5_boundaries/raw_committed_authority_cannot_satisfy_current_basis_transition_api.rs",
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

pub(super) fn runtime_non_assumptions() -> Vec<FoundationalTransitionRuntimeNonAssumption> {
    vec![
        FoundationalTransitionRuntimeNonAssumption::DiagnosticsOntologyAlreadyOwnedHere,
        FoundationalTransitionRuntimeNonAssumption::ProvenanceOntologyBeyondTransitionRowsAlreadyOwnedHere,
        FoundationalTransitionRuntimeNonAssumption::AdoptingRuntimeMergeStrategyParityAlreadyProven,
        FoundationalTransitionRuntimeNonAssumption::BoundaryCrossingPreservesCurrentBasisWithoutReadmission,
        FoundationalTransitionRuntimeNonAssumption::GenericBranchOrMergeEngineExistsInFoundational,
    ]
}

pub(super) fn residual_debt() -> Vec<FoundationalTransitionResidualDebt> {
    vec![
        FoundationalTransitionResidualDebt::AdoptingRuntimeParityDeferred,
        FoundationalTransitionResidualDebt::LaterDiagnosticsAndProvenanceOntologyDeferred,
        FoundationalTransitionResidualDebt::RuntimeStrategyRegistryAndExecutionDeferred,
        FoundationalTransitionResidualDebt::FullLineageSupportBeyondTransitionRowsDeferred,
    ]
}

pub(super) fn phase_gates() -> Vec<FoundationalTransitionPhaseGateEvidence> {
    vec![
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::BranchLocalSeparation,
            "tests/certification/transitions/branch_local.rs",
        ),
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::MergeVerdictLaw,
            "tests/certification/transitions/merge_verdicts.rs",
        ),
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::CommittedAuthorityTransitionLaw,
            "tests/certification/transitions/committed_authority.rs",
        ),
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::CommitReceiptsAndBundles,
            "tests/certification/transitions/receipts_and_bundles.rs",
        ),
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::CanonicalBasisLocatorAndProfileIntegration,
            "tests/certification/transitions/phase5_basis.rs",
        ),
        FoundationalTransitionPhaseGateEvidence::new(
            FoundationalTransitionMilestone5PhaseGate::ProductionReadiness,
            "tests/certification/transitions/readiness.rs",
        ),
    ]
}
