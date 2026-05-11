use serde::{Deserialize, Serialize};

use crate::topology_operators::{TopologyEditDerivedFallbackPolicy, TopologyEditRejectionClass};

use super::super::report::{MilestoneThreeEditFalloutClass, MilestoneThreeHostileScenario};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeDerivedFallbackPolicyDenialRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) strict_fallback_policy: TopologyEditDerivedFallbackPolicy,
    pub(crate) observed_fallout_class: MilestoneThreeEditFalloutClass,
    pub(crate) observed_fallback_count: usize,
    pub(crate) denied_rejection_class: TopologyEditRejectionClass,
    pub(crate) policy_exceeded: bool,
    pub(crate) row_digest: String,
}
