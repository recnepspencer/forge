use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanDownstreamSplitConsumptionInput,
    PlanarBooleanEdgeSplitReplayParityReceipt, PlanarBooleanSplitChainValidationReceipt,
    PlanarBooleanSplitDecisionLogReceipt, PlanarBooleanSplitEdgeChainLedgerReceipt,
    PlanarBooleanSplitPersistentNamingReceipt,
};
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

use super::{
    replay_undo_boundary::admit_boolean_split_replay_undo_boundary,
    BooleanSplitReplayUndoBoundaryRequest, PlanarBooleanLoopReconstructionCloseoutInput,
    WorkloadCompositionError, WorthWorkload,
};

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

    pub fn admit_downstream_split_consumption(
        &self,
        decision_log_receipt: &PlanarBooleanSplitDecisionLogReceipt,
        validation_receipt: &PlanarBooleanSplitChainValidationReceipt,
        persistent_naming_receipt: &PlanarBooleanSplitPersistentNamingReceipt,
        replay_parity_receipt: &PlanarBooleanEdgeSplitReplayParityReceipt,
    ) -> Result<PlanarBooleanDownstreamSplitConsumption, WorkloadCompositionError> {
        self.completed_workload
            .admit_lookup_consumed_workload(&self.lookup_consumed_workload_handoff)?;
        let spatial_touch_authority = self
            .completed_workload
            .admit_spatial_geometry_evidence_touch(&self.split_ledger_receipt)?;
        PlanarBooleanDownstreamSplitConsumption::admit(
            PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
                &self.split_ledger_receipt,
                decision_log_receipt,
                validation_receipt,
                persistent_naming_receipt,
                replay_parity_receipt,
                &spatial_touch_authority,
            ),
        )
        .map_err(WorkloadCompositionError::DownstreamSplitConsumption)
    }

    pub fn admit_split_spatial_touch_authority(
        &self,
    ) -> Result<SpatialGeometryEvidenceTouchAuthority, WorkloadCompositionError> {
        self.completed_workload
            .admit_spatial_geometry_evidence_touch(&self.split_ledger_receipt)
    }

    pub fn admit_boolean_split_replay_undo_boundary(
        &self,
        request: BooleanSplitReplayUndoBoundaryRequest<'_>,
    ) -> Result<super::AdmittedBooleanSplitReplayUndoBoundary, WorkloadCompositionError> {
        admit_boolean_split_replay_undo_boundary(self, request)
    }

    pub fn complete_boolean_loop_reconstruction_from_replay_undo_boundary(
        &self,
        boundary_request: BooleanSplitReplayUndoBoundaryRequest<'_>,
        input: PlanarBooleanLoopReconstructionCloseoutInput<'_>,
    ) -> Result<super::CompletedBooleanLoopReconstructionHandoff, WorkloadCompositionError> {
        self.admit_boolean_split_replay_undo_boundary(boundary_request)?
            .complete_boolean_loop_reconstruction(input)
    }
}
