use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryEntityIdentity, ForgeQueryMutationDelta,
    ForgeQueryMutationKind, QueryExternalIdentityToken,
};
use std::sync::Arc;

fn main() {}

fn removed_mutation_delta_touched_aspects_alias() {
    let delta = ForgeQueryMutationDelta::from_touched_aspects(
        "Task",
        ForgeQueryEntityIdentity::admit_authored_entity_token(QueryExternalIdentityToken::new(
            Arc::from("task-1"),
        )),
        ForgeQueryMutationKind::Updated,
        vec![ForgeQueryAspectTouch::field_path(
            AspectKey::new("title").unwrap(),
            CanonicalFieldPath::single(FieldKey::new("value").unwrap()),
        )],
    );
    let _ = delta.touched_aspects();
}
