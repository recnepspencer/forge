use hadwiger_research::facade::*;

fn color(name: &str) -> TilingColorId {
    TilingColorId::new(name).unwrap()
}

fn rational(value: i128) -> ExactRational {
    ExactRational::integer(value)
}

fn tile(tile_id: &str, color_id: &str, x_min: i128, x_max: i128) -> RectangularTileRegion {
    half_open_tile(tile_id, color_id, x_min, x_max)
}

fn half_open_tile(
    tile_id: &str,
    color_id: &str,
    x_min: i128,
    x_max: i128,
) -> RectangularTileRegion {
    RectangularTileRegion::new(
        tile_id,
        color(color_id),
        rational(x_min),
        rational(x_max),
        rational(0),
        rational(1),
    )
    .unwrap()
    .with_boundary_ownership(BoundaryOwnershipPolicy::owned_half_open("left,bottom").unwrap())
}

fn closed_tile(tile_id: &str, color_id: &str, x_min: i128, x_max: i128) -> RectangularTileRegion {
    RectangularTileRegion::new(
        tile_id,
        color(color_id),
        rational(x_min),
        rational(x_max),
        rational(0),
        rational(1),
    )
    .unwrap()
    .with_boundary_ownership(BoundaryOwnershipPolicy::owned_closed())
}

fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle().unwrap()
}

#[test]
fn equivalent_rectangular_cells_converge_despite_tile_insertion_order() {
    let left = TilingCell::builder("cell-a")
        .with_rectangular_tile(tile("tile-b", "blue", 1, 2))
        .unwrap()
        .with_rectangular_tile(tile("tile-a", "red", 0, 1))
        .unwrap()
        .finish()
        .unwrap();
    let right = TilingCell::builder("cell-a")
        .with_rectangular_tile(tile("tile-a", "red", 0, 1))
        .unwrap()
        .with_rectangular_tile(tile("tile-b", "blue", 1, 2))
        .unwrap()
        .finish()
        .unwrap();

    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert_eq!(left.tile_count(), 2);
    assert_eq!(left.authority_owner().as_str(), "hadwiger_artifact_builder");
}

#[test]
fn changed_bounds_color_or_boundary_policy_changes_cell_digest() {
    let base = TilingCell::builder("cell-drift")
        .with_rectangular_tile(tile("tile-a", "red", 0, 1))
        .unwrap()
        .finish()
        .unwrap();
    let changed_color = TilingCell::builder("cell-drift")
        .with_rectangular_tile(tile("tile-a", "blue", 0, 1))
        .unwrap()
        .finish()
        .unwrap();
    let changed_bounds = TilingCell::builder("cell-drift")
        .with_rectangular_tile(tile("tile-a", "red", 0, 2))
        .unwrap()
        .finish()
        .unwrap();
    let changed_boundary = TilingCell::builder("cell-drift")
        .with_rectangular_tile(
            RectangularTileRegion::new(
                "tile-a",
                color("red"),
                rational(0),
                rational(1),
                rational(0),
                rational(1),
            )
            .unwrap()
            .with_boundary_ownership(BoundaryOwnershipPolicy::open_unowned()),
        )
        .unwrap()
        .finish()
        .unwrap();

    assert_ne!(base.artifact_digest(), changed_color.artifact_digest());
    assert_ne!(base.artifact_digest(), changed_bounds.artifact_digest());
    assert_ne!(base.artifact_digest(), changed_boundary.artifact_digest());
}

#[test]
fn rectangular_cell_builder_rejects_shape_scope_and_boundary_errors() {
    assert!(matches!(
        RectangularTileRegion::new(
            "flat",
            color("red"),
            rational(0),
            rational(0),
            rational(0),
            rational(1),
        ),
        Err(TilingGeometryError::ArtifactShape(
            HadwigerArtifactShapeError::EmptyField {
                field: "rectangular_tile_extent"
            }
        ))
    ));
    let missing_boundary = RectangularTileRegion::new(
        "tile-a",
        color("red"),
        rational(0),
        rational(1),
        rational(0),
        rational(1),
    )
    .unwrap();
    assert_eq!(
        TilingCell::builder("cell-shape")
            .with_rectangular_tile(missing_boundary)
            .unwrap()
            .finish(),
        Err(TilingGeometryError::MissingBoundaryOwnership {
            tile_id: "tile-a".to_string()
        })
    );
    assert!(matches!(
        TilingCell::builder("cell-dupe")
            .with_rectangular_tile(tile("tile-a", "red", 0, 1))
            .unwrap()
            .with_rectangular_tile(tile("tile-a", "blue", 1, 2)),
        Err(TilingGeometryError::DuplicateTile { tile_id }) if tile_id == "tile-a"
    ));
}

#[test]
fn rectangular_cell_builder_rejects_ambiguous_overlap_and_closed_boundary_sharing() {
    let overlap = TilingCell::builder("cell-overlap")
        .with_rectangular_tile(half_open_tile("tile-a", "red", 0, 2))
        .unwrap()
        .with_rectangular_tile(half_open_tile("tile-b", "blue", 1, 3))
        .unwrap()
        .finish();
    assert!(matches!(
        overlap,
        Err(TilingGeometryError::AmbiguousBoundaryOwnership { tile_id })
            if tile_id == "tile-a|tile-b"
    ));

    let shared_closed_boundary = TilingCell::builder("cell-closed-shared")
        .with_rectangular_tile(closed_tile("tile-a", "red", 0, 1))
        .unwrap()
        .with_rectangular_tile(closed_tile("tile-b", "blue", 1, 2))
        .unwrap()
        .finish();
    assert!(matches!(
        shared_closed_boundary,
        Err(TilingGeometryError::AmbiguousBoundaryOwnership { tile_id })
            if tile_id == "tile-a|tile-b"
    ));
}

#[test]
fn geometry_certification_lowers_through_query_cell_declaration() {
    let handle = handle();
    let cell = TilingCell::builder("cell-certified")
        .with_rectangular_tile(tile("tile-a", "red", 0, 1))
        .unwrap()
        .finish()
        .unwrap();
    let certification =
        certify_rectangular_tiling_cell_geometry_checked(&handle, cell.clone()).unwrap();

    assert_eq!(certification.cell().reference(), cell.reference());
    assert!(!certification.query_declaration_digest().is_empty());
    assert_eq!(certification.counters().query_declarations_performed(), 1);
    assert!(!certification.admits_theorem_authority());

    let renamed = TilingCell::builder("cell-certified-renamed")
        .with_rectangular_tile(tile("tile-a", "red", 0, 1))
        .unwrap()
        .finish()
        .unwrap();
    let renamed_certification =
        certify_rectangular_tiling_cell_geometry_checked(&handle, renamed).unwrap();
    assert_ne!(
        certification.artifact_digest(),
        renamed_certification.artifact_digest()
    );
}
