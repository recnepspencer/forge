use crate::certification::DeterministicDigest;

use super::super::super::derived_fallout::MilestoneThreeDerivedReuseLegalityRow;
use super::super::super::report::{MilestoneThreeEditFalloutClass, MilestoneThreeHostileScenario};

impl MilestoneThreeDerivedReuseLegalityRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn recompute_suppression_claimed(&self) -> bool {
        self.recompute_suppression_claimed
    }

    pub fn equivalence_contract_required(&self) -> bool {
        self.equivalence_contract_required
    }

    pub fn replay_materialized_topology_equivalent(&self) -> bool {
        self.replay_materialized_topology_equivalent
    }

    pub fn fallback_count(&self) -> usize {
        self.fallback_count
    }

    pub fn fallout_class(&self) -> MilestoneThreeEditFalloutClass {
        self.fallout_class
    }

    pub fn derived_validation_digest(&self) -> Option<&DeterministicDigest> {
        self.derived_validation_digest.as_ref()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}




