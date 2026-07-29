use crate::facade::WorthUiApplicationBuilder;
use crate::graph::{UiGraphWorldProfile, UiRuntimeInstanceBasisAdmission};

/// Certification-only configuration of synthetic graph-world authority.
pub trait WorthUiApplicationBuilderCertificationExt {
    fn with_graph_world_profile(
        self,
        graph_world_profile: UiGraphWorldProfile,
    ) -> WorthUiApplicationBuilder;

    fn with_runtime_instance_basis_admissions(
        self,
        admissions: impl IntoIterator<Item = UiRuntimeInstanceBasisAdmission>,
    ) -> WorthUiApplicationBuilder;
}

impl WorthUiApplicationBuilderCertificationExt for WorthUiApplicationBuilder {
    fn with_graph_world_profile(
        self,
        graph_world_profile: UiGraphWorldProfile,
    ) -> WorthUiApplicationBuilder {
        WorthUiApplicationBuilder::with_graph_world_profile(self, graph_world_profile)
    }

    fn with_runtime_instance_basis_admissions(
        self,
        admissions: impl IntoIterator<Item = UiRuntimeInstanceBasisAdmission>,
    ) -> WorthUiApplicationBuilder {
        WorthUiApplicationBuilder::with_runtime_instance_basis_admissions(self, admissions)
    }
}
