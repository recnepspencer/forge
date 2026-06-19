use crate::ForgeServerOperationSchedulerCounters;

use super::ForgeServerCompletedProductOperation;

#[derive(Clone, Debug)]
pub struct ForgeServerExecutedProductReadBatch {
    operations: Vec<ForgeServerCompletedProductOperation>,
    counters: ForgeServerOperationSchedulerCounters,
    canonical_digest: String,
}

impl ForgeServerExecutedProductReadBatch {
    pub(crate) fn new(
        operations: Vec<ForgeServerCompletedProductOperation>,
        counters: ForgeServerOperationSchedulerCounters,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-product-read-batch-v1|counters={:?}|operations={}",
            counters,
            operations
                .iter()
                .map(|operation| operation.envelope().canonical_digest())
                .collect::<Vec<_>>()
                .join("|")
        );
        Self {
            operations,
            counters,
            canonical_digest,
        }
    }

    pub fn operations(&self) -> &[ForgeServerCompletedProductOperation] {
        &self.operations
    }

    pub fn counters(&self) -> &ForgeServerOperationSchedulerCounters {
        &self.counters
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
