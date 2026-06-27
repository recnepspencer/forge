use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReplacementPolicy, WorthUiStateOwnerIdentity,
    WorthUiStatePersistencePosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateFamilyHook {
    family_id: WorthUiDurableStateFamilyId,
    owner_identity: Option<WorthUiStateOwnerIdentity>,
    replacement_policy: Option<WorthUiDurableStateReplacementPolicy>,
    persistence_posture: Option<WorthUiStatePersistencePosture>,
    lane_constrained: bool,
}

impl WorthUiDurableStateFamilyHook {
    pub fn custom(family_id: WorthUiDurableStateFamilyId) -> Self {
        Self {
            family_id,
            owner_identity: None,
            replacement_policy: None,
            persistence_posture: None,
            lane_constrained: true,
        }
    }

    pub fn with_owner_identity(mut self, owner_identity: WorthUiStateOwnerIdentity) -> Self {
        self.owner_identity = Some(owner_identity);
        self
    }

    pub fn with_replacement_policy(
        mut self,
        replacement_policy: WorthUiDurableStateReplacementPolicy,
    ) -> Self {
        self.replacement_policy = Some(replacement_policy);
        self
    }

    pub fn with_persistence_posture(
        mut self,
        persistence_posture: WorthUiStatePersistencePosture,
    ) -> Self {
        self.persistence_posture = Some(persistence_posture);
        self
    }

    pub fn with_lane_constrained(mut self, lane_constrained: bool) -> Self {
        self.lane_constrained = lane_constrained;
        self
    }

    pub fn family_id(&self) -> &WorthUiDurableStateFamilyId {
        &self.family_id
    }

    pub fn owner_identity(&self) -> Option<WorthUiStateOwnerIdentity> {
        self.owner_identity.clone()
    }

    pub fn replacement_policy(&self) -> Option<WorthUiDurableStateReplacementPolicy> {
        self.replacement_policy
    }

    pub fn persistence_posture(&self) -> Option<WorthUiStatePersistencePosture> {
        self.persistence_posture
    }

    pub fn is_lane_constrained(&self) -> bool {
        self.lane_constrained
    }
}
