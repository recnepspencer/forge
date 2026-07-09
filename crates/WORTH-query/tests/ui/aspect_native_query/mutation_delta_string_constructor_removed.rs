use worth_query::facade::{
    WorthQueryEntityIdentity, WorthQueryMutationDelta, WorthQueryMutationKind,
    QueryExternalIdentityToken,
};
use std::sync::Arc;

fn main() {
    let _ = WorthQueryMutationDelta::new(
        "Task",
        WorthQueryEntityIdentity::admit_authored_entity_token(QueryExternalIdentityToken::new(
            Arc::from("task-1"),
        )),
        WorthQueryMutationKind::Updated,
        vec!["title.value".to_string()],
    );
}
