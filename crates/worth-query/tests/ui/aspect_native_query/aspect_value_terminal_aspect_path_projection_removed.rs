use worth_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::{WorthQueryAspectMutationBuilder, WorthQueryAspectTouch};

fn main() {
    let command = WorthQueryAspectMutationBuilder::new()
        .set_aspect(WorthQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap())),
            worth_query::facade::runtime::WorthQueryAuthoredAspectValue::string("Title"),
        )
        .build_insert("Task")
        .unwrap();
    let aspect = &command.admitted_aspect_values()[0];
    let _ = aspect.terminal_aspect_path_projection();
}
