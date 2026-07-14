use worth_query::facade::runtime::{WorthQueryGraphTouchDescriptorRow, WorthQueryMutationFamily};

fn main() {
    let _ = WorthQueryGraphTouchDescriptorRow {
        component_index: 0,
        mutation_family: WorthQueryMutationFamily::Delete,
        program_step_kind: None,
        lifecycle_family: None,
        declared_collection: None,
        declared_symbol: None,
        declared_aspect_operations: Vec::new(),
        touched_aspects: Vec::new(),
        has_symbolic_target_reference: false,
        has_existing_truth_binding: false,
        symbolic_aspect_reference_count: 0,
    };
}
