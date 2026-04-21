use forge_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

use crate::live_query::acknowledgment::ContinuationAdvanceReceipt;
use crate::live_query::basis::{StableBasisId, StableBasisReadScope};
use crate::ForegroundIsolationOutcome;

use super::ContinuationBatchId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdmittedNarrowBatchReceipt {
    batch_id: ContinuationBatchId,
    stable_basis_id: StableBasisId,
    cursor_id: String,
    subscriber_id: String,
    branch_id: BranchId,
    feed_shape_id: String,
    schema_interpretation_id: String,
    cursor_semantics_version: u32,
    schema_boundary_artifact_id: String,
    covered_commit_range: (CommitId, CommitId),
    covered_commit_ids: Vec<CommitId>,
    from_frontier_commit_id: CommitId,
    to_frontier_commit_id: CommitId,
    resolved_scope: StableBasisReadScope,
    batch_family_version: u32,
    covered_commit_count: u64,
    narrowed_item_count: u64,
    support_rows_read: u64,
    scope_lookup_count: u64,
    foreground_isolation: ForegroundIsolationOutcome,
}

impl AdmittedNarrowBatchReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        batch_id: ContinuationBatchId,
        stable_basis_id: StableBasisId,
        cursor_id: impl Into<String>,
        subscriber_id: impl Into<String>,
        branch_id: BranchId,
        feed_shape_id: impl Into<String>,
        schema_interpretation_id: impl Into<String>,
        cursor_semantics_version: u32,
        schema_boundary_artifact_id: impl Into<String>,
        covered_commit_range: (CommitId, CommitId),
        covered_commit_ids: Vec<CommitId>,
        from_frontier_commit_id: CommitId,
        to_frontier_commit_id: CommitId,
        resolved_scope: StableBasisReadScope,
        batch_family_version: u32,
        covered_commit_count: u64,
        narrowed_item_count: u64,
        support_rows_read: u64,
        scope_lookup_count: u64,
    ) -> Self {
        Self {
            batch_id,
            stable_basis_id,
            cursor_id: cursor_id.into(),
            subscriber_id: subscriber_id.into(),
            branch_id,
            feed_shape_id: feed_shape_id.into(),
            schema_interpretation_id: schema_interpretation_id.into(),
            cursor_semantics_version,
            schema_boundary_artifact_id: schema_boundary_artifact_id.into(),
            covered_commit_range,
            covered_commit_ids,
            from_frontier_commit_id,
            to_frontier_commit_id,
            resolved_scope,
            batch_family_version,
            covered_commit_count,
            narrowed_item_count,
            support_rows_read,
            scope_lookup_count,
            foreground_isolation: ForegroundIsolationOutcome::stayed_isolated(
                crate::ForegroundReservationClass::Continuation,
            ),
        }
    }

    pub fn batch_id(&self) -> &ContinuationBatchId {
        &self.batch_id
    }
    pub fn stable_basis_id(&self) -> &StableBasisId {
        &self.stable_basis_id
    }
    pub fn cursor_id(&self) -> &str {
        &self.cursor_id
    }
    pub fn subscriber_id(&self) -> &str {
        &self.subscriber_id
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn feed_shape_id(&self) -> &str {
        &self.feed_shape_id
    }
    pub fn schema_interpretation_id(&self) -> &str {
        &self.schema_interpretation_id
    }
    pub fn cursor_semantics_version(&self) -> u32 {
        self.cursor_semantics_version
    }
    pub fn schema_boundary_artifact_id(&self) -> &str {
        &self.schema_boundary_artifact_id
    }
    pub fn covered_commit_range(&self) -> (CommitId, CommitId) {
        self.covered_commit_range
    }
    pub fn covered_commit_ids(&self) -> &[CommitId] {
        &self.covered_commit_ids
    }
    pub fn from_frontier_commit_id(&self) -> CommitId {
        self.from_frontier_commit_id
    }
    pub fn to_frontier_commit_id(&self) -> CommitId {
        self.to_frontier_commit_id
    }
    pub fn resolved_scope(&self) -> &StableBasisReadScope {
        &self.resolved_scope
    }
    pub fn batch_family_version(&self) -> u32 {
        self.batch_family_version
    }
    pub fn covered_commit_count(&self) -> u64 {
        self.covered_commit_count
    }
    pub fn narrowed_item_count(&self) -> u64 {
        self.narrowed_item_count
    }
    pub fn support_rows_read(&self) -> u64 {
        self.support_rows_read
    }
    pub fn scope_lookup_count(&self) -> u64 {
        self.scope_lookup_count
    }
    pub fn foreground_isolation(&self) -> &ForegroundIsolationOutcome {
        &self.foreground_isolation
    }

    pub fn into_advance_receipt(self) -> ContinuationAdvanceReceipt {
        ContinuationAdvanceReceipt::new(self)
    }

    pub(crate) fn with_foreground_isolation(
        mut self,
        foreground_isolation: ForegroundIsolationOutcome,
    ) -> Self {
        self.foreground_isolation = foreground_isolation;
        self
    }
}
