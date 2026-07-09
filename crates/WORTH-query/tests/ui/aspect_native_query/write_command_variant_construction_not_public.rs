use worth_query::facade::{
    WorthQueryEntityIdentity, WorthQueryWriteCommand, QueryExternalIdentityToken,
};
use std::sync::Arc;

fn main() {
    let _ = WorthQueryWriteCommand::Delete {
        entity_identity: WorthQueryEntityIdentity::admit_authored_entity_token(
            QueryExternalIdentityToken::new(Arc::from("task-1")),
        ),
    };
}
