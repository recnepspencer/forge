use forge_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspectMutationBuilder, ForgeQueryAspectTouch};

fn main() {
    let _ = ForgeQueryAspectMutationBuilder::new().set_aspect(
        ForgeQueryAspectTouch::aspect_field_path(
            AspectKey::new("title").unwrap(),
            CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
        ),
        AspectValue::Null,
    );
}
