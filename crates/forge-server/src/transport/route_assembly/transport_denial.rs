#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerTransportDenialCode {
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
pub struct ForgeServerTransportDenial {
    code: ForgeServerTransportDenialCode,
    detail: String,
}

impl ForgeServerTransportDenial {
    pub(crate) fn new(code: ForgeServerTransportDenialCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerTransportDenialCode {
        self.code.clone()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
