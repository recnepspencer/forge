#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationAuthorizationDenialKind {
    Cancelled,
    DeadlineExceeded,
    ExpiredAuthentication,
    ForeignRuntime,
    StaleInstalledSchema,
    StaleInstalledOperation,
    StalePrincipal,
    StaleScope,
    MutationPreconditionRejected,
    CanonicalWorkDenied,
    TrustedTimeUnavailable,
    CapabilityProjectionRejected,
    CapabilityRequired,
    CapabilityExpired,
    AdmissionIdentityExhausted,
    ScopeMismatch,
    PolicyNotInstalled,
    InvalidInstalledPolicy,
    RelationalObservationRejected,
    BridgeEvaluationRejected,
    InconsistentDecision,
    PermissionDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationAuthorizationDenial {
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: String,
}

impl WorthQueryOperationAuthorizationDenial {
    pub(super) fn new(
        kind: WorthQueryOperationAuthorizationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryOperationAuthorizationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for WorthQueryOperationAuthorizationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "operation authorization denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryOperationAuthorizationDenial {}
