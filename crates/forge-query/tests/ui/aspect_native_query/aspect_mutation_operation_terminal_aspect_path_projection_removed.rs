use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch};

fn main() {
    let operation =
        ForgeQueryAspectMutationOperation::set(ForgeQueryAspectTouch::field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())));
    let _ = operation.terminal_aspect_path_projection();
}
