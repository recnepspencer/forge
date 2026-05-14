use super::vocabulary::{
    FoundationalDiagnosticCertifiedSurface, FoundationalDiagnosticCertifiedSurfaceEvidence,
    FoundationalDiagnosticCompileFailBoundary, FoundationalDiagnosticCompileFailEvidence,
    FoundationalDiagnosticForgeProofApi, FoundationalDiagnosticForgeProofApiEvidence,
    FoundationalDiagnosticForgeProofForbiddenSurface, FoundationalDiagnosticForgeProofSurface,
    FoundationalDiagnosticMilestone6PhaseGate, FoundationalDiagnosticPhaseGateEvidence,
    FoundationalDiagnosticResidualDebt, FoundationalDiagnosticRuntimeAssumption,
    FoundationalDiagnosticRuntimeNonAssumption, FoundationalDiagnosticSyntheticPressureEvidence,
    FoundationalDiagnosticSyntheticRuntimePressure,
};

pub(super) fn certified_surfaces() -> Vec<FoundationalDiagnosticCertifiedSurface> {
    vec![
        FoundationalDiagnosticCertifiedSurface::PrimitiveAndCategoryLaw,
        FoundationalDiagnosticCertifiedSurface::OutcomeSubjectAndRowTopology,
        FoundationalDiagnosticCertifiedSurface::MaterializationSupportAndNamedGapLaw,
        FoundationalDiagnosticCertifiedSurface::CanonicalBasisAndComparisonLaw,
        FoundationalDiagnosticCertifiedSurface::CertifiedBundleAndAttachmentCompatibility,
    ]
}

pub(super) fn synthetic_pressures() -> Vec<FoundationalDiagnosticSyntheticRuntimePressure> {
    vec![
        FoundationalDiagnosticSyntheticRuntimePressure::PrimitiveNonSubstitution,
        FoundationalDiagnosticSyntheticRuntimePressure::GenericRowCollapseRejection,
        FoundationalDiagnosticSyntheticRuntimePressure::HiddenRediscoveryDebtRejection,
        FoundationalDiagnosticSyntheticRuntimePressure::ThinOrEmptySupportOverclaimRejection,
        FoundationalDiagnosticSyntheticRuntimePressure::BlindConsumerCanonicalParity,
        FoundationalDiagnosticSyntheticRuntimePressure::HiddenSourceDigestOrCoverageForgery,
        FoundationalDiagnosticSyntheticRuntimePressure::ExplanationProvenanceBoundaryPreservation,
    ]
}

pub(super) fn certified_surface_evidence() -> Vec<FoundationalDiagnosticCertifiedSurfaceEvidence> {
    vec![
        FoundationalDiagnosticCertifiedSurfaceEvidence::new(
            FoundationalDiagnosticCertifiedSurface::PrimitiveAndCategoryLaw,
            FoundationalDiagnosticSyntheticRuntimePressure::PrimitiveNonSubstitution,
            FoundationalDiagnosticCompileFailBoundary::PrimitiveAndCategoryPreserveNonSubstitution,
            "tests/certification/diagnostics/primitives.rs",
            "tests/ui/diagnostics/primitives/scope_cannot_satisfy_code_api.rs",
            "tests/certification/diagnostics/primitives.rs",
        ),
        FoundationalDiagnosticCertifiedSurfaceEvidence::new(
            FoundationalDiagnosticCertifiedSurface::OutcomeSubjectAndRowTopology,
            FoundationalDiagnosticSyntheticRuntimePressure::GenericRowCollapseRejection,
            FoundationalDiagnosticCompileFailBoundary::RowTopologyPreservesFamilyAndLocatorLaw,
            "tests/certification/diagnostics/rows.rs",
            "tests/ui/diagnostics/rows/aggregate_row_cannot_satisfy_decision_row_api.rs",
            "tests/certification/diagnostics/rows.rs",
        ),
        FoundationalDiagnosticCertifiedSurfaceEvidence::new(
            FoundationalDiagnosticCertifiedSurface::MaterializationSupportAndNamedGapLaw,
            FoundationalDiagnosticSyntheticRuntimePressure::ThinOrEmptySupportOverclaimRejection,
            FoundationalDiagnosticCompileFailBoundary::MaterializationAndSupportPreserveExplicitSeams,
            "tests/certification/diagnostics/materialization.rs",
            "tests/ui/diagnostics/materialization/explanation_bundle_cannot_satisfy_support_report_api.rs",
            "tests/certification/diagnostics/materialization.rs",
        ),
        FoundationalDiagnosticCertifiedSurfaceEvidence::new(
            FoundationalDiagnosticCertifiedSurface::CanonicalBasisAndComparisonLaw,
            FoundationalDiagnosticSyntheticRuntimePressure::BlindConsumerCanonicalParity,
            FoundationalDiagnosticCompileFailBoundary::BasisAndComparisonPreserveBlindConsumerCanonicalLaw,
            "tests/certification/diagnostics/basis.rs",
            "tests/ui/diagnostics/basis/raw_support_report_cannot_satisfy_diagnostic_comparison_bundle_api.rs",
            "tests/certification/diagnostics/basis.rs",
        ),
        FoundationalDiagnosticCertifiedSurfaceEvidence::new(
            FoundationalDiagnosticCertifiedSurface::CertifiedBundleAndAttachmentCompatibility,
            FoundationalDiagnosticSyntheticRuntimePressure::HiddenSourceDigestOrCoverageForgery,
            FoundationalDiagnosticCompileFailBoundary::CertifiedBundleAndAttachmentReuseProofLane,
            "tests/certification/diagnostics/certified.rs",
            "tests/ui/diagnostics/certified/raw_support_report_cannot_satisfy_certified_bundle_api.rs",
            "tests/certification/diagnostics/certified.rs",
        ),
    ]
}

