use forge_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{ForgeQueryAspectMutationBuilder, ForgeQueryAspectTouch};

fn main() {
    let command = ForgeQueryAspectMutationBuilder::new()
        .set_aspect(ForgeQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())),
            forge_query::facade::ForgeQueryAuthoredAspectValue::string("Title"),
        )
        .build_insert("Task")
        .unwrap();
    let aspect = &command.admitted_aspect_values()[0];
    let _ = aspect.terminal_aspect_path_projection();
}
