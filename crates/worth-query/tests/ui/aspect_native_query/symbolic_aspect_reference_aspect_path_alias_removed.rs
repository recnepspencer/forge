use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::{
    WorthQueryAspectTouch, WorthQuerySymbolicAspectReference, WorthQuerySymbolicTargetReference,
};

fn main() {
    let reference = WorthQuerySymbolicAspectReference::same_batch_entity_identity(
        WorthQueryAspectTouch::aspect_field_path(AspectKey::new("owner").unwrap(), CanonicalFieldPath::single(FieldKey::new("id").unwrap())),
        WorthQuerySymbolicTargetReference::new("draft-task").unwrap(),
    );
    let _ = reference.aspect_path();
}