pub(super) fn compile_fail_boundaries() -> Vec<FoundationalDiagnosticCompileFailBoundary> {
    vec![
        FoundationalDiagnosticCompileFailBoundary::PrimitiveAndCategoryPreserveNonSubstitution,
        FoundationalDiagnosticCompileFailBoundary::RowTopologyPreservesFamilyAndLocatorLaw,
        FoundationalDiagnosticCompileFailBoundary::MaterializationAndSupportPreserveExplicitSeams,
        FoundationalDiagnosticCompileFailBoundary::BasisAndComparisonPreserveBlindConsumerCanonicalLaw,
        FoundationalDiagnosticCompileFailBoundary::CertifiedBundleAndAttachmentReuseProofLane,
        FoundationalDiagnosticCompileFailBoundary::DiagnosticReadinessRequiresCertifiedArtifact,
        FoundationalDiagnosticCompileFailBoundary::DiagnosticReadinessAuthorityCannotBeMinted,
    ]
}

pub(super) fn forge_proof_required_surfaces() -> Vec<FoundationalDiagnosticForgeProofSurface> {
    vec![
        FoundationalDiagnosticForgeProofSurface::CertifiedDiagnosticAttachmentAuthority,
        FoundationalDiagnosticForgeProofSurface::ProofBearingCertifiedDiagnosticBundle,
        FoundationalDiagnosticForgeProofSurface::CertifiedBundleBoundaryBridge,
        FoundationalDiagnosticForgeProofSurface::CertifiedBundleReadmitWithAuthority,
        FoundationalDiagnosticForgeProofSurface::ProductionReadinessCertificationArtifact,
    ]
}

pub(super) fn forge_proof_api_appendix() -> Vec<FoundationalDiagnosticForgeProofApi> {
    vec![
        FoundationalDiagnosticForgeProofApi::AuthorityWitnessFromAuthorityMarker,
        FoundationalDiagnosticForgeProofApi::ProofFromAuthorityWitness,
        FoundationalDiagnosticForgeProofApi::ArtifactWithProofsAndCurrentBasis,
        FoundationalDiagnosticForgeProofApi::ArtifactBridgeTrustBoundary,
        FoundationalDiagnosticForgeProofApi::ArtifactReadmitWithAuthority,
    ]
}

pub(super) fn forge_proof_api_evidence() -> Vec<FoundationalDiagnosticForgeProofApiEvidence> {
    vec![
        FoundationalDiagnosticForgeProofApiEvidence::new(
            FoundationalDiagnosticForgeProofApi::AuthorityWitnessFromAuthorityMarker,
            "src/diagnostics/readiness/certification.rs",
            "AuthorityWitness::from_authority_marker",
        ),
        FoundationalDiagnosticForgeProofApiEvidence::new(
            FoundationalDiagnosticForgeProofApi::ProofFromAuthorityWitness,
            "src/diagnostics/readiness/certification.rs",
            "Proof::from_authority_witness",
        ),
        FoundationalDiagnosticForgeProofApiEvidence::new(
            FoundationalDiagnosticForgeProofApi::ArtifactWithProofsAndCurrentBasis,
            "src/diagnostics/certified/attachments.rs",
            "Artifact::with_proofs_and_current_basis",
        ),
        FoundationalDiagnosticForgeProofApiEvidence::new(
            FoundationalDiagnosticForgeProofApi::ArtifactBridgeTrustBoundary,
            "src/diagnostics/certified/surfaces.rs",
            ".bridge_trust_boundary()",
        ),
        FoundationalDiagnosticForgeProofApiEvidence::new(
            FoundationalDiagnosticForgeProofApi::ArtifactReadmitWithAuthority,
            "src/diagnostics/certified/surfaces.rs",
            ".readmit_with_authority(",
        ),
    ]
}

