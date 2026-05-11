use crate::topology_operators::TopologyEditNamingOutcome;

use super::super::naming_continuity_breadth_row::MilestoneThreeNamingContinuityBreadthRow;
use super::super::report::MilestoneThreeHostileScenario;

impl MilestoneThreeNamingContinuityBreadthRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn continuity_row_count(&self) -> usize {
        self.continuity_row_count
    }

    pub fn preserved_count(&self) -> usize {
        self.preserved_count
    }

    pub fn ambiguous_count(&self) -> usize {
        self.ambiguous_count
    }

    pub fn rejected_count(&self) -> usize {
        self.rejected_count
    }

    pub fn naming_scope_count(&self) -> usize {
        self.naming_scope_count
    }

    pub fn replay_step_count(&self) -> usize {
        self.replay_step_count
    }

    pub fn replay_checked(&self) -> bool {
        self.replay_checked
    }

    pub fn outcome_class(&self) -> TopologyEditNamingOutcome {
        self.outcome_class
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}
