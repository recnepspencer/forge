use crate::graph::UiGraphWorldProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAdmissionWorld {
    graph_world_profile: UiGraphWorldProfile,
}

impl UiAdmissionWorld {
    pub fn authoritative() -> Self {
        Self::from_graph_world_profile(UiGraphWorldProfile::authoritative())
    }

    pub fn from_graph_world_profile(graph_world_profile: UiGraphWorldProfile) -> Self {
        Self {
            graph_world_profile,
        }
    }

    pub fn graph_world_profile(&self) -> &UiGraphWorldProfile {
        &self.graph_world_profile
    }
}
