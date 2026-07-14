use hadwiger_research::facade::{
    evaluate_tiling_boundary_ownership_checked, hadwiger_research_domain_package,
    BoundaryOwnershipPolicy, ExactRational, HadwigerResearchDomainEntry, HadwigerResearchHandle,
    HadwigerResearchOperatingContext, HadwigerResearchQueryExt, RectangularTileRegion, TilingCell,
    TilingColorId,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

fn main() {
    let handle = installed_declarations().expect("handle admits");
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

fn installed_declarations() -> Result<HadwigerResearchHandle, String> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id").map_err(|error| error.to_string())?;
    let workspace = in_memory_test_runtime().with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-boundary-contact-dx").map_err(|error| error.to_string())?;
    let installed = workspace.domain(HadwigerResearchDomainEntry).map_err(|error| error.to_string())?;
    installed.research_declarations(&workspace, HadwigerResearchOperatingContext::default())
        .map_err(|error| error.to_string())
}
