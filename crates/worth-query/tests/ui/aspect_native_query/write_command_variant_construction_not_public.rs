use worth_query::facade::foundation::{WorthQueryEntityIdentity, QueryExternalIdentityToken};
use worth_query::facade::runtime::WorthQueryWriteCommand;
use std::sync::Arc;

fn main() {
    let _ = WorthQueryWriteCommand::Delete {
        entity_identity: WorthQueryEntityIdentity::admit_authored_entity_token(
            QueryExternalIdentityToken::new(Arc::from("task-1")),
        ),
    };
}
