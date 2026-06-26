use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQuerySymbolicAspectReference, ForgeQuerySymbolicTargetReference,
};

fn main() {
    let reference = ForgeQuerySymbolicAspectReference::same_batch_entity_identity(
        ForgeQueryAspectTouch::aspect_field_path(AspectKey::new("owner").unwrap(), CanonicalFieldPath::single(FieldKey::new("id").unwrap())),
        ForgeQuerySymbolicTargetReference::new("draft-task").unwrap(),
    );
    let _ = reference.aspect_path();
}
