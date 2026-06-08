use forge_query::facade::ForgeQueryWorkspace;

use crate::ForgeServerResolvedRequestContext;

use super::ForgeServerQueryHandoffOperation;

#[derive(Clone, Debug)]
pub struct ForgeServerQueryWorkspaceBindingRequest {
    resolved_request_context: ForgeServerResolvedRequestContext,
    operation: ForgeServerQueryHandoffOperation,
}

impl ForgeServerQueryWorkspaceBindingRequest {
    pub(crate) fn new(
        resolved_request_context: ForgeServerResolvedRequestContext,
        operation: ForgeServerQueryHandoffOperation,
    ) -> Self {
        Self {
            resolved_request_context,
            operation,
        }
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        &self.resolved_request_context
    }

    pub fn operation(&self) -> &ForgeServerQueryHandoffOperation {
        &self.operation
    }
}

pub trait ForgeServerQueryWorkspaceProvider: Send + Sync + 'static {
    fn provider_name(&self) -> &'static str;

    fn bind_workspace(
        &self,
        request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryWorkspaceBindingError {
    stage: &'static str,
    message: String,
}

impl ForgeServerQueryWorkspaceBindingError {
    pub fn new(stage: &'static str, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
        }
    }

    pub fn stage(&self) -> &'static str {
        self.stage
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Clone, Debug)]
pub struct UnavailableWorkspaceProvider;

impl ForgeServerQueryWorkspaceProvider for UnavailableWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "unavailable"
    }

    fn bind_workspace(
        &self,
        _request: &ForgeServerQueryWorkspaceBindingRequest,
    ) -> Result<ForgeQueryWorkspace, ForgeServerQueryWorkspaceBindingError> {
        Err(ForgeServerQueryWorkspaceBindingError::new(
            "workspace_provider",
            "no Forge Query workspace provider has been configured",
        ))
    }
}
