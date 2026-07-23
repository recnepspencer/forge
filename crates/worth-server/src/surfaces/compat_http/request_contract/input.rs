use crate::request_context::DiagnosticRichnessProfile;

use super::WorthServerCompatHttpRouteFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerCompatibilityRequestInput {
    authenticated_principal_id: String,
    admitted_transport_caller: Option<crate::WorthServerAdmittedTransportCaller>,
    tenant_id: String,
    workspace_id: String,
    branch_target: RawWorthServerCompatibilityBranchTarget,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    route_family: WorthServerCompatHttpRouteFamily,
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    query_pairs: Vec<(String, String)>,
    body_content_type: Option<String>,
    body_present: bool,
}

impl WorthServerCompatibilityRequestInput {
    pub fn builder() -> WorthServerCompatibilityRequestInputBuilder {
        WorthServerCompatibilityRequestInputBuilder::default()
    }

    pub(crate) fn authenticated_principal_id(&self) -> &str {
        &self.authenticated_principal_id
    }

    pub(crate) fn admitted_transport_caller(
        &self,
    ) -> Option<&crate::WorthServerAdmittedTransportCaller> {
        self.admitted_transport_caller.as_ref()
    }

    pub(crate) fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub(crate) fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    pub(crate) fn branch_target(&self) -> &RawWorthServerCompatibilityBranchTarget {
        &self.branch_target
    }

    pub(crate) fn diagnostics_profile(&self) -> Option<DiagnosticRichnessProfile> {
        self.diagnostics_profile
    }

    pub(crate) fn route_family(&self) -> WorthServerCompatHttpRouteFamily {
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
pub struct WorthServerCompatibilityRequestInputBuilder {
    authenticated_principal_id: Option<String>,
    admitted_transport_caller: Option<crate::WorthServerAdmittedTransportCaller>,
    tenant_id: Option<String>,
    workspace_id: Option<String>,
    branch_target: Option<RawWorthServerCompatibilityBranchTarget>,
    diagnostics_profile: Option<DiagnosticRichnessProfile>,
    route_family: Option<WorthServerCompatHttpRouteFamily>,
    method: Option<String>,
    path: Option<String>,
    headers: Vec<(String, String)>,
    query_pairs: Vec<(String, String)>,
    body_content_type: Option<String>,
    body_present: bool,
}

impl Default for WorthServerCompatibilityRequestInputBuilder {
    fn default() -> Self {
        Self {
            authenticated_principal_id: None,
            admitted_transport_caller: None,
            tenant_id: None,
            workspace_id: None,
            branch_target: Some(RawWorthServerCompatibilityBranchTarget::Main),
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

impl WorthServerCompatibilityRequestInputBuilder {
    pub fn with_authenticated_principal_id(mut self, value: impl Into<String>) -> Self {
        self.authenticated_principal_id = Some(value.into());
        self
    }

    pub fn with_admitted_transport_caller(
        mut self,
        admitted_transport_caller: crate::WorthServerAdmittedTransportCaller,
    ) -> Self {
        self.authenticated_principal_id =
            Some(admitted_transport_caller.principal_identity().to_string());
        self.admitted_transport_caller = Some(admitted_transport_caller);
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
        self.branch_target = Some(RawWorthServerCompatibilityBranchTarget::Main);
        self
    }

    pub fn with_branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_target = Some(RawWorthServerCompatibilityBranchTarget::Branch {
            branch_id: value.into(),
        });
        self
    }

    pub fn with_preview_id(mut self, value: impl Into<String>) -> Self {
        self.branch_target = Some(RawWorthServerCompatibilityBranchTarget::Preview {
            preview_id: value.into(),
        });
        self
    }

    pub fn with_diagnostics_profile(mut self, value: DiagnosticRichnessProfile) -> Self {
        self.diagnostics_profile = Some(value);
        self
    }

    pub fn with_route_family(mut self, value: WorthServerCompatHttpRouteFamily) -> Self {
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
    ) -> Result<WorthServerCompatibilityRequestInput, WorthServerCompatibilityRequestInputError>
    {
        Ok(WorthServerCompatibilityRequestInput {
            authenticated_principal_id: self.authenticated_principal_id.ok_or(
                WorthServerCompatibilityRequestInputError::MissingAuthenticatedPrincipalId,
            )?,
            admitted_transport_caller: self.admitted_transport_caller,
            tenant_id: self
                .tenant_id
                .ok_or(WorthServerCompatibilityRequestInputError::MissingTenantId)?,
            workspace_id: self
                .workspace_id
                .ok_or(WorthServerCompatibilityRequestInputError::MissingWorkspaceId)?,
            branch_target: self
                .branch_target
                .ok_or(WorthServerCompatibilityRequestInputError::MissingBranchTarget)?,
            diagnostics_profile: self.diagnostics_profile,
            route_family: self
                .route_family
                .ok_or(WorthServerCompatibilityRequestInputError::MissingRouteFamily)?,
            method: self
                .method
                .ok_or(WorthServerCompatibilityRequestInputError::MissingMethod)?,
            path: self
                .path
                .ok_or(WorthServerCompatibilityRequestInputError::MissingPath)?,
            headers: self.headers,
            query_pairs: self.query_pairs,
            body_content_type: self.body_content_type,
            body_present: self.body_present,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerCompatibilityRequestInputError {
    MissingAuthenticatedPrincipalId,
    MissingTenantId,
    MissingWorkspaceId,
    MissingBranchTarget,
    MissingRouteFamily,
    MissingMethod,
    MissingPath,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RawWorthServerCompatibilityBranchTarget {
    Main,
    Branch { branch_id: String },
    Preview { preview_id: String },
}
