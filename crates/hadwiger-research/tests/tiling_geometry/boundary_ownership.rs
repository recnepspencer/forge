use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle().unwrap()
}

fn small_region(
    tile_id: &str,
    color_id: &str,
    x_min: i128,
    x_max: i128,
    boundary: BoundaryOwnershipPolicy,
) -> RectangularTileRegion {
    RectangularTileRegion::new(
        tile_id,
        TilingColorId::new(color_id).unwrap(),
        ExactRational::integer(x_min),
        ExactRational::integer(x_max),
        ExactRational::integer(0),
        ExactRational::fraction(1, 3).unwrap(),
    )
    .unwrap()
    .with_boundary_ownership(boundary)
}

#[test]
fn boundary_ownership_report_retains_query_screening_and_counters() {
    let handle = handle();
    let cell = TilingCell::builder("boundary-cell")
        .with_rectangular_tile(small_region(
            "tile-a",
            "red",
            0,
            1,
            BoundaryOwnershipPolicy::owned_closed(),
        ))
        .unwrap()
        .with_rectangular_tile(small_region(
            "tile-b",
            "blue",
            2,
            3,
            BoundaryOwnershipPolicy::owned_half_open("left,bottom").unwrap(),
        ))
        .unwrap()
        .finish()
        .unwrap();
    let certified = certify_rectangular_tiling_cell_geometry_checked(&handle, cell).unwrap();
    let report = evaluate_tiling_boundary_ownership_checked(&handle, certified.cell()).unwrap();

    assert_eq!(
        report.evaluation().family(),
        CandidateScreeningInvariantFamily::BoundaryOwnership
    );
    assert_eq!(report.counters().tile_count(), 2);
    assert_eq!(report.counters().boundary_ownership_rows_checked(), 2);
    assert_eq!(report.counters().query_declarations_performed(), 1);
    assert!(report.query_declaration_digest().is_some());
    assert!(!report.admits_theorem_authority());
}

#[test]
fn boundary_ownership_rejects_unowned_boundary_rows_without_authority() {
    let handle = handle();
    let cell = TilingCell::builder("boundary-open")
        .with_rectangular_tile(small_region(
            "tile-a",
            "red",
            0,
            1,
            BoundaryOwnershipPolicy::open_unowned(),
        ))
        .unwrap()
        .finish()
        .unwrap();
    let report = evaluate_tiling_boundary_ownership_checked(&handle, &cell).unwrap();

    assert_eq!(
        report.evaluation().verdict(),
        CandidateScreeningVerdict::Rejected
    );
    assert!(!report.evaluation().admits_theorem_authority());
    assert!(!report.admits_theorem_authority());
}

#[test]
fn half_open_boundary_policy_rejects_vague_or_duplicate_conventions() {
    let left_bottom = BoundaryOwnershipPolicy::owned_half_open("left,bottom").unwrap();
    let bottom_left = BoundaryOwnershipPolicy::owned_half_open("bottom,left").unwrap();
    assert_eq!(left_bottom, bottom_left);
    assert_eq!(left_bottom.stable_token(), "owned_half_open:left,bottom");

    assert!(matches!(
        BoundaryOwnershipPolicy::owned_half_open("diagonal"),
        Err(HadwigerArtifactShapeError::EmptyField {
            field: "boundary_half_open_side"
        })
    ));
    assert!(matches!(
        BoundaryOwnershipPolicy::owned_half_open("left,left"),
        Err(HadwigerArtifactShapeError::EmptyField {
            field: "boundary_half_open_side"
        })
    ));
}
