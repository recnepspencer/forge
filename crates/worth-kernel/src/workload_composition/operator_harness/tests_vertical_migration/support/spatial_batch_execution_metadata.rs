use crate::workload_composition::ConflictIndependenceDisposition;

use super::spatial_batch_execution_slice::DerivedSpatialBatchExecutionSlice;

impl DerivedSpatialBatchExecutionSlice {
    pub fn independence_disposition(&self) -> ConflictIndependenceDisposition {
        self.independence_disposition
    }

    pub fn authority_participant_identities(&self) -> &[String] {
        &self.authority_participant_identities
    }
}
