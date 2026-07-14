use worth_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::{WorthQueryAspectMutationBuilder, WorthQueryAspectTouch};

fn main() {
    let _ = WorthQueryAspectMutationBuilder::new().aspect(
        WorthQueryAspectTouch::aspect_field_path(
            AspectKey::new("title").unwrap(),
            CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
        ),
        worth_query::facade::runtime::WorthQueryAuthoredAspectValue::string("blocked"),
    );
}
