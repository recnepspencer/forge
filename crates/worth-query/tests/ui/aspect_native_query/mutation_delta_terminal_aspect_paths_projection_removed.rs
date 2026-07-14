use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::foundation::{WorthQueryEntityIdentity, WorthQueryMutationDelta, WorthQueryMutationKind, QueryExternalIdentityToken};
use worth_query::facade::runtime::WorthQueryAspectTouch;
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

    let _ = delta.terminal_aspect_paths_projection();
}
