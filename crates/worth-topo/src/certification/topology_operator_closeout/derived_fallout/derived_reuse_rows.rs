use crate::certification::DeterministicDigest;
use serde::{Deserialize, Serialize};

use super::super::report::{MilestoneThreeEditFalloutClass, MilestoneThreeHostileScenario};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeDerivedReuseLegalityRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) recompute_suppression_claimed: bool,
    pub(crate) equivalence_contract_required: bool,
    pub(crate) replay_materialized_topology_equivalent: bool,
    pub(crate) fallback_count: usize,
    pub(crate) fallout_class: MilestoneThreeEditFalloutClass,
    pub(crate) derived_validation_digest: Option<DeterministicDigest>,
    pub(crate) row_digest: String,
}




