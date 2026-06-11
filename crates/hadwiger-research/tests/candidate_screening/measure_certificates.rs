use hadwiger_research::facade::*;

use super::support::{complete_graph, handle, transcript};

#[test]
fn periodic_measure_certificates_reject_autocorrelation_density_and_local_density() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();
    let model = periodic_red_block();

    let auto_eval = evaluate_autocorrelation_zero_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        model.clone(),
        autocorrelation_certificate("autocorrelation", 1),
    )
    .unwrap();
    assert!(auto_eval.rejects_candidate());
    assert!(auto_eval.evidence().contains("query_declaration_digest="));
    assert!(auto_eval
        .evidence()
        .contains("autocorrelation_certificate="));

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
    assert!(density_eval
        .evidence()
        .contains("query_declaration_digest="));
    assert!(density_eval.evidence().contains("density_cap_certificate="));

    let local_eval = evaluate_local_density_window_screening_checked(
        &handle,
        &catalog,
        subject,
        model,
        local_density_certificate(),
    )
    .unwrap();
    assert!(local_eval.rejects_candidate());
    assert!(local_eval.evidence().contains("query_declaration_digest="));
    assert!(local_eval
        .evidence()
        .contains("local_density_window_certificate="));
}

#[test]
fn autocorrelation_screening_has_query_declaration_readiness() {
    let handle = handle();

    let readiness =
        research_declaration_entry_readiness::<AutocorrelationZeroScreeningDeclaration>(&handle);

    assert!(!readiness.rows().is_empty());
}

#[test]
fn density_and_local_density_screening_have_query_declaration_readiness() {
    let handle = handle();

    let density_readiness =
        research_declaration_entry_readiness::<DensityCapScreeningDeclaration>(&handle);
    let local_readiness =
        research_declaration_entry_readiness::<LocalDensityWindowScreeningDeclaration>(&handle);

    assert!(!density_readiness.rows().is_empty());
    assert!(!local_readiness.rows().is_empty());
}

#[test]
fn autocorrelation_replay_rejects_non_unit_and_corrupt_overlap() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();
    let model = periodic_red_block();

    let non_unit = AutocorrelationOverlapCertificate::new(
        "red",
        ScreeningRational::integer(2),
        ScreeningRational::integer(0),
        ScreeningRational::integer(1),
        transcript("non-unit-autocorrelation"),
    )
    .unwrap();
    let err = evaluate_autocorrelation_zero_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        model.clone(),
        non_unit,
    )
    .expect_err("non-unit displacement cannot certify autocorrelation failure");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::AutocorrelationZero,
            reason: "autocorrelation_displacement_not_unit"
        }
    );

    let err = evaluate_autocorrelation_zero_screening_checked(
        &handle,
        &catalog,
        subject,
        model,
        autocorrelation_certificate("bad-overlap-autocorrelation", 2),
    )
    .expect_err("claimed overlap must replay exactly");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::AutocorrelationZero,
            reason: "autocorrelation_overlap_not_positive"
        }
    );
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

#[test]
fn density_and_local_density_replay_reject_weak_caps() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();
    let model = periodic_red_block();

    let weak_density = DensityCapCertificate::new(
        "red",
        ScreeningRational::integer(1),
        "retained-density-cap:weak",
        transcript("weak-density"),
    )
    .unwrap();
    let err = evaluate_density_cap_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        model.clone(),
        weak_density,
    )
    .expect_err("density must strictly exceed retained cap");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::DensityCapEachColorClass,
            reason: "density_does_not_exceed_cap"
        }
    );

    let weak_local = LocalDensityWindowCertificate::new(
        "red",
        PeriodicMeasureWindow::rectangle(
            "unit-window",
            ScreeningRational::integer(0),
            ScreeningRational::integer(1),
            ScreeningRational::integer(0),
            ScreeningRational::integer(1),
        )
        .unwrap(),
        ScreeningRational::integer(1),
        "retained-local-window-bound:weak",
        transcript("weak-local"),
    )
    .unwrap();
    let err = evaluate_local_density_window_screening_checked(
        &handle, &catalog, subject, model, weak_local,
    )
    .expect_err("local density must strictly exceed retained bound");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::LocalDensityWindow,
            reason: "local_density_does_not_exceed_cap"
        }
    );
}

fn autocorrelation_certificate(
    label: &str,
    claimed_overlap: i128,
) -> AutocorrelationOverlapCertificate {
    AutocorrelationOverlapCertificate::new(
        "red",
        ScreeningRational::integer(1),
        ScreeningRational::integer(0),
        ScreeningRational::integer(claimed_overlap),
        transcript(label),
    )
    .unwrap()
}

fn local_density_certificate() -> LocalDensityWindowCertificate {
    LocalDensityWindowCertificate::new(
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
    .unwrap()
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