pub(super) fn forge_proof_forbidden_surfaces(
) -> Vec<FoundationalDiagnosticForgeProofForbiddenSurface> {
    vec![
        FoundationalDiagnosticForgeProofForbiddenSurface::PlainDiagnosticPrimitives,
        FoundationalDiagnosticForgeProofForbiddenSurface::PlainDiagnosticRowsAndBundles,
        FoundationalDiagnosticForgeProofForbiddenSurface::PlainMaterializationVocabulary,
        FoundationalDiagnosticForgeProofForbiddenSurface::PlainCanonicalComparisonVocabulary,
    ]
}

pub(super) fn runtime_assumptions() -> Vec<FoundationalDiagnosticRuntimeAssumption> {
    vec![
        FoundationalDiagnosticRuntimeAssumption::Milestone2CanonicalizationRemainsAuthorityForDiagnosticBasis,
        FoundationalDiagnosticRuntimeAssumption::Milestone3ProfilesGovernRichnessSupportAndCertificationPosture,
        FoundationalDiagnosticRuntimeAssumption::Milestone4ArtifactLawGovernsDiagnosticCategoryAndDeliveryMeaning,
        FoundationalDiagnosticRuntimeAssumption::Milestone5TransitionAndCurrentBasisSurfacesRemainAuthorityForTransitionAttachedDiagnostics,
        FoundationalDiagnosticRuntimeAssumption::CertifiedDiagnosticBundlesReuseForgeProofLane,
    ]
}

pub(super) fn runtime_non_assumptions() -> Vec<FoundationalDiagnosticRuntimeNonAssumption> {
    vec![
        FoundationalDiagnosticRuntimeNonAssumption::Milestone7ProvenanceAndReceiptOntologyAlreadyOwnedHere,
        FoundationalDiagnosticRuntimeNonAssumption::OneDiagnosticsStoreOrReplayEngineExistsInFoundational,
        FoundationalDiagnosticRuntimeNonAssumption::AdoptingRuntimeCoverageParityAlreadyProven,
        FoundationalDiagnosticRuntimeNonAssumption::DescriptiveDiagnosticsBecomeAuthority,
        FoundationalDiagnosticRuntimeNonAssumption::BoundaryCrossingPreservesCertifiedCurrentBasisWithoutReadmission,
    ]
}

pub(super) fn residual_debt() -> Vec<FoundationalDiagnosticResidualDebt> {
    vec![
        FoundationalDiagnosticResidualDebt::AdoptingRuntimeParityDeferred,
        FoundationalDiagnosticResidualDebt::Milestone7ProvenanceAndReceiptDeepeningDeferred,
        FoundationalDiagnosticResidualDebt::RuntimeSpecificSupportTaxonomiesDeferred,
    ]
}

pub(super) fn synthetic_pressure_evidence() -> Vec<FoundationalDiagnosticSyntheticPressureEvidence>
{
    vec![
        FoundationalDiagnosticSyntheticPressureEvidence::new(
            FoundationalDiagnosticSyntheticRuntimePressure::PrimitiveNonSubstitution,
            "tests/certification/diagnostics/primitives.rs",
        ),
        FoundationalDiagnosticSyntheticPressureEvidence::new(
            FoundationalDiagnosticSyntheticRuntimePressure::GenericRowCollapseRejection,
            "tests/certification/diagnostics/rows.rs",
        ),
        FoundationalDiagnosticSyntheticPressureEvidence::new(
            FoundationalDiagnosticSyntheticRuntimePressure::HiddenRediscoveryDebtRejection,
            "tests/certification/diagnostics/materialization.rs",
        ),
        FoundationalDiagnosticSyntheticPressureEvidence::new(
            FoundationalDiagnosticSyntheticRuntimePressure::ThinOrEmptySupportOverclaimRejection,
            "tests/certification/diagnostics/materialization.rs",
        ),
        FoundationalDiagnosticSyntheticPressureEvidence::new(
            FoundationalDiagnosticSyntheticRuntimePressure::BlindConsumerCanonicalParity,
            "tests/certification/diagnostics/basis.rs",
        ),
        FoundationalDiagnosticSyntheticPressureEvidence::new(
            FoundationalDiagnosticSyntheticRuntimePressure::HiddenSourceDigestOrCoverageForgery,
            "tests/certification/diagnostics/certified.rs",
        ),
        FoundationalDiagnosticSyntheticPressureEvidence::new(
            FoundationalDiagnosticSyntheticRuntimePressure::ExplanationProvenanceBoundaryPreservation,
            "tests/certification/diagnostics/certified.rs",
        ),
    ]
}

