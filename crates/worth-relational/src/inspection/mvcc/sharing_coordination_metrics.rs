use crate::branch::RelationalBranchCoordinationCellId;

use super::observation::RelationalBranchSharingObservation;

/// Coordination-lane metrics: what the selected branches' own coordination
/// cells report, read live at observation time.
///
/// Coordination is branch-local by construction. Every metric here is summed
/// over the selected branches' cells only; no runtime-global gate, and no
/// unselected branch, can move any of them.
impl RelationalBranchSharingObservation {
    /// Times the selected branches' coordination cells were entered.
    ///
    /// Truth source: the per-branch coordination counters, read live and
    /// summed over the selection. This is a lifetime total for those cells,
    /// not a count of anything this observation did.
    pub const fn coordination_contacts(&self) -> u64 {
        self.coordination_contacts
    }

    /// Times entering those cells actually had to wait.
    ///
    /// Truth source: the same per-branch coordination counters. A wait is
    /// recorded only when an entry on that exact branch found the cell already
    /// held; contention on a different branch never appears here. A value of
    /// zero alongside a large [`Self::coordination_contacts`] is the
    /// observable form of branch-local, uncontended publication.
    pub const fn coordination_waits(&self) -> u64 {
        self.coordination_waits
    }

    /// Identities of the selected branches' coordination cells, in ascending
    /// order.
    ///
    /// Truth source: the branch cells resolved from the selection, one entry
    /// per selected branch. Because these ids are runtime-affine and
    /// branch-named, forks of one root still report distinct cells, which is
    /// how shared truth and separate coordination stay distinguishable.
    pub fn coordination_cells(&self) -> &[RelationalBranchCoordinationCellId] {
        &self.coordination_cells
    }
}
