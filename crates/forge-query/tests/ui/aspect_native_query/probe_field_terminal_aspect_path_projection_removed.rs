use forge_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_foundational::facade::AspectValue;
use forge_query::facade::ForgeQueryAspectTouch;
use forge_query::facade::ForgeQueryExistingTruthProbeField;

fn main() {
    let field = ForgeQueryExistingTruthProbeField::new_native(
        ForgeQueryAspectTouch::field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())),
        AspectValue::String("Title".into()),
    );
    let _ = field.terminal_aspect_path_projection();
}
