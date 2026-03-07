use crate::data::handle::NodeId;

#[derive(Debug, Clone, Copy)]
struct NodeTier<T: Copy> {
    generation: u32,
    tier: T,
}

/// Arena-aligned per-node metadata storage.
#[derive(Debug, Clone)]
pub struct NodeMetaStore<T: Copy> {
    tiers: Vec<Option<NodeTier<T>>>,
}

impl<T: Copy> NodeMetaStore<T> {
    /// Create an empty metadata store.
    pub fn new() -> Self {
        Self { tiers: Vec::new() }
    }

    /// Ensure storage covers node slots up to `len`.
    pub fn ensure_capacity(&mut self, len: usize) {
        if self.tiers.len() < len {
            self.tiers.resize(len, None);
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
}

impl<T: Copy> Default for NodeMetaStore<T> {
    fn default() -> Self {
        Self::new()
    }
}
