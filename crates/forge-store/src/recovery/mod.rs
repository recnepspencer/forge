mod execution;
mod planning;

use crate::wal::{DurableMutationId, RecoveryDecisionClass};
use forge_relational::facade::history::CommitId;
use serde::Serialize;

pub(crate) use execution::{evaluate_recovery_for_mutation, RecoveryAction};
pub(crate) use planning::build_recovery_plan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DurableRecoveryPlan {
    pub pending_durable_mutation_ids: Vec<DurableMutationId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DurableRecoveryDecision {
    pub durable_mutation_id: DurableMutationId,
    pub decision: RecoveryDecisionClass,
    pub commit_id: Option<CommitId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DurableRecoveryOutcome {
    pub decisions: Vec<DurableRecoveryDecision>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum DurableRetryResolution {
    PreviouslyAcknowledgedEquivalentCommit {
        commit_id: CommitId,
    },
    NotPreviouslyPublished,
    RetryRequiresOperatorOrHigherLevelPolicy {
        durable_mutation_id: DurableMutationId,
    },
}
