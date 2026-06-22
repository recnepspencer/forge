use forge_query::facade::runtime::{
    ForgeQueryGraphTouchDescriptorRow, ForgeQueryMutationFamily,
};

fn main() {
    let _ = ForgeQueryGraphTouchDescriptorRow {
        component_index: 0,
        mutation_family: ForgeQueryMutationFamily::Delete,
        program_step_kind: None,
        lifecycle_family: None,
        declared_collection: None,
        declared_symbol: None,
        declared_aspect_operations: Vec::new(),
        touched_aspect_paths: Vec::new(),
        has_symbolic_target_reference: false,
        has_existing_truth_binding: false,
        symbolic_aspect_reference_count: 0,
    };
}
