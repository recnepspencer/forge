#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerTransportDenialCode {
    MissingAuthenticatedPrincipalId,
    MissingTenantId,
    MissingWorkspaceId,
    UnsupportedContentType,
    OversizedBody,
    MalformedJson,
    MissingProductSessionIdentity,
    MissingBranchTarget,
    UnknownRoute,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerTransportDenial {
    code: WorthServerTransportDenialCode,
    detail: String,
}

impl WorthServerTransportDenial {
    pub(crate) fn new(code: WorthServerTransportDenialCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> WorthServerTransportDenialCode {
        self.code.clone()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
