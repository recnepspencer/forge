use crate::topology_operators::TopologyMutationNamingOutcome;
use serde::{Deserialize, Serialize};

use super::report::MilestoneThreeHostileScenario;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeNamingContinuityBreadthRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) continuity_row_count: usize,
    pub(crate) preserved_count: usize,
    pub(crate) ambiguous_count: usize,
    pub(crate) rejected_count: usize,
    pub(crate) naming_scope_count: usize,
    pub(crate) replay_step_count: usize,
    pub(crate) replay_checked: bool,
    pub(crate) outcome_class: TopologyMutationNamingOutcome,
    pub(crate) row_digest: String,
}
