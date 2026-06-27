use super::closeout_input::ReplayUndoConsumerCutoverCloseoutInput;
use super::counters::ReplayUndoConsumerCutoverCounters;
use super::error::{ReplayUndoConsumerCutoverError, ReplayUndoConsumerCutoverErrorKind};
use super::forbidden_surface_denial::ReplayUndoForbiddenConsumerSurfaceDenialLedger;
use super::milestone_thirteen_seed::ReplayUndoMilestoneThirteenSeed;
use super::ordinary_receipt_role_requirements::ORDINARY_RECEIPT_ROLE_REQUIREMENTS;
use super::residue_ledger::ReplayUndoConsumerCutoverResidueLedger;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoConsumerCutoverCloseout {
    transaction_packet_identity: String,
    replay_scope_identity: String,
    undo_scope_identity: String,
    boolean_chain_handoff_identity: String,
    residue_ledger: ReplayUndoConsumerCutoverResidueLedger,
    forbidden_surface_denials: ReplayUndoForbiddenConsumerSurfaceDenialLedger,
    counters: ReplayUndoConsumerCutoverCounters,
    milestone_thirteen_seed: ReplayUndoMilestoneThirteenSeed,
}

impl ReplayUndoConsumerCutoverCloseout {
    pub fn close(
        input: ReplayUndoConsumerCutoverCloseoutInput<'_>,
    ) -> Result<Self, ReplayUndoConsumerCutoverError> {
        require_declared_ordinary_roles(&input)?;
        require_no_undeclared_receipt_consumers(&input)?;
        input
            .forbidden_surface_denials()
            .require_phase_eleven_denials()?;

        let packet = input
            .loop_handoff()
            .require_replay_undo_transaction_boundary_packet()?;
        let residue_ledger =
            ReplayUndoConsumerCutoverResidueLedger::from_inventory(input.inventory())?;
        let forbidden_surface_denials = input.forbidden_surface_denials().clone();
        let counters = ReplayUndoConsumerCutoverCounters::from_inventory_and_packet(
            input.inventory().counters(),
            packet.counters(),
        );
        let milestone_thirteen_seed = ReplayUndoMilestoneThirteenSeed::lower(
            packet.packet_identity(),
            packet.replay_scope_identity().digest(),
            packet.undo_scope_identity().digest(),
            &residue_ledger,
            &counters,
            true,
        );

        Ok(Self {
            transaction_packet_identity: packet.packet_identity().to_string(),
            replay_scope_identity: packet.replay_scope_identity().digest().to_string(),
            undo_scope_identity: packet.undo_scope_identity().digest().to_string(),
            boolean_chain_handoff_identity: input.chain_handoff().handoff_identity().to_string(),
            residue_ledger,
            forbidden_surface_denials,
            counters,
            milestone_thirteen_seed,
        })
    }

    pub fn transaction_packet_identity(&self) -> &str {
        &self.transaction_packet_identity
    }

    pub fn replay_scope_identity(&self) -> &str {
        &self.replay_scope_identity
    }

    pub fn undo_scope_identity(&self) -> &str {
        &self.undo_scope_identity
    }

    pub fn boolean_chain_handoff_identity(&self) -> &str {
        &self.boolean_chain_handoff_identity
    }

    pub const fn counters(&self) -> &ReplayUndoConsumerCutoverCounters {
        &self.counters
    }

    pub const fn residue_ledger(&self) -> &ReplayUndoConsumerCutoverResidueLedger {
        &self.residue_ledger
    }

    pub const fn forbidden_surface_denials(
        &self,
    ) -> &ReplayUndoForbiddenConsumerSurfaceDenialLedger {
        &self.forbidden_surface_denials
    }

    pub const fn milestone_thirteen_seed(&self) -> &ReplayUndoMilestoneThirteenSeed {
        &self.milestone_thirteen_seed
    }
}

fn require_declared_ordinary_roles(
    input: &ReplayUndoConsumerCutoverCloseoutInput<'_>,
) -> Result<(), ReplayUndoConsumerCutoverError> {
    for (source, role) in ORDINARY_RECEIPT_ROLE_REQUIREMENTS {
        input
            .source_firewall()
            .require_declared_receipt_role(*source, *role)?;
    }
    Ok(())
}

fn require_no_undeclared_receipt_consumers(
    input: &ReplayUndoConsumerCutoverCloseoutInput<'_>,
) -> Result<(), ReplayUndoConsumerCutoverError> {
    if input
        .source_firewall()
        .require_no_undeclared_receipt_consumers()
    {
        Ok(())
    } else {
        Err(ReplayUndoConsumerCutoverError::new(
            ReplayUndoConsumerCutoverErrorKind::UndeclaredReceiptConsumer,
            "replay/undo consumer cutover found an undeclared receipt consumer",
        ))
    }
}
