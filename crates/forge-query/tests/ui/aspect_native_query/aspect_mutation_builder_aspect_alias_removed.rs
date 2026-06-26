use forge_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspectMutationBuilder, ForgeQueryAspectTouch};

fn main() {
    let _ = ForgeQueryAspectMutationBuilder::new().aspect(
        ForgeQueryAspectTouch::aspect_field_path(
            AspectKey::new("title").unwrap(),
            CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
        ),
        forge_query::facade::ForgeQueryAuthoredAspectValue::string("blocked"),
    );
}
