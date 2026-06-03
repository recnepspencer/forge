use worth_kernel::facade::diagnostics::query::{
    prepare_primitive_construction_branch_preview_runtime_report,
    prepare_primitive_construction_query_existing_truth_binding_report,
    PrimitiveConstructionQueryBoundaryGapRegister,
};

fn main() {
    let _ = prepare_primitive_construction_branch_preview_runtime_report;
    let _ = prepare_primitive_construction_query_existing_truth_binding_report;
    let _ = std::mem::size_of::<PrimitiveConstructionQueryBoundaryGapRegister>();
}
