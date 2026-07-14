use worth_query::facade::runtime::{WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorKind};

fn main() {
    let _ = WorthQueryGraphTouchDescriptor {
        kind: WorthQueryGraphTouchDescriptorKind::AuthoritativeMutationBatch,
        rows: Vec::new(),
        component_count: 0,
        symbolic_entity_declaration_count: 0,
        symbolic_relation_declaration_count: 0,
        declared_collection_count: 0,
        declared_aspect_touch_count: 0,
        declared_aspect_operation_count: 0,
        touched_aspect_count: 0,
    };
}
