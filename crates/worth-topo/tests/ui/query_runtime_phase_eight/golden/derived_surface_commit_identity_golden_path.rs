use std::sync::Arc;

use forge_query::facade::{admit_external_commit_token, QueryExternalIdentityToken};

fn main() {
    let _commit = admit_external_commit_token(QueryExternalIdentityToken::new(Arc::from(
        "derived-surface:test",
    )));
}
