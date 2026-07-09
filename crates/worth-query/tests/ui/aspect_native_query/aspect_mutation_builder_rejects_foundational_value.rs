use worth_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use worth_query::facade::{WorthQueryAspectMutationBuilder, WorthQueryAspectTouch};

fn main() {
    let _ = WorthQueryAspectMutationBuilder::new().set_aspect(
        WorthQueryAspectTouch::aspect_field_path(
            AspectKey::new("title").unwrap(),
            CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
        ),
        AspectValue::Null,
    );
}
