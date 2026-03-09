use serde::{Deserialize, Serialize};

use crate::data::history::{BranchHead, CommitReference};
use crate::data::replay::CanonicalCommitEnvelope;

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
    pub lineage_event_ids: Vec<u64>,
    pub index_generation_ids: Vec<u64>,
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
