use forge_foundational::{
    certify_foundational_transition_milestone5_production_test_readiness,
    certify_foundational_transition_milestone9_scoped_merge_production_test_readiness,
    foundational_transition_milestone5_readiness_report,
    foundational_transition_milestone9_scoped_merge_readiness_report,
    require_foundational_transition_milestone5_production_test_readiness,
    require_foundational_transition_milestone9_scoped_merge_production_test_readiness,
    FoundationalTransitionCertifiedSurface, FoundationalTransitionCompileFailBoundary,
    FoundationalTransitionForgeProofApi, FoundationalTransitionForgeProofForbiddenSurface,
    FoundationalTransitionForgeProofSurface, FoundationalTransitionMilestone5PhaseGate,
    FoundationalTransitionProductionReadinessCertified,
    FoundationalTransitionProductionTestReadyArtifact, FoundationalTransitionResidualDebt,
    FoundationalTransitionRuntimeAssumption, FoundationalTransitionRuntimeNonAssumption,
    FoundationalTransitionSyntheticRuntimePressure,
};
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::path::Path;

fn accepts_transition_readiness_artifact(_: &FoundationalTransitionProductionTestReadyArtifact) {}
fn accepts_transition_readiness_proof(
    _: &forge_proof::Proof<
        FoundationalTransitionProductionReadinessCertified,
        forge_foundational::FoundationalTransitionProductionReadinessAuthority,
    >,
) {
}

#[test]
fn production_readiness_artifact_carries_complete_machine_checkable_inventory() {
    let readiness = certify_foundational_transition_milestone5_production_test_readiness();
    let report = require_foundational_transition_milestone5_production_test_readiness(&readiness);

    accepts_transition_readiness_artifact(&readiness);
    accepts_transition_readiness_proof(readiness.proofs());
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        readiness.strong_basis().value().milestone(),
        "forge-foundational.milestone-5"
    );

    assert_exact_inventory(
        "certified surfaces",
        report.certified_surfaces(),
        &[
            FoundationalTransitionCertifiedSurface::BranchLocalSeparation,
            FoundationalTransitionCertifiedSurface::MergeVerdictLaw,
            FoundationalTransitionCertifiedSurface::CommittedAuthorityTransitions,
            FoundationalTransitionCertifiedSurface::CommitReceiptsAndBundles,
            FoundationalTransitionCertifiedSurface::CanonicalBasisAndLocatorIntegration,
            FoundationalTransitionCertifiedSurface::ProfileRichnessAndCurrentBasisBehavior,
        ],
    );
    assert_exact_inventory(
        "compile-fail boundaries",
        report.compile_fail_boundaries(),
        &[
            FoundationalTransitionCompileFailBoundary::BranchLocalSurfacesCannotSatisfyAuthorityApis,
            FoundationalTransitionCompileFailBoundary::MergeAdmissionSurfacesRemainNonAuthoritative,
            FoundationalTransitionCompileFailBoundary::CommittedAuthorityRequiresProofBearingAdmission,
            FoundationalTransitionCompileFailBoundary::ReceiptAndCloseoutPreserveAuthoritySeparation,
            FoundationalTransitionCompileFailBoundary::Phase5BasisAndCurrentBasisRequireStrengthenedArtifacts,
            FoundationalTransitionCompileFailBoundary::TransitionReadinessRequiresCertifiedArtifact,
            FoundationalTransitionCompileFailBoundary::TransitionReadinessAuthorityCannotBeMinted,
        ],
    );

    for surface in report.certified_surfaces() {
        assert_eq!(
            report
                .certified_surface_evidence()
                .iter()
                .filter(|evidence| evidence.surface() == *surface)
                .count(),
            1,
            "each certified surface must have exactly one evidence row"
        );
    }
}

