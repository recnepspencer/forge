use serde::{Deserialize, Serialize};

use super::report::MilestoneThreeHostileScenario;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MilestoneThreeEditedTopologyQueryTraversalView {
    PostEditLocalRewireNeighborhood,
    PostEditLoopCycleNeighborhood,
}

impl MilestoneThreeEditedTopologyQueryTraversalView {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostEditLocalRewireNeighborhood => "post_edit_local_rewire_view",
            Self::PostEditLoopCycleNeighborhood => "post_edit_loop_cycle_view",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeEditedTopologyQueryTraversalRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) view: MilestoneThreeEditedTopologyQueryTraversalView,
    pub(crate) left_view_digest: String,
    pub(crate) replay_view_digest: String,
    pub(crate) parity_verified: bool,
    pub(crate) request_count: usize,
    pub(crate) relationship_proof_admission_count: usize,
    pub(crate) traversal_count: usize,
    pub(crate) row_digest: String,
}
