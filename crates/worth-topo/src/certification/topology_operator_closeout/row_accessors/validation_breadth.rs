use super::super::report::{MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario};
use super::super::validation_breadth_row::MilestoneThreeValidationBreadthRow;

impl MilestoneThreeValidationBreadthRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn outcome_class(&self) -> MilestoneThreeHostileOutcomeClass {
        self.outcome_class
    }

    pub fn validator_family_count(&self) -> usize {
        self.validator_family_count
    }

    pub fn validator_name_count(&self) -> usize {
        self.validator_name_count
    }

    pub fn mutation_family_count(&self) -> usize {
        self.mutation_family_count
    }

    pub fn changed_scope_count(&self) -> usize {
        self.changed_scope_count
    }

    pub fn naming_scope_count(&self) -> usize {
        self.naming_scope_count
    }

    pub fn derived_region_count(&self) -> usize {
        self.derived_region_count
    }

    pub fn derived_validation_row_count(&self) -> usize {
        self.derived_validation_row_count
    }

    pub fn localized_rejection_boundary_count(&self) -> usize {
        self.localized_rejection_boundary_count
    }

    pub fn replay_checked(&self) -> bool {
        self.replay_checked
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}