#[test]
fn scoped_merge_readiness_artifact_carries_milestone9_basis_and_inventory() {
    let readiness =
        certify_foundational_transition_milestone9_scoped_merge_production_test_readiness();
    let report = require_foundational_transition_milestone9_scoped_merge_production_test_readiness(
        &readiness,
    );

    accepts_transition_readiness_artifact(&readiness);
    accepts_transition_readiness_proof(readiness.proofs());
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        readiness.strong_basis().value().milestone(),
        "forge-foundational.milestone-9.scoped-merge"
    );
    assert_eq!(
        report.scope().milestone(),
        "forge-foundational.milestone-9.scoped-merge"
    );

    assert_exact_inventory(
        "scoped certified surfaces",
        report.certified_surfaces(),
        &[
            FoundationalTransitionCertifiedSurface::ScopedMergeRequestVocabulary,
            FoundationalTransitionCertifiedSurface::ScopedMergeAdmissionEvidence,
            FoundationalTransitionCertifiedSurface::ScopedMergeDenialUnavailableTopology,
            FoundationalTransitionCertifiedSurface::ScopedMergeCanonicalLocatorDiagnostics,
            FoundationalTransitionCertifiedSurface::ScopedMergeAdoptionContract,
        ],
    );
    assert_exact_inventory(
        "scoped compile-fail boundaries",
        report.compile_fail_boundaries(),
        &[
            FoundationalTransitionCompileFailBoundary::ScopedMergeScopeRequiresTypedLoci,
            FoundationalTransitionCompileFailBoundary::SelectedScopeLocatorRequiresTypedLoci,
            FoundationalTransitionCompileFailBoundary::SelectedNodeAndAspectRequestsAreNotSubstitutable,
            FoundationalTransitionCompileFailBoundary::TransitionReadinessRequiresCertifiedArtifact,
            FoundationalTransitionCompileFailBoundary::TransitionReadinessAuthorityCannotBeMinted,
        ],
    );
}

#[test]
fn scoped_merge_readiness_names_runtime_handoff_and_residual_debt() {
    let report = foundational_transition_milestone9_scoped_merge_readiness_report();

    assert_exact_inventory(
        "scoped hostile pressures",
        report.synthetic_pressures(),
        &[
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeCategorySubstitutionHostility,
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeProducerDiversityHostility,
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeUnavailableDenialHonesty,
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeCanonicalLocatorStability,
            FoundationalTransitionSyntheticRuntimePressure::ScopedMergeRuntimeBoundaryHonesty,
        ],
    );
    assert!(report.assumptions().contains(
        &FoundationalTransitionRuntimeAssumption::ScopedMergeVocabularyMustPrecedeRuntimeExecution
    ));
    assert!(report.non_assumptions().contains(
        &FoundationalTransitionRuntimeNonAssumption::FoundationalExecutesScopedMergeOrCherryPick
    ));
    assert!(report.non_assumptions().contains(
        &FoundationalTransitionRuntimeNonAssumption::AdoptingCratesMayInventScopedMergeDialect
    ));
    assert_exact_inventory(
        "scoped residual debt",
        report.residual_debt(),
        &[
            FoundationalTransitionResidualDebt::AdoptingCrateScopedMergeExecutionDeferred,
            FoundationalTransitionResidualDebt::NativeCherryPickExecutionDeferred,
            FoundationalTransitionResidualDebt::RuntimeConflictMaterializationDeferred,
        ],
    );
}

#[test]
fn scoped_merge_readiness_evidence_paths_and_forge_proof_snippets_are_real() {
    let report = foundational_transition_milestone9_scoped_merge_readiness_report();

    for evidence in report.certified_surface_evidence() {
        assert!(report.certified_surfaces().contains(&evidence.surface()));
        assert!(crate_root_path(evidence.owning_test_path()).is_file());
        assert!(crate_root_path(evidence.compile_fail_evidence_path()).is_file());
        assert!(crate_root_path(evidence.blind_consumer_evidence_path()).is_file());
        assert!(
            evidence.owning_test_path().starts_with("tests/"),
            "scoped surface evidence must name a test owner, not only docs"
        );
    }

    let adoption_evidence = report
        .certified_surface_evidence()
        .iter()
        .find(|evidence| {
            evidence.surface()
                == FoundationalTransitionCertifiedSurface::ScopedMergeAdoptionContract
        })
        .expect("scoped adoption contract evidence");
    assert_eq!(
        adoption_evidence.blind_consumer_evidence_path(),
        "docs/scoped-merge-adoption.md"
    );

    for evidence in report.synthetic_pressure_evidence() {
        assert!(report.synthetic_pressures().contains(&evidence.pressure()));
        assert!(crate_root_path(evidence.owning_test_path()).is_file());
    }

    for evidence in report.forge_proof_api_evidence() {
        let source = std::fs::read_to_string(crate_root_path(evidence.source_path()))
            .expect("scoped forge-proof evidence source");
        assert!(
            source.contains(evidence.source_snippet()),
            "scoped forge-proof evidence for {:?} drifted",
            evidence.api()
        );
    }
}

