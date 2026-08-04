use crate::request_context::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerWorthNativeSessionInput {
    authenticated_principal_id: String,
    tenant_id: String,
    workspace_id: String,
    application_authority_proof_identity: Option<String>,
    branch_target: RawWorthServerWorthNativeBranchTarget,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
}

impl WorthServerWorthNativeSessionInput {
    pub fn builder() -> WorthServerWorthNativeSessionInputBuilder {
        WorthServerWorthNativeSessionInputBuilder::default()
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

    pub(crate) fn application_authority_proof_identity(&self) -> Option<&str> {
        self.application_authority_proof_identity.as_deref()
    }

    pub(crate) fn branch_target(&self) -> &RawWorthServerWorthNativeBranchTarget {
        &self.branch_target
    }

    pub(crate) fn diagnostics_profile(&self) -> Option<DiagnosticRichnessProfile> {
        self.diagnostics_profile
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerWorthNativeSessionInputBuilder {
    authenticated_principal_id: Option<String>,
    tenant_id: Option<String>,
    workspace_id: Option<String>,
    application_authority_proof_identity: Option<String>,
    branch_target: Option<RawWorthServerWorthNativeBranchTarget>,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
}

impl Default for WorthServerWorthNativeSessionInputBuilder {
    fn default() -> Self {
        Self {
            authenticated_principal_id: None,
            tenant_id: None,
            workspace_id: None,
            application_authority_proof_identity: None,
            branch_target: Some(RawWorthServerWorthNativeBranchTarget::Main),
            diagnostics_profile: None,
        }
    }
}

impl WorthServerWorthNativeSessionInputBuilder {
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

    pub fn with_application_authority_proof_identity(
        mut self,
        proof_identity: impl Into<String>,
    ) -> Self {
        self.application_authority_proof_identity = Some(proof_identity.into());
        self
    }

    pub fn with_main_branch(mut self) -> Self {
        self.branch_target = Some(RawWorthServerWorthNativeBranchTarget::Main);
        self
    }

    pub fn with_branch_id(mut self, branch_id: impl Into<String>) -> Self {
        self.branch_target = Some(RawWorthServerWorthNativeBranchTarget::Branch {
            branch_id: branch_id.into(),
        });
        self
    }

    pub fn with_preview_id(mut self, preview_id: impl Into<String>) -> Self {
        self.branch_target = Some(RawWorthServerWorthNativeBranchTarget::Preview {
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
    ) -> Result<WorthServerWorthNativeSessionInput, WorthServerWorthNativeSessionInputError> {
        Ok(WorthServerWorthNativeSessionInput {
            authenticated_principal_id: self
                .authenticated_principal_id
                .ok_or(WorthServerWorthNativeSessionInputError::MissingAuthenticatedPrincipalId)?,
            tenant_id: self
                .tenant_id
                .ok_or(WorthServerWorthNativeSessionInputError::MissingTenantId)?,
            workspace_id: self
                .workspace_id
                .ok_or(WorthServerWorthNativeSessionInputError::MissingWorkspaceId)?,
            application_authority_proof_identity: self.application_authority_proof_identity,
            branch_target: self
                .branch_target
                .ok_or(WorthServerWorthNativeSessionInputError::MissingBranchTarget)?,
            diagnostics_profile: self.diagnostics_profile,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerWorthNativeSessionInputError {
    MissingAuthenticatedPrincipalId,
    MissingTenantId,
    MissingWorkspaceId,
    MissingBranchTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RawWorthServerWorthNativeBranchTarget {
    Main,
    Branch { branch_id: String },
    Preview { preview_id: String },
}
