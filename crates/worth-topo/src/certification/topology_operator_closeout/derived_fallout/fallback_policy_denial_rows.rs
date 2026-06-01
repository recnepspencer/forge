use serde::{Deserialize, Serialize};

use crate::topology_operators::{
    TopologyMutationDerivedFallbackPolicy, TopologyMutationRejectionClass,
};

use super::super::report::{MilestoneThreeHostileScenario, MilestoneThreeMutationFalloutClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeDerivedFallbackPolicyDenialRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) strict_fallback_policy: TopologyMutationDerivedFallbackPolicy,
    pub(crate) observed_fallout_class: MilestoneThreeMutationFalloutClass,
    pub(crate) observed_fallback_count: usize,
    pub(crate) denied_rejection_class: TopologyMutationRejectionClass,
    pub(crate) policy_exceeded: bool,
    pub(crate) row_digest: String,
}
