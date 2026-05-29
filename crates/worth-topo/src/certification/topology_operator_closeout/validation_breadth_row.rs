use serde::{Deserialize, Serialize};

use super::report::{MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeValidationBreadthRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) outcome_class: MilestoneThreeHostileOutcomeClass,
    pub(crate) validator_family_count: usize,
    pub(crate) validator_name_count: usize,
    pub(crate) edit_family_count: usize,
    pub(crate) changed_scope_count: usize,
    pub(crate) naming_scope_count: usize,
    pub(crate) derived_region_count: usize,
    pub(crate) derived_validation_row_count: usize,
    pub(crate) localized_rejection_boundary_count: usize,
    pub(crate) replay_checked: bool,
    pub(crate) row_digest: String,
}




