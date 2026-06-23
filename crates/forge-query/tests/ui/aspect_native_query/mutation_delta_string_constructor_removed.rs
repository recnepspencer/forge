use forge_query::facade::{
    ForgeQueryEntityIdentity, ForgeQueryMutationDelta, ForgeQueryMutationKind,
    QueryExternalIdentityToken,
};
use std::sync::Arc;

fn main() {
    let _ = ForgeQueryMutationDelta::new(
        "Task",
        ForgeQueryEntityIdentity::admit_authored_entity_token(QueryExternalIdentityToken::new(
            Arc::from("task-1"),
        )),
        ForgeQueryMutationKind::Updated,
        vec!["title.value".to_string()],
    );
}