#[test]
fn production_readiness_report_names_forge_proof_dependency_boundary_and_runtime_handoff() {
    let report = foundational_transition_milestone5_readiness_report();

    assert_exact_inventory(
        "forge-proof required surfaces",
        report.forge_proof_required_surfaces(),
        &[
            FoundationalTransitionForgeProofSurface::TransitionOutcomeAdmissionLane,
            FoundationalTransitionForgeProofSurface::AuthorityWitnessScopedAdmission,
            FoundationalTransitionForgeProofSurface::ProofBearingCommittedAuthorityArtifact,
            FoundationalTransitionForgeProofSurface::ProofBearingCommitReceiptArtifact,
            FoundationalTransitionForgeProofSurface::CurrentBasisArtifactConstructor,
            FoundationalTransitionForgeProofSurface::BoundaryBridgeTrustBoundary,
            FoundationalTransitionForgeProofSurface::BoundaryReadmitWithAuthority,
            FoundationalTransitionForgeProofSurface::ProductionReadinessCertificationArtifact,
        ],
    );
    assert_exact_inventory(
        "forge-proof api appendix",
        report.forge_proof_api_appendix(),
        &[
            FoundationalTransitionForgeProofApi::TransitionOutcomeStructuredCategories,
            FoundationalTransitionForgeProofApi::AuthorityWitnessFromAuthorityMarker,
            FoundationalTransitionForgeProofApi::ProofFromAuthorityWitness,
            FoundationalTransitionForgeProofApi::ArtifactWithProofsAndCurrentBasis,
            FoundationalTransitionForgeProofApi::ArtifactWithCurrentBasis,
            FoundationalTransitionForgeProofApi::ArtifactBridgeTrustBoundary,
            FoundationalTransitionForgeProofApi::ArtifactReadmitWithAuthority,
        ],
    );
    assert_exact_inventory(
        "forge-proof forbidden surfaces",
        report.forge_proof_forbidden_surfaces(),
        &[
            FoundationalTransitionForgeProofForbiddenSurface::PlainBranchLocalVocabulary,
            FoundationalTransitionForgeProofForbiddenSurface::PlainMergeVerdictVocabulary,
            FoundationalTransitionForgeProofForbiddenSurface::PlainReceiptAndBundleVocabulary,
            FoundationalTransitionForgeProofForbiddenSurface::PlainCanonicalBasisAndLocatorVocabulary,
        ],
    );
    assert!(report.assumptions().contains(
        &FoundationalTransitionRuntimeAssumption::Milestone2CanonicalizationRemainsAuthorityForTransitionBasisReadiness
    ));
    assert!(report.assumptions().contains(
        &FoundationalTransitionRuntimeAssumption::StrongerCommittedAuthorityAndReceiptClaimsUseForgeProof
    ));
    assert!(report.non_assumptions().contains(
        &FoundationalTransitionRuntimeNonAssumption::BoundaryCrossingPreservesCurrentBasisWithoutReadmission
    ));
    assert!(report
        .residual_debt()
        .contains(&FoundationalTransitionResidualDebt::AdoptingRuntimeParityDeferred));
}

