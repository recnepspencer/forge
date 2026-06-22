use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryGraphObligationOrchestrationDispatch,
    ForgeQueryGraphObligationOrchestrationDispatchError,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryContributionComposedCheckedTopologyKind,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
};

use super::artifact::ForgeQueryContributionComposedOrchestration;
use super::aspect::ForgeQueryContributionComposedDeclarationAspectRecord;
use super::composition::ForgeQueryContributionComposedStop;
use super::intent_result::ForgeQueryContributionComposedIntentRequestDescriptor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryContributionComposedOrchestrationCheckedKind {
    Deferred,
    DeclarationDenied,
    ContributionDenied,
    Stale,
    RebindRequired,
    Unsupported,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedOrchestrationPosture<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
    stop: ForgeQueryContributionComposedStop,
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    reason: String,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    contribution_digest: Option<String>,
    declaration_aspect_record: Option<ForgeQueryContributionComposedDeclarationAspectRecord>,
    primary_intent_descriptor: Option<ForgeQueryContributionComposedIntentRequestDescriptor>,
    graph_obligation_dispatch: Option<ForgeQueryGraphObligationOrchestrationDispatch>,
    graph_obligation_dispatch_error: Option<ForgeQueryGraphObligationOrchestrationDispatchError>,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContributionComposedOrchestrationPosture<D, I>
{
    pub fn new(
        kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
        stop: ForgeQueryContributionComposedStop,
        stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        reason: impl Into<String>,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
        contribution_digest: Option<String>,
        declaration_aspect_record: Option<ForgeQueryContributionComposedDeclarationAspectRecord>,
        primary_intent_descriptor: Option<ForgeQueryContributionComposedIntentRequestDescriptor>,
    ) -> Self {
        Self {
            kind,
            stop,
            stop_stage,
            reason: reason.into(),
            linked_artifacts,
            contribution_digest,
            declaration_aspect_record,
            primary_intent_descriptor,
            graph_obligation_dispatch: None,
            graph_obligation_dispatch_error: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn with_graph_obligation_dispatch(
        mut self,
        dispatch: Option<ForgeQueryGraphObligationOrchestrationDispatch>,
    ) -> Self {
        self.graph_obligation_dispatch = dispatch;
        self
    }

    pub(crate) fn with_graph_obligation_dispatch_error(
        mut self,
        error: ForgeQueryGraphObligationOrchestrationDispatchError,
    ) -> Self {
        self.graph_obligation_dispatch_error = Some(error);
        self
    }

    pub fn kind(&self) -> ForgeQueryContributionComposedOrchestrationCheckedKind {
        self.kind
    }

    pub fn stop(&self) -> ForgeQueryContributionComposedStop {
        self.stop
    }

    pub fn stop_stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
        self.stop_stage
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn linked_artifacts(&self) -> &ForgeQueryBindingLinkedArtifacts {
        &self.linked_artifacts
    }

    pub fn contribution_digest(&self) -> Option<&str> {
        self.contribution_digest.as_deref()
    }

    pub fn declaration_aspect_record(
        &self,
    ) -> Option<&ForgeQueryContributionComposedDeclarationAspectRecord> {
        self.declaration_aspect_record.as_ref()
    }

    pub fn primary_intent_descriptor(
        &self,
    ) -> Option<&ForgeQueryContributionComposedIntentRequestDescriptor> {
        self.primary_intent_descriptor.as_ref()
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryGraphObligationOrchestrationDispatch> {
        self.graph_obligation_dispatch.as_ref()
    }

    pub fn graph_obligation_dispatch_error(
        &self,
    ) -> Option<&ForgeQueryGraphObligationOrchestrationDispatchError> {
        self.graph_obligation_dispatch_error.as_ref()
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContributionComposedOrchestrationOutcome<D, I>
{
    pub(crate) fn with_graph_obligation_dispatch(
        self,
        dispatch: Option<ForgeQueryGraphObligationOrchestrationDispatch>,
    ) -> Self {
        match self {
            Self::Bound(value) => Self::Bound(value.with_graph_obligation_dispatch(dispatch)),
            Self::Deferred(value) => Self::Deferred(value.with_graph_obligation_dispatch(dispatch)),
            Self::DeclarationDenied(value) => {
                Self::DeclarationDenied(value.with_graph_obligation_dispatch(dispatch))
            }
            Self::ContributionDenied(value) => {
                Self::ContributionDenied(value.with_graph_obligation_dispatch(dispatch))
            }
            Self::Stale(value) => Self::Stale(value.with_graph_obligation_dispatch(dispatch)),
            Self::RebindRequired(value) => {
                Self::RebindRequired(value.with_graph_obligation_dispatch(dispatch))
            }
            Self::Unsupported(value) => {
                Self::Unsupported(value.with_graph_obligation_dispatch(dispatch))
            }
            Self::Failed(value) => Self::Failed(value.with_graph_obligation_dispatch(dispatch)),
        }
    }

    pub(crate) fn with_graph_obligation_dispatch_error(
        self,
        error: ForgeQueryGraphObligationOrchestrationDispatchError,
    ) -> Self {
        match self {
            Self::Bound(value) => Self::Bound(value),
            Self::Deferred(value) => {
                Self::Deferred(value.with_graph_obligation_dispatch_error(error))
            }
            Self::DeclarationDenied(value) => {
                Self::DeclarationDenied(value.with_graph_obligation_dispatch_error(error))
            }
            Self::ContributionDenied(value) => {
                Self::ContributionDenied(value.with_graph_obligation_dispatch_error(error))
            }
            Self::Stale(value) => Self::Stale(value.with_graph_obligation_dispatch_error(error)),
            Self::RebindRequired(value) => {
                Self::RebindRequired(value.with_graph_obligation_dispatch_error(error))
            }
            Self::Unsupported(value) => {
                Self::Unsupported(value.with_graph_obligation_dispatch_error(error))
            }
            Self::Failed(value) => Self::Failed(value.with_graph_obligation_dispatch_error(error)),
        }
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryGraphObligationOrchestrationDispatch> {
        match self {
            Self::Bound(value) => value.graph_obligation_dispatch(),
            Self::Deferred(value)
            | Self::DeclarationDenied(value)
            | Self::ContributionDenied(value)
            | Self::Stale(value)
            | Self::RebindRequired(value)
            | Self::Unsupported(value)
            | Self::Failed(value) => value.graph_obligation_dispatch(),
        }
    }

    pub fn graph_obligation_dispatch_error(
        &self,
    ) -> Option<&ForgeQueryGraphObligationOrchestrationDispatchError> {
        match self {
            Self::Bound(_) => None,
            Self::Deferred(value)
            | Self::DeclarationDenied(value)
            | Self::ContributionDenied(value)
            | Self::Stale(value)
            | Self::RebindRequired(value)
            | Self::Unsupported(value)
            | Self::Failed(value) => value.graph_obligation_dispatch_error(),
        }
    }
}

pub enum ForgeQueryContributionComposedOrchestrationOutcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Bound(ForgeQueryContributionComposedOrchestration<D, I>),
    Deferred(ForgeQueryContributionComposedOrchestrationPosture<D, I>),
    DeclarationDenied(ForgeQueryContributionComposedOrchestrationPosture<D, I>),
    ContributionDenied(ForgeQueryContributionComposedOrchestrationPosture<D, I>),
    Stale(ForgeQueryContributionComposedOrchestrationPosture<D, I>),
    RebindRequired(ForgeQueryContributionComposedOrchestrationPosture<D, I>),
    Unsupported(ForgeQueryContributionComposedOrchestrationPosture<D, I>),
    Failed(ForgeQueryContributionComposedOrchestrationPosture<D, I>),
}

pub type ForgeQueryContributionComposedOrchestrationChecked<D, I> =
    ForgeQueryContributionComposedOrchestrationOutcome<D, I>;

pub(crate) fn ordinary_outcome_from_contribution_composed_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryContributionComposedOrchestrationChecked<D, I>,
) -> ForgeQueryOrdinaryOutcome<ForgeQueryContributionComposedOrchestration<D, I>> {
    match checked {
        ForgeQueryContributionComposedOrchestrationOutcome::Bound(value) => {
            ForgeQueryOrdinaryOutcome::Bound(value)
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Deferred(value) => {
            ForgeQueryOrdinaryOutcome::Deferred(ordinary_posture(
                value,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Deferred,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value) => {
            ForgeQueryOrdinaryOutcome::Denied(ordinary_posture(
                value,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::DeclarationDenied,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value) => {
            ForgeQueryOrdinaryOutcome::Denied(ordinary_posture(
                value,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Stale(value) => {
            ForgeQueryOrdinaryOutcome::Stale(ordinary_posture(
                value,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Stale,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value) => {
            ForgeQueryOrdinaryOutcome::RebindRequired(ordinary_posture(
                value,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::RebindRequired,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value) => {
            ForgeQueryOrdinaryOutcome::Unsupported(ordinary_posture(
                value,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Unsupported,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => {
            ForgeQueryOrdinaryOutcome::Failed(ordinary_posture(
                value,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Failed,
            ))
        }
    }
}

fn ordinary_posture<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    posture: ForgeQueryContributionComposedOrchestrationPosture<D, I>,
    kind: ForgeQueryOrdinaryContributionComposedCheckedTopologyKind,
) -> ForgeQueryOrdinaryPosture {
    ForgeQueryOrdinaryPosture::new(
        posture.reason().to_string(),
        ordinary_posture_kind(posture.kind()),
        ordinary_next_step(posture.kind()),
        ForgeQueryOrdinaryCheckedTopology::contribution_composed(
            kind,
            posture.linked_artifacts().clone(),
            posture.contribution_digest().map(str::to_string),
        ),
    )
}

fn ordinary_posture_kind(
    kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
) -> crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind {
    match kind {
        ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred => {
            crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Deferred
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied
        | ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied => {
            crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Denied
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Stale => {
            crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Stale
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired => {
            crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::RebindRequired
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported => {
            crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Unsupported
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Failed => {
            crate::ordinary_outcome::ForgeQueryOrdinaryPostureKind::Failed
        }
    }
}

fn ordinary_next_step(
    kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
) -> ForgeQueryOrdinaryNextStep {
    match kind {
        ForgeQueryContributionComposedOrchestrationCheckedKind::Deferred => {
            ForgeQueryOrdinaryNextStep::RetryLater
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied
        | ForgeQueryContributionComposedOrchestrationCheckedKind::ContributionDenied => {
            ForgeQueryOrdinaryNextStep::InspectCheckedLane
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Stale => {
            ForgeQueryOrdinaryNextStep::RefreshBasis
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::RebindRequired => {
            ForgeQueryOrdinaryNextStep::RebindContext
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Unsupported => {
            ForgeQueryOrdinaryNextStep::CheckSupport
        }
        ForgeQueryContributionComposedOrchestrationCheckedKind::Failed => {
            ForgeQueryOrdinaryNextStep::EscalateFailure
        }
    }
}
