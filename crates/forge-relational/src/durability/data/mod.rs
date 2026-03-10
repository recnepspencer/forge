use serde::{Deserialize, Serialize};

use crate::history::data::{BranchHead, CommitReference};
use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexGeneration};
use crate::lineage::data::{CorrespondenceCandidate, LineageEventRecord, LineageNode};
use crate::replay::data::CanonicalCommitEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurabilityMode {
    InMemoryCanonical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCommitEnvelope {
    pub envelope: CanonicalCommitEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCheckpoint {
    pub up_to_commit: Option<CommitReference>,
    pub branches: Vec<BranchHead>,
    pub envelopes: Vec<CanonicalCommitEnvelope>,
    pub lineage_nodes: Vec<LineageNode>,
    pub lineage_events: Vec<LineageEventRecord>,
    pub correspondence_candidates: Vec<CorrespondenceCandidate>,
    pub index_definitions: Vec<DerivedIndexDefinition>,
    pub index_generations: Vec<DerivedIndexGeneration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub config: crate::logic::runtime::RelationalRuntimeConfig,
    pub checkpoint: Option<DurableCheckpoint>,
    pub tail_log: Vec<DurableCommitEnvelope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryFailureClass {
    SchemaMismatch,
    CorruptCheckpoint,
    MissingParentChain,
    ReplayFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurabilityError {
    pub class: RecoveryFailureClass,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryOutcome {
    pub recovered_commits: usize,
    pub latest_commit: Option<crate::history::data::CommitReference>,
    pub restored_branches: usize,
}
