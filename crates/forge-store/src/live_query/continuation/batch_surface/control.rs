use forge_relational::facade::history::CommitId;
use serde::Serialize;

use crate::live_query::basis::StableBasisReadScope;

use super::ContinuationBatchId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlLaneBatchReceipt {
    batch_id: ContinuationBatchId,
    covered_commit_range: (CommitId, CommitId),
    covered_commit_ids: Vec<CommitId>,
    from_frontier_commit_id: CommitId,
    to_frontier_commit_id: CommitId,
    resolved_scope: StableBasisReadScope,
    batch_family_version: u32,
    covered_commit_count: u64,
    control_replay_breadth: u64,
    support_rows_read: u64,
    scope_lookup_count: u64,
    fallback_class: String,
}

impl ControlLaneBatchReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        batch_id: ContinuationBatchId,
        covered_commit_range: (CommitId, CommitId),
        covered_commit_ids: Vec<CommitId>,
        from_frontier_commit_id: CommitId,
        to_frontier_commit_id: CommitId,
        resolved_scope: StableBasisReadScope,
        batch_family_version: u32,
        covered_commit_count: u64,
        control_replay_breadth: u64,
        support_rows_read: u64,
        scope_lookup_count: u64,
        fallback_class: impl Into<String>,
    ) -> Self {
        Self {
            batch_id,
            covered_commit_range,
            covered_commit_ids,
            from_frontier_commit_id,
            to_frontier_commit_id,
            resolved_scope,
            batch_family_version,
            covered_commit_count,
            control_replay_breadth,
            support_rows_read,
            scope_lookup_count,
            fallback_class: fallback_class.into(),
        }
    }

    pub fn batch_id(&self) -> &ContinuationBatchId { &self.batch_id }
    pub fn covered_commit_range(&self) -> (CommitId, CommitId) { self.covered_commit_range }
    pub fn covered_commit_ids(&self) -> &[CommitId] { &self.covered_commit_ids }
    pub fn from_frontier_commit_id(&self) -> CommitId { self.from_frontier_commit_id }
    pub fn to_frontier_commit_id(&self) -> CommitId { self.to_frontier_commit_id }
    pub fn resolved_scope(&self) -> &StableBasisReadScope { &self.resolved_scope }
    pub fn batch_family_version(&self) -> u32 { self.batch_family_version }
    pub fn covered_commit_count(&self) -> u64 { self.covered_commit_count }
    pub fn control_replay_breadth(&self) -> u64 { self.control_replay_breadth }
    pub fn support_rows_read(&self) -> u64 { self.support_rows_read }
    pub fn scope_lookup_count(&self) -> u64 { self.scope_lookup_count }
    pub fn fallback_class(&self) -> &str { &self.fallback_class }
}
