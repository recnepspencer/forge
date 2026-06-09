use hadwiger_research::facade::*;

use super::support::{complete_graph, handle, transcript};

#[test]
fn rectangular_ownership_screening_rejects_exact_conflicts() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();

    assert_rejects(evaluate_exact_unit_distance_conflict_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        ExactUnitDistanceConflictCertificate::new(
            unit_square("left", 0),
            unit_square("right", 1),
            transcript("unit-conflict"),
        )
        .unwrap(),
    ));
    assert_rejects(evaluate_same_color_separation_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        SameColorSeparationCertificate::new(
            unit_square("left", 0),
            unit_square("right", 1),
            transcript("same-color"),
        )
        .unwrap(),
    ));
    assert_rejects(evaluate_tile_diameter_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        TileDiameterCertificate::new(unit_square("tile", 0), transcript("diameter")).unwrap(),
    ));
    assert_rejects(evaluate_exact_conflict_graph_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        ExactConflictGraphEdgeCertificate::new(
            "left",
            "right",
            unit_square("left", 0),
            unit_square("right", 1),
            transcript("conflict-edge"),
        )
        .unwrap(),
    ));
    assert_rejects(evaluate_numerical_margin_screening_checked(
        &handle,
        &catalog,
        subject,
        NumericalMarginCertificate::new(
            unit_square("left", 0),
            unit_square("right", 1),
            transcript("margin"),
        )
        .unwrap(),
    ));
}

#[test]
fn rectangular_ownership_screening_has_query_readiness() {
    let handle = handle();

    assert!(
        !research_declaration_entry_readiness::<ExactUnitDistanceConflictScreeningDeclaration>(
            &handle
        )
        .rows()
        .is_empty()
    );
    assert!(
        !research_declaration_entry_readiness::<SameColorSeparationScreeningDeclaration>(&handle)
            .rows()
            .is_empty()
    );
    assert!(
        !research_declaration_entry_readiness::<TileDiameterScreeningDeclaration>(&handle)
            .rows()
            .is_empty()
    );
    assert!(
        !research_declaration_entry_readiness::<ExactConflictGraphScreeningDeclaration>(&handle)
            .rows()
            .is_empty()
    );
    assert!(
        !research_declaration_entry_readiness::<NumericalMarginScreeningDeclaration>(&handle)
            .rows()
            .is_empty()
    );
}

#[test]
fn rectangular_ownership_replay_rejects_false_certificates() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();

    let err = evaluate_exact_unit_distance_conflict_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        ExactUnitDistanceConflictCertificate::new(
            unit_square("left", 0),
            unit_square("far", 4),
            transcript("bad-unit"),
        )
        .unwrap(),
    )
    .expect_err("unit-distance conflict must replay");
    assert_replay_error(
        err,
        CandidateScreeningInvariantFamily::ExactUnitDistanceConflict,
        "exact_unit_distance_conflict_not_replayed",
    );

    let err = evaluate_same_color_separation_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        SameColorSeparationCertificate::new(
            unit_square("left", 0),
            unit_square("far", 4),
            transcript("bad-same-color"),
        )
        .unwrap(),
    )
    .expect_err("same-color distance interval must contain unit");
    assert_replay_error(
        err,
        CandidateScreeningInvariantFamily::SameColorSeparationDistanceSet,
        "same_color_distance_set_misses_unit",
    );

    let err = evaluate_tile_diameter_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        TileDiameterCertificate::new(tiny_square("tiny"), transcript("bad-diameter")).unwrap(),
    )
    .expect_err("safe diameter cannot reject");
    assert_replay_error(
        err,
        CandidateScreeningInvariantFamily::TileDiameterSafety,
        "tile_diameter_below_unit",
    );

    let err = evaluate_exact_conflict_graph_screening_checked(
        &handle,
        &catalog,
        subject,
        ExactConflictGraphEdgeCertificate::new(
            "left",
            "far",
            unit_square("left", 0),
            unit_square("far", 4),
            transcript("bad-edge"),
        )
        .unwrap(),
    )
    .expect_err("conflict edge must replay");
    assert_replay_error(
        err,
        CandidateScreeningInvariantFamily::ExactConflictGraphConstruction,
        "conflict_graph_edge_not_certified",
    );

    let err = evaluate_numerical_margin_screening_checked(
        &handle,
        &catalog,
        complete_graph(2).reference(),
        NumericalMarginCertificate::new(
            unit_square("left", 0),
            unit_square("far", 4),
            transcript("bad-margin"),
        )
        .unwrap(),
    )
    .expect_err("clear distance interval margin cannot quarantine");
    assert_replay_error(
        err,
        CandidateScreeningInvariantFamily::NumericalMargin,
        "distance_interval_has_clear_margin",
    );
}

fn assert_rejects(result: Result<CandidateScreeningEvaluation, CandidateScreeningError>) {
    let evaluation = result.unwrap();
    assert!(evaluation.rejects_candidate());
    assert!(evaluation.evidence().contains("query_declaration_digest="));
}

fn assert_replay_error(
    error: CandidateScreeningError,
    family: CandidateScreeningInvariantFamily,
    reason: &'static str,
) {
    assert_eq!(
        error,
        CandidateScreeningError::CertificateReplayRejected { family, reason }
    );
}

fn unit_square(region_id: &str, x_offset: i128) -> ScreeningRectangularRegion {
    ScreeningRectangularRegion::new(
        region_id,
        ScreeningRational::integer(x_offset),
        ScreeningRational::integer(x_offset + 1),
        ScreeningRational::integer(0),
        ScreeningRational::integer(1),
    )
    .unwrap()
}

fn tiny_square(region_id: &str) -> ScreeningRectangularRegion {
    ScreeningRectangularRegion::new(
        region_id,
        ScreeningRational::integer(0),
        ScreeningRational::fraction(1, 4).unwrap(),
        ScreeningRational::integer(0),
        ScreeningRational::fraction(1, 4).unwrap(),
    )
    .unwrap()
}
