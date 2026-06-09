use crate::request_context::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerForgeNativeSessionInput {
    authenticated_principal_id: String,
    tenant_id: String,
    workspace_id: String,
    branch_target: RawForgeServerForgeNativeBranchTarget,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
}

impl ForgeServerForgeNativeSessionInput {
    pub fn builder() -> ForgeServerForgeNativeSessionInputBuilder {
        ForgeServerForgeNativeSessionInputBuilder::default()
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

    pub(crate) fn branch_target(&self) -> &RawForgeServerForgeNativeBranchTarget {
        &self.branch_target
    }

    pub(crate) fn diagnostics_profile(&self) -> Option<DiagnosticRichnessProfile> {
        self.diagnostics_profile
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerForgeNativeSessionInputBuilder {
    authenticated_principal_id: Option<String>,
    tenant_id: Option<String>,
    workspace_id: Option<String>,
    branch_target: Option<RawForgeServerForgeNativeBranchTarget>,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
}

impl Default for ForgeServerForgeNativeSessionInputBuilder {
    fn default() -> Self {
        Self {
            authenticated_principal_id: None,
            tenant_id: None,
            workspace_id: None,
            branch_target: Some(RawForgeServerForgeNativeBranchTarget::Main),
            diagnostics_profile: None,
        }
    }
}

impl ForgeServerForgeNativeSessionInputBuilder {
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
        self.branch_target = Some(RawForgeServerForgeNativeBranchTarget::Main);
        self
    }

    pub fn with_branch_id(mut self, branch_id: impl Into<String>) -> Self {
        self.branch_target = Some(RawForgeServerForgeNativeBranchTarget::Branch {
            branch_id: branch_id.into(),
        });
        self
    }

    pub fn with_preview_id(mut self, preview_id: impl Into<String>) -> Self {
        self.branch_target = Some(RawForgeServerForgeNativeBranchTarget::Preview {
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
    ) -> Result<ForgeServerForgeNativeSessionInput, ForgeServerForgeNativeSessionInputError> {
        Ok(ForgeServerForgeNativeSessionInput {
            authenticated_principal_id: self
                .authenticated_principal_id
                .ok_or(ForgeServerForgeNativeSessionInputError::MissingAuthenticatedPrincipalId)?,
            tenant_id: self
                .tenant_id
                .ok_or(ForgeServerForgeNativeSessionInputError::MissingTenantId)?,
            workspace_id: self
                .workspace_id
                .ok_or(ForgeServerForgeNativeSessionInputError::MissingWorkspaceId)?,
            branch_target: self
                .branch_target
                .ok_or(ForgeServerForgeNativeSessionInputError::MissingBranchTarget)?,
            diagnostics_profile: self.diagnostics_profile,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerForgeNativeSessionInputError {
    MissingAuthenticatedPrincipalId,
    MissingTenantId,
    MissingWorkspaceId,
    MissingBranchTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RawForgeServerForgeNativeBranchTarget {
    Main,
    Branch { branch_id: String },
    Preview { preview_id: String },
}
