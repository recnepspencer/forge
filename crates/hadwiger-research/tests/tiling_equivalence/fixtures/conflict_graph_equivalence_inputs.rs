use hadwiger_research::facade::*;

pub fn conflict_graph(handle: &HadwigerResearchHandle, extraction_id: &str) -> TilingConflictGraph {
    conflict_graph_with_required_color_count(handle, extraction_id, None)
}

pub fn conflict_graph_with_required_color_count(
    handle: &HadwigerResearchHandle,
    extraction_id: &str,
    required_color_count: Option<u32>,
) -> TilingConflictGraph {
    let cell = TilingCell::builder(format!("{extraction_id}-cell"))
        .with_rectangular_tile(rectangular_tile("tile-a", 0, 1))
        .unwrap()
        .with_rectangular_tile(rectangular_tile("tile-b", 1, 2))
        .unwrap()
        .finish()
        .unwrap();
    let contact =
        evaluate_tiling_same_color_contact_checked(handle, &cell, "tile-a", "tile-b").unwrap();
    let mut request =
        TilingConflictGraphExtractionRequest::from_tiling_contact_report(extraction_id, contact);
    if let Some(color_count) = required_color_count {
        request = request.with_required_color_count(color_count).unwrap();
    }
    extract_conflict_graph_checked(handle, request).unwrap()
}

fn rectangular_tile(tile_id: &str, x_min: i128, x_max: i128) -> RectangularTileRegion {
    RectangularTileRegion::new(
        tile_id,
        TilingColorId::new("red").unwrap(),
        ExactRational::integer(x_min),
        ExactRational::integer(x_max),
        ExactRational::integer(0),
        ExactRational::integer(1),
    )
    .unwrap()
    .with_boundary_ownership(BoundaryOwnershipPolicy::owned_half_open("left,bottom").unwrap())
}
