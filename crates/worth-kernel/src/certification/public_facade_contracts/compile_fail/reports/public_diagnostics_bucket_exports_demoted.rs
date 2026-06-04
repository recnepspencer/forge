use worth_kernel::facade::diagnostics::{
    prepare_primitive_construction_branch_preview_runtime_report,
};

fn main() {
    let _ = prepare_primitive_construction_branch_preview_runtime_report::<
        worth_kernel::facade::authoring::construction::PrimitiveConstructionIntent,
    >;
}
