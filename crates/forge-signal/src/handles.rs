//! Typed generational handle for signal graph nodes.
//!
//! DOMAIN: Stable identity for reactive signal nodes.
//!
//! INVARIANTS:
//! - Each node has a unique `(index, generation)` pair
//! - Stale handles are detected via generation mismatch
//! - Handles are `Copy` for cheap passing
//!
//! DEPENDENCIES: None

/// A typed, generational handle for a signal graph node.
///
/// - `index`: slot position in the graph's node arena
/// - `generation`: incremented when a slot is reused after deletion
///
/// Stale handles (from deleted nodes) are detected by generation
/// mismatch, preventing use-after-free without `unsafe`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId {
    index: u32,
    generation: u32,
}

impl NodeId {
    /// Create a new handle. Typically only called by the graph allocator.
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// The slot index in the arena.
    pub fn index(self) -> u32 {
        self.index
    }

    /// The generation counter (for stale-handle detection).
    pub fn generation(self) -> u32 {
        self.generation
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId({}:gen{})", self.index, self.generation)
    }
}
