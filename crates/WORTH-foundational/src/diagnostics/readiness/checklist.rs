use super::production_test_contract::{
    FoundationalDiagnosticAdoptionShapedFollowthrough,
    FoundationalDiagnosticCanonicalGoldenArtifact, FoundationalDiagnosticHarnessExpansionPoint,
    FoundationalDiagnosticPropertySeed, FoundationalDiagnosticRuntimeAdoptionFailurePressure,
};
use super::report::FoundationalDiagnosticProductionReadinessReport;
use super::vocabulary::{
    FoundationalDiagnosticCertifiedSurface, FoundationalDiagnosticCompileFailBoundary,
    FoundationalDiagnosticWORTHProofApi, FoundationalDiagnosticWORTHProofForbiddenSurface,
    FoundationalDiagnosticWORTHProofSurface, FoundationalDiagnosticMilestone6PhaseGate,
    FoundationalDiagnosticResidualDebt, FoundationalDiagnosticRuntimeAssumption,
    FoundationalDiagnosticRuntimeNonAssumption, FoundationalDiagnosticSyntheticRuntimePressure,
};
use std::collections::BTreeSet;

impl FoundationalDiagnosticProductionReadinessReport {
    pub fn passes_readiness_checklist(&self) -> bool {
        self.has_all_certified_surfaces()
            && self.has_evidence_for_each_certified_surface()
            && self.has_all_synthetic_pressures()
            && self.has_evidence_for_each_synthetic_pressure()
            && self.has_all_compile_fail_boundaries()
            && self.has_evidence_for_each_compile_fail_boundary()
            && self.has_all_canonical_golden_artifacts()
            && self.has_evidence_for_each_canonical_golden_artifact()
            && self.has_all_property_seeds()
            && self.has_evidence_for_each_property_seed()
            && self.has_all_harness_expansion_points()
            && self.has_evidence_for_each_harness_expansion_point()
            && self.has_all_required_worth_proof_surfaces()
            && self.has_named_worth_proof_api_appendix()
            && self.has_evidence_for_each_worth_proof_api()
            && self.has_all_forbidden_worth_proof_surfaces()
            && self.has_runtime_assumption_boundary()
            && self.has_runtime_adoption_failure_pressures()
            && self.has_named_residual_debt()
            && self.has_adoption_shaped_followthrough()
            && self.has_linear_phase_gates()
    }

    fn has_all_certified_surfaces(&self) -> bool {
        exact_inventory(
            &self.certified_surfaces,
            &[
                FoundationalDiagnosticCertifiedSurface::PrimitiveAndCategoryLaw,
                FoundationalDiagnosticCertifiedSurface::OutcomeSubjectAndRowTopology,
                FoundationalDiagnosticCertifiedSurface::MaterializationSupportAndNamedGapLaw,
                FoundationalDiagnosticCertifiedSurface::CanonicalBasisAndComparisonLaw,
                FoundationalDiagnosticCertifiedSurface::CertifiedBundleAndAttachmentCompatibility,
            ],
        )
    }

    fn has_evidence_for_each_certified_surface(&self) -> bool {
        self.certified_surface_evidence.len() == self.certified_surfaces.len()
            && self.certified_surface_evidence.iter().all(|evidence| {
                self.certified_surfaces.contains(&evidence.surface())
                    && self
                        .certified_surface_evidence
                        .iter()
                        .filter(|candidate| candidate.surface() == evidence.surface())
                        .count()
                        == 1
            })
    }

    fn has_all_synthetic_pressures(&self) -> bool {
        exact_inventory(
            &self.synthetic_pressures,
            &[
                FoundationalDiagnosticSyntheticRuntimePressure::PrimitiveNonSubstitution,
                FoundationalDiagnosticSyntheticRuntimePressure::GenericRowCollapseRejection,
                FoundationalDiagnosticSyntheticRuntimePressure::HiddenRediscoveryDebtRejection,
                FoundationalDiagnosticSyntheticRuntimePressure::ThinOrEmptySupportOverclaimRejection,
                FoundationalDiagnosticSyntheticRuntimePressure::BlindConsumerCanonicalParity,
                FoundationalDiagnosticSyntheticRuntimePressure::HiddenSourceDigestOrCoverageWORTHry,
                FoundationalDiagnosticSyntheticRuntimePressure::ExplanationProvenanceBoundaryPreservation,
            ],
        )
    }

