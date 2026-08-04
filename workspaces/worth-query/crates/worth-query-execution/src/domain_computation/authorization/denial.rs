//! Typed authorization denial topology.

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
    CapabilityNotRequired,
    CapabilityExpired,
    ElevationRequired,
    ElevationNotApplicable,
    ElevationProjectionRejected,
    ElevationExpired,
    ElevationInactive,
    ElevationSelfApproval,
    ElevationApproverConflict,
    DelegationRejected,
    DelegationDepthExceeded,
    DelegationCycle,
    DelegationLineageChanged,
    StaleAuthorization,
    AdmissionIdentityExhausted,
    GraphWorkAdmissionUnavailable,
    ScopeMismatch,
    PolicyNotInstalled,
    InvalidInstalledPolicy,
    RelationalObservationRejected,
    GrantSelectionLimitExceeded,
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

    pub(in crate::domain_computation) fn inconsistent(subject: impl Into<String>) -> Self {
        Self::new(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            subject,
        )
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
