use super::super::query_traversal_proof::{
    MilestoneThreeEditedTopologyQueryTraversalRow, MilestoneThreeEditedTopologyQueryTraversalView,
};
use super::super::report::MilestoneThreeHostileScenario;

impl MilestoneThreeEditedTopologyQueryTraversalRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn view(&self) -> MilestoneThreeEditedTopologyQueryTraversalView {
        self.view
    }

    pub fn left_view_digest(&self) -> &str {
        self.left_view_digest.as_str()
    }

    pub fn replay_view_digest(&self) -> &str {
        self.replay_view_digest.as_str()
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn request_count(&self) -> usize {
        self.request_count
    }

    pub fn relationship_proof_admission_count(&self) -> usize {
        self.relationship_proof_admission_count
    }

    pub fn traversal_count(&self) -> usize {
        self.traversal_count
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}