    fn has_evidence_for_each_synthetic_pressure(&self) -> bool {
        self.synthetic_pressure_evidence.len() == self.synthetic_pressures.len()
            && self.synthetic_pressure_evidence.iter().all(|evidence| {
                self.synthetic_pressures.contains(&evidence.pressure())
                    && self
                        .synthetic_pressure_evidence
                        .iter()
                        .filter(|candidate| candidate.pressure() == evidence.pressure())
                        .count()
                        == 1
            })
    }

    fn has_all_compile_fail_boundaries(&self) -> bool {
        exact_inventory(
            &self.compile_fail_boundaries,
            &[
                FoundationalDiagnosticCompileFailBoundary::PrimitiveAndCategoryPreserveNonSubstitution,
                FoundationalDiagnosticCompileFailBoundary::RowTopologyPreservesFamilyAndLocatorLaw,
                FoundationalDiagnosticCompileFailBoundary::MaterializationAndSupportPreserveExplicitSeams,
                FoundationalDiagnosticCompileFailBoundary::BasisAndComparisonPreserveBlindConsumerCanonicalLaw,
                FoundationalDiagnosticCompileFailBoundary::CertifiedBundleAndAttachmentReuseProofLane,
                FoundationalDiagnosticCompileFailBoundary::DiagnosticReadinessRequiresCertifiedArtifact,
                FoundationalDiagnosticCompileFailBoundary::DiagnosticReadinessAuthorityCannotBeMinted,
            ],
        )
    }

    fn has_evidence_for_each_compile_fail_boundary(&self) -> bool {
        self.compile_fail_evidence.len() == self.compile_fail_boundaries.len()
            && self.compile_fail_evidence.iter().all(|evidence| {
                self.compile_fail_boundaries.contains(&evidence.boundary())
                    && self
                        .compile_fail_evidence
                        .iter()
                        .filter(|candidate| candidate.boundary() == evidence.boundary())
                        .count()
                        == 1
            })
    }

    fn has_all_canonical_golden_artifacts(&self) -> bool {
        exact_inventory(
            &self.canonical_golden_artifacts,
            &[
                FoundationalDiagnosticCanonicalGoldenArtifact::PrimitiveCategoryAndMaterializationMeaning,
                FoundationalDiagnosticCanonicalGoldenArtifact::FamilyDistinctRowTopologyMeaning,
                FoundationalDiagnosticCanonicalGoldenArtifact::MaterializationRichnessAndDebtMeaning,
                FoundationalDiagnosticCanonicalGoldenArtifact::CanonicalBundleAndComparisonMeaning,
                FoundationalDiagnosticCanonicalGoldenArtifact::CertifiedCoverageAndAttachmentMeaning,
            ],
        )
    }

    fn has_evidence_for_each_canonical_golden_artifact(&self) -> bool {
        self.canonical_golden_artifact_evidence.len() == self.canonical_golden_artifacts.len()
            && self
                .canonical_golden_artifact_evidence
                .iter()
                .all(|evidence| {
                    self.canonical_golden_artifacts
                        .contains(&evidence.artifact())
                        && self
                            .canonical_golden_artifact_evidence
                            .iter()
                            .filter(|candidate| candidate.artifact() == evidence.artifact())
                            .count()
                            == 1
                })
    }

    fn has_all_property_seeds(&self) -> bool {
        exact_inventory(
            &self.property_seed_inventory,
            &[
                FoundationalDiagnosticPropertySeed::PrimitiveOrderingAndTokenCanonicalization,
                FoundationalDiagnosticPropertySeed::RowFamilyOrderingAndSemanticTieBreaks,
                FoundationalDiagnosticPropertySeed::RichnessElisionPreservesTruthUnderPartiality,
                FoundationalDiagnosticPropertySeed::IndependentProducerCanonicalParity,
                FoundationalDiagnosticPropertySeed::CertifiedCoverageNamedGapParity,
            ],
        )
    }

