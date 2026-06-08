use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryOrdinaryBindingCheckedTopologyKind {
    Ambiguous,
    AspectConflict,
    AuthorityMismatch,
    BasisMismatch,
    ExplicitNarrowingRequired,
    MissingRequiredAspect,
    RebindRequired,
    Stale,
    Unavailable,
    Unsupported,
    WrongHandle,
    WrongWorld,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryOrdinaryContinuationCheckedTopologyKind {
    Ambiguous,
    AsyncRequestDrift,
    AuthorityMismatch,
    BasisMismatch,
    Deferred,
    Denied,
    Failed,
    PreviewCrossedResidue,
    RemaskDrift,
    RebindRequired,
    ReplayDrift,
    Stale,
    StaleCompletion,
    Unavailable,
    Unsupported,
    WrongHandle,
    WrongWorld,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind {
    Ambiguous,
    AspectConflict,
    AuthorityMismatch,
    BasisMismatch,
    Deferred,
    Denied,
    Failed,
    MissingRequiredAspect,
    RebindRequired,
    Stale,
    Unavailable,
    Unsupported,
    WrongHandle,
    WrongWorld,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryOrdinaryContributionComposedCheckedTopologyKind {
    Deferred,
    DeclarationDenied,
    ContributionDenied,
    Stale,
    RebindRequired,
    Unsupported,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ForgeQueryOrdinaryCheckedTopologyRepr {
    Orchestration {
        stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        refusal_class: Option<ForgeQueryDeclarationEntryOrchestrationRefusalClass>,
    },
    Binding {
        kind: ForgeQueryOrdinaryBindingCheckedTopologyKind,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    },
    Continuation {
        kind: ForgeQueryOrdinaryContinuationCheckedTopologyKind,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    },
    SignalCompatibilityOrchestration {
        kind: ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    },
    ContributionComposed {
        kind: ForgeQueryOrdinaryContributionComposedCheckedTopologyKind,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
        contribution_digest: Option<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryOrdinaryCheckedTopology {
    repr: ForgeQueryOrdinaryCheckedTopologyRepr,
}

impl ForgeQueryOrdinaryCheckedTopology {
    pub(crate) fn orchestration(
        stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        retained_digest: Option<String>,
        refusal_class: Option<ForgeQueryDeclarationEntryOrchestrationRefusalClass>,
    ) -> Self {
        Self {
            repr: ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration {
                stop_stage,
                retained_digest,
                refusal_class,
            },
        }
    }

    pub(crate) fn binding(
        kind: ForgeQueryOrdinaryBindingCheckedTopologyKind,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            repr: ForgeQueryOrdinaryCheckedTopologyRepr::Binding {
                kind,
                linked_artifacts,
            },
        }
    }

    pub(crate) fn continuation(
        kind: ForgeQueryOrdinaryContinuationCheckedTopologyKind,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            repr: ForgeQueryOrdinaryCheckedTopologyRepr::Continuation {
                kind,
                linked_artifacts,
            },
        }
    }

    pub(crate) fn signal_compatibility_orchestration(
        kind: ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            repr: ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration {
                kind,
                linked_artifacts,
            },
        }
    }

    pub(crate) fn contribution_composed(
        kind: ForgeQueryOrdinaryContributionComposedCheckedTopologyKind,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
        contribution_digest: Option<String>,
    ) -> Self {
        Self {
            repr: ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed {
                kind,
                linked_artifacts,
                contribution_digest,
            },
        }
    }

    pub fn orchestration_stop_stage(&self) -> Option<ForgeQueryDeclarationEntryOrchestrationStage> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { stop_stage, .. } => {
                Some(*stop_stage)
            }
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { .. } => None,
        }
    }

    pub fn orchestration_retained_digest(&self) -> Option<&str> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration {
                retained_digest, ..
            } => retained_digest.as_deref(),
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { .. } => None,
        }
    }

    pub fn orchestration_refusal_class(
        &self,
    ) -> Option<ForgeQueryDeclarationEntryOrchestrationRefusalClass> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { refusal_class, .. } => {
                *refusal_class
            }
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { .. } => None,
        }
    }

    pub fn binding_kind(&self) -> Option<ForgeQueryOrdinaryBindingCheckedTopologyKind> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding { kind, .. } => Some(*kind),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { .. } => None,
        }
    }

    pub fn binding_linked_artifacts(&self) -> Option<&ForgeQueryBindingLinkedArtifacts> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding {
                linked_artifacts, ..
            } => Some(linked_artifacts),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { .. } => None,
        }
    }

    pub fn continuation_kind(&self) -> Option<ForgeQueryOrdinaryContinuationCheckedTopologyKind> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { kind, .. } => Some(*kind),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { .. } => None,
        }
    }

    pub fn continuation_linked_artifacts(&self) -> Option<&ForgeQueryBindingLinkedArtifacts> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Continuation {
                linked_artifacts, ..
            } => Some(linked_artifacts),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { .. } => None,
        }
    }

    pub fn signal_compatibility_orchestration_kind(
        &self,
    ) -> Option<ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration {
                kind,
                ..
            } => Some(*kind),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { .. } => None,
        }
    }

    pub fn signal_compatibility_orchestration_linked_artifacts(
        &self,
    ) -> Option<&ForgeQueryBindingLinkedArtifacts> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration {
                linked_artifacts,
                ..
            } => Some(linked_artifacts),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { .. } => None,
        }
    }

    pub fn contribution_composed_kind(
        &self,
    ) -> Option<ForgeQueryOrdinaryContributionComposedCheckedTopologyKind> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed { kind, .. } => Some(*kind),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. } => {
                None
            }
        }
    }

    pub fn contribution_composed_linked_artifacts(
        &self,
    ) -> Option<&ForgeQueryBindingLinkedArtifacts> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed {
                linked_artifacts,
                ..
            } => Some(linked_artifacts),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. } => {
                None
            }
        }
    }

    pub fn contribution_composed_digest(&self) -> Option<&str> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::ContributionComposed {
                contribution_digest,
                ..
            } => contribution_digest.as_deref(),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::Continuation { .. }
            | ForgeQueryOrdinaryCheckedTopologyRepr::SignalCompatibilityOrchestration { .. } => {
                None
            }
        }
    }
}
