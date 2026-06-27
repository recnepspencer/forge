use super::deletion_ledger::ReplayUndoHardDeletionLedger;
use super::residue_cap_audit::ReplayUndoResidueCapAudit;
use super::source_firewall::ReplayUndoHardDeletionSourceFirewall;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayUndoHardDeletionCounters {
    deletion_row_count: usize,
    residue_cap_row_count: usize,
    uncapped_residue_count: usize,
    scanned_source_count: usize,
    source_firewall_violation_count: usize,
}

impl ReplayUndoHardDeletionCounters {
    pub(crate) fn from_parts(
        deletion_ledger: &ReplayUndoHardDeletionLedger,
        residue_cap_audit: &ReplayUndoResidueCapAudit,
        source_firewall: &ReplayUndoHardDeletionSourceFirewall,
    ) -> Self {
        Self {
            deletion_row_count: deletion_ledger.row_count(),
            residue_cap_row_count: residue_cap_audit.row_count(),
            uncapped_residue_count: residue_cap_audit.uncapped_residue_count(),
            scanned_source_count: source_firewall.scanned_source_count(),
            source_firewall_violation_count: source_firewall.violation_count(),
        }
    }

    pub const fn deletion_row_count(self) -> usize {
        self.deletion_row_count
    }

    pub const fn residue_cap_row_count(self) -> usize {
        self.residue_cap_row_count
    }

    pub const fn uncapped_residue_count(self) -> usize {
        self.uncapped_residue_count
    }

    pub const fn scanned_source_count(self) -> usize {
        self.scanned_source_count
    }

    pub const fn source_firewall_violation_count(self) -> usize {
        self.source_firewall_violation_count
    }
}
