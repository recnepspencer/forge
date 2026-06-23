use forge_foundational::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use forge_query::facade::ForgeQueryAspectTouch;
use forge_query::facade::ForgeQueryExistingTruthProbeField;

fn main() {
    let field = ForgeQueryExistingTruthProbeField::from_admitted_aspect_touch(
        ForgeQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())),
        AspectValue::String("Title".to_string().into()),
    );
    let _ = field.terminal_aspect_path_projection();
}
