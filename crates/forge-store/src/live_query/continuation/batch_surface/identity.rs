use serde::Serialize;

use crate::live_query::basis::{StableBasisHandle, StableBasisReadScope};
use forge_relational::facade::history::CommitId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ContinuationBatchId(String);

impl ContinuationBatchId {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_parts(
        stable_basis: &StableBasisHandle,
        cursor_id: &str,
        subscriber_id: &str,
        covered_commit_range: (CommitId, CommitId),
        resolved_scope: &StableBasisReadScope,
        batch_family_version: u32,
    ) -> Self {
        Self(format!(
            "continuation-batch|{}|{}|{}|{}|{}|{}|{}",
            stable_basis.stable_basis_id().as_str(),
            cursor_id,
            subscriber_id,
            covered_commit_range.0 .0,
            covered_commit_range.1 .0,
            resolved_scope.fingerprint(),
            batch_family_version,
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
