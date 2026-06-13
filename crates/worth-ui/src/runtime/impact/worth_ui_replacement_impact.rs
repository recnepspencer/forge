use crate::runtime::{WorthUiLaneImpactClassification, WorthUiReplacementScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiReplacementImpact {
    NoOp,
    LocalSubtree(WorthUiReplacementScope),
    StructuralReplacement(WorthUiReplacementScope),
    BroadReplacement(WorthUiReplacementScope),
    LaneAffecting {
        lane_impact: WorthUiLaneImpactClassification,
        scope: WorthUiReplacementScope,
    },
}

impl WorthUiReplacementImpact {
    pub fn is_noop(&self) -> bool {
        matches!(self, Self::NoOp)
    }
}
