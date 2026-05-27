use std::marker::PhantomData;

use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryContributionComposedCheckedTopologyKind,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

use super::artifact::ForgeQueryContributionComposedOrchestration;

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
    stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
    reason: String,
    linked_artifacts: ForgeQueryBindingLinkedArtifacts,
    contribution_digest: Option<String>,
    _marker: PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContributionComposedOrchestrationPosture<D, I>
{
    pub(crate) fn new(
        kind: ForgeQueryContributionComposedOrchestrationCheckedKind,
        stop_stage: ForgeQueryDeclarationEntryOrchestrationStage,
        reason: impl Into<String>,
        linked_artifacts: ForgeQueryBindingLinkedArtifacts,
        contribution_digest: Option<String>,
    ) -> Self {
        Self {
            kind,
            stop_stage,
            reason: reason.into(),
            linked_artifacts,
            contribution_digest,
            _marker: PhantomData,
        }
    }

    pub fn kind(&self) -> ForgeQueryContributionComposedOrchestrationCheckedKind {
        self.kind
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
            ForgeQueryOrdinaryOutcome::Deferred(to_posture(
                value,
                ForgeQueryOrdinaryPostureKind::Deferred,
                ForgeQueryOrdinaryNextStep::RetryLater,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Deferred,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value) => {
            ForgeQueryOrdinaryOutcome::Denied(to_posture(
                value,
                ForgeQueryOrdinaryPostureKind::Denied,
                ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::DeclarationDenied,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value) => {
            ForgeQueryOrdinaryOutcome::Denied(to_posture(
                value,
                ForgeQueryOrdinaryPostureKind::Denied,
                ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Stale(value) => {
            ForgeQueryOrdinaryOutcome::Stale(to_posture(
                value,
                ForgeQueryOrdinaryPostureKind::Stale,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Stale,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value) => {
            ForgeQueryOrdinaryOutcome::RebindRequired(to_posture(
                value,
                ForgeQueryOrdinaryPostureKind::RebindRequired,
                ForgeQueryOrdinaryNextStep::RebindContext,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::RebindRequired,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value) => {
            ForgeQueryOrdinaryOutcome::Unsupported(to_posture(
                value,
                ForgeQueryOrdinaryPostureKind::Unsupported,
                ForgeQueryOrdinaryNextStep::CheckSupport,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Unsupported,
            ))
        }
        ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => {
            ForgeQueryOrdinaryOutcome::Failed(to_posture(
                value,
                ForgeQueryOrdinaryPostureKind::Failed,
                ForgeQueryOrdinaryNextStep::InspectProofLane,
                ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Failed,
            ))
        }
    }
}

fn to_posture<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    value: ForgeQueryContributionComposedOrchestrationPosture<D, I>,
    kind: ForgeQueryOrdinaryPostureKind,
    next_step: ForgeQueryOrdinaryNextStep,
    topology_kind: ForgeQueryOrdinaryContributionComposedCheckedTopologyKind,
) -> ForgeQueryOrdinaryPosture {
    let topology = ForgeQueryOrdinaryCheckedTopology::contribution_composed(
        topology_kind,
        value.linked_artifacts,
        value.contribution_digest,
    );
    ForgeQueryOrdinaryPosture::new(value.reason, kind, next_step, topology)
}
