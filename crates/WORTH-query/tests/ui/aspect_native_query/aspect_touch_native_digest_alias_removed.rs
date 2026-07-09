use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::WorthQueryAspectTouch;

fn main() {}

fn removed_aspect_touch_native_digest_alias() {
    let touch = WorthQueryAspectTouch::aspect_field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
    let _ = touch.native_digest_part();
}
