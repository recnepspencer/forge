use worth_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_foundational::facade::AspectValue;
use worth_query::facade::runtime::{WorthQueryAuthoredAspectMutation, WorthQueryAspectTouch};

fn main() {
    let _ = WorthQueryAuthoredAspectMutation::new_set(
        WorthQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())),
        AspectValue::Null,
    );
}
