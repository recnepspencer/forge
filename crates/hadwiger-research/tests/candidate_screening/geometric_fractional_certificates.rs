use hadwiger_research::facade::*;

use super::support::{complete_graph, handle, path_graph, transcript};

#[test]
fn geometric_fractional_replay_prioritizes_checked_frontier_evidence() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = complete_graph(4);
    let certificate = geometric_certificate(
        "k4-geometric",
        4,
        4,
        vec![
            ("v0".to_string(), ScreeningRational::integer(1)),
            ("v1".to_string(), ScreeningRational::integer(1)),
            ("v2".to_string(), ScreeningRational::integer(1)),
            ("v3".to_string(), ScreeningRational::integer(1)),
        ],
        Vec::new(),
    );

    let checked = evaluate_geometric_fractional_chromatic_screening_checked(
        &handle,
        &catalog,
        &graph,
        certificate,
    )
    .unwrap();

    assert_eq!(
        checked.family(),
        CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber
    );
    assert_eq!(checked.verdict(), CandidateScreeningVerdict::Priority);
    assert_eq!(
        checked.mode(),
        CandidateScreeningEvaluationMode::CheckedCertificate
    );
    assert!(checked.evidence().contains("query_declaration_digest="));
    assert!(checked
        .evidence()
        .contains("geometric_fractional_certificate"));
    assert!(!checked.admits_theorem_authority());
}

#[test]
fn geometric_fractional_replay_checks_weight_shape_and_dual_constraints() {
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle()).unwrap();
    let graph = complete_graph(2);
    let violating = geometric_certificate(
        "bad-geometric-dual",
        2,
        2,
        vec![("v0".to_string(), ScreeningRational::integer(2))],
        Vec::new(),
    );

    let err =
        evaluate_geometric_fractional_chromatic_certificate_checked(&catalog, &graph, violating)
            .expect_err("singleton independent set should violate geometric dual");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber,
            reason: "geometric_independent_set_constraint_violated"
        }
    );

    let negative = geometric_certificate(
        "negative-geometric-dual",
        -1,
        -1,
        vec![("v0".to_string(), ScreeningRational::integer(-1))],
        Vec::new(),
    );
    let err =
        evaluate_geometric_fractional_chromatic_certificate_checked(&catalog, &graph, negative)
            .expect_err("negative dual weight should reject");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber,
            reason: "negative_certificate_weight"
        }
    );
}

#[test]
fn geometric_fractional_replay_requires_exact_isometry_witnesses() {
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle()).unwrap();
    let graph = path_graph(3);
    let valid = geometric_certificate(
        "valid-isometry-adjustment",
        1,
        1,
        vec![("v0".to_string(), ScreeningRational::integer(1))],
        vec![edge_translation_adjustment(1, 1)],
    );

    let checked =
        evaluate_geometric_fractional_chromatic_certificate_checked(&catalog, &graph, valid)
            .unwrap();
    assert_eq!(checked.verdict(), CandidateScreeningVerdict::Priority);

    let broken_witness = GeometricSubsetIsometryWitness::new(
        "missing-distance",
        edge_translation_mapping(),
        Vec::new(),
    )
    .unwrap();
    let broken = geometric_certificate(
        "broken-isometry-adjustment",
        1,
        1,
        vec![("v0".to_string(), ScreeningRational::integer(1))],
        vec![edge_translation_adjustment_with_witness(broken_witness)],
    );
    let err = evaluate_geometric_fractional_chromatic_certificate_checked(&catalog, &graph, broken)
        .expect_err("pairwise distance witness should be mandatory");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber,
            reason: "missing_pairwise_isometry_distance"
        }
    );

    let mismatched = geometric_certificate(
        "mismatched-isometry-adjustment",
        1,
        1,
        vec![("v0".to_string(), ScreeningRational::integer(1))],
        vec![edge_translation_adjustment(1, 2)],
    );
    let err =
        evaluate_geometric_fractional_chromatic_certificate_checked(&catalog, &graph, mismatched)
            .expect_err("pairwise distance mismatch should reject isometry authority");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber,
            reason: "pairwise_isometry_distance_mismatch"
        }
    );
}

#[test]
fn geometric_fractional_screening_has_query_declaration_readiness() {
    let readiness = research_declaration_entry_readiness::<
        GeometricFractionalChromaticScreeningDeclaration,
    >(&handle());

    assert!(!readiness.rows().is_empty());
}

#[test]
fn geometric_fractional_replay_suppresses_moser_scope_improvement_without_escape_evidence() {
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle()).unwrap();
    let graph = complete_graph(5);
    let certificate = geometric_certificate(
        "moser-confined-improvement",
        5,
        5,
        vec![
            ("v0".to_string(), ScreeningRational::integer(1)),
            ("v1".to_string(), ScreeningRational::integer(1)),
            ("v2".to_string(), ScreeningRational::integer(1)),
            ("v3".to_string(), ScreeningRational::integer(1)),
            ("v4".to_string(), ScreeningRational::integer(1)),
        ],
        Vec::new(),
    )
    .with_moser_lattice_reproduction_scope();

    let err =
        evaluate_geometric_fractional_chromatic_certificate_checked(&catalog, &graph, certificate)
            .expect_err("Moser-confined improvement should be suppressed");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber,
            reason: "suppressed_moser_scope_without_escape_evidence"
        }
    );
}

fn edge_translation_adjustment(
    left_distance: i128,
    right_distance: i128,
) -> GeometricFractionalEqualityAdjustment {
    let witness = GeometricSubsetIsometryWitness::new(
        "edge-translation",
        edge_translation_mapping(),
        vec![GeometricPairwiseSquaredDistance::new(
            "v0",
            "v1",
            "v1",
            "v2",
            ScreeningRational::integer(left_distance),
            ScreeningRational::integer(right_distance),
        )
        .unwrap()],
    )
    .unwrap();
    edge_translation_adjustment_with_witness(witness)
}

fn edge_translation_adjustment_with_witness(
    witness: GeometricSubsetIsometryWitness,
) -> GeometricFractionalEqualityAdjustment {
    GeometricFractionalEqualityAdjustment::new(
        vec!["v0".to_string(), "v1".to_string()],
        vec!["v1".to_string(), "v2".to_string()],
        ScreeningRational::integer(0),
        witness,
    )
    .unwrap()
}

fn edge_translation_mapping() -> Vec<(String, String)> {
    vec![
        ("v0".to_string(), "v1".to_string()),
        ("v1".to_string(), "v2".to_string()),
    ]
}

fn geometric_certificate(
    label: &str,
    target: i128,
    lower_bound: i128,
    vertex_weights: Vec<(String, ScreeningRational)>,
    adjustments: Vec<GeometricFractionalEqualityAdjustment>,
) -> GeometricFractionalChromaticCertificate {
    GeometricFractionalChromaticCertificate::new(
        label,
        ScreeningRational::integer(target),
        vertex_weights,
        adjustments,
        ScreeningRational::integer(lower_bound),
        transcript(label),
    )
    .unwrap()
}
