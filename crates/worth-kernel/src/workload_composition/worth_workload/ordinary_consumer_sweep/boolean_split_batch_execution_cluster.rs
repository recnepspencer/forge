use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanDownstreamSplitConsumptionInput,
    PlanarBooleanEdgeSplitReplayParityReceipt, PlanarBooleanSplitChainValidationReceipt,
    PlanarBooleanSplitDecisionLogReceipt, PlanarBooleanSplitPersistentNamingReceipt,
};
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

use super::{super::super::WorkloadCompositionError, LookupConsumedBatchExecutionCluster};
use crate::workload_composition::{
    AdmittedBooleanSplitReplayUndoBoundary, BooleanSplitReplayUndoBoundaryRequest,
    CompletedBooleanSplitHandoff,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletedBooleanSplitBatchExecutionCluster {
    split_handoff: CompletedBooleanSplitHandoff,
    lookup_consumed_cluster: LookupConsumedBatchExecutionCluster,
}

impl CompletedBooleanSplitBatchExecutionCluster {
    pub(crate) fn new(
        split_handoff: CompletedBooleanSplitHandoff,
        lookup_consumed_cluster: LookupConsumedBatchExecutionCluster,
    ) -> Self {
        Self {
            split_handoff,
            lookup_consumed_cluster,
        }
    }

    pub(crate) fn split_handoff(&self) -> &CompletedBooleanSplitHandoff {
        &self.split_handoff
    }

    pub fn lookup_consumed_cluster(&self) -> &LookupConsumedBatchExecutionCluster {
        &self.lookup_consumed_cluster
    }

    pub(crate) fn lookup_consumed_workload_handoff(
        &self,
    ) -> &EvidenceLookupConsumedWorkloadHandoff {
        self.split_handoff.lookup_consumed_workload_handoff()
    }

    pub(crate) fn workload_stage_index_identity(&self) -> &str {
        self.split_handoff.workload_stage_index_identity()
    }

    pub(crate) fn admit_split_spatial_touch_authority(
        &self,
    ) -> Result<SpatialGeometryEvidenceTouchAuthority, WorkloadCompositionError> {
        self.split_handoff.admit_split_spatial_touch_authority()
    }

    pub fn admit_downstream_split_consumption(
        &self,
        decision_log_receipt: &PlanarBooleanSplitDecisionLogReceipt,
        validation_receipt: &PlanarBooleanSplitChainValidationReceipt,
        persistent_naming_receipt: &PlanarBooleanSplitPersistentNamingReceipt,
        replay_parity_receipt: &PlanarBooleanEdgeSplitReplayParityReceipt,
    ) -> Result<PlanarBooleanDownstreamSplitConsumption, WorkloadCompositionError> {
        let spatial_touch_authority = self.admit_split_spatial_touch_authority()?;
        PlanarBooleanDownstreamSplitConsumption::admit(
            PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
                self.split_handoff.split_ledger_receipt(),
                decision_log_receipt,
                validation_receipt,
                persistent_naming_receipt,
                replay_parity_receipt,
                &spatial_touch_authority,
            ),
        )
        .map_err(WorkloadCompositionError::DownstreamSplitConsumption)
    }

    pub fn admit_boolean_split_replay_undo_boundary(
        &self,
        request: BooleanSplitReplayUndoBoundaryRequest<'_>,
    ) -> Result<AdmittedBooleanSplitReplayUndoBoundary, WorkloadCompositionError> {
        super::super::replay_undo_boundary::admit_boolean_split_replay_undo_boundary(self, request)
    }
}
