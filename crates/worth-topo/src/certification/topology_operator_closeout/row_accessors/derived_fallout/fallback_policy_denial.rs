use super::super::super::derived_fallout::MilestoneThreeDerivedFallbackPolicyDenialRow;
use super::super::super::report::{
    MilestoneThreeHostileScenario, MilestoneThreeMutationFalloutClass,
};
use crate::topology_operators::{
    TopologyMutationDerivedFallbackPolicy, TopologyMutationRejectionClass,
};

impl MilestoneThreeDerivedFallbackPolicyDenialRow {
    pub fn scenario(&self) -> MilestoneThreeHostileScenario {
        self.scenario
    }

    pub fn strict_fallback_policy(&self) -> TopologyMutationDerivedFallbackPolicy {
        self.strict_fallback_policy
    }

    pub fn observed_fallout_class(&self) -> MilestoneThreeMutationFalloutClass {
        self.observed_fallout_class
    }

    pub fn observed_fallback_count(&self) -> usize {
        self.observed_fallback_count
    }

    pub fn denied_rejection_class(&self) -> TopologyMutationRejectionClass {
        self.denied_rejection_class
    }

    pub fn policy_exceeded(&self) -> bool {
        self.policy_exceeded
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}
