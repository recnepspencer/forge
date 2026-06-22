use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::ForgeQueryAspectTouch;

fn main() {
    let touch = ForgeQueryAspectTouch::field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap()));
    let _ = touch.terminal_aspect_path_projection();
}
