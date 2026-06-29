use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

use super::super::{
    LookupConsumedWorkloadComposition, LookupConsumedWorkloadDenial, WorkloadCompositionError,
    WorthWorkload,
};
use crate::workload_composition::{AdmittedSpatialConflictInput, BatchAdmissionExecutionReceipt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LookupConsumedBatchExecutionCluster {
    workload: WorthWorkload,
    lookup_consumed: LookupConsumedWorkloadComposition,
    batch_execution: BatchAdmissionExecutionReceipt,
}

impl LookupConsumedBatchExecutionCluster {
    pub fn workload(&self) -> &WorthWorkload {
        &self.workload
    }

    pub fn lookup_consumed(&self) -> &LookupConsumedWorkloadComposition {
        &self.lookup_consumed
    }

    pub fn batch_execution(&self) -> &BatchAdmissionExecutionReceipt {
        &self.batch_execution
    }

    pub fn admit_spatial_conflict_input<'a>(
        &'a self,
        authority: &'a SpatialGeometryEvidenceTouchAuthority,
        execution_receipt: &'a EvidenceLookupExecutionReceipt,
    ) -> Result<AdmittedSpatialConflictInput<'a>, WorkloadCompositionError> {
        self.lookup_consumed
            .admit_spatial_conflict_input(authority, execution_receipt)
    }
}

impl LookupConsumedWorkloadComposition {
    pub(crate) fn admit_batch_execution_cluster(
        &self,
        batch_execution: &BatchAdmissionExecutionReceipt,
    ) -> Result<LookupConsumedBatchExecutionCluster, WorkloadCompositionError> {
        let Some(bound_batch_execution) = self.workload().batch_admission_execution() else {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                LookupConsumedWorkloadDenial::MissingWorkloadAttachedBatchAdmissionExecutionReceipt,
            ));
        };
        if bound_batch_execution.execution_receipt_digest()
            != batch_execution.execution_receipt_digest()
        {
            return Err(WorkloadCompositionError::LookupConsumedWorkload(
                LookupConsumedWorkloadDenial::SuppliedBatchAdmissionExecutionReceiptMismatch,
            ));
        }

        Ok(LookupConsumedBatchExecutionCluster {
            workload: self.workload().clone(),
            lookup_consumed: self.clone(),
            batch_execution: batch_execution.clone(),
        })
    }
}

impl WorthWorkload {
    pub fn admit_lookup_consumed_batch_execution_cluster(
        &self,
        handoff: &worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff,
        batch_execution: &BatchAdmissionExecutionReceipt,
    ) -> Result<LookupConsumedBatchExecutionCluster, WorkloadCompositionError> {
        self.admit_lookup_consumed_workload(handoff)?
            .admit_batch_execution_cluster(batch_execution)
    }
}
