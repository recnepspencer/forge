use std::sync::Arc;

use forge_query::facade::{
    admit_external_commit_token, ForgeQueryCommitIdentity, QueryExternalIdentityToken,
};

pub(crate) fn derived_surface_commit_identity(label: &str) -> ForgeQueryCommitIdentity {
    admit_external_commit_token(QueryExternalIdentityToken::new(Arc::from(label)))
}
