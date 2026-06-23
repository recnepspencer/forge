#![allow(dead_code)]

use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::ForgeQueryAspectTouch;

pub mod graph_index_inventory;
pub mod graph_read_access;
pub mod public_bridge_runtime;
pub mod test_entity_identities;

pub fn aspect_touch(authored_touch_text: &str) -> ForgeQueryAspectTouch {
    let mut segments = authored_touch_text.split('.');
    let aspect = segments
        .next()
        .and_then(AspectKey::new)
        .expect("fixture authored touch aspect should admit");
    let fields = segments
        .map(|segment| FieldKey::new(segment).expect("fixture authored touch field should admit"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        ForgeQueryAspectTouch::whole_aspect(aspect)
    } else {
        ForgeQueryAspectTouch::aspect_field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("fixture authored touch should have fields"),
        )
    }
}
