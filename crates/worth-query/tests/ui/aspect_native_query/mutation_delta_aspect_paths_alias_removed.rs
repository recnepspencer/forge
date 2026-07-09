use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::{
    WorthQueryAspectTouch, WorthQueryEntityIdentity, WorthQueryMutationDelta,
    WorthQueryMutationKind, QueryExternalIdentityToken,
};
use std::sync::Arc;

fn main() {
    let delta = WorthQueryMutationDelta::from_touched_aspects(
        "Task",
        WorthQueryEntityIdentity::admit_authored_entity_token(QueryExternalIdentityToken::new(
            Arc::from("task-1"),
        )),
        WorthQueryMutationKind::Updated,
        vec![WorthQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap()))],
    );

    let _ = delta.aspect_paths();
}
