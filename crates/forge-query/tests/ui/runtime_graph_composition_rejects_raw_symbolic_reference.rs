use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryGraphRelationMutationBuilder, ForgeQuerySymbolicTargetReference,
};

fn main() {
    let reference = ForgeQuerySymbolicTargetReference::new("draft-task")
        .expect("symbolic reference should build");
    let source_identity =
        ForgeQueryAspectTouch::aspect_field_path(AspectKey::new("edge").unwrap(), CanonicalFieldPath::single(FieldKey::new("source_identity").unwrap()));
    let _ = ForgeQueryGraphRelationMutationBuilder::new()
        .symbolic_entity_identity(source_identity, &reference);
}
