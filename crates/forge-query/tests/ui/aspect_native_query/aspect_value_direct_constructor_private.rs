use forge_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_foundational::facade::AspectValue;
use forge_query::facade::{ForgeQueryAspectTouch, ForgeQueryAspectValue};

fn main() {
    let _ = ForgeQueryAspectValue::new_set(
        ForgeQueryAspectTouch::field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())),
        AspectValue::String("blocked".into()),
    );
}
