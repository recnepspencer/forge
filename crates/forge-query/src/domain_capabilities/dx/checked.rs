use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionFailure,
    ForgeQueryDomainCapabilityRebindRequired, ForgeQueryDomainCapabilityStale,
};
use crate::domain_capabilities::{
    ForgeQueryDomainCapabilityTargetKind, ForgeQueryDomainCapabilityTransitionOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDomainCapabilityOutcomeKind {
    Materialized,
    Denied,
    Stale,
    RebindRequired,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryDomainCapabilityMaterializationError {
    Denied(ForgeQueryDomainCapabilityProgressionDenial),
    Stale(ForgeQueryDomainCapabilityStale),
    RebindRequired(ForgeQueryDomainCapabilityRebindRequired),
    Failed(ForgeQueryDomainCapabilityProgressionFailure),
}

impl ForgeQueryDomainCapabilityMaterializationError {
    pub fn kind(&self) -> ForgeQueryDomainCapabilityOutcomeKind {
        match self {
            Self::Denied(_) => ForgeQueryDomainCapabilityOutcomeKind::Denied,
            Self::Stale(_) => ForgeQueryDomainCapabilityOutcomeKind::Stale,
            Self::RebindRequired(_) => ForgeQueryDomainCapabilityOutcomeKind::RebindRequired,
            Self::Failed(_) => ForgeQueryDomainCapabilityOutcomeKind::Failed,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryCheckedDomainCapabilityOutcome<T> {
    category: &'static str,
    target_kind: ForgeQueryDomainCapabilityTargetKind,
    semantic_posture: &'static str,
    inner: ForgeQueryDomainCapabilityTransitionOutcome<T>,
}

impl<T> ForgeQueryCheckedDomainCapabilityOutcome<T> {
    pub(crate) fn from_transition_outcome(
        category: &'static str,
        target_kind: ForgeQueryDomainCapabilityTargetKind,
        semantic_posture: &'static str,
        inner: ForgeQueryDomainCapabilityTransitionOutcome<T>,
    ) -> Self {
        Self {
            category,
            target_kind,
            semantic_posture,
            inner,
        }
    }

    pub fn category(&self) -> &str {
        self.category
    }

    pub fn target_kind(&self) -> ForgeQueryDomainCapabilityTargetKind {
        self.target_kind
    }

    pub fn semantic_posture(&self) -> &str {
        self.semantic_posture
    }

    pub fn kind(&self) -> ForgeQueryDomainCapabilityOutcomeKind {
        match self.inner {
            forge_proof::TransitionOutcome::Success(_) => {
                ForgeQueryDomainCapabilityOutcomeKind::Materialized
            }
            forge_proof::TransitionOutcome::Denied(_) => {
                ForgeQueryDomainCapabilityOutcomeKind::Denied
            }
            forge_proof::TransitionOutcome::Stale(_) => {
                ForgeQueryDomainCapabilityOutcomeKind::Stale
            }
            forge_proof::TransitionOutcome::RebindRequired(_) => {
                ForgeQueryDomainCapabilityOutcomeKind::RebindRequired
            }
            forge_proof::TransitionOutcome::Failed(_) => {
                ForgeQueryDomainCapabilityOutcomeKind::Failed
            }
            forge_proof::TransitionOutcome::Deferred(never) => match never {},
        }
    }

    pub fn transition_outcome(&self) -> &ForgeQueryDomainCapabilityTransitionOutcome<T> {
        &self.inner
    }

    pub fn into_transition_outcome(self) -> ForgeQueryDomainCapabilityTransitionOutcome<T> {
        self.inner
    }

    pub fn materialized(&self) -> Option<&T> {
        match &self.inner {
            forge_proof::TransitionOutcome::Success(value) => Some(value),
            _ => None,
        }
    }

    pub fn denial(&self) -> Option<&ForgeQueryDomainCapabilityProgressionDenial> {
        match &self.inner {
            forge_proof::TransitionOutcome::Denied(value) => Some(value),
            _ => None,
        }
    }

    pub fn stale(&self) -> Option<&ForgeQueryDomainCapabilityStale> {
        match &self.inner {
            forge_proof::TransitionOutcome::Stale(value) => Some(value),
            _ => None,
        }
    }

    pub fn rebind_required(&self) -> Option<&ForgeQueryDomainCapabilityRebindRequired> {
        match &self.inner {
            forge_proof::TransitionOutcome::RebindRequired(value) => Some(value),
            _ => None,
        }
    }

    pub fn failure(&self) -> Option<&ForgeQueryDomainCapabilityProgressionFailure> {
        match &self.inner {
            forge_proof::TransitionOutcome::Failed(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_result(self) -> Result<T, ForgeQueryDomainCapabilityMaterializationError> {
        match self.inner {
            forge_proof::TransitionOutcome::Success(value) => Ok(value),
            forge_proof::TransitionOutcome::Denied(value) => Err(
                ForgeQueryDomainCapabilityMaterializationError::Denied(value),
            ),
            forge_proof::TransitionOutcome::Stale(value) => {
                Err(ForgeQueryDomainCapabilityMaterializationError::Stale(value))
            }
            forge_proof::TransitionOutcome::RebindRequired(value) => {
                Err(ForgeQueryDomainCapabilityMaterializationError::RebindRequired(value))
            }
            forge_proof::TransitionOutcome::Failed(value) => Err(
                ForgeQueryDomainCapabilityMaterializationError::Failed(value),
            ),
            forge_proof::TransitionOutcome::Deferred(never) => match never {},
        }
    }
}
