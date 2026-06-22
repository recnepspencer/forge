use crate::application::{
    dispatch_graph_obligations_for_orchestration, ForgeQueryAdmittedConfiguredDomainHandle,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryGraphObligationOrchestrationBoundary, ForgeQueryGraphObligationOrchestrationDispatch,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;

use super::super::declaration_record::declaration_aspect_record_from_lowering;
use super::super::lower::DeclarationLowering;
use super::super::mapping::composed_outcome;
use super::super::outcome::{
    ForgeQueryContributionComposedOrchestrationCheckedKind,
    ForgeQueryContributionComposedOrchestrationOutcome,
};

pub(super) enum ContributionOrchestrationGraphDispatch<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Continue(Option<ForgeQueryGraphObligationOrchestrationDispatch>),
    Stop(ForgeQueryContributionComposedOrchestrationOutcome<D, I>),
}

pub(super) fn dispatch_contribution_orchestration_graph_obligations<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: &DeclarationLowering<D, I>,
    linked: &ForgeQueryBindingLinkedArtifacts,
) -> ContributionOrchestrationGraphDispatch<D, I> {
    let graph_obligation_dispatch = match dispatch_graph_obligations_for_orchestration(
        ForgeQueryGraphObligationOrchestrationBoundary::ContributionComposed,
        handle.operating_context_identity_digest(),
        I::Family::orchestration_graph_touch_descriptor(),
        I::Family::orchestration_graph_touch_collection(),
        I::Family::orchestration_graph_obligation_registrations(),
    ) {
        Ok(dispatch) => dispatch,
        Err(error) => {
            let dispatch_error = error.clone();
            let outcome = graph_obligation_dispatch_failure_outcome(
                declaration,
                linked,
                format!("graph obligation orchestration dispatch failed: {error:?}"),
            )
            .with_graph_obligation_dispatch_error(dispatch_error);
            return ContributionOrchestrationGraphDispatch::Stop(outcome);
        }
    };
    if graph_obligation_dispatch
        .as_ref()
        .is_some_and(|dispatch| dispatch.blocking_denial_projection().is_some())
    {
        let outcome = graph_obligation_dispatch_denial_outcome(declaration, linked)
            .with_graph_obligation_dispatch(graph_obligation_dispatch.clone());
        return ContributionOrchestrationGraphDispatch::Stop(outcome);
    }
    ContributionOrchestrationGraphDispatch::Continue(graph_obligation_dispatch)
}

fn graph_obligation_dispatch_failure_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    declaration: &DeclarationLowering<D, I>,
    linked: &ForgeQueryBindingLinkedArtifacts,
    reason: String,
) -> ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
    composed_outcome(
        ForgeQueryContributionComposedOrchestrationCheckedKind::Failed,
        super::super::composition::ForgeQueryContributionComposedStop::Failed,
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        reason,
        linked.clone(),
        None,
        Some(declaration_aspect_record_from_lowering(declaration)),
        None,
    )
}

fn graph_obligation_dispatch_denial_outcome<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    declaration: &DeclarationLowering<D, I>,
    linked: &ForgeQueryBindingLinkedArtifacts,
) -> ForgeQueryContributionComposedOrchestrationOutcome<D, I> {
    composed_outcome(
        ForgeQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
        super::super::composition::ForgeQueryContributionComposedStop::DeclarationDenied,
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        "contribution-composed orchestration denied by graph obligation dispatch",
        linked.clone(),
        None,
        Some(declaration_aspect_record_from_lowering(declaration)),
        None,
    )
}
