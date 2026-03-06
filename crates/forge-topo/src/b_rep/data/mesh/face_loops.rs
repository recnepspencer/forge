//! Loop topology for a face: exactly one outer loop plus zero or more inner loops.

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::handles::LoopId;

/// A face's loop structure with compile-time enforced outer-loop presence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceLoops {
    outer: LoopId,
    inners: SmallVec<[LoopId; 2]>,
}

impl FaceLoops {
    /// Construct with a required outer boundary loop.
    pub fn new(outer: LoopId) -> Self {
        Self {
            outer,
            inners: SmallVec::new(),
        }
    }

    /// The outer boundary loop.
    pub fn outer(&self) -> LoopId {
        self.outer
    }

    /// Replace the outer boundary loop.
    pub fn set_outer(&mut self, outer: LoopId) {
        self.outer = outer;
    }

    /// Inner boundary loops (holes).
    pub fn inners(&self) -> &[LoopId] {
        &self.inners
    }

    /// Add an inner loop.
    pub fn add_inner(&mut self, id: LoopId) {
        self.inners.push(id);
    }

    /// Remove an inner loop by ID.
    ///
    /// Returns `true` if found/removed.
    pub fn remove_inner(&mut self, id: LoopId) -> bool {
        if let Some(pos) = self.inners.iter().position(|&l| l == id) {
            self.inners.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// All loops in deterministic order: outer first, then inners.
    pub fn all_loops(&self) -> SmallVec<[LoopId; 3]> {
        let mut loops = SmallVec::<[LoopId; 3]>::with_capacity(1 + self.inners.len());
        loops.push(self.outer);
        loops.extend_from_slice(&self.inners);
        loops
    }
}
