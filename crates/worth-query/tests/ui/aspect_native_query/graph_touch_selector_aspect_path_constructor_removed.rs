use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::{WorthQueryAspectTouch, WorthQueryGraphTouchSelector};

fn main() {
    let touch = WorthQueryAspectTouch::aspect_field_path(AspectKey::new("identity").unwrap(), CanonicalFieldPath::single(FieldKey::new("id").unwrap()));
    let _ = WorthQueryGraphTouchSelector::aspect_path(touch);
}
