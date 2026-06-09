use forge_proof::TransitionOutcome;

use crate::{
    ForgeServerAdmission, ForgeServerMiddlewareDeferred, ForgeServerMiddlewareFailure,
    ForgeServerMiddlewareRebindRequired, ForgeServerMiddlewareStale,
    ForgeServerRequestContextDeferred, ForgeServerRequestContextFailure,
    ForgeServerRequestContextRebindRequired, ForgeServerRequestContextStale,
};

use crate::ForgeServerExternalRequestContract;

pub type ForgeServerCompatibilityRequestOutcome = TransitionOutcome<
    ForgeServerCompatibilityRequest,
    ForgeServerCompatibilityDenial,
    ForgeServerCompatibilityDeferred,
    ForgeServerCompatibilityStale,
    ForgeServerCompatibilityRebindRequired,
    ForgeServerCompatibilityFailure,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityPreparedRequest {
    admission: ForgeServerAdmission,
    request_contract: ForgeServerExternalRequestContract,
}

impl ForgeServerCompatibilityPreparedRequest {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        request_contract: ForgeServerExternalRequestContract,
    ) -> Self {
        Self {
            admission,
            request_contract,
        }
    }

    pub fn admission(&self) -> &ForgeServerAdmission {
        &self.admission
    }

    pub fn request_contract(&self) -> &ForgeServerExternalRequestContract {
        &self.request_contract
    }

    pub fn request_context_digest(&self) -> String {
        let request_context = self.admission.request_context();
        format!(
            "surface={:?};transport={:?};principal={};tenant={};workspace={};branch={:?};diagnostics={:?}",
            self.admission.resolved_request_context().surface_family(),
            self.admission.resolved_request_context().transport_class(),
            request_context.authenticated_principal().principal_id(),
            request_context.workspace_target().tenant_id(),
            request_context.workspace_target().workspace_id(),
            request_context.branch_target(),
            request_context.diagnostics_profile(),
        )
    }

    pub fn into_request(self) -> ForgeServerCompatibilityRequest {
        ForgeServerCompatibilityRequest {
            admission: self.admission,
            request_contract: self.request_contract,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityRequest {
    admission: ForgeServerAdmission,
    request_contract: ForgeServerExternalRequestContract,
}

impl ForgeServerCompatibilityRequest {
    pub fn admission(&self) -> &ForgeServerAdmission {
        &self.admission
    }

    pub fn request_contract(&self) -> &ForgeServerExternalRequestContract {
        &self.request_contract
    }

    pub fn request_context_digest(&self) -> String {
        ForgeServerCompatibilityPreparedRequest::new(
            self.admission.clone(),
            self.request_contract.clone(),
        )
        .request_context_digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerCompatibilityDenialCode {
    CompatHttpSurfaceAbsent,
    CompatHttpSurfaceDisabled,
    UnsupportedRouteFamily,
    UnsupportedHttpMethod,
    InvalidPath,
    InvalidHeader,
    AmbiguousForwardingHeaders,
    InvalidQueryPair,
    InvalidBodyContentType,
    BodyMetadataWithoutBody,
    UnexpectedRequestBody,
    UnsupportedRepresentation,
    UnsupportedApiVersion,
    IncompatibleMethodForRouteFamily,
    RequestContextDenied,
    MiddlewareDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityDenial {
    code: ForgeServerCompatibilityDenialCode,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
    detail: String,
}

impl ForgeServerCompatibilityDenial {
    pub(crate) fn new(
        code: ForgeServerCompatibilityDenialCode,
        diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerCompatibilityDenialCode {
        self.code.clone()
    }

    pub fn diagnostics_profile(&self) -> crate::request_context::DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerCompatibilityDeferred {
    RequestContext(ForgeServerRequestContextDeferred),
    Middleware(ForgeServerMiddlewareDeferred),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerCompatibilityStale {
    RequestContext(ForgeServerRequestContextStale),
    Middleware(ForgeServerMiddlewareStale),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerCompatibilityRebindRequired {
    RequestContext(ForgeServerRequestContextRebindRequired),
    Middleware(ForgeServerMiddlewareRebindRequired),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerCompatibilityFailure {
    RequestContext(ForgeServerRequestContextFailure),
    Middleware(ForgeServerMiddlewareFailure),
}
