use worth_foundational::{
    certify_foundational_boundary_artifact_milestone4_production_test_readiness,
    foundational_boundary_artifact_milestone4_readiness_report,
    require_foundational_boundary_artifact_milestone4_production_test_readiness,
    FoundationalBoundaryArtifactCertifiedSurface, FoundationalBoundaryArtifactCompileFailBoundary,
    FoundationalBoundaryArtifactWORTHProofApi,
    FoundationalBoundaryArtifactWORTHProofForbiddenSurface,
    FoundationalBoundaryArtifactWORTHProofSurface, FoundationalBoundaryArtifactMilestone4PhaseGate,
    FoundationalBoundaryArtifactProductionReadinessCertified,
    FoundationalBoundaryArtifactProductionTestReadyArtifact,
    FoundationalBoundaryArtifactResidualDebt, FoundationalBoundaryArtifactRuntimeAssumption,
    FoundationalBoundaryArtifactRuntimeNonAssumption,
    FoundationalBoundaryArtifactSyntheticRuntimePressure,
};
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::path::Path;

fn accepts_boundary_artifact_readiness_artifact(
    _: &FoundationalBoundaryArtifactProductionTestReadyArtifact,
) {
}

fn accepts_boundary_artifact_readiness_proof(
    _: &worth_proof::Proof<
        FoundationalBoundaryArtifactProductionReadinessCertified,
        worth_foundational::FoundationalBoundaryArtifactProductionReadinessAuthority,
    >,
) {
}

