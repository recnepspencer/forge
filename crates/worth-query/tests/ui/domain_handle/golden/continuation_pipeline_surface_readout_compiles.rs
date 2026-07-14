use worth_query::facade::foundation::WorthQueryOrchestrationSurfaceInventory;
use worth_query::facade::certification::WorthQueryPublicDocCoverageInventory;

fn main() {
    let coverage = WorthQueryPublicDocCoverageInventory::current();
    let surfaces = WorthQueryOrchestrationSurfaceInventory::current();

    let coverage_row = coverage
        .row_for_public_name("prepare_continuation_from_target")
        .expect("continuation coverage row should exist");
    let surface_row = surfaces
        .row_for_public_name("execute_prepared_continuation_outcome")
        .expect("continuation outcome row should exist");

    let _ = coverage_row.readme_discovery_label();
    let _ = coverage_row.golden_transcript().unwrap().dx_focus();
    let _ = surface_row.binding_projection().as_str();
    let _ = surface_row.proof_contract().transcript_family().as_str();
}
