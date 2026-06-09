use crate::request_context::DiagnosticRichnessProfile;

use super::ForgeServerCompatHttpRouteFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityRequestInput {
    authenticated_principal_id: String,
    tenant_id: String,
    workspace_id: String,
    branch_target: RawForgeServerCompatibilityBranchTarget,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    route_family: ForgeServerCompatHttpRouteFamily,
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    query_pairs: Vec<(String, String)>,
    body_content_type: Option<String>,
    body_present: bool,
}

impl ForgeServerCompatibilityRequestInput {
    pub fn builder() -> ForgeServerCompatibilityRequestInputBuilder {
        ForgeServerCompatibilityRequestInputBuilder::default()
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

    pub(crate) fn branch_target(&self) -> &RawForgeServerCompatibilityBranchTarget {
        &self.branch_target
    }

    pub(crate) fn diagnostics_profile(&self) -> Option<DiagnosticRichnessProfile> {
        self.diagnostics_profile
    }

    pub(crate) fn route_family(&self) -> ForgeServerCompatHttpRouteFamily {
        self.route_family
    }

    pub(crate) fn method(&self) -> &str {
        &self.method
    }

    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    pub(crate) fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    pub(crate) fn query_pairs(&self) -> &[(String, String)] {
        &self.query_pairs
    }

    pub(crate) fn body_content_type(&self) -> Option<&str> {
        self.body_content_type.as_deref()
    }

    pub(crate) fn body_present(&self) -> bool {
        self.body_present
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityRequestInputBuilder {
    authenticated_principal_id: Option<String>,
    tenant_id: Option<String>,
    workspace_id: Option<String>,
    branch_target: Option<RawForgeServerCompatibilityBranchTarget>,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    route_family: Option<ForgeServerCompatHttpRouteFamily>,
    method: Option<String>,
    path: Option<String>,
    headers: Vec<(String, String)>,
    query_pairs: Vec<(String, String)>,
    body_content_type: Option<String>,
    body_present: bool,
}

impl Default for ForgeServerCompatibilityRequestInputBuilder {
    fn default() -> Self {
        Self {
            authenticated_principal_id: None,
            tenant_id: None,
            workspace_id: None,
            branch_target: Some(RawForgeServerCompatibilityBranchTarget::Main),
            diagnostics_profile: None,
            route_family: None,
            method: None,
            path: None,
            headers: Vec::new(),
            query_pairs: Vec::new(),
            body_content_type: None,
            body_present: false,
        }
    }
}

impl ForgeServerCompatibilityRequestInputBuilder {
    pub fn with_authenticated_principal_id(mut self, value: impl Into<String>) -> Self {
        self.authenticated_principal_id = Some(value.into());
        self
    }

    pub fn with_tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn with_workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn with_main_branch(mut self) -> Self {
        self.branch_target = Some(RawForgeServerCompatibilityBranchTarget::Main);
        self
    }

    pub fn with_branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_target = Some(RawForgeServerCompatibilityBranchTarget::Branch {
            branch_id: value.into(),
        });
        self
    }

    pub fn with_preview_id(mut self, value: impl Into<String>) -> Self {
        self.branch_target = Some(RawForgeServerCompatibilityBranchTarget::Preview {
            preview_id: value.into(),
        });
        self
    }

    pub fn with_diagnostics_profile(mut self, value: DiagnosticRichnessProfile) -> Self {
        self.diagnostics_profile = Some(value);
        self
    }

    pub fn with_route_family(mut self, value: ForgeServerCompatHttpRouteFamily) -> Self {
        self.route_family = Some(value);
        self
    }

    pub fn with_method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn with_path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn with_query_pair(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_pairs.push((name.into(), value.into()));
        self
    }

    pub fn with_body_content_type(mut self, value: impl Into<String>) -> Self {
        self.body_content_type = Some(value.into());
        self
    }

    pub fn with_body_present(mut self, value: bool) -> Self {
        self.body_present = value;
        self
    }

    pub fn build(
        self,
    ) -> Result<ForgeServerCompatibilityRequestInput, ForgeServerCompatibilityRequestInputError>
    {
        Ok(ForgeServerCompatibilityRequestInput {
            authenticated_principal_id: self.authenticated_principal_id.ok_or(
                ForgeServerCompatibilityRequestInputError::MissingAuthenticatedPrincipalId,
            )?,
            tenant_id: self
                .tenant_id
                .ok_or(ForgeServerCompatibilityRequestInputError::MissingTenantId)?,
            workspace_id: self
                .workspace_id
                .ok_or(ForgeServerCompatibilityRequestInputError::MissingWorkspaceId)?,
            branch_target: self
                .branch_target
                .ok_or(ForgeServerCompatibilityRequestInputError::MissingBranchTarget)?,
            diagnostics_profile: self.diagnostics_profile,
            route_family: self
                .route_family
                .ok_or(ForgeServerCompatibilityRequestInputError::MissingRouteFamily)?,
            method: self
                .method
                .ok_or(ForgeServerCompatibilityRequestInputError::MissingMethod)?,
            path: self
                .path
                .ok_or(ForgeServerCompatibilityRequestInputError::MissingPath)?,
            headers: self.headers,
            query_pairs: self.query_pairs,
            body_content_type: self.body_content_type,
            body_present: self.body_present,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerCompatibilityRequestInputError {
    MissingAuthenticatedPrincipalId,
    MissingTenantId,
    MissingWorkspaceId,
    MissingBranchTarget,
    MissingRouteFamily,
    MissingMethod,
    MissingPath,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RawForgeServerCompatibilityBranchTarget {
    Main,
    Branch { branch_id: String },
    Preview { preview_id: String },
}
