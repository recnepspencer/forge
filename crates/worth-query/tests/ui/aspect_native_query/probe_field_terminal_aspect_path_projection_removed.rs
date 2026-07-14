use worth_foundational::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::WorthQueryAspectTouch;
use worth_query::facade::runtime::WorthQueryExistingTruthProbeField;

fn main() {
    let field = WorthQueryExistingTruthProbeField::from_admitted_aspect_touch(
        WorthQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())),
        AspectValue::String("Title".to_string().into()),
    );
    let _ = field.terminal_aspect_path_projection();
}