pub(super) fn compile_fail_evidence() -> Vec<FoundationalDiagnosticCompileFailEvidence> {
    vec![
        FoundationalDiagnosticCompileFailEvidence::new(
            FoundationalDiagnosticCompileFailBoundary::PrimitiveAndCategoryPreserveNonSubstitution,
            "tests/ui/diagnostics/primitives/scope_cannot_satisfy_code_api.rs",
        ),
        FoundationalDiagnosticCompileFailEvidence::new(
            FoundationalDiagnosticCompileFailBoundary::RowTopologyPreservesFamilyAndLocatorLaw,
            "tests/ui/diagnostics/rows/aggregate_row_cannot_satisfy_decision_row_api.rs",
        ),
        FoundationalDiagnosticCompileFailEvidence::new(
            FoundationalDiagnosticCompileFailBoundary::MaterializationAndSupportPreserveExplicitSeams,
            "tests/ui/diagnostics/materialization/explanation_bundle_cannot_satisfy_support_report_api.rs",
        ),
        FoundationalDiagnosticCompileFailEvidence::new(
            FoundationalDiagnosticCompileFailBoundary::BasisAndComparisonPreserveBlindConsumerCanonicalLaw,
            "tests/ui/diagnostics/basis/raw_support_report_cannot_satisfy_diagnostic_comparison_bundle_api.rs",
        ),
        FoundationalDiagnosticCompileFailEvidence::new(
            FoundationalDiagnosticCompileFailBoundary::CertifiedBundleAndAttachmentReuseProofLane,
            "tests/ui/diagnostics/certified/raw_support_report_cannot_satisfy_certified_bundle_api.rs",
        ),
        FoundationalDiagnosticCompileFailEvidence::new(
            FoundationalDiagnosticCompileFailBoundary::DiagnosticReadinessRequiresCertifiedArtifact,
            "tests/ui/diagnostics/readiness_boundaries/plain_report_cannot_satisfy_diagnostic_production_readiness.rs",
        ),
        FoundationalDiagnosticCompileFailEvidence::new(
            FoundationalDiagnosticCompileFailBoundary::DiagnosticReadinessAuthorityCannotBeMinted,
            "tests/ui/diagnostics/readiness_boundaries/diagnostic_readiness_authority_cannot_be_minted.rs",
        ),
    ]
}

pub(super) fn phase_gates() -> Vec<FoundationalDiagnosticPhaseGateEvidence> {
    vec![
        FoundationalDiagnosticPhaseGateEvidence::new(
            FoundationalDiagnosticMilestone6PhaseGate::PrimitiveAndCategoryLaw,
            "tests/certification/diagnostics/primitives.rs",
        ),
        FoundationalDiagnosticPhaseGateEvidence::new(
            FoundationalDiagnosticMilestone6PhaseGate::OutcomeSubjectAndRowTopology,
            "tests/certification/diagnostics/rows.rs",
        ),
        FoundationalDiagnosticPhaseGateEvidence::new(
            FoundationalDiagnosticMilestone6PhaseGate::MaterializationSupportAndNamedGapLaw,
            "tests/certification/diagnostics/materialization.rs",
        ),
        FoundationalDiagnosticPhaseGateEvidence::new(
            FoundationalDiagnosticMilestone6PhaseGate::CanonicalBasisAndComparisonLaw,
            "tests/certification/diagnostics/basis.rs",
        ),
        FoundationalDiagnosticPhaseGateEvidence::new(
            FoundationalDiagnosticMilestone6PhaseGate::CertifiedBundleAndAttachmentCompatibility,
            "tests/certification/diagnostics/certified.rs",
        ),
        FoundationalDiagnosticPhaseGateEvidence::new(
            FoundationalDiagnosticMilestone6PhaseGate::ProductionReadiness,
            "tests/certification/diagnostics/readiness.rs",
        ),
    ]
}
