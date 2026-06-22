use forge_foundational::facade::AspectValue;
use forge_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspectMutationBuilder, ForgeQueryAspectTouch};

fn main() {
    let command = ForgeQueryAspectMutationBuilder::new()
        .aspect(
            ForgeQueryAspectTouch::field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())),
            AspectValue::String("hello".into()),
        )
        .build_insert("Task")
        .unwrap();
    let aspect = &command.admitted_aspect_values()[0];
    let _ = aspect.aspect_path();
}
