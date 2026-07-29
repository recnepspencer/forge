use std::sync::Arc;

use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub enum UiProjectionAvailability<V> {
    Unavailable(UiProjectionUnavailableReceipt),
    Present(UiPresentProjection<V>),
    Stopped(UiProjectionFactStopReceipt),
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub enum UiPresentProjection<V> {
    Current(V),
    RetainedStale {
        value: V,
        activity: UiProjectionRetainedActivityReceipt,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionUnavailableKind {
    Pending,
    Failed,
    Cancelled,
    Retried,
    Superseded,
    Denied,
    Unsupported,
    Remasked,
    BasisDrift,
    GenerationDrift,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionUnavailableReceipt {
    kind: UiProjectionUnavailableKind,
    query_transition_identity: WorthQueryEvidenceIdentity,
}

impl UiProjectionUnavailableReceipt {
    pub fn kind(&self) -> UiProjectionUnavailableKind {
        self.kind
    }

    pub fn query_transition_identity_for_reporting(&self) -> &str {
        self.query_transition_identity
            .terminal_projection_for_reporting()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionRetainedActivityKind {
    Idle,
    Revalidating,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionRetainedActivityReceipt {
    kind: UiProjectionRetainedActivityKind,
    query_transition_identity: WorthQueryEvidenceIdentity,
}

impl UiProjectionRetainedActivityReceipt {
    pub fn kind(&self) -> UiProjectionRetainedActivityKind {
        self.kind
    }

    pub fn query_transition_identity_for_reporting(&self) -> &str {
        self.query_transition_identity
            .terminal_projection_for_reporting()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionFactStopKind {
    SchemaMismatch,
    PayloadShapeMismatch,
    NativeFamilyMismatch,
    WrongWorld,
    StaleBindingGeneration,
    StaleResultGeneration,
    BasisMismatch,
    Unsupported,
    Remasked,
    BudgetExceeded,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionFactStopReceipt {
    kind: UiProjectionFactStopKind,
    attempt_identity: WorthQueryEvidenceIdentity,
    predecessor_fact_identity: Option<WorthQueryEvidenceIdentity>,
    summary: Arc<str>,
}

impl UiProjectionFactStopReceipt {
    pub fn kind(&self) -> UiProjectionFactStopKind {
        self.kind
    }

    pub fn attempt_identity_for_reporting(&self) -> &str {
        self.attempt_identity.terminal_projection_for_reporting()
    }

    pub fn predecessor_fact_identity_for_reporting(&self) -> Option<&str> {
        self.predecessor_fact_identity
            .as_ref()
            .map(WorthQueryEvidenceIdentity::terminal_projection_for_reporting)
    }

    pub fn summary(&self) -> &str {
        self.summary.as_ref()
    }
}
