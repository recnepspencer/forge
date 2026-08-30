use crate::data::handle::NodeId;

#[derive(Debug, Clone, Copy)]
struct NodeTier<T: Copy> {
    generation: u32,
    tier: T,
}

/// Arena-aligned per-node metadata storage.
#[derive(Debug, Clone)]
pub struct NodeMetaStore<T: Copy> {
    tiers: crate::data::persistent_vector::PersistentVector<Option<NodeTier<T>>>,
}

impl<T: Copy> NodeMetaStore<T> {
    /// Create an empty metadata store.
    pub fn new() -> Self {
        Self {
            tiers: crate::data::persistent_vector::PersistentVector::new(),
        }
    }

    /// Ensure storage covers node slots up to `len`.
    pub fn ensure_capacity(&mut self, len: usize) {
        if self.tiers.len() < len {
            let missing = len - self.tiers.len();
            self.tiers.extend(std::iter::repeat_n(None, missing));
        }
    }

    /// Assign a tier to a live node slot.
    pub fn set_tier(&mut self, node: NodeId, tier: T) {
        let index = node.index() as usize;
        self.ensure_capacity(index + 1);
        self.tiers[index] = Some(NodeTier {
            generation: node.generation(),
            tier,
        });
    }

    /// Clear metadata for one node slot.
    pub fn clear_node(&mut self, node_index: usize) {
        if node_index < self.tiers.len() {
            self.tiers[node_index] = None;
        }
    }

    /// Read one tier by validated node handle.
    pub fn tier_for_node(&self, node: NodeId) -> Option<T> {
        let index = node.index() as usize;
        self.tiers
            .get(index)
            .and_then(|entry| *entry)
            .filter(|entry| entry.generation == node.generation())
            .map(|entry| entry.tier)
    }

    pub(crate) fn prune_slots(&mut self, mut keep: impl FnMut(usize, u32) -> bool) {
        for (index, entry) in self.tiers.iter_mut().enumerate() {
            if entry.is_some_and(|tier| !keep(index, tier.generation)) {
                *entry = None;
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn occupied_slot_count(&self) -> usize {
        self.tiers.iter().filter(|entry| entry.is_some()).count()
    }

    #[cfg(test)]
    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        self.tiers.shares_storage_with(&other.tiers)
    }
}

impl<T: Copy> Default for NodeMetaStore<T> {
    fn default() -> Self {
        Self::new()
    }
}
