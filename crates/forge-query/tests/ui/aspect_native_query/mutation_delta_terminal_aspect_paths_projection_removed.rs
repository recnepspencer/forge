use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryEntityIdentity, ForgeQueryMutationDelta,
    ForgeQueryMutationKind, QueryExternalIdentityToken,
};
use std::sync::Arc;

fn main() {
    let delta = ForgeQueryMutationDelta::from_touched_aspects(
        "Task",
        ForgeQueryEntityIdentity::admit_authored_entity_token(QueryExternalIdentityToken::new(
            Arc::from("task-1"),
        )),
        ForgeQueryMutationKind::Updated,
        vec![ForgeQueryAspectTouch::aspect_field_path(AspectKey::new("title").unwrap(), CanonicalFieldPath::single(FieldKey::new("value").unwrap()))],
    );

    let _ = delta.terminal_aspect_paths_projection();
}
