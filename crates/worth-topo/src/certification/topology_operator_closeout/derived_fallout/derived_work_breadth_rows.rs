use serde::{Deserialize, Serialize};

use super::super::report::MilestoneThreeHostileScenario;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MilestoneThreeDerivedWorkBreadthClass {
    DeclaredRegions,
    WholeViewFallback,
    WholeHistoryFallback,
    RejectedBeforeDerivedWork,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeDerivedWorkBreadthRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) invalidation_breadth_class: MilestoneThreeDerivedWorkBreadthClass,
    pub(crate) rebuild_breadth_class: MilestoneThreeDerivedWorkBreadthClass,
    pub(crate) declared_changed_scope_count: usize,
    pub(crate) declared_derived_region_count: usize,
    pub(crate) actual_derived_validation_row_count: usize,
    pub(crate) fallback_count: usize,
    pub(crate) locality_claimed: bool,
    pub(crate) locality_claim_mismatch: bool,
    pub(crate) row_digest: String,
}
