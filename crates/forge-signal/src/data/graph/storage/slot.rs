use serde::{Deserialize, Serialize};

use crate::data::node::NodeEntry;

/// A slot in the node arena that may be occupied or vacant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Slot {
    /// The node data, if occupied.
    pub(crate) data: Option<NodeEntry>,
    /// Generation counter (bumped on each reuse).
    pub(crate) generation: u32,
    /// Whether this slot has been permanently retired after generation wrap.
    pub(crate) retired: bool,
}

impl Slot {
    /// Create a new vacant slot at generation 0.
    pub(crate) fn vacant() -> Self {
        Self {
            data: None,
            generation: 0,
            retired: false,
        }
    }

    /// Occupy this slot, returning the current generation.
    pub(crate) fn occupy(&mut self, entry: NodeEntry) -> u32 {
        self.data = Some(entry);
        self.generation
    }

    /// Vacate this slot, bumping the generation and retiring on wrap.
    pub(crate) fn vacate(&mut self) -> Option<NodeEntry> {
        self.generation = self.generation.wrapping_add(1);
        if self.generation == 0 {
            self.retired = true;
        }
        self.data.take()
    }

    /// Whether this slot is occupied.
    pub(crate) fn is_occupied(&self) -> bool {
        self.data.is_some()
    }

    pub(crate) fn is_retired(&self) -> bool {
        self.retired
    }
}
