use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::ForgeQueryAspectTouch;

fn main() {}

fn removed_aspect_touch_native_digest_alias() {
    let touch = ForgeQueryAspectTouch::field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
    let _ = touch.native_digest_part();
}
