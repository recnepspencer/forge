use crate::replay_undo_consumer_cutover::{
    current_replay_undo_forbidden_surface_denial_ledger,
    current_replay_undo_hard_deletion_source_firewall, ReplayUndoConsumerCutoverCloseout,
    ReplayUndoConsumerCutoverCloseoutInput, ReplayUndoHardDeletionCloseout,
    ReplayUndoHardDeletionCloseoutInput, ReplayUndoMilestoneTwelvePublicCloseout,
    ReplayUndoMilestoneTwelvePublicCloseoutInput,
};
use crate::replay_undo_inventory::{
    current_replay_undo_inventory_report, current_replay_undo_source_firewall_report,
};

use super::{
    AdmittedBooleanSplitReplayUndoBoundary, BooleanChainIntegrationHandoff,
    CompletedBooleanLoopReconstructionHandoff, PlanarBooleanLoopReconstructionCloseoutInput,
    WorkloadCompositionError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BooleanChainReplayUndoBoundaryHandoff {
    loop_handoff: CompletedBooleanLoopReconstructionHandoff,
    chain_handoff: BooleanChainIntegrationHandoff,
    consumer_cutover_closeout: ReplayUndoConsumerCutoverCloseout,
    hard_deletion_closeout: ReplayUndoHardDeletionCloseout,
    public_closeout: ReplayUndoMilestoneTwelvePublicCloseout,
}

impl BooleanChainReplayUndoBoundaryHandoff {
    pub(super) fn from_admitted_replay_undo_boundary(
        admitted_boundary: &AdmittedBooleanSplitReplayUndoBoundary,
        loop_closeout_input: PlanarBooleanLoopReconstructionCloseoutInput<'_>,
    ) -> Result<Self, WorkloadCompositionError> {
        let loop_handoff =
            admitted_boundary.complete_boolean_loop_reconstruction(loop_closeout_input)?;
        let chain_handoff = BooleanChainIntegrationHandoff::from_completed_handoffs(
            admitted_boundary.completed_split_handoff(),
            &loop_handoff,
        )?;
        close_replay_undo_boundary_handoff(loop_handoff, chain_handoff)
    }
}

fn close_replay_undo_boundary_handoff(
    loop_handoff: CompletedBooleanLoopReconstructionHandoff,
    chain_handoff: BooleanChainIntegrationHandoff,
) -> Result<BooleanChainReplayUndoBoundaryHandoff, WorkloadCompositionError> {
    let replay_undo_inventory = current_replay_undo_inventory_report().map_err(|error| {
        WorkloadCompositionError::BooleanChainHandoff(error.detail().to_string())
    })?;
    let replay_undo_source_firewall = current_replay_undo_source_firewall_report();
    let forbidden_surface_denials = current_replay_undo_forbidden_surface_denial_ledger();
    let consumer_cutover_closeout = ReplayUndoConsumerCutoverCloseout::close(
        ReplayUndoConsumerCutoverCloseoutInput::new(
            &loop_handoff,
            &chain_handoff,
            &replay_undo_inventory,
            &replay_undo_source_firewall,
            &forbidden_surface_denials,
        ),
    )
    .map_err(|error| WorkloadCompositionError::BooleanChainHandoff(error.detail().to_string()))?;
    let hard_deletion_closeout = ReplayUndoHardDeletionCloseout::close(
        ReplayUndoHardDeletionCloseoutInput::from_cutover(
            &consumer_cutover_closeout,
            &replay_undo_inventory,
            current_replay_undo_hard_deletion_source_firewall(),
        ),
    )
    .map_err(|error| WorkloadCompositionError::BooleanChainHandoff(error.detail().to_string()))?;
    let public_closeout = ReplayUndoMilestoneTwelvePublicCloseout::publish(
        ReplayUndoMilestoneTwelvePublicCloseoutInput::from_parts(
            &consumer_cutover_closeout,
            &hard_deletion_closeout,
            &replay_undo_inventory,
        )
        .map_err(|error| {
            WorkloadCompositionError::BooleanChainHandoff(error.detail().to_string())
        })?,
    )
    .map_err(|error| WorkloadCompositionError::BooleanChainHandoff(error.detail().to_string()))?;

    Ok(BooleanChainReplayUndoBoundaryHandoff {
        loop_handoff,
        chain_handoff,
        consumer_cutover_closeout,
        hard_deletion_closeout,
        public_closeout,
    })
}

impl BooleanChainReplayUndoBoundaryHandoff {
    pub fn loop_handoff(&self) -> &CompletedBooleanLoopReconstructionHandoff {
        &self.loop_handoff
    }

    pub fn chain_handoff(&self) -> &BooleanChainIntegrationHandoff {
        &self.chain_handoff
    }

    pub fn consumer_cutover_closeout(&self) -> &ReplayUndoConsumerCutoverCloseout {
        &self.consumer_cutover_closeout
    }

    pub fn hard_deletion_closeout(&self) -> &ReplayUndoHardDeletionCloseout {
        &self.hard_deletion_closeout
    }

    pub fn public_closeout(&self) -> &ReplayUndoMilestoneTwelvePublicCloseout {
        &self.public_closeout
    }

    pub fn into_loop_handoff(self) -> CompletedBooleanLoopReconstructionHandoff {
        self.loop_handoff
    }
}
