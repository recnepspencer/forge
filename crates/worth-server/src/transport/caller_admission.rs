use std::sync::Arc;
use std::time::Instant;

use sha2::{Digest, Sha256};

use super::{WorthServerTransportDenial, WorthServerTransportDenialCode};

#[derive(Clone)]
pub struct WorthServerTransportCallerAdmissionRequest {
    operation_name: String,
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    cancellation: super::WorthServerTransportRequestCancellation,
}

impl WorthServerTransportCallerAdmissionRequest {
    pub(crate) fn new(
        operation_name: impl Into<String>,
        method: impl Into<String>,
        path: impl Into<String>,
        headers: Vec<(String, String)>,
        cancellation: super::WorthServerTransportRequestCancellation,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            method: method.into(),
            path: path.into(),
            headers,
            cancellation,
        }
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    /// Derives a Query request scope from this transport request's live
    /// cancellation lifecycle and the verifier-admitted deadline.
    pub fn query_request_scope(
        &self,
        scope_identity: impl Into<String>,
        deadline: Instant,
        initially_cancelled: bool,
    ) -> super::WorthServerQueryRequestScope {
        self.cancellation
            .query_scope(scope_identity.into(), deadline, initially_cancelled)
    }
}

pub trait WorthServerTransportCallerVerifier: std::fmt::Debug + Send + Sync {
    fn verify(
        &self,
        request: &WorthServerTransportCallerAdmissionRequest,
    ) -> WorthServerTransportCallerVerification;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerTransportCallerVerification {
    NotApplicable,
    Verified(WorthServerVerifiedTransportCaller),
    Denied(WorthServerTransportCallerDenial),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerVerifiedTransportCaller {
    principal_identity: String,
    authority_identity: String,
    caller_identity: String,
    verifier_profile: String,
    verifier_revision: u64,
    query_request_scope: Option<super::WorthServerQueryRequestScope>,
}

impl WorthServerVerifiedTransportCaller {
    pub fn new(
        principal_identity: impl Into<String>,
        authority_identity: impl Into<String>,
        caller_identity: impl Into<String>,
        verifier_profile: impl Into<String>,
        verifier_revision: u64,
    ) -> Result<Self, String> {
        let verified = Self {
            principal_identity: principal_identity.into(),
            authority_identity: authority_identity.into(),
            caller_identity: caller_identity.into(),
            verifier_profile: verifier_profile.into(),
            verifier_revision,
            query_request_scope: None,
        };
        for (label, value) in [
            ("principal identity", verified.principal_identity.as_str()),
            ("authority identity", verified.authority_identity.as_str()),
            ("caller identity", verified.caller_identity.as_str()),
            ("verifier profile", verified.verifier_profile.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(format!(
                    "verified transport caller {label} must not be blank"
                ));
            }
        }
        if verified.verifier_revision == 0 {
            return Err("verified transport caller revision must be positive".to_string());
        }
        Ok(verified)
    }

    #[must_use]
    pub fn with_query_request_scope(
        mut self,
        query_request_scope: super::WorthServerQueryRequestScope,
    ) -> Self {
        self.query_request_scope = Some(query_request_scope);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerTransportCallerDenial {
    reason_key: String,
    detail: String,
}

impl WorthServerTransportCallerDenial {
    pub fn new(reason_key: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            reason_key: reason_key.into(),
            detail: detail.into(),
        }
    }

    pub fn reason_key(&self) -> &str {
        &self.reason_key
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthServerAdmittedTransportCaller {
    principal_identity: String,
    authority_identity: String,
    caller_identity: String,
    verifier_profile: String,
    verifier_revision: u64,
    request_receipt: String,
    query_request_scope: Option<super::WorthServerQueryRequestScope>,
}

impl WorthServerAdmittedTransportCaller {
    fn admit(
        verified: WorthServerVerifiedTransportCaller,
        request: &WorthServerTransportCallerAdmissionRequest,
    ) -> Self {
        let request_receipt = caller_receipt(&verified, request);
        Self {
            principal_identity: verified.principal_identity,
            authority_identity: verified.authority_identity,
            caller_identity: verified.caller_identity,
            verifier_profile: verified.verifier_profile,
            verifier_revision: verified.verifier_revision,
            request_receipt,
            query_request_scope: verified.query_request_scope,
        }
    }

    pub fn principal_identity(&self) -> &str {
        &self.principal_identity
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub fn caller_identity(&self) -> &str {
        &self.caller_identity
    }

    pub fn verifier_profile(&self) -> &str {
        &self.verifier_profile
    }

    pub fn verifier_revision(&self) -> u64 {
        self.verifier_revision
    }

    pub fn request_receipt(&self) -> &str {
        &self.request_receipt
    }

    pub fn query_request_scope(&self) -> Option<&super::WorthServerQueryRequestScope> {
        self.query_request_scope.as_ref()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct WorthServerTransportCallerAdmission {
    verifier: Option<Arc<dyn WorthServerTransportCallerVerifier>>,
}

impl WorthServerTransportCallerAdmission {
    pub(crate) fn new(verifier: Option<Arc<dyn WorthServerTransportCallerVerifier>>) -> Self {
        Self { verifier }
    }

    pub(crate) fn admit(
        &self,
        request: &WorthServerTransportCallerAdmissionRequest,
        caller_asserted_principal: Option<&str>,
    ) -> Result<WorthServerTransportPrincipal, WorthServerTransportDenial> {
        match self
            .verifier
            .as_ref()
            .map(|verifier| verifier.verify(request))
            .unwrap_or(WorthServerTransportCallerVerification::NotApplicable)
        {
            WorthServerTransportCallerVerification::Verified(verified) => {
                let admitted = WorthServerAdmittedTransportCaller::admit(verified, request);
                Ok(WorthServerTransportPrincipal::Admitted(admitted))
            }
            WorthServerTransportCallerVerification::Denied(denial) => {
                Err(WorthServerTransportDenial::new(
                    WorthServerTransportDenialCode::CallerAdmissionDenied,
                    format!("{}: {}", denial.reason_key(), denial.detail()),
                )
                .with_reason_key(denial.reason_key()))
            }
            WorthServerTransportCallerVerification::NotApplicable => caller_asserted_principal
                .filter(|principal| !principal.trim().is_empty())
                .map(|principal| WorthServerTransportPrincipal::CallerAsserted {
                    principal_identity: principal.to_string(),
                })
                .ok_or_else(|| {
                    WorthServerTransportDenial::new(
                        WorthServerTransportDenialCode::MissingAuthenticatedPrincipalId,
                        "compat route requests require `x-principal-id` when no transport caller verifier applies",
                    )
                }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthServerTransportPrincipal {
    CallerAsserted { principal_identity: String },
    Admitted(WorthServerAdmittedTransportCaller),
}

impl WorthServerTransportPrincipal {
    pub(crate) fn principal_identity(&self) -> &str {
        match self {
            Self::CallerAsserted { principal_identity } => principal_identity,
            Self::Admitted(caller) => caller.principal_identity(),
        }
    }

    pub(crate) fn admitted_caller(&self) -> Option<&WorthServerAdmittedTransportCaller> {
        match self {
            Self::CallerAsserted { .. } => None,
            Self::Admitted(caller) => Some(caller),
        }
    }
}

fn caller_receipt(
    caller: &WorthServerVerifiedTransportCaller,
    request: &WorthServerTransportCallerAdmissionRequest,
) -> String {
    let mut digest = Sha256::new();
    for component in [
        "worth-server-transport-caller-receipt-v1",
        request.operation_name(),
        request.method(),
        request.path(),
        caller.principal_identity.as_str(),
        caller.authority_identity.as_str(),
        caller.caller_identity.as_str(),
        caller.verifier_profile.as_str(),
        &caller.verifier_revision.to_string(),
    ] {
        digest.update((component.len() as u64).to_be_bytes());
        digest.update(component.as_bytes());
    }
    if let Some(scope) = &caller.query_request_scope {
        let identity = scope.identity();
        digest.update((identity.len() as u64).to_be_bytes());
        digest.update(identity.as_bytes());
    }
    format!("sha256:{:x}", digest.finalize())
}
