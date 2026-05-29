use super::super::super::derived_fallout::{
    MilestoneThreeDerivedWorkBreadthClass, MilestoneThreeDerivedWorkBreadthRow,
};
use super::super::super::report::MilestoneThreeHostileScenario;

impl MilestoneThreeDerivedWorkBreadthRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn invalidation_breadth_class(&self) -> MilestoneThreeDerivedWorkBreadthClass {
        self.invalidation_breadth_class
    }

    pub fn rebuild_breadth_class(&self) -> MilestoneThreeDerivedWorkBreadthClass {
        self.rebuild_breadth_class
    }

    pub fn declared_changed_scope_count(&self) -> usize {
        self.declared_changed_scope_count
    }

    pub fn declared_derived_region_count(&self) -> usize {
        self.declared_derived_region_count
    }

    pub fn actual_derived_validation_row_count(&self) -> usize {
        self.actual_derived_validation_row_count
    }

    pub fn fallback_count(&self) -> usize {
        self.fallback_count
    }

    pub fn locality_claimed(&self) -> bool {
        self.locality_claimed
    }

    pub fn locality_claim_mismatch(&self) -> bool {
        self.locality_claim_mismatch
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}
