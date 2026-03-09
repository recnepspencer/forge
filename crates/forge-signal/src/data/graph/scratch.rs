use crate::data::bitset::DenseBitset;
use crate::data::handle::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum ScratchLeaseKind {
    Evaluation,
    Invalidation,
    Gc,
    Churn,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TraversalScratch {
    pub(crate) visited: VisitMarks,
    pub(crate) node_buffer_a: Vec<NodeId>,
    pub(crate) node_buffer_b: Vec<NodeId>,
    pub(crate) gc_liveness_generations: Vec<u32>,
    pub(crate) gc_liveness_alive: DenseBitset,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct VisitMarks {
    marks: Vec<u32>,
    epoch: u32,
}

impl VisitMarks {
    pub(crate) fn next_pass(&mut self, len: usize) {
        if self.marks.len() < len {
            self.marks.resize(len, 0);
        }
        if self.epoch == u32::MAX {
            self.marks.fill(0);
            self.epoch = 1;
        } else {
            self.epoch += 1;
        }
    }

    pub(crate) fn is_marked(&self, idx: usize) -> bool {
        idx < self.marks.len() && self.marks[idx] == self.epoch
    }

    pub(crate) fn mark(&mut self, idx: usize) -> bool {
        if idx >= self.marks.len() {
            self.marks.resize(idx + 1, 0);
        }
        if self.marks[idx] == self.epoch {
            false
        } else {
            self.marks[idx] = self.epoch;
            true
        }
    }
}
