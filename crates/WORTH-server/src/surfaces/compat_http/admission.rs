use worth_proof::TransitionOutcome;

use crate::{
    WorthServerAdmission, WorthServerMiddlewareDeferred, WorthServerMiddlewareFailure,
    WorthServerMiddlewareRebindRequired, WorthServerMiddlewareStale,
    WorthServerRequestContextDeferred, WorthServerRequestContextFailure,
    WorthServerRequestContextRebindRequired, WorthServerRequestContextStale,
};

use crate::WorthServerExternalRequestContract;

pub type WorthServerCompatibilityRequestOutcome = TransitionOutcome<
    WorthServerCompatibilityRequest,
    WorthServerCompatibilityDenial,
    WorthServerCompatibilityDeferred,
    WorthServerCompatibilityStale,
    WorthServerCompatibilityRebindRequired,
    WorthServerCompatibilityFailure,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerCompatibilityPreparedRequest {
    admission: WorthServerAdmission,
    request_contract: WorthServerExternalRequestContract,
}

impl WorthServerCompatibilityPreparedRequest {
    pub(crate) fn new(
        admission: WorthServerAdmission,
        request_contract: WorthServerExternalRequestContract,
    ) -> Self {
        Self {
            admission,
            request_contract,
        }
    }

    pub fn admission(&self) -> &WorthServerAdmission {
        &self.admission
    }

    pub fn request_contract(&self) -> &WorthServerExternalRequestContract {
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

    pub fn into_request(self) -> WorthServerCompatibilityRequest {
        WorthServerCompatibilityRequest {
            admission: self.admission,
            request_contract: self.request_contract,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerCompatibilityRequest {
    admission: WorthServerAdmission,
    request_contract: WorthServerExternalRequestContract,
}

impl WorthServerCompatibilityRequest {
    pub fn admission(&self) -> &WorthServerAdmission {
        &self.admission
    }

    pub fn request_contract(&self) -> &WorthServerExternalRequestContract {
        &self.request_contract
    }

    pub fn request_context_digest(&self) -> String {
        WorthServerCompatibilityPreparedRequest::new(
            self.admission.clone(),
            self.request_contract.clone(),
        )
        .request_context_digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerCompatibilityDenialCode {
    CompatHttpSurfaceAbsent,
    CompatHttpSurfaceDisabled,
    UnsupportedRouteFamily,
    OperationFamilyNotRegistered,
    OperationFamilyDisabled,
    OperationFamilyNotExposedOnSurface,
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
pub struct WorthServerCompatibilityDenial {
    code: WorthServerCompatibilityDenialCode,
    diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
    detail: String,
    pub(crate) abuse_budget_receipt: Option<crate::WorthServerAbuseBudgetReceipt>,
}

impl WorthServerCompatibilityDenial {
    pub(crate) fn new(
        code: WorthServerCompatibilityDenialCode,
        diagnostics_profile: crate::request_context::DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
            abuse_budget_receipt: None,
        }
    }

    pub(crate) fn with_abuse_budget_receipt(
        mut self,
        abuse_budget_receipt: crate::WorthServerAbuseBudgetReceipt,
    ) -> Self {
        self.abuse_budget_receipt = Some(abuse_budget_receipt);
        self
    }

    pub fn code(&self) -> WorthServerCompatibilityDenialCode {
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
pub enum WorthServerCompatibilityDeferred {
    RequestContext(WorthServerRequestContextDeferred),
    Middleware(WorthServerMiddlewareDeferred),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerCompatibilityStale {
    RequestContext(WorthServerRequestContextStale),
    Middleware(WorthServerMiddlewareStale),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerCompatibilityRebindRequired {
    RequestContext(WorthServerRequestContextRebindRequired),
    Middleware(WorthServerMiddlewareRebindRequired),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerCompatibilityFailure {
    RequestContext(WorthServerRequestContextFailure),
    Middleware(WorthServerMiddlewareFailure),
}
