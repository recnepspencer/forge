use crate::{
    authority::digest_from_string,
    authority::{FetchedAuthoritativeCommit, PersistedAuthoritativeCommit},
    snapshot::{SnapshotId, SnapshotImageBundle},
    wal::WalRecord,
};
use forge_relational::facade::history::{BranchId, CommitId};
use forge_relational::facade::replay::CanonicalCommitEnvelope;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmbeddedCheckpointClassification {
    DerivedDurable,
    Ephemeral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmbeddedCheckpointRecord {
    pub checkpoint_id: String,
    pub source_runtime_id: String,
    pub basis_branch_id: Option<BranchId>,
    pub basis_commit_id: Option<CommitId>,
    pub classification: EmbeddedCheckpointClassification,
    pub contained_commit_ids: Vec<CommitId>,
    pub metadata: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotBasisRecord {
    pub snapshot_id: SnapshotId,
    pub snapshot_family_version: u32,
    pub snapshot_basis_version: u32,
    pub snapshot_image_format_version: u32,
    pub snapshot_branch_id: BranchId,
    pub snapshot_frontier_commit_id: CommitId,
    pub snapshot_history_range: Vec<CommitId>,
    pub snapshot_canonicalization_version: u32,
    pub snapshot_authority_digest: String,
    pub snapshot_image_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotImageRecord {
    pub snapshot_id: SnapshotId,
    pub image: SnapshotImageBundle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StoreState {
    pub canonicalization_version: u32,
    pub next_commit_sequence: u64,
    pub next_head_update_sequence: u64,
    pub branch_records: BTreeMap<String, BranchRecord>,
    pub branch_head_records: BTreeMap<String, BranchHeadRecord>,
    pub commit_envelopes: BTreeMap<u64, StoredCommitEnvelope>,
    pub commit_parent_records: BTreeMap<String, CommitParentRecord>,
    pub authoritative_artifact_digests: BTreeMap<String, AuthoritativeArtifactDigestRecord>,
    #[serde(default)]
    pub embedded_checkpoint_records: BTreeMap<String, EmbeddedCheckpointRecord>,
    #[serde(default)]
    pub next_snapshot_id: u64,
    #[serde(default)]
    pub snapshot_basis_records: BTreeMap<u64, SnapshotBasisRecord>,
    #[serde(default)]
    pub snapshot_image_records: BTreeMap<u64, SnapshotImageRecord>,
    #[serde(default)]
    pub next_durable_mutation_id: u64,
    #[serde(default)]
    pub next_wal_sequence: u64,
    #[serde(default)]
    pub wal_records: BTreeMap<u64, WalRecord>,
}