#[test]
fn production_readiness_artifact_carries_complete_machine_checkable_inventory() {
    let readiness = certify_foundational_boundary_artifact_milestone4_production_test_readiness();
    let report =
        require_foundational_boundary_artifact_milestone4_production_test_readiness(&readiness);

    accepts_boundary_artifact_readiness_artifact(&readiness);
    accepts_boundary_artifact_readiness_proof(readiness.proofs());
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        readiness.strong_basis().value().milestone(),
        "worth-foundational.milestone-4"
    );

    assert_exact_inventory(
        "certified surfaces",
        report.certified_surfaces(),
        &[
            FoundationalBoundaryArtifactCertifiedSurface::CategoryVocabulary,
            FoundationalBoundaryArtifactCertifiedSurface::RoleAndAuthorityLaw,
            FoundationalBoundaryArtifactCertifiedSurface::MaterializationAndBundles,
            FoundationalBoundaryArtifactCertifiedSurface::CanonicalBasisParticipation,
            FoundationalBoundaryArtifactCertifiedSurface::CurrentBasisProofLane,
            FoundationalBoundaryArtifactCertifiedSurface::DescriptiveExtensionLaw,
        ],
    );
    assert_exact_inventory(
        "compile-fail boundaries",
        report.compile_fail_boundaries(),
        &[
            FoundationalBoundaryArtifactCompileFailBoundary::CategoryWrapperCollapseRejected,
            FoundationalBoundaryArtifactCompileFailBoundary::IllegalRoleAndAuthorityClaimsRejected,
            FoundationalBoundaryArtifactCompileFailBoundary::PlainPayloadCannotBypassMaterializationContracts,
            FoundationalBoundaryArtifactCompileFailBoundary::RawMaterializedOutputsCannotSatisfyCanonicalBasisApis,
            FoundationalBoundaryArtifactCompileFailBoundary::RawMaterializedOutputsCannotSatisfyCurrentBasisApis,
            FoundationalBoundaryArtifactCompileFailBoundary::DescriptiveExtensionsCannotSatisfyAuthorityOrReservedAuthorityApis,
            FoundationalBoundaryArtifactCompileFailBoundary::BoundaryArtifactReadinessRequiresCertifiedArtifact,
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
fn production_readiness_report_names_worth_proof_dependency_boundary_and_runtime_handoff() {
    let report = foundational_boundary_artifact_milestone4_readiness_report();

    assert_exact_inventory(
        "worth-proof required surfaces",
        report.worth_proof_required_surfaces(),
        &[
            FoundationalBoundaryArtifactWORTHProofSurface::AuthorityWitness,
            FoundationalBoundaryArtifactWORTHProofSurface::AuthorityAdmissionProofBearingClaim,
            FoundationalBoundaryArtifactWORTHProofSurface::TransitionOutcome,
            FoundationalBoundaryArtifactWORTHProofSurface::CurrentBasisArtifactConstructor,
            FoundationalBoundaryArtifactWORTHProofSurface::BoundaryBridgeTrustBoundary,
            FoundationalBoundaryArtifactWORTHProofSurface::BoundaryReadmitWithAuthority,
            FoundationalBoundaryArtifactWORTHProofSurface::ProductionReadinessCertificationArtifact,
        ],
    );
    assert_exact_inventory(
        "worth-proof api appendix",
        report.worth_proof_api_appendix(),
        &[
            FoundationalBoundaryArtifactWORTHProofApi::AuthorityWitnessFromAuthorityMarker,
            FoundationalBoundaryArtifactWORTHProofApi::ProofFromAuthorityWitness,
            FoundationalBoundaryArtifactWORTHProofApi::ArtifactWithCurrentBasisProofs,
            FoundationalBoundaryArtifactWORTHProofApi::ArtifactWithProofsAndCurrentBasis,
            FoundationalBoundaryArtifactWORTHProofApi::TransitionOutcomeStructuredCategories,
            FoundationalBoundaryArtifactWORTHProofApi::ArtifactBridgeTrustBoundary,
            FoundationalBoundaryArtifactWORTHProofApi::ArtifactReadmitWithAuthority,
        ],
    );
    assert_exact_inventory(
        "worth-proof forbidden surfaces",
        report.worth_proof_forbidden_surfaces(),
        &[
            FoundationalBoundaryArtifactWORTHProofForbiddenSurface::PlainCategoryVocabulary,
            FoundationalBoundaryArtifactWORTHProofForbiddenSurface::PlainRoleAndMaterializationVocabulary,
            FoundationalBoundaryArtifactWORTHProofForbiddenSurface::PlainBundleMembershipData,
            FoundationalBoundaryArtifactWORTHProofForbiddenSurface::PlainSameFamilyDescriptiveNouns,
        ],
    );
    assert!(report.assumptions().contains(
        &FoundationalBoundaryArtifactRuntimeAssumption::Milestone2CanonicalizationRemainsAuthorityForBasisReadiness
    ));
    assert!(report.assumptions().contains(
        &FoundationalBoundaryArtifactRuntimeAssumption::StrongerAuthorityAndCurrentBasisClaimsRequireProofLane
    ));
    assert!(report.non_assumptions().contains(
        &FoundationalBoundaryArtifactRuntimeNonAssumption::ReceiptSemanticsBeyondCategoryLawAlreadyOwnedHere
    ));
    assert!(report.residual_debt().contains(
        &FoundationalBoundaryArtifactResidualDebt::ReservedAuthorityTransitionOntologyDeferred
    ));
}

#[test]
fn production_readiness_surface_evidence_binds_surfaces_to_real_hostile_compile_fail_and_blind_consumer_tests(
) {
    let report = foundational_boundary_artifact_milestone4_readiness_report();

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
fn production_readiness_phase_gates_are_linear_and_evidence_backed() {
    let report = foundational_boundary_artifact_milestone4_readiness_report();
    let gates: Vec<_> = report
        .phase_gates()
        .iter()
        .map(|evidence| evidence.gate())
        .collect();

    assert_eq!(
        gates,
        vec![
            FoundationalBoundaryArtifactMilestone4PhaseGate::Categories,
            FoundationalBoundaryArtifactMilestone4PhaseGate::RoleAndAuthority,
            FoundationalBoundaryArtifactMilestone4PhaseGate::MaterializationAndBundles,
            FoundationalBoundaryArtifactMilestone4PhaseGate::CanonicalBasisParticipation,
            FoundationalBoundaryArtifactMilestone4PhaseGate::CurrentBasisProofLane,
            FoundationalBoundaryArtifactMilestone4PhaseGate::DescriptiveExtensions,
            FoundationalBoundaryArtifactMilestone4PhaseGate::ProductionReadiness,
        ]
    );
    assert!(report
        .phase_gates()
        .iter()
        .all(|evidence| crate_root_path(evidence.evidence_path()).exists()));
}

#[test]
fn production_readiness_hostile_pressure_inventory_stays_exact() {
    let report = foundational_boundary_artifact_milestone4_readiness_report();

    assert_exact_inventory(
        "synthetic pressures",
        report.synthetic_pressures(),
        &[
            FoundationalBoundaryArtifactSyntheticRuntimePressure::CategoryAdjacencyHostility,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::AuthorityDerivationSeparation,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::MaterializationSeamHonesty,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::CanonicalBasisParity,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::CurrentBasisReadmissionBoundary,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::ReservedAuthorityTransitionFailClosedBoundary,
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
