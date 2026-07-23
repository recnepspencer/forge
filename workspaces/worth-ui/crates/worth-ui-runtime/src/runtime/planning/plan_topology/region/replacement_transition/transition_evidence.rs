use super::WorthUiPlanRegionIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPlanRegionTransition {
    Reused,
    Replaced,
    Reparented,
    Rebound,
    LaneTransitioned,
    Inserted,
    Retired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanRegionTransitionEvidence {
    region_identity: WorthUiPlanRegionIdentity,
    transition: WorthUiPlanRegionTransition,
}

impl WorthUiPlanRegionTransitionEvidence {
    pub(crate) fn new(
        region_identity: WorthUiPlanRegionIdentity,
        transition: WorthUiPlanRegionTransition,
    ) -> Self {
        Self {
            region_identity,
            transition,
        }
    }

    pub fn region_identity(&self) -> &WorthUiPlanRegionIdentity {
        &self.region_identity
    }

    pub fn transition(&self) -> WorthUiPlanRegionTransition {
        self.transition
    }
}
