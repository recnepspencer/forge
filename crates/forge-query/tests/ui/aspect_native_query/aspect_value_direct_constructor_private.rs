use forge_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_foundational::facade::AspectValue;
use forge_query::facade::{ForgeQueryAdmittedAspectValue, ForgeQueryAspectTouch};

fn main() {
    let _ = ForgeQueryAdmittedAspectValue::new_set(
        ForgeQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())),
        AspectValue::Null,
    );
}
