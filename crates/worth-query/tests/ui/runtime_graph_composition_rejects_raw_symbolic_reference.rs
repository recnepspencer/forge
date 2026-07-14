use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::{WorthQueryAspectTouch, WorthQueryGraphRelationMutationBuilder, WorthQuerySymbolicTargetReference};

fn main() {
    let reference = WorthQuerySymbolicTargetReference::new("draft-task")
        .expect("symbolic reference should build");
    let source_identity =
        WorthQueryAspectTouch::aspect_field_path(AspectKey::new("edge").unwrap(), CanonicalFieldPath::single(FieldKey::new("source_identity").unwrap()));
    let _ = WorthQueryGraphRelationMutationBuilder::new()
        .symbolic_entity_identity(source_identity, &reference);
}
