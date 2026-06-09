use hadwiger_research::facade::{
    BoundaryOwnershipPolicy, ExactRational, RectangularTileRegion, TilingCell, TilingColorId,
};

fn main() {
    let red = TilingColorId::new("red").expect("color admits");
    let cell = TilingCell::builder("cell-a")
        .with_rectangular_tile(
            RectangularTileRegion::new(
                "tile-a",
                red,
                ExactRational::integer(0),
                ExactRational::integer(1),
                ExactRational::integer(0),
                ExactRational::integer(1),
            )
            .expect("region admits")
            .with_boundary_ownership(BoundaryOwnershipPolicy::owned_closed()),
        )
        .expect("tile admits")
        .finish()
        .expect("cell admits");

    assert_eq!(cell.tile_count(), 1);
}
