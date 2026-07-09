use crate::WorthServerOperationSchedulerCounters;

use super::WorthServerCompletedProductOperation;

#[derive(Clone, Debug)]
pub struct WorthServerExecutedProductReadBatch {
    operations: Vec<WorthServerCompletedProductOperation>,
    counters: WorthServerOperationSchedulerCounters,
    canonical_digest: String,
}

impl WorthServerExecutedProductReadBatch {
    pub(crate) fn new(
        operations: Vec<WorthServerCompletedProductOperation>,
        counters: WorthServerOperationSchedulerCounters,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-product-read-batch-v1|counters={:?}|operations={}",
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

    pub fn operations(&self) -> &[WorthServerCompletedProductOperation] {
        &self.operations
    }

    pub fn counters(&self) -> &WorthServerOperationSchedulerCounters {
        &self.counters
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
