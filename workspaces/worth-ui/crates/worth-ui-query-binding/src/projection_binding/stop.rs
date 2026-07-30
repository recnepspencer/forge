use std::sync::Arc;

use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionBindingStopKind {
    MissingInstalledView,
    ShapeMismatch,
    SchemaMismatch,
    NativeFamilyMismatch,
    PayloadShapeMismatch,
    RowIdentityMismatch,
    LifecycleMismatch,
    WrongWorld,
    RebindRequired,
    BudgetExceeded,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionBindingStopReceipt {
    kind: UiProjectionBindingStopKind,
    attempt_identity: WorthQueryEvidenceIdentity,
    predecessor_binding_identity: Option<Arc<str>>,
    summary: Arc<str>,
}

impl UiProjectionBindingStopReceipt {
    pub(crate) fn initial(
        kind: UiProjectionBindingStopKind,
        attempt_identity: WorthQueryEvidenceIdentity,
        summary: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            attempt_identity,
            predecessor_binding_identity: None,
            summary: summary.into(),
        }
    }

    pub(crate) fn replacement(
        kind: UiProjectionBindingStopKind,
        attempt_identity: WorthQueryEvidenceIdentity,
        predecessor_binding_identity: Arc<str>,
        summary: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            attempt_identity,
            predecessor_binding_identity: Some(predecessor_binding_identity),
            summary: summary.into(),
        }
    }

    pub fn kind(&self) -> UiProjectionBindingStopKind {
        self.kind
    }

    pub fn attempt_identity_for_reporting(&self) -> &str {
        self.attempt_identity.terminal_projection_for_reporting()
    }

    pub fn predecessor_binding_identity_for_reporting(&self) -> Option<&str> {
        self.predecessor_binding_identity
            .as_ref()
            .map(AsRef::as_ref)
    }

    pub fn summary(&self) -> &str {
        self.summary.as_ref()
    }
}
