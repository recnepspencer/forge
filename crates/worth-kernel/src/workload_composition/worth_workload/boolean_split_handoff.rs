use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedgerReceipt;
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

use super::{
    CompletedBooleanSplitBatchExecutionCluster, LookupConsumedBatchExecutionCluster,
    LookupConsumedWorkloadDenial, WorkloadCompositionError, WorthWorkload,
};
use crate::workload_composition::BatchAdmissionExecutionReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletedBooleanSplitHandoff {
    completed_workload: WorthWorkload,
    split_ledger_receipt: PlanarBooleanSplitEdgeChainLedgerReceipt,
    lookup_consumed_workload_handoff: EvidenceLookupConsumedWorkloadHandoff,
}

impl CompletedBooleanSplitHandoff {
    pub(crate) fn new(
        completed_workload: WorthWorkload,
        split_ledger_receipt: PlanarBooleanSplitEdgeChainLedgerReceipt,
        lookup_consumed_workload_handoff: EvidenceLookupConsumedWorkloadHandoff,
    ) -> Self {
        Self {
            completed_workload,
            split_ledger_receipt,
            lookup_consumed_workload_handoff,
        }
    }

    pub fn completed_workload(&self) -> &WorthWorkload {
        &self.completed_workload
    }

    pub fn split_ledger_receipt(&self) -> &PlanarBooleanSplitEdgeChainLedgerReceipt {
        &self.split_ledger_receipt
    }

    pub fn lookup_consumed_workload_handoff(&self) -> &EvidenceLookupConsumedWorkloadHandoff {
        &self.lookup_consumed_workload_handoff
    }

    pub fn workload_stage_index_identity(&self) -> &str {
        self.completed_workload
            .evidence_ledger()
            .stage_index()
            .index_identity()
    }

    pub fn require_boolean_split(&self) -> Result<(), WorkloadCompositionError> {
        self.completed_workload
            .require_boolean_split(&self.split_ledger_receipt)
    }

    pub(crate) fn with_batch_admission_execution(
        &self,
        batch_execution: &BatchAdmissionExecutionReceipt,
    ) -> Result<Self, WorkloadCompositionError> {
        Ok(Self::new(
            self.completed_workload
                .with_batch_admission_execution(batch_execution.clone())?,
            self.split_ledger_receipt.clone(),
            self.lookup_consumed_workload_handoff.clone(),
        ))
    }

    pub(crate) fn admit_lookup_consumed_batch_execution_cluster(
        &self,
    ) -> Result<LookupConsumedBatchExecutionCluster, WorkloadCompositionError> {
        let batch_execution = self
            .completed_workload
            .batch_admission_execution()
            .ok_or_else(|| {
                WorkloadCompositionError::LookupConsumedWorkload(
                    LookupConsumedWorkloadDenial::MissingWorkloadAttachedBatchAdmissionExecutionReceipt,
                )
            })?;
        self.completed_workload
            .admit_lookup_consumed_batch_execution_cluster(
                &self.lookup_consumed_workload_handoff,
                batch_execution,
            )
    }

    pub fn admit_batch_execution_cluster(
        &self,
    ) -> Result<CompletedBooleanSplitBatchExecutionCluster, WorkloadCompositionError> {
        Ok(CompletedBooleanSplitBatchExecutionCluster::new(
            self.clone(),
            self.admit_lookup_consumed_batch_execution_cluster()?,
        ))
    }

    pub fn admit_split_spatial_touch_authority(
        &self,
    ) -> Result<SpatialGeometryEvidenceTouchAuthority, WorkloadCompositionError> {
        self.completed_workload
            .admit_spatial_geometry_evidence_touch(&self.split_ledger_receipt)
    }
}
