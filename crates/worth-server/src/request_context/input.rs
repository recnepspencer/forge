use crate::WorthServerSurfaceFamily;

use super::{DiagnosticRichnessProfile, WorthServerTransportClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRequestContextInput {
    surface_family: WorthServerSurfaceFamily,
    transport_class: WorthServerTransportClass,
    authenticated_principal_id: String,
    admitted_transport_caller: Option<crate::WorthServerAdmittedTransportCaller>,
    application_authority_proof_identity: Option<String>,
    tenant_id: String,
    workspace_id: String,
    branch_target: RawWorthServerBranchTarget,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
}

impl WorthServerRequestContextInput {
    pub fn builder() -> WorthServerRequestContextInputBuilder {
        WorthServerRequestContextInputBuilder::default()
    }

    pub(crate) fn surface_family(&self) -> WorthServerSurfaceFamily {
        self.surface_family
    }

    pub(crate) fn transport_class(&self) -> WorthServerTransportClass {
        self.transport_class
    }

    pub(crate) fn authenticated_principal_id(&self) -> &str {
        &self.authenticated_principal_id
    }

    pub(crate) fn admitted_transport_caller(
        &self,
    ) -> Option<&crate::WorthServerAdmittedTransportCaller> {
        self.admitted_transport_caller.as_ref()
    }

    pub(crate) fn application_authority_proof_identity(&self) -> Option<&str> {
        self.application_authority_proof_identity.as_deref()
    }

    pub(crate) fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub(crate) fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    pub(crate) fn branch_target(&self) -> &RawWorthServerBranchTarget {
        &self.branch_target
    }

    pub(crate) fn diagnostics_profile(&self) -> Option<DiagnosticRichnessProfile> {
        self.diagnostics_profile
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRequestContextInputBuilder {
    surface_family: Option<WorthServerSurfaceFamily>,
    transport_class: Option<WorthServerTransportClass>,
    authenticated_principal_id: Option<String>,
    admitted_transport_caller: Option<crate::WorthServerAdmittedTransportCaller>,
    application_authority_proof_identity: Option<String>,
    tenant_id: Option<String>,
    workspace_id: Option<String>,
    branch_target: Option<RawWorthServerBranchTarget>,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
}

impl Default for WorthServerRequestContextInputBuilder {
    fn default() -> Self {
        Self {
            surface_family: None,
            transport_class: None,
            authenticated_principal_id: None,
            admitted_transport_caller: None,
            application_authority_proof_identity: None,
            tenant_id: None,
            workspace_id: None,
            branch_target: Some(RawWorthServerBranchTarget::Main),
            diagnostics_profile: None,
        }
    }
}

impl WorthServerRequestContextInputBuilder {
    pub fn with_surface_family(mut self, surface_family: WorthServerSurfaceFamily) -> Self {
        self.surface_family = Some(surface_family);
        self
    }

    pub fn with_transport_class(mut self, transport_class: WorthServerTransportClass) -> Self {
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

    pub fn with_admitted_transport_caller(
        mut self,
        admitted_transport_caller: crate::WorthServerAdmittedTransportCaller,
    ) -> Self {
        self.authenticated_principal_id =
            Some(admitted_transport_caller.principal_identity().to_string());
        self.application_authority_proof_identity =
            Some(admitted_transport_caller.authority_identity().to_string());
        self.admitted_transport_caller = Some(admitted_transport_caller);
        self
    }

    pub fn with_application_authority_proof_identity(
        mut self,
        proof_identity: impl Into<String>,
    ) -> Self {
        self.application_authority_proof_identity = Some(proof_identity.into());
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
        self.branch_target = Some(RawWorthServerBranchTarget::Main);
        self
    }

    pub fn with_branch_id(mut self, branch_id: impl Into<String>) -> Self {
        self.branch_target = Some(RawWorthServerBranchTarget::Branch {
            branch_id: branch_id.into(),
        });
        self
    }

    pub fn with_preview_id(mut self, preview_id: impl Into<String>) -> Self {
        self.branch_target = Some(RawWorthServerBranchTarget::Preview {
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
    ) -> Result<WorthServerRequestContextInput, WorthServerRequestContextInputError> {
        Ok(WorthServerRequestContextInput {
            surface_family: self
                .surface_family
                .ok_or(WorthServerRequestContextInputError::MissingSurfaceFamily)?,
            transport_class: self
                .transport_class
                .ok_or(WorthServerRequestContextInputError::MissingTransportClass)?,
            authenticated_principal_id: self
                .authenticated_principal_id
                .ok_or(WorthServerRequestContextInputError::MissingAuthenticatedPrincipalId)?,
            admitted_transport_caller: self.admitted_transport_caller,
            application_authority_proof_identity: self.application_authority_proof_identity,
            tenant_id: self
                .tenant_id
                .ok_or(WorthServerRequestContextInputError::MissingTenantId)?,
            workspace_id: self
                .workspace_id
                .ok_or(WorthServerRequestContextInputError::MissingWorkspaceId)?,
            branch_target: self
                .branch_target
                .ok_or(WorthServerRequestContextInputError::MissingBranchTarget)?,
            diagnostics_profile: self.diagnostics_profile,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerRequestContextInputError {
    MissingSurfaceFamily,
    MissingTransportClass,
    MissingAuthenticatedPrincipalId,
    MissingTenantId,
    MissingWorkspaceId,
    MissingBranchTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RawWorthServerBranchTarget {
    Main,
    Branch { branch_id: String },
    Preview { preview_id: String },
}
