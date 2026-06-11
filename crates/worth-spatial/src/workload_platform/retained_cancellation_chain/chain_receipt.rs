use super::{
    chain_checkpoint::RetainedCancellationCheckpoint,
    chain_counters::RetainedCancellationChainCounters,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedCancellationChainReceipt {
    chain_digest: String,
    workload_identity: String,
    retained_basis_identity: String,
    projection_consumed_identity: String,
    checkpoints: Vec<RetainedCancellationCheckpoint>,
    counters: RetainedCancellationChainCounters,
}

impl RetainedCancellationChainReceipt {
    pub(crate) fn new(
        chain_digest: String,
        workload_identity: String,
        retained_basis_identity: String,
        projection_consumed_identity: String,
        checkpoints: Vec<RetainedCancellationCheckpoint>,
        counters: RetainedCancellationChainCounters,
    ) -> Self {
        Self {
            chain_digest,
            workload_identity,
            retained_basis_identity,
            projection_consumed_identity,
            checkpoints,
            counters,
        }
    }

    pub fn chain_digest(&self) -> &str {
        &self.chain_digest
    }

    pub fn workload_identity(&self) -> &str {
        &self.workload_identity
    }

    pub fn retained_basis_identity(&self) -> &str {
        &self.retained_basis_identity
    }

    pub fn projection_consumed_identity(&self) -> &str {
        &self.projection_consumed_identity
    }

    pub fn checkpoints(&self) -> &[RetainedCancellationCheckpoint] {
        &self.checkpoints
    }

    pub fn trigger_checkpoint(&self) -> Option<&RetainedCancellationCheckpoint> {
        self.checkpoints
            .iter()
            .find(|checkpoint| checkpoint.trigger().is_some())
    }

    pub fn counters(&self) -> RetainedCancellationChainCounters {
        self.counters
    }
}
