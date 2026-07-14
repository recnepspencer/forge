use worth_query::facade::runtime::WorthQueryWorkspace;

use crate::{
    WorthServerDirectDeclarationSourceKind, WorthServerQueryHandoffOperation,
    WorthServerResolvedRequestContext,
};

#[derive(Clone, Debug)]
pub struct WorthServerQueryWorkspaceBindingRequest {
    resolved_request_context: WorthServerResolvedRequestContext,
    target: WorthServerQueryWorkspaceBindingTarget,
}

impl WorthServerQueryWorkspaceBindingRequest {
    pub(crate) fn for_query_handoff(
        resolved_request_context: WorthServerResolvedRequestContext,
        operation: WorthServerQueryHandoffOperation,
    ) -> Self {
        Self {
            resolved_request_context,
            target: WorthServerQueryWorkspaceBindingTarget::QueryHandoff { operation },
        }
    }

    pub(crate) fn for_direct_declaration(
        resolved_request_context: WorthServerResolvedRequestContext,
        source_kind: WorthServerDirectDeclarationSourceKind,
        binding_label: impl Into<String>,
    ) -> Self {
        Self {
            resolved_request_context,
            target: WorthServerQueryWorkspaceBindingTarget::DirectDeclaration {
                source_kind,
                binding_label: binding_label.into(),
            },
        }
    }

    pub fn resolved_request_context(&self) -> &WorthServerResolvedRequestContext {
        &self.resolved_request_context
    }

    pub fn target(&self) -> &WorthServerQueryWorkspaceBindingTarget {
        &self.target
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerQueryWorkspaceBindingTarget {
    QueryHandoff {
        operation: WorthServerQueryHandoffOperation,
    },
    DirectDeclaration {
        source_kind: WorthServerDirectDeclarationSourceKind,
        binding_label: String,
    },
}

impl WorthServerQueryWorkspaceBindingTarget {
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

pub trait WorthServerQueryWorkspaceProvider: Send + Sync + 'static {
    fn provider_name(&self) -> &'static str;

    fn bind_workspace(
        &self,
        request: &WorthServerQueryWorkspaceBindingRequest,
    ) -> Result<WorthQueryWorkspace, WorthServerQueryWorkspaceBindingError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerQueryWorkspaceBindingError {
    stage: &'static str,
    message: String,
}

impl WorthServerQueryWorkspaceBindingError {
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

impl WorthServerQueryWorkspaceProvider for UnavailableWorkspaceProvider {
    fn provider_name(&self) -> &'static str {
        "unavailable"
    }

    fn bind_workspace(
        &self,
        _request: &WorthServerQueryWorkspaceBindingRequest,
    ) -> Result<WorthQueryWorkspace, WorthServerQueryWorkspaceBindingError> {
        Err(WorthServerQueryWorkspaceBindingError::new(
            "workspace_provider",
            "no WORTH Query workspace provider has been configured",
        ))
    }
}
