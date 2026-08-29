use crate::data::handle::NodeId;
use crate::logic::planner::{EligibleTask, StageCursor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScratchLeaseKind {
    Evaluation,
    Invalidation,
    Gc,
    Churn,
}
#[derive(Debug, Clone, Default)]
pub(crate) struct TraversalScratch {
    pub(crate) node_buffer_a: Vec<NodeId>,
    pub(crate) node_buffer_b: Vec<NodeId>,
    pub(crate) planner_targets: Vec<NodeId>,
    pub(crate) planner_tasks: Vec<EligibleTask>,
    pub(crate) planner_stages: Vec<StageCursor>,
}

#[derive(Debug)]
pub(crate) struct GraphScratch<'a> {
    traversal: &'a mut TraversalScratch,
}

impl<'a> GraphScratch<'a> {
    pub(crate) fn new(traversal: &'a mut TraversalScratch) -> Self {
        Self { traversal }
    }

    pub(crate) fn traversal_mut(&mut self) -> &mut TraversalScratch {
        self.traversal
    }
}
