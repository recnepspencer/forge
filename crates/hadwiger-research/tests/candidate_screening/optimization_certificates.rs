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

    let mut entries = vec![vec![ScreeningRational::integer(0); 7]; 7];
    for index in 0..7 {
        entries[index][index] = ScreeningRational::integer(1);
    }
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
fn periodic_measure_certificates_reject_autocorrelation_density_and_local_density() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();
    let model = periodic_red_block();

    let autocorrelation = AutocorrelationOverlapCertificate::new(
        "red",
        ScreeningRational::integer(1),
        ScreeningRational::integer(0),
        ScreeningRational::integer(1),
        transcript("autocorrelation"),
    )
    .unwrap();
    let auto_eval = evaluate_autocorrelation_zero_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        model.clone(),
        autocorrelation,
    )
    .unwrap();
    assert!(auto_eval.rejects_candidate());

    let density = DensityCapCertificate::new(
        "red",
        ScreeningRational::fraction(1, 2).unwrap(),
        "retained-density-cap:test",
        transcript("density"),
    )
    .unwrap();
    let density_eval = evaluate_density_cap_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        model.clone(),
        density,
    )
    .unwrap();
    assert!(density_eval.rejects_candidate());

    let local = LocalDensityWindowCertificate::new(
        "red",
        PeriodicMeasureWindow::rectangle(
            "unit-window",
            ScreeningRational::integer(0),
            ScreeningRational::integer(1),
            ScreeningRational::integer(0),
            ScreeningRational::integer(1),
        )
        .unwrap(),
        ScreeningRational::fraction(1, 2).unwrap(),
        "retained-local-window-bound:test",
        transcript("local-density"),
    )
    .unwrap();
    let local_eval =
        evaluate_local_density_window_screening_checked(&handle, &catalog, subject, model, local)
            .unwrap();
    assert!(local_eval.rejects_candidate());
}

#[test]
fn measure_model_rejects_zero_area_and_out_of_period_cells() {
    assert!(PeriodicMeasureCell::rectangle(
        "red",
        ScreeningRational::integer(0),
        ScreeningRational::integer(0),
        ScreeningRational::integer(0),
        ScreeningRational::integer(1),
    )
    .is_err());

    let out_of_period = PeriodicColorClassMeasureModel::new(
        "bad-period",
        ScreeningRational::integer(1),
        ScreeningRational::integer(1),
        vec![PeriodicMeasureCell::rectangle(
            "red",
            ScreeningRational::integer(0),
            ScreeningRational::integer(2),
            ScreeningRational::integer(0),
            ScreeningRational::integer(1),
        )
        .unwrap()],
    );
    assert!(out_of_period.is_err());
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
        transcript(label),
    )
    .unwrap();
    evaluate_lovasz_theta_certificate_checked(catalog, graph, certificate)
}

fn periodic_red_block() -> PeriodicColorClassMeasureModel {
    PeriodicColorClassMeasureModel::new(
        "periodic-red-block",
        ScreeningRational::integer(3),
        ScreeningRational::integer(1),
        vec![PeriodicMeasureCell::rectangle(
            "red",
            ScreeningRational::integer(0),
            ScreeningRational::integer(2),
            ScreeningRational::integer(0),
            ScreeningRational::integer(1),
        )
        .unwrap()],
    )
    .unwrap()
}
