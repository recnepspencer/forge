use crate::application::{
    dispatch_graph_obligations_for_orchestration, WorthQueryAdmittedConfiguredDomainHandle,
    WorthQueryDeclarationEntryOrchestrationStage, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryGraphObligationOrchestrationBoundary, WorthQueryGraphObligationOrchestrationDispatch,
};
use crate::binding_pipeline::WorthQueryBindingLinkedArtifacts;

use super::super::declaration_record::declaration_aspect_record_from_lowering;
use super::super::lower::DeclarationLowering;
use super::super::mapping::composed_outcome;
use super::super::outcome::{
    WorthQueryContributionComposedOrchestrationCheckedKind,
    WorthQueryContributionComposedOrchestrationOutcome,
};

pub(super) enum ContributionOrchestrationGraphDispatch<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Continue(Option<WorthQueryGraphObligationOrchestrationDispatch>),
    Stop(WorthQueryContributionComposedOrchestrationOutcome<D, I>),
}

pub(super) fn dispatch_contribution_orchestration_graph_obligations<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    declaration: &DeclarationLowering<D, I>,
    linked: &WorthQueryBindingLinkedArtifacts,
) -> ContributionOrchestrationGraphDispatch<D, I> {
    let graph_obligation_dispatch = match dispatch_graph_obligations_for_orchestration(
        WorthQueryGraphObligationOrchestrationBoundary::ContributionComposed,
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
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    declaration: &DeclarationLowering<D, I>,
    linked: &WorthQueryBindingLinkedArtifacts,
    reason: String,
) -> WorthQueryContributionComposedOrchestrationOutcome<D, I> {
    composed_outcome(
        WorthQueryContributionComposedOrchestrationCheckedKind::Failed,
        super::super::composition::WorthQueryContributionComposedStop::Failed,
        WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        reason,
        linked.clone(),
        None,
        Some(declaration_aspect_record_from_lowering(declaration)),
        None,
    )
}

fn graph_obligation_dispatch_denial_outcome<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    declaration: &DeclarationLowering<D, I>,
    linked: &WorthQueryBindingLinkedArtifacts,
) -> WorthQueryContributionComposedOrchestrationOutcome<D, I> {
    composed_outcome(
        WorthQueryContributionComposedOrchestrationCheckedKind::DeclarationDenied,
        super::super::composition::WorthQueryContributionComposedStop::DeclarationDenied,
        WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        "contribution-composed orchestration denied by graph obligation dispatch",
        linked.clone(),
        None,
        Some(declaration_aspect_record_from_lowering(declaration)),
        None,
    )
}
