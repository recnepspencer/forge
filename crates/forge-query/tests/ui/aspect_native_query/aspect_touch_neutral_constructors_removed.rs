use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::ForgeQueryAspectTouch;

fn main() {
    let _ = ForgeQueryAspectTouch::aspect(AspectKey::new("title").unwrap());
    let _ = ForgeQueryAspectTouch::field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
}
