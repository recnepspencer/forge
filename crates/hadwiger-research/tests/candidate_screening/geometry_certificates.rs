use hadwiger_research::facade::*;

use super::support::{complete_graph, handle, transcript};

#[test]
fn rectangular_geometry_screening_rejects_unit_conflicts() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();

    let minkowski = evaluate_minkowski_difference_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        MinkowskiUnitIntersectionCertificate::new(
            unit_square("left", 0),
            unit_square("right", 1),
            transcript("minkowski"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(minkowski.rejects_candidate());
    assert!(minkowski.evidence().contains("query_declaration_digest="));

    let forbidden = evaluate_forbidden_displacement_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        ForbiddenDisplacementCertificate::new(
            unit_square("tile", 0),
            ScreeningRational::integer(1),
            ScreeningRational::integer(0),
            transcript("forbidden"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(forbidden.rejects_candidate());
    assert!(forbidden.evidence().contains("query_declaration_digest="));

    let periodic = evaluate_periodic_quotient_graph_screening_checked(
        &handle,
        &catalog,
        subject,
        periodic_model(),
        PeriodicQuotientConflictCertificate::new(
            "left",
            "right",
            ScreeningRational::integer(-1),
            ScreeningRational::integer(0),
            transcript("periodic"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(periodic.rejects_candidate());
    assert!(periodic.evidence().contains("query_declaration_digest="));
}

#[test]
fn rectangular_geometry_screening_has_query_declaration_readiness() {
    let handle = handle();

    let minkowski =
        research_declaration_entry_readiness::<MinkowskiDifferenceScreeningDeclaration>(&handle);
    let forbidden =
        research_declaration_entry_readiness::<ForbiddenDisplacementScreeningDeclaration>(&handle);
    let periodic =
        research_declaration_entry_readiness::<PeriodicQuotientGraphScreeningDeclaration>(&handle);

    assert!(!minkowski.rows().is_empty());
    assert!(!forbidden.rows().is_empty());
    assert!(!periodic.rows().is_empty());
}

#[test]
fn rectangular_geometry_replay_rejects_false_certificates() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();

    let err = evaluate_minkowski_difference_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        MinkowskiUnitIntersectionCertificate::new(
            unit_square("left", 0),
            unit_square("far", 4),
            transcript("bad-minkowski"),
        )
        .unwrap(),
    )
    .expect_err("far rectangles cannot certify unit-circle intersection");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::MinkowskiDifferenceGeometry,
            reason: "minkowski_difference_misses_unit_circle"
        }
    );

    let err = evaluate_forbidden_displacement_screening_checked(
        &handle,
        &catalog,
        subject.clone(),
        ForbiddenDisplacementCertificate::new(
            unit_square("tile", 0),
            ScreeningRational::integer(3),
            ScreeningRational::integer(0),
            transcript("bad-forbidden"),
        )
        .unwrap(),
    )
    .expect_err("far displacement cannot certify forbidden set membership");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::ForbiddenDisplacementSet,
            reason: "displacement_not_forbidden_for_rectangle"
        }
    );

    let err = evaluate_periodic_quotient_graph_screening_checked(
        &handle,
        &catalog,
        subject,
        periodic_model(),
        PeriodicQuotientConflictCertificate::new(
            "left",
            "right",
            ScreeningRational::integer(3),
            ScreeningRational::integer(0),
            transcript("bad-periodic"),
        )
        .unwrap(),
    )
    .expect_err("translated quotient pair must replay exact conflict");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::PeriodicQuotientGraph,
            reason: "periodic_translated_pair_has_no_unit_conflict"
        }
    );
}

#[test]
fn periodic_quotient_replay_requires_same_color_tiles() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();
    let model = PeriodicQuotientRectangleModel::new(
        "different-color-periodic-rectangles",
        vec![
            PeriodicQuotientTile::new("left", "red", unit_square("left-region", 0)).unwrap(),
            PeriodicQuotientTile::new("right", "blue", unit_square("right-region", 2)).unwrap(),
        ],
    )
    .unwrap();

    let err = evaluate_periodic_quotient_graph_screening_checked(
        &handle,
        &catalog,
        subject,
        model,
        PeriodicQuotientConflictCertificate::new(
            "left",
            "right",
            ScreeningRational::integer(-1),
            ScreeningRational::integer(0),
            transcript("different-color-periodic"),
        )
        .unwrap(),
    )
    .expect_err("periodic quotient rejection must be a same-color unit conflict");
    assert_eq!(
        err,
        CandidateScreeningError::CertificateReplayRejected {
            family: CandidateScreeningInvariantFamily::PeriodicQuotientGraph,
            reason: "periodic_conflict_tiles_not_same_color"
        }
    );
}

fn periodic_model() -> PeriodicQuotientRectangleModel {
    PeriodicQuotientRectangleModel::new(
        "periodic-rectangles",
        vec![
            PeriodicQuotientTile::new("left", "red", unit_square("left-region", 0)).unwrap(),
            PeriodicQuotientTile::new("right", "red", unit_square("right-region", 2)).unwrap(),
        ],
    )
    .unwrap()
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
