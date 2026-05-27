use forge_query::facade::{
    ForgeQueryOrchestrationSurfaceInventory, ForgeQueryPublicDocCoverageInventory,
};

fn main() {
    let coverage = ForgeQueryPublicDocCoverageInventory::current();
    let surfaces = ForgeQueryOrchestrationSurfaceInventory::current();

    let coverage_row = coverage
        .row_for_public_name("prepare_preview_for_active_face_selection")
        .expect("family helper coverage row should exist");
    let surface_row = surfaces
        .row_for_public_name("orchestrate_material_attachment_for_active_face_selection_proof")
        .expect("family helper proof row should exist");

    let _ = coverage_row.doc_reference().path();
    let _ = coverage_row.golden_transcript().unwrap().label();
    let _ = surface_row.family().as_str();
}