    fn has_evidence_for_each_property_seed(&self) -> bool {
        self.property_seed_evidence.len() == self.property_seed_inventory.len()
            && self.property_seed_evidence.iter().all(|evidence| {
                self.property_seed_inventory.contains(&evidence.seed())
                    && self
                        .property_seed_evidence
                        .iter()
                        .filter(|candidate| candidate.seed() == evidence.seed())
                        .count()
                        == 1
            })
    }

    fn has_all_harness_expansion_points(&self) -> bool {
        exact_inventory(
            &self.harness_expansion_points,
            &[
                FoundationalDiagnosticHarnessExpansionPoint::IndependentProducerDiagnosticParityMatrix,
                FoundationalDiagnosticHarnessExpansionPoint::RichnessAvailabilityAndFallbackReplayMatrix,
                FoundationalDiagnosticHarnessExpansionPoint::BlindConsumerInterpretationReplaySuite,
                FoundationalDiagnosticHarnessExpansionPoint::CertifiedCoverageAttachmentParityMatrix,
            ],
        )
    }

    fn has_evidence_for_each_harness_expansion_point(&self) -> bool {
        self.harness_expansion_evidence.len() == self.harness_expansion_points.len()
            && self.harness_expansion_evidence.iter().all(|evidence| {
                self.harness_expansion_points.contains(&evidence.point())
                    && self
                        .harness_expansion_evidence
                        .iter()
                        .filter(|candidate| candidate.point() == evidence.point())
                        .count()
                        == 1
            })
    }

    fn has_all_required_worth_proof_surfaces(&self) -> bool {
        exact_inventory(
            &self.worth_proof_required_surfaces,
            &[
                FoundationalDiagnosticWORTHProofSurface::CertifiedDiagnosticAttachmentAuthority,
                FoundationalDiagnosticWORTHProofSurface::ProofBearingCertifiedDiagnosticBundle,
                FoundationalDiagnosticWORTHProofSurface::CertifiedBundleBoundaryBridge,
                FoundationalDiagnosticWORTHProofSurface::CertifiedBundleReadmitWithAuthority,
                FoundationalDiagnosticWORTHProofSurface::ProductionReadinessCertificationArtifact,
            ],
        )
    }

    fn has_named_worth_proof_api_appendix(&self) -> bool {
        exact_inventory(
            &self.worth_proof_api_appendix,
            &[
                FoundationalDiagnosticWORTHProofApi::AuthorityWitnessFromAuthorityMarker,
                FoundationalDiagnosticWORTHProofApi::ProofFromAuthorityWitness,
                FoundationalDiagnosticWORTHProofApi::ArtifactWithProofsAndCurrentBasis,
                FoundationalDiagnosticWORTHProofApi::ArtifactBridgeTrustBoundary,
                FoundationalDiagnosticWORTHProofApi::ArtifactReadmitWithAuthority,
            ],
        )
    }

    fn has_evidence_for_each_worth_proof_api(&self) -> bool {
        self.worth_proof_api_evidence.len() == self.worth_proof_api_appendix.len()
            && self.worth_proof_api_evidence.iter().all(|evidence| {
                self.worth_proof_api_appendix.contains(&evidence.api())
                    && self
                        .worth_proof_api_evidence
                        .iter()
                        .filter(|candidate| candidate.api() == evidence.api())
                        .count()
                        == 1
            })
    }

    fn has_all_forbidden_worth_proof_surfaces(&self) -> bool {
        exact_inventory(
            &self.worth_proof_forbidden_surfaces,
            &[
                FoundationalDiagnosticWORTHProofForbiddenSurface::PlainDiagnosticPrimitives,
                FoundationalDiagnosticWORTHProofForbiddenSurface::PlainDiagnosticRowsAndBundles,
                FoundationalDiagnosticWORTHProofForbiddenSurface::PlainMaterializationVocabulary,
                FoundationalDiagnosticWORTHProofForbiddenSurface::PlainCanonicalComparisonVocabulary,
            ],
        )
    }

