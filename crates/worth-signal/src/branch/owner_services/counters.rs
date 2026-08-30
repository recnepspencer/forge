/// Exact structural work observed across Signal owner-service operations.
///
/// Counts are updated by their eventual owner work sites. This snapshot carries
/// no authority and makes no approximate byte or elapsed-time claim.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SignalOwnerServiceCostSnapshot {
    owner_upgrade_attempts: u64,
    branch_registry_lookups: u64,
    branch_registry_reservations: u64,
    branch_registry_entries_scanned: u64,
    target_cell_contacts: u64,
    target_cell_waits: u64,
    canonical_movements: u64,
    retention_registry_contacts: u64,
    fork_source_captures: u64,
    fork_destination_installations: u64,
    forked_mutable_graph_nodes_copied: u64,
    diagnostic_events_recorded: u64,
    diagnostic_events_dropped: u64,
    close_batches: u64,
}

impl SignalOwnerServiceCostSnapshot {
    pub const fn owner_upgrade_attempts(&self) -> u64 {
        self.owner_upgrade_attempts
    }

    pub const fn branch_registry_lookups(&self) -> u64 {
        self.branch_registry_lookups
    }

    pub const fn branch_registry_reservations(&self) -> u64 {
        self.branch_registry_reservations
    }

    pub const fn branch_registry_entries_scanned(&self) -> u64 {
        self.branch_registry_entries_scanned
    }

    pub const fn target_cell_contacts(&self) -> u64 {
        self.target_cell_contacts
    }

    pub const fn target_cell_waits(&self) -> u64 {
        self.target_cell_waits
    }

    pub const fn canonical_movements(&self) -> u64 {
        self.canonical_movements
    }

    pub const fn retention_registry_contacts(&self) -> u64 {
        self.retention_registry_contacts
    }

    pub const fn fork_source_captures(&self) -> u64 {
        self.fork_source_captures
    }

    pub const fn fork_destination_installations(&self) -> u64 {
        self.fork_destination_installations
    }

    pub const fn forked_mutable_graph_nodes_copied(&self) -> u64 {
        self.forked_mutable_graph_nodes_copied
    }

    pub const fn diagnostic_events_recorded(&self) -> u64 {
        self.diagnostic_events_recorded
    }

    pub const fn diagnostic_events_dropped(&self) -> u64 {
        self.diagnostic_events_dropped
    }

    pub const fn close_batches(&self) -> u64 {
        self.close_batches
    }
}

#[cfg(test)]
mod tests {
    use super::SignalOwnerServiceCostSnapshot;

    #[test]
    fn snapshot_accessors_preserve_every_exact_structural_count() {
        let snapshot = SignalOwnerServiceCostSnapshot {
            owner_upgrade_attempts: 1,
            branch_registry_lookups: 2,
            branch_registry_reservations: 3,
            branch_registry_entries_scanned: 4,
            target_cell_contacts: 5,
            target_cell_waits: 6,
            canonical_movements: 7,
            retention_registry_contacts: 8,
            fork_source_captures: 9,
            fork_destination_installations: 10,
            forked_mutable_graph_nodes_copied: 11,
            diagnostic_events_recorded: 12,
            diagnostic_events_dropped: 13,
            close_batches: 14,
        };

        assert_eq!(snapshot.owner_upgrade_attempts(), 1);
        assert_eq!(snapshot.branch_registry_lookups(), 2);
        assert_eq!(snapshot.branch_registry_reservations(), 3);
        assert_eq!(snapshot.branch_registry_entries_scanned(), 4);
        assert_eq!(snapshot.target_cell_contacts(), 5);
        assert_eq!(snapshot.target_cell_waits(), 6);
        assert_eq!(snapshot.canonical_movements(), 7);
        assert_eq!(snapshot.retention_registry_contacts(), 8);
        assert_eq!(snapshot.fork_source_captures(), 9);
        assert_eq!(snapshot.fork_destination_installations(), 10);
        assert_eq!(snapshot.forked_mutable_graph_nodes_copied(), 11);
        assert_eq!(snapshot.diagnostic_events_recorded(), 12);
        assert_eq!(snapshot.diagnostic_events_dropped(), 13);
        assert_eq!(snapshot.close_batches(), 14);
    }
}
