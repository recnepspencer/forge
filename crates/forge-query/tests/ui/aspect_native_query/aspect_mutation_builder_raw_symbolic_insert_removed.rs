use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryAspectTouch, ForgeQueryAuthoredAspectValue,
};

fn main() {
    let title = ForgeQueryAspectTouch::aspect_field_path(
        AspectKey::new("title").unwrap(),
        CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
    );
    let _ = ForgeQueryAspectMutationBuilder::new()
        .set_aspect(title, ForgeQueryAuthoredAspectValue::string("Draft"))
        .build_insert_symbolic("draft-task", "Task");
}
