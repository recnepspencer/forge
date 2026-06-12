use forge_query::facade::ForgeQueryWorkspace;

use crate::{
    ForgeServerDirectDeclarationSourceKind, ForgeServerQueryHandoffOperation,
    ForgeServerResolvedRequestContext,
};

#[derive(Clone, Debug)]
pub struct ForgeServerQueryWorkspaceBindingRequest {
    resolved_request_context: ForgeServerResolvedRequestContext,
    target: ForgeServerQueryWorkspaceBindingTarget,
}

impl ForgeServerQueryWorkspaceBindingRequest {
    pub(crate) fn for_query_handoff(
        resolved_request_context: ForgeServerResolvedRequestContext,
        operation: ForgeServerQueryHandoffOperation,
    ) -> Self {
        Self {
            resolved_request_context,
            target: ForgeServerQueryWorkspaceBindingTarget::QueryHandoff { operation },
        }
    }

    pub(crate) fn for_direct_declaration(
        resolved_request_context: ForgeServerResolvedRequestContext,
        source_kind: ForgeServerDirectDeclarationSourceKind,
        binding_label: impl Into<String>,
    ) -> Self {
        Self {
            resolved_request_context,
            target: ForgeServerQueryWorkspaceBindingTarget::DirectDeclaration {
                source_kind,
                binding_label: binding_label.into(),
            },
        }
    }

    pub fn resolved_request_context(&self) -> &ForgeServerResolvedRequestContext {
        &self.resolved_request_context
    }

    pub fn target(&self) -> &ForgeServerQueryWorkspaceBindingTarget {
        &self.target
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryWorkspaceBindingTarget {
    QueryHandoff {
        operation: ForgeServerQueryHandoffOperation,
    },
    DirectDeclaration {
        source_kind: ForgeServerDirectDeclarationSourceKind,
        binding_label: String,
    },
}

impl ForgeServerQueryWorkspaceBindingTarget {
    pub fn canonical_label(&self) -> String {
        match self {
            Self::QueryHandoff { operation } => operation.canonical_label(),
            Self::DirectDeclaration {
                source_kind,
                binding_label,
            } => format!(
                "direct-declaration:{}:{}",
                source_kind.as_str(),
                binding_label
            ),
        }
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
