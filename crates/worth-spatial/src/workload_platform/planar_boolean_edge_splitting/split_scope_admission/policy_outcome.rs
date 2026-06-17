use crate::workload_platform::vocabulary::WorkloadStageSupport;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitPolicyOutcomeKind {
    Admitted,
    Unsupported,
    Blocked,
    Denied,
    PolicyRequired,
    IntegrityMismatch,
}

impl PlanarBooleanEdgeSplitPolicyOutcomeKind {
    pub fn stable_name(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Unsupported => "unsupported",
            Self::Blocked => "blocked",
            Self::Denied => "denied",
            Self::PolicyRequired => "policy-required",
            Self::IntegrityMismatch => "integrity-mismatch",
        }
    }

    pub fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitPolicyOutcome {
    kind: PlanarBooleanEdgeSplitPolicyOutcomeKind,
    event_ledger_identity: String,
    split_request_identity: String,
    support: WorkloadStageSupport,
}

impl PlanarBooleanEdgeSplitPolicyOutcome {
    pub(crate) fn new(
        kind: PlanarBooleanEdgeSplitPolicyOutcomeKind,
        event_ledger_identity: impl Into<String>,
        split_request_identity: impl Into<String>,
        support: WorkloadStageSupport,
    ) -> Self {
        Self {
            kind,
            event_ledger_identity: event_ledger_identity.into(),
            split_request_identity: split_request_identity.into(),
            support,
        }
    }

    pub(crate) fn admitted(
        event_ledger_identity: impl Into<String>,
        split_request_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            PlanarBooleanEdgeSplitPolicyOutcomeKind::Admitted,
            event_ledger_identity,
            split_request_identity,
            WorkloadStageSupport::Admitted,
        )
    }

    pub(crate) fn unsupported(
        event_ledger_identity: impl Into<String>,
        split_request_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            PlanarBooleanEdgeSplitPolicyOutcomeKind::Unsupported,
            event_ledger_identity,
            split_request_identity,
            WorkloadStageSupport::Unsupported,
        )
    }

    pub fn kind(&self) -> PlanarBooleanEdgeSplitPolicyOutcomeKind {
        self.kind
    }

    pub fn event_ledger_identity(&self) -> &str {
        &self.event_ledger_identity
    }

    pub fn split_request_identity(&self) -> &str {
        &self.split_request_identity
    }

    pub fn support(&self) -> WorkloadStageSupport {
        self.support
    }

    pub fn is_admitted_for_source_edge_recovery(&self) -> bool {
        self.kind.is_admitted() && self.support == WorkloadStageSupport::Admitted
    }
}
