use crate::application::{
    aspect_coverage_from_publication, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext,
};
use crate::binding_pipeline::{
    bind_continuation_from_target_on_handle, WorthQueryBindingCandidateRecord,
    WorthQueryBindingNarrowingDecision, WorthQueryBindingOutcome,
    WorthQueryBindingRequestDescriptor, WorthQueryBindingTranscript, WorthQueryBindingUnavailable,
    WorthQueryContinuationBindingInput, WorthQueryContinuationBindingRequest,
    WorthQueryResolveContinuationFromTargetRequest,
};
use crate::target_binding::WorthQueryBindingTargetWitness;

use super::common::{ambiguous, denied_on_fit, digest_for, fit_allowed, fit_rank};

pub(crate) fn bind_continuation_request_from_context_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryInstalledDomainDeclarationContext<D, C>,
    request: WorthQueryContinuationBindingRequest<D, I>,
) -> WorthQueryBindingTranscript<WorthQueryContinuationBindingInput<D, I>> {
    let (candidates, contract, allowed_sources, allow_superset, partial_narrowing, bridge_request) =
        request.into_parts();
    let mut selected = Vec::new();
    let mut records = Vec::new();
    let mut best_denied = None;
    for candidate in candidates {
        let (label, source_kind, specificity, envelope) = candidate.into_parts();
        if !allowed_sources.is_empty() && !allowed_sources.contains(&source_kind) {
            continue;
        }
        let coverage = aspect_coverage_from_publication(envelope.aspect_publication());
        let fit = coverage.fit_against(&contract);
        let binding_target = envelope.binding_target().into_erased_target();
        records.push(WorthQueryBindingCandidateRecord::new(
            label,
            source_kind,
            specificity,
            Some(&binding_target),
        ));
        if fit_allowed(fit, allow_superset) {
            selected.push((
                fit_rank(fit),
                usize::MAX - coverage.present().len(),
                specificity,
                envelope,
            ));
        } else if best_denied
            .as_ref()
            .map(
                |(best_fit, _): &(
                    crate::application::WorthQueryDeclarationAspectFit,
                    crate::application::WorthQueryDeclarationAspectCoverage,
                )| fit_rank(fit) > fit_rank(*best_fit),
            )
            .unwrap_or(true)
        {
            best_denied = Some((fit, coverage));
        }
    }
    if selected.is_empty() {
        if let Some((fit, coverage)) = best_denied {
            return denied_on_fit(
                I::Family::semantic_family_key(),
                "bind_continuation_request_from_context",
                contract,
                coverage,
                fit,
                partial_narrowing,
                records,
            );
        }
        return WorthQueryBindingTranscript::new(
            WorthQueryBindingRequestDescriptor::new(
                I::Family::semantic_family_key(),
                "bind_continuation_request_from_context",
                contract.clone(),
            ),
            WorthQueryBindingOutcome::Unavailable(WorthQueryBindingUnavailable::new(
                "no admissible envelope context candidates were available",
            )),
            records,
            Vec::new(),
            None,
            vec![WorthQueryBindingNarrowingDecision::new(
                "binding stopped because no allowed envelope candidates were available",
            )],
            None,
            digest_for(
                I::Family::semantic_family_key(),
                "bind_continuation_request_from_context",
                &contract,
                "unavailable",
            ),
            crate::binding_pipeline::WorthQueryBindingLinkedArtifacts::new(),
        );
    }
    selected.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then(right.1.cmp(&left.1))
            .then(right.2.cmp(&left.2))
    });
    if selected.len() > 1
        && selected[0].0 == selected[1].0
        && selected[0].1 == selected[1].1
        && selected[0].2 == selected[1].2
    {
        return ambiguous(
            I::Family::semantic_family_key(),
            "bind_continuation_request_from_context",
            contract,
            records,
        );
    }
    let envelope = selected.remove(0).3;
    let request = match bridge_request {
        Some(bridge_request) => {
            WorthQueryResolveContinuationFromTargetRequest::new(envelope, contract)
                .with_bridge_request(bridge_request)
                .with_partial_denial_if(!partial_narrowing)
                .with_exact_fit_only_if(!allow_superset)
        }
        None => WorthQueryResolveContinuationFromTargetRequest::new(envelope, contract)
            .with_partial_denial_if(!partial_narrowing)
            .with_exact_fit_only_if(!allow_superset),
    };
    bind_continuation_from_target_on_handle(handle, request)
}

trait ContinuationRequestFlags<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> {
    fn with_partial_denial_if(self, deny: bool) -> Self;
    fn with_exact_fit_only_if(self, exact: bool) -> Self;
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    ContinuationRequestFlags<D, I> for WorthQueryResolveContinuationFromTargetRequest<D, I>
{
    fn with_partial_denial_if(self, deny: bool) -> Self {
        if deny {
            self.with_partial_denial()
        } else {
            self
        }
    }

    fn with_exact_fit_only_if(self, exact: bool) -> Self {
        if exact {
            self.with_exact_fit_only()
        } else {
            self
        }
    }
}
