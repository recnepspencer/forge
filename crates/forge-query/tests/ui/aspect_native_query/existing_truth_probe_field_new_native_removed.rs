use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspectTouch, ForgeQueryExistingTruthProbeField};

fn main() {
    let touch = ForgeQueryAspectTouch::aspect_field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
    let _ = ForgeQueryExistingTruthProbeField::new_native(
        touch,
        forge_query::facade::ForgeQueryAuthoredAspectValue::string("Buy milk"),
    );
}
