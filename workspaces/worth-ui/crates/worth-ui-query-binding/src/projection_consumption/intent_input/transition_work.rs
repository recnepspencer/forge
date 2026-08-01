#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiProjectionInputTransitionWork {
    replaced_rows: usize,
    change_operations: usize,
    key_probes: usize,
    node_copies: usize,
}

impl UiProjectionInputTransitionWork {
    pub const fn replaced_rows(self) -> usize {
        self.replaced_rows
    }

    pub const fn change_operations(self) -> usize {
        self.change_operations
    }

    pub const fn key_probes(self) -> usize {
        self.key_probes
    }

    pub const fn node_copies(self) -> usize {
        self.node_copies
    }

    pub(super) const fn with_key_probes(key_probes: usize) -> Self {
        Self {
            replaced_rows: 0,
            change_operations: 0,
            key_probes,
            node_copies: 0,
        }
    }

    pub(super) fn record_replaced_row(&mut self, mutation: Self) -> Result<(), ()> {
        self.replaced_rows = self.replaced_rows.checked_add(1).ok_or(())?;
        self.merge_index_work(mutation)
    }

    pub(super) fn record_change(&mut self, mutation: Self) -> Result<(), ()> {
        self.change_operations = self.change_operations.checked_add(1).ok_or(())?;
        self.merge_index_work(mutation)
    }

    pub(super) fn record_key_probe(&mut self) {
        self.key_probes = self.key_probes.saturating_add(1);
    }

    pub(super) fn record_node_copy(&mut self) {
        self.node_copies = self.node_copies.saturating_add(1);
    }

    fn merge_index_work(&mut self, mutation: Self) -> Result<(), ()> {
        self.key_probes = self.key_probes.checked_add(mutation.key_probes).ok_or(())?;
        self.node_copies = self
            .node_copies
            .checked_add(mutation.node_copies)
            .ok_or(())?;
        Ok(())
    }
}
