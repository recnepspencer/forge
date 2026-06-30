use crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket;

use super::super::{
    BooleanChainReplayUndoBoundaryHandoff, CompletedBooleanLoopReconstructionHandoff,
    CompletedBooleanSplitHandoff, PlanarBooleanLoopReconstructionCloseoutInput,
    WorkloadCompositionError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedBooleanSplitReplayUndoBoundary {
    completed_split_handoff: CompletedBooleanSplitHandoff,
    transaction_boundary_packet: ReplayUndoTransactionBoundaryPacket,
}

impl AdmittedBooleanSplitReplayUndoBoundary {
    pub(crate) fn new(
        completed_split_handoff: CompletedBooleanSplitHandoff,
        transaction_boundary_packet: ReplayUndoTransactionBoundaryPacket,
    ) -> Self {
        Self {
            completed_split_handoff,
            transaction_boundary_packet,
        }
    }

    pub fn completed_split_handoff(&self) -> &CompletedBooleanSplitHandoff {
        &self.completed_split_handoff
    }

    pub fn transaction_boundary_packet(&self) -> &ReplayUndoTransactionBoundaryPacket {
        &self.transaction_boundary_packet
    }

    pub fn packet_identity(&self) -> &str {
        self.transaction_boundary_packet.packet_identity()
    }

    pub fn complete_boolean_loop_reconstruction(
        &self,
        input: PlanarBooleanLoopReconstructionCloseoutInput<'_>,
    ) -> Result<CompletedBooleanLoopReconstructionHandoff, WorkloadCompositionError> {
        self.completed_split_handoff
            .complete_boolean_loop_reconstruction_from_admitted_replay_undo_boundary(self, input)
    }

    pub fn complete_boolean_chain_integration(
        &self,
        input: PlanarBooleanLoopReconstructionCloseoutInput<'_>,
    ) -> Result<BooleanChainReplayUndoBoundaryHandoff, WorkloadCompositionError> {
        super::super::BooleanChainReplayUndoBoundaryHandoff::from_admitted_replay_undo_boundary(
            self, input,
        )
    }
}