#[test]
fn production_readiness_surface_evidence_binds_surfaces_to_real_hostile_compile_fail_and_blind_consumer_tests(
) {
    let report = foundational_transition_milestone5_readiness_report();

    for evidence in report.certified_surface_evidence() {
        assert!(report.certified_surfaces().contains(&evidence.surface()));
        assert!(report
            .synthetic_pressures()
            .contains(&evidence.hostile_pressure()));
        assert!(report
            .compile_fail_boundaries()
            .contains(&evidence.compile_fail_boundary()));
        assert!(crate_root_path(evidence.owning_test_path()).is_file());
        assert!(crate_root_path(evidence.compile_fail_evidence_path()).is_file());
        assert!(crate_root_path(evidence.blind_consumer_evidence_path()).is_file());
        assert!(
            evidence
                .compile_fail_evidence_path()
                .starts_with("tests/ui/"),
            "compile-fail evidence must point at a trybuild fixture"
        );
    }
}

#[test]
fn production_readiness_hostile_pressures_and_compile_fail_boundaries_have_real_evidence_paths() {
    let report = foundational_transition_milestone5_readiness_report();

    for evidence in report.synthetic_pressure_evidence() {
        assert!(report.synthetic_pressures().contains(&evidence.pressure()));
        assert!(crate_root_path(evidence.owning_test_path()).is_file());
    }

    for evidence in report.compile_fail_evidence() {
        assert!(report
            .compile_fail_boundaries()
            .contains(&evidence.boundary()));
        assert!(crate_root_path(evidence.evidence_path()).is_file());
        assert!(
            evidence.evidence_path().starts_with("tests/ui/"),
            "compile-fail evidence must point at a trybuild fixture"
        );
    }
}

#[test]
fn production_readiness_forge_proof_appendix_is_bound_to_real_transition_source_lanes() {
    let report = foundational_transition_milestone5_readiness_report();

    for evidence in report.forge_proof_api_evidence() {
        assert!(report.forge_proof_api_appendix().contains(&evidence.api()));
        let source = std::fs::read_to_string(crate_root_path(evidence.source_path()))
            .expect("forge-proof api evidence source must be readable");
        assert!(
            source.contains(evidence.source_snippet()),
            "forge-proof api evidence for {:?} drifted from the named source lane",
            evidence.api()
        );
    }
}

#[test]
fn production_readiness_phase_gates_are_linear_and_evidence_backed() {
    let report = foundational_transition_milestone5_readiness_report();
    let gates: Vec<_> = report
        .phase_gates()
        .iter()
        .map(|evidence| evidence.gate())
        .collect();

    assert_eq!(
        gates,
        vec![
            FoundationalTransitionMilestone5PhaseGate::BranchLocalSeparation,
            FoundationalTransitionMilestone5PhaseGate::MergeVerdictLaw,
            FoundationalTransitionMilestone5PhaseGate::CommittedAuthorityTransitionLaw,
            FoundationalTransitionMilestone5PhaseGate::CommitReceiptsAndBundles,
            FoundationalTransitionMilestone5PhaseGate::CanonicalBasisLocatorAndProfileIntegration,
            FoundationalTransitionMilestone5PhaseGate::ProductionReadiness,
        ]
    );
    assert!(report
        .phase_gates()
        .iter()
        .all(|evidence| crate_root_path(evidence.evidence_path()).exists()));
}

#[test]
fn production_readiness_hostile_pressure_inventory_stays_exact() {
    let report = foundational_transition_milestone5_readiness_report();

    assert_exact_inventory(
        "synthetic pressures",
        report.synthetic_pressures(),
        &[
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
        ],
    );
}

fn crate_root_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn assert_exact_inventory<T>(name: &str, actual: &[T], expected: &[T])
where
    T: Copy + Debug + Ord,
{
    let actual_set: BTreeSet<_> = actual.iter().copied().collect();
    let expected_set: BTreeSet<_> = expected.iter().copied().collect();

    assert_eq!(
        actual.len(),
        expected.len(),
        "{name} contains duplicate rows"
    );
    assert_eq!(
        actual_set, expected_set,
        "{name} changed without updating readiness certification"
    );
}
