use std::collections::BTreeSet;
use std::fmt::Debug;
use std::path::Path;
use worth_foundational::{
    certify_foundational_profile_milestone3_production_test_readiness,
    foundational_profile_milestone3_readiness_report, profiles_api::FoundationalProfilePublicLane,
    require_foundational_profile_milestone3_production_test_readiness,
    FoundationalProfileCertifiedSurface, FoundationalProfileCompileFailBoundary,
    FoundationalProfileMilestone3PhaseGate, FoundationalProfileProductionReadinessCertified,
    FoundationalProfileProductionTestReadyArtifact, FoundationalProfileResidualDebt,
    FoundationalProfileRuntimeAssumption, FoundationalProfileRuntimeNonAssumption,
    FoundationalProfileSyntheticRuntimePressure, FoundationalProfileWORTHProofApi,
    FoundationalProfileWORTHProofForbiddenSurface, FoundationalProfileWORTHProofSurface,
};

fn accepts_profile_readiness_artifact(_: &FoundationalProfileProductionTestReadyArtifact) {}
fn accepts_profile_readiness_proof(
    _: &worth_proof::Proof<
        FoundationalProfileProductionReadinessCertified,
        worth_foundational::FoundationalProfileProductionReadinessAuthority,
    >,
) {
}

#[test]
fn production_readiness_artifact_carries_complete_machine_checkable_inventory() {
    let readiness = certify_foundational_profile_milestone3_production_test_readiness();
    let report = require_foundational_profile_milestone3_production_test_readiness(&readiness);

    accepts_profile_readiness_artifact(&readiness);
    accepts_profile_readiness_proof(readiness.proofs());
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        readiness.strong_basis().value().milestone(),
        "worth-foundational.milestone-3"
    );

    assert_exact_inventory(
        "certified surfaces",
        report.certified_surfaces(),
        &[
            FoundationalProfileCertifiedSurface::ProfileFamilies,
            FoundationalProfileCertifiedSurface::ProfileComposition,
            FoundationalProfileCertifiedSurface::ProgressionAndAttachment,
            FoundationalProfileCertifiedSurface::CanonicalIdentityAndDifference,
            FoundationalProfileCertifiedSurface::MaterializationAndElision,
            FoundationalProfileCertifiedSurface::CertificationStrengthening,
        ],
    );
    assert_exact_inventory(
        "compile-fail boundaries",
        report.compile_fail_boundaries(),
        &[
            FoundationalProfileCompileFailBoundary::RawLabelsCannotSatisfyProfileFamilyApis,
            FoundationalProfileCompileFailBoundary::PartialOrBagConstructionCannotSatisfyProfileSetApis,
            FoundationalProfileCompileFailBoundary::PlainPayloadCannotSatisfyAttachmentApis,
            FoundationalProfileCompileFailBoundary::RawDigestCannotSatisfyProfileIdentityApis,
            FoundationalProfileCompileFailBoundary::IllegalTargetSurfaceInventoriesCannotBeWorthd,
            FoundationalProfileCompileFailBoundary::WrongStrengthProofBearingCertificationCannotSatisfyStrongerApis,
            FoundationalProfileCompileFailBoundary::ProfileReadinessRequiresCertifiedArtifact,
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
    let report = foundational_profile_milestone3_readiness_report();

    assert_exact_inventory(
        "worth-proof required surfaces",
        report.worth_proof_required_surfaces(),
        &[
            FoundationalProfileWORTHProofSurface::ArtifactCarrier,
            FoundationalProfileWORTHProofSurface::TransitionOutcome,
            FoundationalProfileWORTHProofSurface::AuthorityWitness,
            FoundationalProfileWORTHProofSurface::BoundaryBridgeTrustBoundary,
            FoundationalProfileWORTHProofSurface::BoundaryReadmitWithAuthority,
            FoundationalProfileWORTHProofSurface::CurrentBasisArtifactConstructor,
        ],
    );
    assert_exact_inventory(
        "worth-proof api appendix",
        report.worth_proof_api_appendix(),
        &[
            FoundationalProfileWORTHProofApi::AuthorityWitnessFromAuthorityMarker,
            FoundationalProfileWORTHProofApi::ArtifactNew,
            FoundationalProfileWORTHProofApi::ArtifactWithCurrentBasis,
            FoundationalProfileWORTHProofApi::ArtifactWithProofsAndCurrentBasis,
            FoundationalProfileWORTHProofApi::TransitionOutcomeStructuredCategories,
            FoundationalProfileWORTHProofApi::ArtifactBridgeTrustBoundary,
            FoundationalProfileWORTHProofApi::ArtifactReadmitWithAuthority,
        ],
    );
    assert_exact_inventory(
        "worth-proof forbidden surfaces",
        report.worth_proof_forbidden_surfaces(),
        &[
            FoundationalProfileWORTHProofForbiddenSurface::PlainProfileFamilyVocabulary,
            FoundationalProfileWORTHProofForbiddenSurface::PlainProfileCompositionData,
            FoundationalProfileWORTHProofForbiddenSurface::PlainDescriptiveSurfaceVocabulary,
            FoundationalProfileWORTHProofForbiddenSurface::PlainProfileIdentityBasisEntries,
        ],
    );
    assert!(report
        .assumptions()
        .contains(&FoundationalProfileRuntimeAssumption::ReducedRichnessAffectsOnlyOptionalDescriptiveSurfaces));
    assert!(report
        .non_assumptions()
        .contains(&FoundationalProfileRuntimeNonAssumption::BoundaryCrossingPreservesStrongerCertificationWithoutReadmission));
    assert!(report
        .residual_debt()
        .contains(&FoundationalProfileResidualDebt::AdoptingCrateParityDeferred));
    assert!(crate_root_path(report.public_surface_evidence_path()).is_file());
    assert!(crate_root_path(report.public_surface_compile_fail_path()).is_file());
}

#[test]
fn production_readiness_surface_evidence_binds_surfaces_to_real_hostile_and_compile_fail_tests() {
    let report = foundational_profile_milestone3_readiness_report();

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
    let report = foundational_profile_milestone3_readiness_report();
    let gates: Vec<_> = report
        .phase_gates()
        .iter()
        .map(|evidence| evidence.gate())
        .collect();

    assert_eq!(
        gates,
        vec![
            FoundationalProfileMilestone3PhaseGate::TypedFamilies,
            FoundationalProfileMilestone3PhaseGate::ComposedProfileSet,
            FoundationalProfileMilestone3PhaseGate::ProgressionAndAttachment,
            FoundationalProfileMilestone3PhaseGate::CanonicalIdentityAndDifference,
            FoundationalProfileMilestone3PhaseGate::MaterializationAndElision,
            FoundationalProfileMilestone3PhaseGate::CertificationStrengthening,
            FoundationalProfileMilestone3PhaseGate::ProductionReadiness,
        ]
    );
    assert!(report
        .phase_gates()
        .iter()
        .all(|evidence| crate_root_path(evidence.evidence_path()).exists()));
}

#[test]
fn production_readiness_hostile_pressure_inventory_stays_exact() {
    let report = foundational_profile_milestone3_readiness_report();

    assert_exact_inventory(
        "synthetic pressures",
        report.synthetic_pressures(),
        &[
            FoundationalProfileSyntheticRuntimePressure::FamilyAdjacencyHostility,
            FoundationalProfileSyntheticRuntimePressure::IndependentConstructionParity,
            FoundationalProfileSyntheticRuntimePressure::ReducedRichnessSuppression,
            FoundationalProfileSyntheticRuntimePressure::AttachmentTargetLaw,
            FoundationalProfileSyntheticRuntimePressure::ProofBearingCertificationBoundary,
        ],
    );
}

#[test]
fn production_readiness_public_surface_inventory_stays_exact() {
    let report = foundational_profile_milestone3_readiness_report();

    assert_eq!(report.public_surface_inventory().len(), 9);
    assert_eq!(
        report
            .public_surface_inventory()
            .iter()
            .filter(|entry| entry.lane() == FoundationalProfilePublicLane::CommonPath)
            .count(),
        1
    );
    assert_eq!(
        report
            .public_surface_inventory()
            .iter()
            .filter(|entry| entry.lane() == FoundationalProfilePublicLane::StrongerLane)
            .count(),
        2
    );
    assert!(report
        .public_surface_inventory()
        .iter()
        .any(|entry| { entry.path() == "worth_foundational::profiles_api::common_path" }));
    assert!(report.public_surface_inventory().iter().all(|entry| {
        !entry.teaches().trim().is_empty() && !entry.does_not_hide().trim().is_empty()
    }));
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
