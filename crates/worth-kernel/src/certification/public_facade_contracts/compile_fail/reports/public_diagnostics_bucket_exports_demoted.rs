use worth_kernel::facade::diagnostics::{
    prepare_primitive_construction_move_witness_resolution_report,
    prepare_primitive_construction_preview_surface_report, PrimitiveConstructionQueryBoundaryGapRegister,
    PrimitiveIntentConflict,
};

fn main() {
    let _ = PrimitiveIntentConflict::analyze;
    let _ = prepare_primitive_construction_preview_surface_report;
    let _ = prepare_primitive_construction_move_witness_resolution_report;
    let _ = std::mem::size_of::<PrimitiveConstructionQueryBoundaryGapRegister>();
}
