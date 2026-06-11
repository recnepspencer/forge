use hadwiger_research::facade::*;

use super::support::{complete_graph, handle, path_graph, transcript};

#[test]
fn fractional_chromatic_replay_rejects_k7_and_rejects_corrupt_dual() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = complete_graph(7);
    let checked =
        evaluate_fractional_chromatic_screening_checked(&handle, &catalog, &graph).unwrap();

    assert!(checked.rejects_candidate());
    assert_eq!(
        checked.mode(),
        CandidateScreeningEvaluationMode::SolverBackedCertificate
    );
    assert!(checked.evidence().contains("query_declaration_digest="));
    assert!(checked.evidence().contains("fractional_dual_certificate"));

    let corrupt = FractionalChromaticCertificate::new(
        "bad-dual",
        vec![("v0".to_string(), ScreeningRational::integer(2))],
        ScreeningRational::integer(2),
        transcript("bad-fractional"),
    )
    .unwrap();
    let err = evaluate_fractional_chromatic_certificate_checked(&catalog, &graph, corrupt)
        .expect_err("dual constraint violation should reject certificate");

    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::FractionalChromaticNumber,
            reason: "independent_set_dual_constraint_violated"
        }
    );
}

#[test]
fn fractional_chromatic_replay_rejects_duplicate_and_negative_weights() {
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle()).unwrap();
    let graph = complete_graph(3);

    let duplicate = FractionalChromaticCertificate::new(
        "duplicate-dual",
        vec![
            ("v0".to_string(), ScreeningRational::integer(1)),
            ("v0".to_string(), ScreeningRational::integer(1)),
        ],
        ScreeningRational::integer(2),
        transcript("duplicate"),
    )
    .unwrap();
    let err = evaluate_fractional_chromatic_certificate_checked(&catalog, &graph, duplicate)
        .expect_err("duplicate vertex weight should reject certificate");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::FractionalChromaticNumber,
            reason: "duplicate_certificate_vertex"
        }
    );

    let negative = FractionalChromaticCertificate::new(
        "negative-dual",
        vec![("v0".to_string(), ScreeningRational::integer(-1))],
        ScreeningRational::integer(-1),
        transcript("negative"),
    )
    .unwrap();
    let err = evaluate_fractional_chromatic_certificate_checked(&catalog, &graph, negative)
        .expect_err("negative dual weight should reject certificate");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::FractionalChromaticNumber,
            reason: "negative_certificate_weight"
        }
    );
}

#[test]
fn fractional_chromatic_replay_does_not_reject_six_colorable_path() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = path_graph(7);
    let checked =
        evaluate_fractional_chromatic_screening_checked(&handle, &catalog, &graph).unwrap();

    assert_eq!(checked.verdict(), CandidateScreeningVerdict::Passed);
}

#[test]
fn fractional_chromatic_screening_has_query_declaration_readiness() {
    let handle = handle();

    let readiness =
        research_declaration_entry_readiness::<FractionalChromaticScreeningDeclaration>(&handle);

    assert!(!readiness.rows().is_empty());
}

#[test]
fn lovasz_theta_replay_rejects_k7_and_bad_psd_witnesses() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = complete_graph(7);
    let checked = evaluate_lovasz_theta_screening_checked(&handle, &catalog, &graph).unwrap();

    assert!(checked.rejects_candidate());
    assert_eq!(
        checked.mode(),
        CandidateScreeningEvaluationMode::SolverBackedCertificate
    );
    assert!(checked.evidence().contains("query_declaration_digest="));
    assert!(checked.evidence().contains("lovasz_theta_certificate"));

    let bad_dimension = ScreeningMatrixCertificate::new(vec![
        vec![ScreeningRational::integer(1), ScreeningRational::integer(0)],
        vec![ScreeningRational::integer(0), ScreeningRational::integer(1)],
    ])
    .unwrap();
    let err = replay_bad_theta(&catalog, &graph, bad_dimension, "dimension")
        .expect_err("dimension mismatch should reject theta certificate");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::LovaszThetaBound,
            reason: "theta_matrix_dimension_mismatch"
        }
    );

    let mut entries = theta_identity_entries(7);
    entries[0][1] = ScreeningRational::integer(1);
    entries[1][0] = ScreeningRational::integer(1);
    let off_diagonal = ScreeningMatrixCertificate::new(entries).unwrap();
    let err = replay_bad_theta(&catalog, &graph, off_diagonal, "off-diagonal")
        .expect_err("non-diagonal Gram subclass should reject theta certificate");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::LovaszThetaBound,
            reason: "theta_psd_witness_not_diagonal_gram"
        }
    );
}

#[test]
fn lovasz_theta_replay_rejects_constraint_and_psd_witness_mismatches() {
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle()).unwrap();
    let graph = path_graph(3);
    let one_third = ScreeningRational::fraction(1, 3).unwrap();
    let all_positive = ScreeningMatrixCertificate::new(vec![
        vec![one_third.clone(), one_third.clone(), one_third.clone()],
        vec![one_third.clone(), one_third.clone(), one_third.clone()],
        vec![one_third.clone(), one_third.clone(), one_third.clone()],
    ])
    .unwrap();
    let constraint = LovaszThetaCertificate::new(
        "bad-complement-constraint",
        ScreeningRational::integer(3),
        all_positive.clone(),
        ScreeningPsdWitnessCertificate::constant_rank_one(one_third).unwrap(),
        transcript("bad-complement"),
    )
    .unwrap();
    let err = evaluate_lovasz_theta_certificate_checked(&catalog, &graph, constraint)
        .expect_err("nonedge of conflict graph must force theta matrix zero");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::LovaszThetaBound,
            reason: "theta_complement_zero_constraint_violated"
        }
    );

    let diagonal_claim = LovaszThetaCertificate::new(
        "bad-diagonal-psd",
        ScreeningRational::integer(3),
        all_positive,
        ScreeningPsdWitnessCertificate::diagonal_gram(),
        transcript("bad-diagonal"),
    )
    .unwrap();
    let err =
        evaluate_lovasz_theta_certificate_checked(&catalog, &complete_graph(3), diagonal_claim)
            .expect_err("diagonal witness cannot certify off-diagonal matrix");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::LovaszThetaBound,
            reason: "theta_psd_witness_not_diagonal_gram"
        }
    );
}

#[test]
fn lovasz_theta_screening_has_query_declaration_readiness() {
    let handle = handle();

    let readiness =
        research_declaration_entry_readiness::<LovaszThetaScreeningDeclaration>(&handle);

    assert!(!readiness.rows().is_empty());
}

fn replay_bad_theta(
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    matrix: ScreeningMatrixCertificate,
    label: &str,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let certificate = LovaszThetaCertificate::new(
        format!("bad-theta-{label}"),
        ScreeningRational::integer(7),
        matrix,
        ScreeningPsdWitnessCertificate::diagonal_gram(),
        transcript(label),
    )
    .unwrap();
    evaluate_lovasz_theta_certificate_checked(catalog, graph, certificate)
}

fn theta_identity_entries(dimension: usize) -> Vec<Vec<ScreeningRational>> {
    let mut entries = vec![vec![ScreeningRational::integer(0); dimension]; dimension];
    for (index, row) in entries.iter_mut().enumerate() {
        row[index] = ScreeningRational::integer(1);
    }
    entries
}
