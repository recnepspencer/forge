use crate::graph::UiGraphWorldProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphTouchWorld {
    world_profile: UiGraphWorldProfile,
}

impl UiGraphTouchWorld {
    pub(crate) fn from_profile(world_profile: &UiGraphWorldProfile) -> Self {
        Self {
            world_profile: world_profile.clone(),
        }
    }

    pub fn world_profile(&self) -> &UiGraphWorldProfile {
        &self.world_profile
    }
}
