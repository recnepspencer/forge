use forge_query::facade::{
    ForgeQueryEntityIdentity, ForgeQueryWriteCommand, QueryExternalIdentityToken,
};
use std::sync::Arc;

fn main() {
    let _ = ForgeQueryWriteCommand::Delete {
        entity_identity: ForgeQueryEntityIdentity::admit_authored_entity_token(
            QueryExternalIdentityToken::new(Arc::from("task-1")),
        ),
    };
}
