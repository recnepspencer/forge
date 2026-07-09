use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::{WorthQueryAspectMutationOperation, WorthQueryAspectTouch};

fn main() {
    let operation =
        WorthQueryAspectMutationOperation::set(WorthQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())));
    let _ = operation.aspect_path();
}
