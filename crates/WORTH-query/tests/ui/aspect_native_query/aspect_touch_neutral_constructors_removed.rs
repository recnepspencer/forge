use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::WorthQueryAspectTouch;

fn main() {
    let _ = WorthQueryAspectTouch::aspect(AspectKey::new("title").unwrap());
    let _ = WorthQueryAspectTouch::field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
}
