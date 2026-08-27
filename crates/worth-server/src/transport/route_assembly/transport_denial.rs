#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerTransportDenialCode {
    CallerAdmissionDenied,
    MissingAuthenticatedPrincipalId,
    MissingTenantId,
    MissingWorkspaceId,
    UnsupportedContentType,
    OversizedBody,
    MalformedJson,
    InvalidIdempotencyKey,
    MissingProductSessionIdentity,
    MissingBranchTarget,
    RouteExecutionFailed,
    UnknownRoute,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerTransportDenial {
    code: WorthServerTransportDenialCode,
    reason_key: Option<String>,
    detail: String,
}

impl WorthServerTransportDenial {
    pub(crate) fn new(code: WorthServerTransportDenialCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            reason_key: None,
            detail: detail.into(),
        }
    }

    pub(crate) fn with_reason_key(mut self, reason_key: impl Into<String>) -> Self {
        self.reason_key = Some(reason_key.into());
        self
    }

    pub fn code(&self) -> WorthServerTransportDenialCode {
        self.code.clone()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn reason_key(&self) -> Option<&str> {
        self.reason_key.as_deref()
    }
}
