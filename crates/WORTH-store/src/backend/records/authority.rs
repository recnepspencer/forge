use crate::{
    authority::digest_from_string,
    authority::{FetchedAuthoritativeCommit, PersistedAuthoritativeCommit},
    StableBasisId, StableBasisReadRequest,
};
use worth_relational::facade::history::{BranchId, CommitId};
use worth_relational::facade::lineage::{
    LineageArtifactCounters, LineageDecisionLogDigestBasis, LineageDigestBasis,
    LineageEventBatchDigestBasis, LineageEventRecord,
};
use worth_relational::facade::replay::CanonicalCommitEnvelope;
use worth_relational::facade::schema::{
    DescriptorSemanticsVersion, SchemaContinuationDescriptor, SchemaReconciliationDescriptor,
    SchemaTransitionArtifact, SchemaVersionId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchRecord {
    pub branch_id: BranchId,
    pub created_from_branch: Option<BranchId>,
    pub created_from_commit_id: Option<CommitId>,
    pub created_at_commit_sequence: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchHeadRecord {
    pub branch_id: BranchId,
    pub head_commit_id: Option<CommitId>,
    pub head_commit_digest: Option<String>,
    pub head_update_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredCommitEnvelope {
    pub envelope: CanonicalCommitEnvelope,
    pub envelope_digest: String,
    pub canonicalization_version: u32,
    pub commit_sequence: u64,
}

impl StoredCommitEnvelope {
    pub fn into_persisted(self) -> PersistedAuthoritativeCommit {
        PersistedAuthoritativeCommit::new(
            self.envelope,
            digest_from_string(self.envelope_digest),
            self.canonicalization_version,
            self.commit_sequence,
        )
    }

    pub fn into_fetched(self) -> FetchedAuthoritativeCommit {
        FetchedAuthoritativeCommit::new(
            self.envelope,
            digest_from_string(self.envelope_digest),
            self.canonicalization_version,
            self.commit_sequence,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitParentRecord {
    pub commit_id: CommitId,
    pub parent_position: usize,
    pub parent_commit_id: CommitId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthoritativeArtifactDigestRecord {
    pub artifact_family: AuthoritativeArtifactFamily,
    pub artifact_id: String,
    pub canonicalization_version: u32,
    pub digest_algorithm: String,
    pub artifact_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthoritativeArtifactFamily {
    BranchRecord,
    BranchHeadRecord,
    CommitEnvelope,
    CommitParentRecord,
    CommitSupportSummary,
    SchemaSupportRecord,
    LineageSupportRecord,
    DurableCursorIdentityRecord,
    SubscriberCheckpointRecord,
    StableBasisRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitSupportSummaryRecord {
    pub commit_id: CommitId,
    pub branch_id: BranchId,
    pub schema_support_artifact_id: Option<String>,
    pub lineage_support_artifact_id: Option<String>,
    #[serde(default)]
    pub milestone_6_published_layout_request_artifact_ids: Vec<String>,
    pub emitted_schema_artifact: bool,
    pub emitted_lineage_artifact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaSupportRecord {
    pub artifact_id: String,
    pub commit_id: CommitId,
    pub branch_id: BranchId,
    pub schema_version_id: SchemaVersionId,
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
    pub schema_transition: Option<SchemaTransitionArtifact>,
    pub schema_continuation_descriptor: Option<SchemaContinuationDescriptor>,
    pub schema_reconciliation_descriptor: Option<SchemaReconciliationDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LineageSupportRecord {
    pub artifact_id: String,
    pub commit_id: CommitId,
    pub branch_id: BranchId,
    pub lineage_event_ids: Vec<u64>,
    pub lineage_events: Vec<LineageEventRecord>,
    pub lineage_digest_basis: LineageDigestBasis,
    pub event_batch_digest_basis: LineageEventBatchDigestBasis,
    pub decision_log_digest_basis: LineageDecisionLogDigestBasis,
    pub lineage_artifact_counters: LineageArtifactCounters,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DurableCursorIdentityRecord {
    pub artifact_id: String,
    pub cursor_id: String,
    pub subscriber_id: String,
    pub branch_id: BranchId,
    pub feed_shape_id: String,
    pub schema_interpretation_id: String,
    pub cursor_semantics_version: u32,
    pub latest_checkpoint_sequence: u64,
    pub latest_basis_commit_id: CommitId,
    pub latest_schema_support_artifact_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubscriberCheckpointRecord {
    pub artifact_id: String,
    pub cursor_id: String,
    pub subscriber_id: String,
    pub branch_id: BranchId,
    pub feed_shape_id: String,
    pub schema_interpretation_id: String,
    pub cursor_semantics_version: u32,
    pub checkpoint_sequence: u64,
    pub basis_commit_id: CommitId,
    pub schema_support_artifact_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StableBasisRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub request: StableBasisReadRequest,
    pub minimum_retained_commit_id: CommitId,
    pub required_support_artifact_set: Vec<String>,
    pub schema_boundary_dependency: String,
    pub authority_replay_fallback_class: String,
    pub snapshot_tail_fallback_class: String,
    pub descriptor_version: u32,
}

impl StableBasisRecord {
    pub fn requested_stable_basis_id(&self) -> StableBasisId {
        StableBasisId::from_request(&self.request)
    }
}
