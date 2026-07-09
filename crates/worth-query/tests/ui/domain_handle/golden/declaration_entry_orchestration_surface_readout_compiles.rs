use worth_query::facade::{
    WorthQueryOrchestrationSurfaceInventory, WorthQueryPublicDocCoverageInventory,
};

fn main() {
    let coverage = WorthQueryPublicDocCoverageInventory::current();
    let surfaces = WorthQueryOrchestrationSurfaceInventory::current();

    let coverage_row = coverage
        .row_for_public_name("orchestrate_declaration_entry")
        .expect("declaration-entry coverage row should exist");
    let surface_row = surfaces
        .row_for_public_name("orchestrate_declaration_entry")
        .expect("declaration-entry surface row should exist");

    let _ = coverage_row.doc_reference().path();
    let _ = coverage_row.golden_transcript().unwrap().label();
    let _ = surface_row.proof_contract().checked_type_name();
    let _ = surface_row.proof_contract().proof_type_name();
}
