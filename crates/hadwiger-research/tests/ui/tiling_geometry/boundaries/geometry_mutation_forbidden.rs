use hadwiger_research::facade::{BoundaryOwnershipPolicy, ExactRational, RectangularTileRegion, TilingCell, TilingColorId};

fn main() {
    let mut cell = TilingCell::builder("cell-a")
        .with_rectangular_tile(
            RectangularTileRegion::new(
                "tile-a",
                TilingColorId::new("red").unwrap(),
                ExactRational::integer(0),
                ExactRational::integer(1),
                ExactRational::integer(0),
                ExactRational::integer(1),
            )
            .unwrap()
            .with_boundary_ownership(BoundaryOwnershipPolicy::owned_closed()),
        )
        .unwrap()
        .finish()
        .unwrap();
    cell.tiles_mut().clear();
}
