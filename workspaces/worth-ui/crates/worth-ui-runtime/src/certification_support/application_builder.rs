use crate::facade::entry::{UiApplicationHostBound, UiApplicationHostUnbound};
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

impl<ChangeProfileState, IntentWiringState, HostBindingState>
    WorthUiApplicationBuilderCertificationExt
    for WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState, HostBindingState>
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

impl<ChangeProfileState, IntentWiringState>
    WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState, UiApplicationHostUnbound>
{
    #[allow(dead_code)]
    pub(crate) fn bind_certification_host(
        self,
    ) -> WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState, UiApplicationHostBound>
    {
        self.bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            super::UiCertificationBuilderHost,
        )
    }
}
