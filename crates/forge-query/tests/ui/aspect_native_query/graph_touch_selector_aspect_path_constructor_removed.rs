use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspectTouch, ForgeQueryGraphTouchSelector};

fn main() {
    let touch = ForgeQueryAspectTouch::aspect_field_path(AspectKey::new("identity").unwrap(), CanonicalFieldPath::single(FieldKey::new("id").unwrap()));
    let _ = ForgeQueryGraphTouchSelector::aspect_path(touch);
}
