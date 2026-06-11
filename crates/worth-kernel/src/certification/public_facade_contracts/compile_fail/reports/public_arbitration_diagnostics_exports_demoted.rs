use worth_kernel::facade::diagnostics::arbitration::{
    prepare_primitive_intent_conflict_dx_surface_report, PrimitiveIntentConflict,
};

fn main() {
    let _ = prepare_primitive_intent_conflict_dx_surface_report;
    let _ = std::mem::size_of::<PrimitiveIntentConflict>();
}