    fn has_runtime_assumption_boundary(&self) -> bool {
        exact_inventory(
            &self.assumptions,
            &[
                FoundationalDiagnosticRuntimeAssumption::Milestone2CanonicalizationRemainsAuthorityForDiagnosticBasis,
                FoundationalDiagnosticRuntimeAssumption::Milestone3ProfilesGovernRichnessSupportAndCertificationPosture,
                FoundationalDiagnosticRuntimeAssumption::Milestone4ArtifactLawGovernsDiagnosticCategoryAndDeliveryMeaning,
                FoundationalDiagnosticRuntimeAssumption::Milestone5TransitionAndCurrentBasisSurfacesRemainAuthorityForTransitionAttachedDiagnostics,
                FoundationalDiagnosticRuntimeAssumption::CertifiedDiagnosticBundlesReuseWORTHProofLane,
            ],
        ) && exact_inventory(
            &self.non_assumptions,
            &[
                FoundationalDiagnosticRuntimeNonAssumption::Milestone7ProvenanceAndReceiptOntologyAlreadyOwnedHere,
                FoundationalDiagnosticRuntimeNonAssumption::OneDiagnosticsStoreOrReplayEngineExistsInFoundational,
                FoundationalDiagnosticRuntimeNonAssumption::AdoptingRuntimeCoverageParityAlreadyProven,
                FoundationalDiagnosticRuntimeNonAssumption::DescriptiveDiagnosticsBecomeAuthority,
                FoundationalDiagnosticRuntimeNonAssumption::BoundaryCrossingPreservesCertifiedCurrentBasisWithoutReadmission,
            ],
        )
    }

    fn has_runtime_adoption_failure_pressures(&self) -> bool {
        exact_inventory(
            &self.runtime_adoption_failure_pressures,
            &[
                FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeLoweringMayMisclassifyEvidencePosture,
                FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeMaterializersMayOverclaimDurableOrCertifiedSupport,
                FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeCanonicalRowOrderingMayDriftAcrossStorageLayouts,
                FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeCoverageMatricesMayOmitRequiredFamilies,
                FoundationalDiagnosticRuntimeAdoptionFailurePressure::RuntimeProvenanceReadyRowsMayCollapseIntoExplanationRows,
            ],
        )
    }

    fn has_named_residual_debt(&self) -> bool {
        exact_inventory(
            &self.residual_debt,
            &[
                FoundationalDiagnosticResidualDebt::AdoptingRuntimeParityDeferred,
                FoundationalDiagnosticResidualDebt::Milestone7ProvenanceAndReceiptDeepeningDeferred,
                FoundationalDiagnosticResidualDebt::RuntimeSpecificSupportTaxonomiesDeferred,
            ],
        )
    }

    fn has_adoption_shaped_followthrough(&self) -> bool {
        exact_inventory(
            &self.adoption_shaped_followthrough,
            &[
                FoundationalDiagnosticAdoptionShapedFollowthrough::WORTHHarnessDiagnosticProducerParityMatrix,
                FoundationalDiagnosticAdoptionShapedFollowthrough::WORTHHarnessRichnessAvailabilityAndFallbackReplaySuite,
                FoundationalDiagnosticAdoptionShapedFollowthrough::AdoptingRuntimeDiagnosticLoweringParityPressure,
                FoundationalDiagnosticAdoptionShapedFollowthrough::AdoptingRuntimeCertifiedCoverageAndAttachmentHostility,
            ],
        )
    }

    fn has_linear_phase_gates(&self) -> bool {
        self.phase_gates.iter().map(|evidence| evidence.gate()).eq([
            FoundationalDiagnosticMilestone6PhaseGate::PrimitiveAndCategoryLaw,
            FoundationalDiagnosticMilestone6PhaseGate::OutcomeSubjectAndRowTopology,
            FoundationalDiagnosticMilestone6PhaseGate::MaterializationSupportAndNamedGapLaw,
            FoundationalDiagnosticMilestone6PhaseGate::CanonicalBasisAndComparisonLaw,
            FoundationalDiagnosticMilestone6PhaseGate::CertifiedBundleAndAttachmentCompatibility,
            FoundationalDiagnosticMilestone6PhaseGate::ProductionReadiness,
        ]) && self.phase_gates.len() == 6
    }
}

fn exact_inventory<T>(actual: &[T], expected: &[T]) -> bool
where
    T: Copy + Ord,
{
    actual.len() == expected.len()
        && actual.iter().copied().collect::<BTreeSet<_>>()
            == expected.iter().copied().collect::<BTreeSet<_>>()
}
