use super::closeout_input::ReplayUndoHardDeletionCloseoutInput;
use super::counters::ReplayUndoHardDeletionCounters;
use super::deletion_ledger::ReplayUndoHardDeletionLedger;
use super::error::ReplayUndoHardDeletionError;
use super::residue_cap_audit::ReplayUndoResidueCapAudit;
use super::source_firewall::ReplayUndoHardDeletionSourceFirewall;
use crate::replay_undo_consumer_cutover::ReplayUndoMilestoneThirteenSeed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoHardDeletionCloseout {
    deletion_ledger: ReplayUndoHardDeletionLedger,
    residue_cap_audit: ReplayUndoResidueCapAudit,
    source_firewall: ReplayUndoHardDeletionSourceFirewall,
    counters: ReplayUndoHardDeletionCounters,
    milestone_thirteen_seed: ReplayUndoMilestoneThirteenSeed,
}

impl ReplayUndoHardDeletionCloseout {
    pub fn close(
        input: ReplayUndoHardDeletionCloseoutInput<'_>,
    ) -> Result<Self, ReplayUndoHardDeletionError> {
        input.source_firewall().require_clean()?;
        let deletion_ledger = ReplayUndoHardDeletionLedger::from_inventory(input.inventory())?;
        let residue_cap_audit = ReplayUndoResidueCapAudit::from_inventory(input.inventory())?;
        residue_cap_audit.require_capped()?;
        let source_firewall = input.source_firewall().clone();
        let counters = ReplayUndoHardDeletionCounters::from_parts(
            &deletion_ledger,
            &residue_cap_audit,
            &source_firewall,
        );
        let milestone_thirteen_seed = ReplayUndoMilestoneThirteenSeed::lower_after_hard_deletion(
            input.consumer_cutover().milestone_thirteen_seed(),
            deletion_ledger.ledger_digest(),
            residue_cap_audit.audit_digest(),
            source_firewall.report_digest(),
        );
        Ok(Self {
            deletion_ledger,
            residue_cap_audit,
            source_firewall,
            counters,
            milestone_thirteen_seed,
        })
    }

    pub const fn deletion_ledger(&self) -> &ReplayUndoHardDeletionLedger {
        &self.deletion_ledger
    }

    pub const fn residue_cap_audit(&self) -> &ReplayUndoResidueCapAudit {
        &self.residue_cap_audit
    }

    pub const fn source_firewall(&self) -> &ReplayUndoHardDeletionSourceFirewall {
        &self.source_firewall
    }

    pub const fn counters(&self) -> ReplayUndoHardDeletionCounters {
        self.counters
    }

    pub const fn milestone_thirteen_seed(&self) -> &ReplayUndoMilestoneThirteenSeed {
        &self.milestone_thirteen_seed
    }

    pub fn uncapped_residue_count(&self) -> usize {
        self.residue_cap_audit.uncapped_residue_count()
    }
}
