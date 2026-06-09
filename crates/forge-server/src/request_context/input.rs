use crate::ForgeServerSurfaceFamily;

use super::{DiagnosticRichnessProfile, ForgeServerTransportClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContextInput {
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
    authenticated_principal_id: String,
    tenant_id: String,
    workspace_id: String,
    branch_target: RawForgeServerBranchTarget,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
}

impl ForgeServerRequestContextInput {
    pub fn builder() -> ForgeServerRequestContextInputBuilder {
        ForgeServerRequestContextInputBuilder::default()
    }

    pub(crate) fn surface_family(&self) -> ForgeServerSurfaceFamily {
        self.surface_family
    }

    pub(crate) fn transport_class(&self) -> ForgeServerTransportClass {
        self.transport_class
    }

    pub(crate) fn authenticated_principal_id(&self) -> &str {
        &self.authenticated_principal_id
    }

    pub(crate) fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub(crate) fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    pub(crate) fn branch_target(&self) -> &RawForgeServerBranchTarget {
        &self.branch_target
    }

    pub(crate) fn diagnostics_profile(&self) -> Option<DiagnosticRichnessProfile> {
        self.diagnostics_profile
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContextInputBuilder {
    surface_family: Option<ForgeServerSurfaceFamily>,
    transport_class: Option<ForgeServerTransportClass>,
    authenticated_principal_id: Option<String>,
    tenant_id: Option<String>,
    workspace_id: Option<String>,
    branch_target: Option<RawForgeServerBranchTarget>,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
}

impl Default for ForgeServerRequestContextInputBuilder {
    fn default() -> Self {
        Self {
            surface_family: None,
            transport_class: None,
            authenticated_principal_id: None,
            tenant_id: None,
            workspace_id: None,
            branch_target: Some(RawForgeServerBranchTarget::Main),
            diagnostics_profile: None,
        }
    }
}

impl ForgeServerRequestContextInputBuilder {
    pub fn with_surface_family(mut self, surface_family: ForgeServerSurfaceFamily) -> Self {
        self.surface_family = Some(surface_family);
        self
    }

    pub fn with_transport_class(mut self, transport_class: ForgeServerTransportClass) -> Self {
        self.transport_class = Some(transport_class);
        self
    }

    pub fn with_authenticated_principal_id(
        mut self,
        authenticated_principal_id: impl Into<String>,
    ) -> Self {
        self.authenticated_principal_id = Some(authenticated_principal_id.into());
        self
    }

    pub fn with_tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    pub fn with_workspace_id(mut self, workspace_id: impl Into<String>) -> Self {
        self.workspace_id = Some(workspace_id.into());
        self
    }

    pub fn with_main_branch(mut self) -> Self {
        self.branch_target = Some(RawForgeServerBranchTarget::Main);
        self
    }

    pub fn with_branch_id(mut self, branch_id: impl Into<String>) -> Self {
        self.branch_target = Some(RawForgeServerBranchTarget::Branch {
            branch_id: branch_id.into(),
        });
        self
    }

    pub fn with_preview_id(mut self, preview_id: impl Into<String>) -> Self {
        self.branch_target = Some(RawForgeServerBranchTarget::Preview {
            preview_id: preview_id.into(),
        });
        self
    }

    pub fn with_diagnostics_profile(
        mut self,
        diagnostics_profile: DiagnosticRichnessProfile,
    ) -> Self {
        self.diagnostics_profile = Some(diagnostics_profile);
        self
    }

    pub fn build(
        self,
    ) -> Result<ForgeServerRequestContextInput, ForgeServerRequestContextInputError> {
        Ok(ForgeServerRequestContextInput {
            surface_family: self
                .surface_family
                .ok_or(ForgeServerRequestContextInputError::MissingSurfaceFamily)?,
            transport_class: self
                .transport_class
                .ok_or(ForgeServerRequestContextInputError::MissingTransportClass)?,
            authenticated_principal_id: self
                .authenticated_principal_id
                .ok_or(ForgeServerRequestContextInputError::MissingAuthenticatedPrincipalId)?,
            tenant_id: self
                .tenant_id
                .ok_or(ForgeServerRequestContextInputError::MissingTenantId)?,
            workspace_id: self
                .workspace_id
                .ok_or(ForgeServerRequestContextInputError::MissingWorkspaceId)?,
            branch_target: self
                .branch_target
                .ok_or(ForgeServerRequestContextInputError::MissingBranchTarget)?,
            diagnostics_profile: self.diagnostics_profile,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerRequestContextInputError {
    MissingSurfaceFamily,
    MissingTransportClass,
    MissingAuthenticatedPrincipalId,
    MissingTenantId,
    MissingWorkspaceId,
    MissingBranchTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RawForgeServerBranchTarget {
    Main,
    Branch { branch_id: String },
    Preview { preview_id: String },
}
