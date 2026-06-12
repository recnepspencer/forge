use hadwiger_research::facade::{
    admit_hadwiger_research_handle, evaluate_tiling_boundary_ownership_checked,
    BoundaryOwnershipPolicy, ExactRational, HadwigerResearchOperatingContext,
    RectangularTileRegion, TilingCell, TilingColorId,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");
    let cell = TilingCell::builder("cell-a")
        .with_rectangular_tile(
            RectangularTileRegion::new(
                "tile-a",
                TilingColorId::new("red").expect("color admits"),
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
    let report = evaluate_tiling_boundary_ownership_checked(&handle, &cell).expect("checks");

    assert!(!report.admits_theorem_authority());
}
