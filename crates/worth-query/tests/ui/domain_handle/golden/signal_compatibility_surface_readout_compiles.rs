use worth_query::facade::foundation::WorthQueryOrchestrationSurfaceInventory;
use worth_query::facade::certification::WorthQueryPublicDocCoverageInventory;

fn main() {
    let coverage = WorthQueryPublicDocCoverageInventory::current();
    let surfaces = WorthQueryOrchestrationSurfaceInventory::current();

    let coverage_row = coverage
        .row_for_public_name("orchestrate_signal_compatibility_outcome")
        .expect("signal coverage row should exist");
    let surface_row = surfaces
        .row_for_public_name("orchestrate_signal_compatibility_proof")
        .expect("signal proof row should exist");

    let _ = coverage_row.doc_reference().section();
    let _ = coverage_row.journey().unwrap().as_str();
    let _ = surface_row.proof_contract().support_surface().as_str();
}
