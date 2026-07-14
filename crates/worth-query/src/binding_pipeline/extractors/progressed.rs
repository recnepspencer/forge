use crate::application::{
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryInstalledDomainDeclarationContext,
};
use crate::binding_pipeline::{
    bind_envelope_from_target_on_handle, bind_receipt_from_target_on_handle,
    bind_route_from_target_on_handle, WorthQueryBindingUnavailable,
    WorthQueryEnvelopeBindingRequest, WorthQueryReceiptBindingRequest,
    WorthQueryResolveEnvelopeFromTargetRequest, WorthQueryResolveReceiptFromTargetRequest,
    WorthQueryResolveRouteFromTargetRequest, WorthQueryRouteBindingRequest,
};
use crate::target_binding::WorthQueryBindingTargetWitness;

use super::common::{ambiguous, denied_on_fit, digest_for, fit_allowed, fit_rank};

macro_rules! bind_from_progression_context {
    ($fn_name:ident, $request_ty:ident, $source_enum:ident, $resolver_ty:ident, $resolver_call:ident, $kind:literal, $out:ty) => {
        pub(crate) fn $fn_name<
            D: WorthQueryDomainEntryMarker,
            C: WorthQueryDomainOperatingContext<D>,
            I: WorthQueryDeclarationInput<D>,
        >(
            handle: &WorthQueryInstalledDomainDeclarationContext<D, C>,
            request: $request_ty<D, I>,
        ) -> crate::binding_pipeline::WorthQueryBindingTranscript<$out> {
            let (candidates, contract, allowed_sources, allow_superset, partial_narrowing, route_intent) =
                request.into_parts();
            let mut selected = Vec::new();
            let mut records = Vec::new();
            let mut best_denied = None;
            for candidate in candidates {
                let (label, source_kind, specificity, progressed) = candidate.into_parts();
                if !allowed_sources.is_empty() && !allowed_sources.contains(&source_kind) {
                    continue;
                }
                let coverage = progressed.reviewed_aspect_coverage().clone();
                let fit = coverage.fit_against(&contract);
                let binding_target = progressed.binding_target().into_erased_target();
                records.push(crate::binding_pipeline::WorthQueryBindingCandidateRecord::new(
                    label,
                    source_kind,
                    specificity,
                    Some(&binding_target),
                ));
                if fit_allowed(fit, allow_superset) {
                    selected.push((fit_rank(fit), usize::MAX - coverage.present().len(), specificity, progressed));
                } else if best_denied
                    .as_ref()
                    .map(|(best_fit, _): &(crate::application::WorthQueryDeclarationAspectFit, crate::application::WorthQueryDeclarationAspectCoverage)| fit_rank(fit) > fit_rank(*best_fit))
                    .unwrap_or(true)
                {
                    best_denied = Some((fit, coverage));
                }
            }
            if selected.is_empty() {
                if let Some((fit, coverage)) = best_denied {
                    return denied_on_fit(
                        I::Family::semantic_family_key(),
                        $kind,
                        contract,
                        coverage,
                        fit,
                        partial_narrowing,
                        records,
                    );
                }
                return crate::binding_pipeline::WorthQueryBindingTranscript::new(
                    crate::binding_pipeline::WorthQueryBindingRequestDescriptor::new(
                        I::Family::semantic_family_key(),
                        $kind,
                        contract.clone(),
                    ),
                    crate::binding_pipeline::WorthQueryBindingOutcome::Unavailable(
                        WorthQueryBindingUnavailable::new(
                            "no admissible progression context candidates were available",
                        ),
                    ),
                    records,
                    Vec::new(),
                    None,
                    vec![crate::binding_pipeline::WorthQueryBindingNarrowingDecision::new(
                        "binding stopped because no allowed progression candidates were available",
                    )],
                    None,
                    digest_for(
                        I::Family::semantic_family_key(),
                        $kind,
                        &contract,
                        "unavailable",
                    ),
                    crate::binding_pipeline::WorthQueryBindingLinkedArtifacts::new(),
                );
            }
            selected.sort_by(|left, right| right.0.cmp(&left.0).then(right.1.cmp(&left.1)).then(right.2.cmp(&left.2)));
            if selected.len() > 1
                && selected[0].0 == selected[1].0
                && selected[0].1 == selected[1].1
                && selected[0].2 == selected[1].2
            {
                return ambiguous(I::Family::semantic_family_key(), $kind, contract, records);
            }
            let progressed = selected.remove(0).3;
            $resolver_call(
                handle,
                match route_intent {
                    Some(intent) => $resolver_ty::new(
                        crate::binding_pipeline::$source_enum::Progression(progressed),
                        contract,
                    )
                    .with_route_intent(intent)
                    .with_partial_denial_if(!partial_narrowing)
                    .with_exact_fit_only_if(!allow_superset),
                    None => $resolver_ty::new(
                        crate::binding_pipeline::$source_enum::Progression(progressed),
                        contract,
                    )
                    .with_partial_denial_if(!partial_narrowing)
                    .with_exact_fit_only_if(!allow_superset),
                },
            )
        }
    };
}

trait ResolverRequestFlags<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> {
    fn with_partial_denial_if(self, deny: bool) -> Self;
    fn with_exact_fit_only_if(self, exact: bool) -> Self;
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> ResolverRequestFlags<D, I>
    for WorthQueryResolveRouteFromTargetRequest<D, I>
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
impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> ResolverRequestFlags<D, I>
    for WorthQueryResolveReceiptFromTargetRequest<D, I>
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
impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> ResolverRequestFlags<D, I>
    for WorthQueryResolveEnvelopeFromTargetRequest<D, I>
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

bind_from_progression_context!(
    bind_route_request_from_context_on_handle,
    WorthQueryRouteBindingRequest,
    WorthQueryRouteResolverSubject,
    WorthQueryResolveRouteFromTargetRequest,
    bind_route_from_target_on_handle,
    "bind_route_request_from_context",
    crate::application::WorthQueryDeclarationRoutePlanInput<D, I>
);
bind_from_progression_context!(
    bind_receipt_request_from_context_on_handle,
    WorthQueryReceiptBindingRequest,
    WorthQueryReceiptResolverSubject,
    WorthQueryResolveReceiptFromTargetRequest,
    bind_receipt_from_target_on_handle,
    "bind_receipt_request_from_context",
    crate::application::WorthQueryDeclarationReceiptInput<D, I>
);
bind_from_progression_context!(
    bind_envelope_request_from_context_on_handle,
    WorthQueryEnvelopeBindingRequest,
    WorthQueryEnvelopeResolverSubject,
    WorthQueryResolveEnvelopeFromTargetRequest,
    bind_envelope_from_target_on_handle,
    "bind_envelope_request_from_context",
    crate::application::WorthQueryDeclarationEnvelopeInput<D, I>
);
