use worth_query::facade::{
    WorthQueryOrchestrationSurfaceInventory, WorthQueryPublicDocCoverageInventory,
};

fn main() {
    let coverage = WorthQueryPublicDocCoverageInventory::current();
    let surfaces = WorthQueryOrchestrationSurfaceInventory::current();

    let coverage_row = coverage
        .row_for_public_name("orchestrate_declaration_with_contributions")
        .expect("contribution coverage row should exist");
    let surface_row = surfaces
        .row_for_public_name("orchestrate_declaration_with_contributions_checked")
        .expect("contribution checked row should exist");

    let _ = coverage_row.golden_transcript().unwrap().path();
    let _ = coverage_row.readme_discovery_label();
    let _ = surface_row.proof_contract().checked_topology_kind().as_str();
}
