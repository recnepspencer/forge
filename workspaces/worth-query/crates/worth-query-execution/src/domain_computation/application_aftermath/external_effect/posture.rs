//! Opaque public projection of external-effect causal events.

use super::identity::{ExternalEffectCausalLink, ExternalEffectPostureIdentity};

/// Meaning of one external-effect causal event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExternalEffectPostureKind {
    ProviderCommit,
    EmittedApplicationCausality,
    DispatchAttempt,
    ExternalAcknowledgement,
    ExternalCompletion,
    Compensation,
    Reconciliation,
}

/// Read-only posture projection. Construction stays in the stage-typed owner
/// ladder; callers can inspect but cannot recover or recycle transition proof.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalEffectPosture {
    kind: ExternalEffectPostureKind,
    identity: ExternalEffectPostureIdentity,
    predecessor: Option<ExternalEffectCausalLink>,
}

impl ExternalEffectPosture {
    pub(super) const fn root(
        kind: ExternalEffectPostureKind,
        identity: ExternalEffectPostureIdentity,
    ) -> Self {
        Self {
            kind,
            identity,
            predecessor: None,
        }
    }

    pub(super) const fn successor(
        kind: ExternalEffectPostureKind,
        identity: ExternalEffectPostureIdentity,
        predecessor: &ExternalEffectPosture,
    ) -> Self {
        Self {
            kind,
            identity,
            predecessor: Some(ExternalEffectCausalLink::to(predecessor.identity())),
        }
    }

    pub const fn kind(&self) -> ExternalEffectPostureKind {
        self.kind
    }

    pub const fn identity(&self) -> &ExternalEffectPostureIdentity {
        &self.identity
    }

    pub const fn predecessor(&self) -> Option<&ExternalEffectCausalLink> {
        self.predecessor.as_ref()
    }
}
