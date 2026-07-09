use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::WorthQueryAspectTouch;

fn main() {
    let touch = WorthQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap()));
    let _ = touch.terminal_projection_for_boundary();
}
