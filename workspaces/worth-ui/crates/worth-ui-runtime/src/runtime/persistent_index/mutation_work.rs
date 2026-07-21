/// Exact structural work performed by one persistent ordered-index mutation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiPersistentIndexMutationWork {
    key_probes: usize,
    node_copies: usize,
}

impl UiPersistentIndexMutationWork {
    pub(crate) fn key_probes(self) -> usize {
        self.key_probes
    }

    pub(crate) fn node_copies(self) -> usize {
        self.node_copies
    }

    pub(super) fn record_key_probe(&mut self) {
        self.key_probes += 1;
    }

    pub(super) fn record_node_copy(&mut self) {
        self.node_copies += 1;
    }

    pub(crate) fn merge(&mut self, other: Self) -> Result<(), ()> {
        self.key_probes = self.key_probes.checked_add(other.key_probes).ok_or(())?;
        self.node_copies = self.node_copies.checked_add(other.node_copies).ok_or(())?;
        Ok(())
    }

    pub(crate) fn with_key_probes(key_probes: usize) -> Self {
        Self {
            key_probes,
            node_copies: 0,
        }
    }
}
