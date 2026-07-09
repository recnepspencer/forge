use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::{
    WorthQueryAspectMutationBuilder, WorthQueryAspectTouch, WorthQueryAuthoredAspectValue,
};

fn main() {
    let title = WorthQueryAspectTouch::aspect_field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
    let _ = WorthQueryAspectMutationBuilder::new()
        .set_aspect(title, WorthQueryAuthoredAspectValue::string("Draft"))
        .build_insert_symbolic("draft-task", "Task");
}
