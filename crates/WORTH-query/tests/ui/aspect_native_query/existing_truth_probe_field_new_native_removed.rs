use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::{WorthQueryAspectTouch, WorthQueryExistingTruthProbeField};

fn main() {
    let touch = WorthQueryAspectTouch::aspect_field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
    let _ = WorthQueryExistingTruthProbeField::new_native(
        touch,
        worth_query::facade::WorthQueryAuthoredAspectValue::string("Buy milk"),
    );
}
