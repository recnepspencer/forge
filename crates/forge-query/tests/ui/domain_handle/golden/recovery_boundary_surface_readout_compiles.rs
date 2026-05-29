use forge_query::facade::{
    ForgeQueryOrchestrationSurfaceInventory, ForgeQueryPublicDocCoverageInventory,
};

fn main() {
    let coverage = ForgeQueryPublicDocCoverageInventory::current();
    let surfaces = ForgeQueryOrchestrationSurfaceInventory::current();

    let coverage_row = coverage
        .row_for_public_name("recover_from_outcome")
        .expect("recovery coverage row should exist");
    let surface_row = surfaces
        .row_for_public_name("recover_from_contribution_composed_proof")
        .expect("recovery proof row should exist");

    let _ = coverage_row.doc_reference().section();
    let _ = coverage_row.journey().unwrap().as_str();
    let _ = surface_row.proof_contract().support_surface().as_str();
}
