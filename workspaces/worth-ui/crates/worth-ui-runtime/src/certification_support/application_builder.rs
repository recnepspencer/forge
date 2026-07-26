use crate::facade::WorthUiApplicationBuilder;
use crate::graph::UiGraphWorldProfile;

/// Certification-only configuration of synthetic graph-world authority.
pub trait WorthUiApplicationBuilderCertificationExt {
    fn with_graph_world_profile(
        self,
        graph_world_profile: UiGraphWorldProfile,
    ) -> WorthUiApplicationBuilder;
}

impl WorthUiApplicationBuilderCertificationExt for WorthUiApplicationBuilder {
    fn with_graph_world_profile(
        self,
        graph_world_profile: UiGraphWorldProfile,
    ) -> WorthUiApplicationBuilder {
        WorthUiApplicationBuilder::with_graph_world_profile(self, graph_world_profile)
    }
}
