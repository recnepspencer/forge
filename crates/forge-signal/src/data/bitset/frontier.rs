use super::DenseBitset;

/// Two-board frontier for deterministic breadth-style stepping.
#[derive(Debug, Clone, Default)]
pub struct BitsetFrontier {
    current: DenseBitset,
    next: DenseBitset,
}

impl BitsetFrontier {
    /// Create an empty frontier.
    pub fn new() -> Self {
        Self {
            current: DenseBitset::new(),
            next: DenseBitset::new(),
        }
    }

    /// Clear both boards.
    pub fn clear(&mut self) {
        self.current.clear_all();
        self.next.clear_all();
    }

    /// Seed one index into current board.
    pub fn seed(&mut self, idx: usize) {
        self.current.mark(idx);
    }

    /// Mark one index for the next step.
    pub fn mark_next(&mut self, idx: usize) {
        self.next.mark(idx);
    }

    /// Current frontier indices in ascending deterministic order.
    pub fn current_indices(&self) -> Vec<usize> {
        self.current.marked_indices()
    }

    /// Current frontier indices in ascending deterministic order.
    pub fn current_iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.current.iter_marked()
    }

    /// Whether the current board is non-empty.
    pub fn has_current(&self) -> bool {
        self.current.any()
    }

    /// Advance one step.
    pub fn advance(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
        self.next.clear_all();
    }
}
