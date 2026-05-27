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

    pub fn orchestration_stop_stage(&self) -> Option<ForgeQueryDeclarationEntryOrchestrationStage> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { stop_stage, .. } => {
                Some(*stop_stage)
            }
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. } => None,
        }
    }

    pub fn orchestration_retained_digest(&self) -> Option<&str> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration {
                retained_digest, ..
            } => retained_digest.as_deref(),
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. } => None,
        }
    }

    pub fn orchestration_refusal_class(
        &self,
    ) -> Option<ForgeQueryDeclarationEntryOrchestrationRefusalClass> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { refusal_class, .. } => {
                *refusal_class
            }
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding { .. } => None,
        }
    }

    pub fn binding_kind(&self) -> Option<ForgeQueryOrdinaryBindingCheckedTopologyKind> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding { kind, .. } => Some(*kind),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. } => None,
        }
    }

    pub fn binding_linked_artifacts(&self) -> Option<&ForgeQueryBindingLinkedArtifacts> {
        match &self.repr {
            ForgeQueryOrdinaryCheckedTopologyRepr::Binding {
                linked_artifacts, ..
            } => Some(linked_artifacts),
            ForgeQueryOrdinaryCheckedTopologyRepr::Orchestration { .. } => None,
        }
    }
}
