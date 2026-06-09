use std::{fmt, sync::Arc};

use crate::query_handoff::{ForgeServerQueryWorkspaceProvider, UnavailableWorkspaceProvider};

#[derive(Clone)]
pub struct ForgeServerQueryHandoffConfig {
    workspace_provider: Arc<dyn ForgeServerQueryWorkspaceProvider>,
}

impl ForgeServerQueryHandoffConfig {
    pub fn builder() -> ForgeServerQueryHandoffConfigBuilder {
        ForgeServerQueryHandoffConfigBuilder::default()
    }

    pub fn workspace_provider(&self) -> &Arc<dyn ForgeServerQueryWorkspaceProvider> {
        &self.workspace_provider
    }
}

impl fmt::Debug for ForgeServerQueryHandoffConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForgeServerQueryHandoffConfig")
            .field(
                "workspace_provider",
                &self.workspace_provider.provider_name(),
            )
            .finish()
    }
}

#[derive(Clone)]
pub struct ForgeServerQueryHandoffConfigBuilder {
    workspace_provider: Arc<dyn ForgeServerQueryWorkspaceProvider>,
}

impl Default for ForgeServerQueryHandoffConfigBuilder {
    fn default() -> Self {
        Self {
            workspace_provider: Arc::new(UnavailableWorkspaceProvider),
        }
    }
}

impl fmt::Debug for ForgeServerQueryHandoffConfigBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForgeServerQueryHandoffConfigBuilder")
            .field(
                "workspace_provider",
                &self.workspace_provider.provider_name(),
            )
            .finish()
    }
}

impl ForgeServerQueryHandoffConfigBuilder {
    pub fn with_workspace_provider(
        mut self,
        workspace_provider: impl ForgeServerQueryWorkspaceProvider,
    ) -> Self {
        self.workspace_provider = Arc::new(workspace_provider);
        self
    }

    pub fn with_workspace_provider_arc(
        mut self,
        workspace_provider: Arc<dyn ForgeServerQueryWorkspaceProvider>,
    ) -> Self {
        self.workspace_provider = workspace_provider;
        self
    }

    pub fn build(
        self,
    ) -> Result<ForgeServerQueryHandoffConfig, ForgeServerQueryHandoffConfigError> {
        Ok(ForgeServerQueryHandoffConfig {
            workspace_provider: self.workspace_provider,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryHandoffConfigError {}
