use forge_query::facade::{
    ForgeQueryOrchestrationSurfaceInventory, ForgeQueryPublicDocCoverageInventory,
};

fn main() {
    let coverage = ForgeQueryPublicDocCoverageInventory::current();
    let surfaces = ForgeQueryOrchestrationSurfaceInventory::current();

    let coverage_row = coverage
        .row_for_public_name("orchestrate_local_neighborhood_for_active_face_selection")
        .expect("grouped coverage row should exist");
    let surface_row = surfaces
        .row_for_public_name("orchestrate_local_neighborhood_for_active_face_selection_checked")
        .expect("grouped checked row should exist");

    let _ = coverage_row.readme_discovery_label();
    let _ = coverage_row.journey().unwrap().as_str();
    let _ = surface_row.proof_contract().proof_type_name();
}
