use crate::facade::WorthUiApplicationBuilder;
use crate::graph::{UiGraphWorldProfile, UiRuntimeInstanceBasisAdmission};

/// Certification-only configuration of synthetic graph-world authority.
pub trait WorthUiApplicationBuilderCertificationExt {
    fn with_graph_world_profile(self, graph_world_profile: UiGraphWorldProfile) -> Self;

    fn with_runtime_instance_basis_admissions(
        self,
        admissions: impl IntoIterator<Item = UiRuntimeInstanceBasisAdmission>,
    ) -> Self;
}

impl<ChangeProfileState, IntentWiringState> WorthUiApplicationBuilderCertificationExt
    for WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState>
{
    fn with_graph_world_profile(self, graph_world_profile: UiGraphWorldProfile) -> Self {
        WorthUiApplicationBuilder::with_graph_world_profile(self, graph_world_profile)
    }

    fn with_runtime_instance_basis_admissions(
        self,
        admissions: impl IntoIterator<Item = UiRuntimeInstanceBasisAdmission>,
    ) -> Self {
        WorthUiApplicationBuilder::with_runtime_instance_basis_admissions(self, admissions)
    }
}
