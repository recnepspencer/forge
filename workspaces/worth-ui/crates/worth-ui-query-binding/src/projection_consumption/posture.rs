use std::sync::Arc;

use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub enum UiProjectionAvailability<V> {
    Unavailable(UiProjectionUnavailableReceipt),
    Present(UiPresentProjection<V>),
    Stopped(UiProjectionFactStopReceipt),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionTransitionPosture {
    Unavailable(UiProjectionUnavailableKind),
    Current,
    RetainedStale(UiProjectionRetainedActivityKind),
    Stopped(UiProjectionFactStopKind),
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionPostureTrace {
    postures: Box<[UiProjectionTransitionPosture]>,
}

impl UiProjectionPostureTrace {
    pub(crate) fn admitted(postures: impl Into<Box<[UiProjectionTransitionPosture]>>) -> Self {
        Self {
            postures: postures.into(),
        }
    }

    pub fn postures(&self) -> &[UiProjectionTransitionPosture] {
        &self.postures
    }
}

impl<V> UiProjectionAvailability<V> {
    pub(crate) fn transition_posture(&self) -> UiProjectionTransitionPosture {
        match self {
            Self::Unavailable(receipt) => {
                UiProjectionTransitionPosture::Unavailable(receipt.kind())
            }
            Self::Present(UiPresentProjection::Current(_)) => {
                UiProjectionTransitionPosture::Current
            }
            Self::Present(UiPresentProjection::RetainedStale { activity, .. }) => {
                UiProjectionTransitionPosture::RetainedStale(activity.kind())
            }
            Self::Stopped(receipt) => UiProjectionTransitionPosture::Stopped(receipt.kind()),
        }
    }
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
    pub(crate) fn query_issued(
        kind: UiProjectionUnavailableKind,
        query_transition_identity: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            kind,
            query_transition_identity,
        }
    }

    pub fn kind(&self) -> UiProjectionUnavailableKind {
        self.kind
    }

    pub fn query_transition_identity(&self) -> crate::UiQueryEvidenceReference {
        crate::UiQueryEvidenceReference::query_issued(&self.query_transition_identity)
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
    pub(crate) fn query_issued(
        kind: UiProjectionRetainedActivityKind,
        query_transition_identity: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            kind,
            query_transition_identity,
        }
    }

    pub fn kind(&self) -> UiProjectionRetainedActivityKind {
        self.kind
    }

    pub fn query_transition_identity(&self) -> crate::UiQueryEvidenceReference {
        crate::UiQueryEvidenceReference::query_issued(&self.query_transition_identity)
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
    ResetRequired,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionFactStopReceipt {
    kind: UiProjectionFactStopKind,
    attempt_identity: WorthQueryEvidenceIdentity,
    predecessor_fact_identity: Option<WorthQueryEvidenceIdentity>,
    summary: Arc<str>,
}

impl UiProjectionFactStopReceipt {
    pub(crate) fn query_issued(
        kind: UiProjectionFactStopKind,
        attempt_identity: WorthQueryEvidenceIdentity,
        predecessor_fact_identity: Option<WorthQueryEvidenceIdentity>,
        summary: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            attempt_identity,
            predecessor_fact_identity,
            summary: summary.into(),
        }
    }

    pub fn kind(&self) -> UiProjectionFactStopKind {
        self.kind
    }

    pub fn attempt_identity(&self) -> crate::UiQueryEvidenceReference {
        crate::UiQueryEvidenceReference::query_issued(&self.attempt_identity)
    }

    pub fn predecessor_fact_identity(&self) -> Option<crate::UiQueryEvidenceReference> {
        self.predecessor_fact_identity
            .as_ref()
            .map(crate::UiQueryEvidenceReference::query_issued)
    }

    pub fn summary(&self) -> &str {
        self.summary.as_ref()
    }
}
