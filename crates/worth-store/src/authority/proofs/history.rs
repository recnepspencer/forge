use crate::backend::records::{LineageSupportRecord, SchemaSupportRecord};
use worth_relational::facade::{
    history::{BranchId, CommitId},
    identity::LineageId,
    lineage::{
        LineageDecisionLogDigestBasis, LineageDigestBasis, LineageEventBatchDigestBasis,
        LineageEventRecord,
    },
};

#[derive(Debug, Clone)]
pub struct FetchedSchemaSupportArtifact {
    record: SchemaSupportRecord,
}
impl FetchedSchemaSupportArtifact {
    pub(crate) fn new(record: SchemaSupportRecord) -> Self {
        Self { record }
    }
    pub fn record(&self) -> &SchemaSupportRecord {
        &self.record
    }
}

pub type FetchedSchemaBoundaryArtifact = FetchedSchemaSupportArtifact;

#[derive(Debug, Clone)]
pub struct FetchedLineageSupportArtifact {
    record: LineageSupportRecord,
}
impl FetchedLineageSupportArtifact {
    pub(crate) fn new(record: LineageSupportRecord) -> Self {
        Self { record }
    }
    pub fn record(&self) -> &LineageSupportRecord {
        &self.record
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalIdentityRequest {
    commit_id: CommitId,
    branch_id: BranchId,
    lineage_id: LineageId,
}
impl HistoricalIdentityRequest {
    pub fn new(commit_id: CommitId, branch_id: BranchId, lineage_id: LineageId) -> Self {
        Self {
            commit_id,
            branch_id,
            lineage_id,
        }
    }
    pub fn commit_id(&self) -> CommitId {
        self.commit_id
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn lineage_id(&self) -> LineageId {
        self.lineage_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalIdentityResolution {
    commit_id: CommitId,
    branch_id: BranchId,
    lineage_id: LineageId,
    support_artifact_id: String,
    resolved_lineage_ids: Vec<LineageId>,
    matching_events: Vec<LineageEventRecord>,
    lineage_digest_basis: LineageDigestBasis,
    event_batch_digest_basis: LineageEventBatchDigestBasis,
    decision_log_digest_basis: LineageDecisionLogDigestBasis,
}
impl HistoricalIdentityResolution {
    pub(crate) fn new(
        commit_id: CommitId,
        branch_id: BranchId,
        lineage_id: LineageId,
        support_artifact_id: String,
        resolved_lineage_ids: Vec<LineageId>,
        matching_events: Vec<LineageEventRecord>,
        lineage_digest_basis: LineageDigestBasis,
        event_batch_digest_basis: LineageEventBatchDigestBasis,
        decision_log_digest_basis: LineageDecisionLogDigestBasis,
    ) -> Self {
        Self {
            commit_id,
            branch_id,
            lineage_id,
            support_artifact_id,
            resolved_lineage_ids,
            matching_events,
            lineage_digest_basis,
            event_batch_digest_basis,
            decision_log_digest_basis,
        }
    }
    pub fn commit_id(&self) -> CommitId {
        self.commit_id
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn lineage_id(&self) -> LineageId {
        self.lineage_id
    }
    pub fn support_artifact_id(&self) -> &str {
        &self.support_artifact_id
    }
    pub fn resolved_lineage_ids(&self) -> &[LineageId] {
        &self.resolved_lineage_ids
    }
    pub fn matching_events(&self) -> &[LineageEventRecord] {
        &self.matching_events
    }
    pub fn matching_event_ids(&self) -> Vec<u64> {
        self.matching_events
            .iter()
            .map(|event| event.event_id())
            .collect()
    }
    pub fn lineage_digest_basis(&self) -> &LineageDigestBasis {
        &self.lineage_digest_basis
    }
    pub fn event_batch_digest_basis(&self) -> &LineageEventBatchDigestBasis {
        &self.event_batch_digest_basis
    }
    pub fn decision_log_digest_basis(&self) -> &LineageDecisionLogDigestBasis {
        &self.decision_log_digest_basis
    }
}
