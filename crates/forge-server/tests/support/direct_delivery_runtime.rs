use forge_query::facade::{ForgeQueryRuntimeSupportProfile, ForgeQueryWorkspace};
use forge_server::{
    ForgeServerQueryWorkspaceBindingError, ForgeServerQueryWorkspaceBindingRequest,
    ForgeServerQueryWorkspaceBindingTarget, ForgeServerQueryWorkspaceProvider,
};

use crate::query_handoff_runtime::{ProfiledTestWorkspaceProvider, TestWorkspaceProvider};

#[derive(Clone, Debug)]
pub(crate) struct DeclarationAdmitsButLiveDeliveryDeniesProvider {
    handoff_provider: ProfiledTestWorkspaceProvider,
}

impl DeclarationAdmitsButLiveDeliveryDeniesProvider {
    pub(crate) fn new(handoff_profile: ForgeQueryRuntimeSupportProfile) -> Self {
        Self {
            handoff_provider: ProfiledTestWorkspaceProvider::new(handoff_profile),
        }
    }
}

impl ForgeServerQueryWorkspaceProvider for DeclarationAdmitsButLiveDeliveryDeniesProvider {
    fn provider_name(&self) -> &'static str {
        "declaration-admits-but-live-delivery-denies-provider"
    }

    fn bind_workspace(
        &self,
        request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError> {
        match request.target() {
            ForgeServerQueryWorkspaceBindingTarget::DirectDeclaration { .. } => {
                TestWorkspaceProvider.bind_workspace(request)
            }
            ForgeServerQueryWorkspaceBindingTarget::QueryHandoff { .. } => {
                self.handoff_provider.bind_workspace(request)
            }
        }
    }
}
