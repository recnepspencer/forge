use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle().unwrap()
}

fn tile(tile_id: &str, color_id: &str, x_min: i128, x_max: i128) -> RectangularTileRegion {
    RectangularTileRegion::new(
        tile_id,
        TilingColorId::new(color_id).unwrap(),
        ExactRational::integer(x_min),
        ExactRational::integer(x_max),
        ExactRational::integer(0),
        ExactRational::integer(1),
    )
    .unwrap()
    .with_boundary_ownership(BoundaryOwnershipPolicy::owned_half_open("left,bottom").unwrap())
}

fn unit_crossing_cell() -> TilingCell {
    TilingCell::builder("contact-cell")
        .with_rectangular_tile(tile("tile-a", "red", 0, 1))
        .unwrap()
        .with_rectangular_tile(tile("tile-b", "red", 1, 2))
        .unwrap()
        .finish()
        .unwrap()
}

fn internal_named_tile_cell() -> TilingCell {
    TilingCell::builder("diameter-cell")
        .with_rectangular_tile(tile("tile_internal", "red", 0, 1))
        .unwrap()
        .finish()
        .unwrap()
}

#[test]
fn same_color_contact_lowers_through_query_and_exact_screening() {
    let handle = handle();
    let cell = unit_crossing_cell();
    let report =
        evaluate_tiling_same_color_contact_checked(&handle, &cell, "tile-a", "tile-b").unwrap();

    assert_eq!(
        report.evaluation().family(),
        CandidateScreeningInvariantFamily::SameColorSeparationDistanceSet
    );
    assert_eq!(
        report.evaluation().verdict(),
        CandidateScreeningVerdict::Rejected
    );
    assert_eq!(report.contact_fact().left_tile_id(), "tile-a");
    assert_eq!(report.contact_fact().right_tile_id(), "tile-b");
    assert_eq!(report.counters().contact_pairs_checked(), 1);
    assert_eq!(report.counters().query_declarations_performed(), 2);
    assert!(report.contact_witness_declaration_digest().is_some());
    assert!(report.query_declaration_digest().is_some());
    assert!(!report.admits_theorem_authority());
}

#[test]
fn reversed_contact_order_converges_to_same_replay_digest() {
    let handle = handle();
    let cell = unit_crossing_cell();
    let forward =
        evaluate_tiling_same_color_contact_checked(&handle, &cell, "tile-a", "tile-b").unwrap();
    let reversed =
        evaluate_tiling_same_color_contact_checked(&handle, &cell, "tile-b", "tile-a").unwrap();

    assert_eq!(forward.contact_fact(), reversed.contact_fact());
    assert_eq!(forward.artifact_digest(), reversed.artifact_digest());
}

#[test]
fn tile_diameter_replay_does_not_collide_with_internal_named_tile() {
    let handle = handle();
    let cell = internal_named_tile_cell();
    let report = evaluate_tiling_tile_diameter_checked(&handle, &cell, "tile_internal").unwrap();

    assert!(report.is_exact_replay());
    assert_eq!(
        report.contact_fact().role(),
        TilingContactRole::DiameterSafety
    );
    assert_eq!(report.counters().query_declarations_performed(), 1);
    assert!(report.contact_witness_declaration_digest().is_none());
}

#[test]
fn minkowski_contact_replay_rejects_only_from_exact_certificate() {
    let handle = handle();
    let cell = unit_crossing_cell();
    let report =
        evaluate_tiling_minkowski_contact_checked(&handle, &cell, "tile-a", "tile-b").unwrap();

    assert_eq!(
        report.evaluation().family(),
        CandidateScreeningInvariantFamily::MinkowskiDifferenceGeometry
    );
    assert!(report.is_exact_replay());
    assert_eq!(
        report.contact_fact().role(),
        TilingContactRole::MinkowskiUnitContact
    );
    assert!(!report.admits_theorem_authority());
}

#[test]
fn contact_replay_rejects_missing_and_self_contact_scope() {
    let handle = handle();
    let cell = unit_crossing_cell();

    assert_eq!(
        evaluate_tiling_same_color_contact_checked(&handle, &cell, "tile-a", "missing"),
        Err(TilingGeometryError::MissingTile {
            tile_id: "missing".to_string()
        })
    );
    assert_eq!(
        evaluate_tiling_same_color_contact_checked(&handle, &cell, "tile-a", "tile-a"),
        Err(TilingGeometryError::SameTileContact {
            tile_id: "tile-a".to_string()
        })
    );
}
