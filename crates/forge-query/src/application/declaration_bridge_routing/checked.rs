use crate::application::{
    ForgeQueryDeclarationAspectFit, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeDeferred,
    ForgeQueryDeclarationEnvelopeDenied, ForgeQueryDeclarationEnvelopeFailed,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryLowerAuthorityRouteFamily,
};

use super::{
    artifact::{ForgeQueryDeclarationBridgeRouting, ForgeQueryDeclarationBridgeRoutingClass},
    aspect_gate::BridgeAuthorityAspectGate,
    contract::ForgeQueryDeclarationBridgeRoutingSupportStatus,
    denial::{
        ForgeQueryDeclarationBridgeRoutingDeferred, ForgeQueryDeclarationBridgeRoutingDenialCause,
        ForgeQueryDeclarationBridgeRoutingDenied, ForgeQueryDeclarationBridgeRoutingFailed,
    },
    digest::derive_bridge_routing_digest,
    explain::ForgeQueryDeclarationBridgeRoutingExplanation,
    handle_gate::{envelope_matches_handle, subject_matches_handle},
    lower::forge_query_lower_bridge_binding,
};

pub enum ForgeQueryDeclarationBridgeRoutingInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Enveloped(ForgeQueryDeclarationEnvelope<D, I>),
    Deferred(ForgeQueryDeclarationEnvelopeDeferred<D, I>),
    Denied(ForgeQueryDeclarationEnvelopeDenied<D, I>),
    Failed(ForgeQueryDeclarationEnvelopeFailed<D, I>),
    EnvelopeChecked(ForgeQueryDeclarationEnvelopeChecked<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationBridgeRoutingInput<D, I>
{
    pub fn enveloped(envelope: ForgeQueryDeclarationEnvelope<D, I>) -> Self {
        Self::Enveloped(envelope)
    }

    pub fn deferred(envelope: ForgeQueryDeclarationEnvelopeDeferred<D, I>) -> Self {
        Self::Deferred(envelope)
    }

    pub fn denied(envelope: ForgeQueryDeclarationEnvelopeDenied<D, I>) -> Self {
        Self::Denied(envelope)
    }

    pub fn failed(envelope: ForgeQueryDeclarationEnvelopeFailed<D, I>) -> Self {
        Self::Failed(envelope)
    }

    pub fn envelope_checked(checked: ForgeQueryDeclarationEnvelopeChecked<D, I>) -> Self {
        Self::EnvelopeChecked(checked)
    }
}

pub enum ForgeQueryDeclarationBridgeRoutingChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Routed(ForgeQueryDeclarationBridgeRouting<D, I>),
    Deferred(ForgeQueryDeclarationBridgeRoutingDeferred<D, I>),
    Denied(ForgeQueryDeclarationBridgeRoutingDenied<D, I>),
    Failed(ForgeQueryDeclarationBridgeRoutingFailed<D, I>),
}

pub(crate) fn forge_query_checked_declaration_bridge_routing<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I> {
    match input {
        ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(checked) => {
            forge_query_checked_declaration_bridge_routing(lower_checked_input(checked))
        }
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => route_enveloped(envelope),
        ForgeQueryDeclarationBridgeRoutingInput::Deferred(envelope) => {
            let reason = envelope.reason();
            ForgeQueryDeclarationBridgeRoutingChecked::Deferred(
                ForgeQueryDeclarationBridgeRoutingDeferred::new(
                    envelope.into_envelope(),
                    reason,
                ),
            )
        }
        ForgeQueryDeclarationBridgeRoutingInput::Denied(envelope) => {
            ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    ForgeQueryDeclarationBridgeRoutingDenialCause::EnvelopeNotCoveredForBridgeRouting,
                ),
            )
        }
        ForgeQueryDeclarationBridgeRoutingInput::Failed(envelope) => {
            let reason = envelope.reason();
            ForgeQueryDeclarationBridgeRoutingChecked::Failed(
                ForgeQueryDeclarationBridgeRoutingFailed::new(
                    envelope.into_envelope(),
                    reason,
                ),
            )
        }
    }
}

pub(crate) fn forge_query_checked_declaration_bridge_routing_on_handle<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    support_status: ForgeQueryDeclarationBridgeRoutingSupportStatus,
    input: ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I> {
    match input {
        ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(checked) => {
            forge_query_checked_declaration_bridge_routing_on_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                support_status,
                lower_checked_input(checked),
            )
        }
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => {
            if !envelope_matches_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                &envelope,
            ) {
                return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                    ForgeQueryDeclarationBridgeRoutingDenied::new(
                        envelope,
                        I::Family::bridge_continuation_contract()
                            .map(|contract| contract.request()),
                        I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                        ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                    ),
                );
            }
            if support_status != ForgeQueryDeclarationBridgeRoutingSupportStatus::Admitted {
                return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                    ForgeQueryDeclarationBridgeRoutingDenied::new(
                        envelope,
                        I::Family::bridge_continuation_contract()
                            .map(|contract| contract.request()),
                        I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                        ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeAuthorityUnavailable,
                    ),
                );
            }
            route_enveloped(envelope)
        }
        other => {
            if !subject_matches_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                &other,
            ) {
                return deny_non_success_mismatch(other);
            }
            forge_query_checked_declaration_bridge_routing(other)
        }
    }
}

fn route_enveloped<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I> {
    let Some(contract) = I::Family::bridge_continuation_contract() else {
        return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
            ForgeQueryDeclarationBridgeRoutingDenied::new(
                envelope,
                None,
                None,
                ForgeQueryDeclarationBridgeRoutingDenialCause::UnsupportedContinuationMode,
            ),
        );
    };
    let denied_request = Some(contract.request());
    let denied_family = Some(contract.family());
    let Some(route_plan) = envelope.route_plan() else {
        return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
            ForgeQueryDeclarationBridgeRoutingDenied::new(
                envelope,
                denied_request,
                denied_family,
                ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
            ),
        );
    };
    if !route_plan
        .route_families()
        .contains(&ForgeQueryLowerAuthorityRouteFamily::Bridge)
    {
        return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
            ForgeQueryDeclarationBridgeRoutingDenied::new(
                envelope,
                denied_request,
                denied_family,
                ForgeQueryDeclarationBridgeRoutingDenialCause::NonBridgeRoutePlan,
            ),
        );
    }

    let authority_aspects =
        BridgeAuthorityAspectGate::from_envelope(&envelope, contract.required_aspects());
    match authority_aspects.fit() {
        ForgeQueryDeclarationAspectFit::Conflict => {
            return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope,
                    denied_request,
                    denied_family,
                    ForgeQueryDeclarationBridgeRoutingDenialCause::AspectConflict,
                ),
            );
        }
        ForgeQueryDeclarationAspectFit::MissingRequired => {
            return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope,
                    denied_request,
                    denied_family,
                    ForgeQueryDeclarationBridgeRoutingDenialCause::MissingRequiredAspect,
                ),
            );
        }
        ForgeQueryDeclarationAspectFit::Partial => {
            return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope,
                    denied_request,
                    denied_family,
                    ForgeQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectGap,
                ),
            );
        }
        ForgeQueryDeclarationAspectFit::Exact
        | ForgeQueryDeclarationAspectFit::CompatibleSuperset => {}
    }
    if matches!(
        authority_aspects.mapping_fit(),
        ForgeQueryDeclarationAspectFit::Partial
    ) {
        return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
            ForgeQueryDeclarationBridgeRoutingDenied::new(
                envelope,
                denied_request,
                denied_family,
                ForgeQueryDeclarationBridgeRoutingDenialCause::AuthorityAspectGap,
            ),
        );
    }

    let mixed_origin = route_plan.route_count() > 1;
    let class = if mixed_origin {
        ForgeQueryDeclarationBridgeRoutingClass::MixedAuthorityBridgeContinuation
    } else {
        ForgeQueryDeclarationBridgeRoutingClass::ExclusiveBridgeContinuation
    };
    let (continuation_request, continuation_family, binding) =
        forge_query_lower_bridge_binding(&envelope, contract);
    let digest = derive_bridge_routing_digest(
        &envelope,
        class,
        continuation_request,
        continuation_family,
        binding.surface(),
        authority_aspects.contract(),
        authority_aspects.coverage(),
        authority_aspects.coverage_basis(),
        authority_aspects.fit(),
        authority_aspects.mapped_aspects(),
        authority_aspects.mapping_fit(),
        envelope.route_denial_cause(),
        envelope.receipt_denial_cause(),
    );
    let mut retained_truths = envelope.explain().retained_truths().to_vec();
    retained_truths.push(format!(
        "bridge-continuation-mode:{}",
        continuation_request.mode().as_str()
    ));
    retained_truths.push(format!(
        "bridge-truth-context:{}",
        continuation_request.truth_context().as_str()
    ));
    retained_truths.push(format!(
        "bridge-continuation-family:{}",
        continuation_family.as_str()
    ));
    retained_truths.push(format!("bridge-aspect-fit:{:?}", authority_aspects.fit()));
    retained_truths.push(format!(
        "bridge-mapping-fit:{:?}",
        authority_aspects.mapping_fit()
    ));
    let explanation = ForgeQueryDeclarationBridgeRoutingExplanation::new(
        envelope.explain().crossing_posture(),
        continuation_request.mode(),
        continuation_request.truth_context(),
        continuation_family,
        binding.surface(),
        retained_truths,
        envelope
            .explain()
            .route_governing_reason()
            .map(ToOwned::to_owned),
        envelope.route_denial_cause(),
        envelope.explain().receipt_governing_reason().to_string(),
        envelope.receipt_denial_cause(),
        envelope.evidence_origin(),
        mixed_origin,
    );
    ForgeQueryDeclarationBridgeRoutingChecked::Routed(ForgeQueryDeclarationBridgeRouting::new(
        class,
        continuation_request,
        continuation_family,
        binding,
        authority_aspects.contract().clone(),
        authority_aspects.coverage().clone(),
        authority_aspects.coverage_basis(),
        authority_aspects.fit(),
        authority_aspects.mapped_aspects().clone(),
        authority_aspects.mapping_fit(),
        envelope,
        digest,
        explanation,
    ))
}

fn lower_checked_input<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    checked: ForgeQueryDeclarationEnvelopeChecked<D, I>,
) -> ForgeQueryDeclarationBridgeRoutingInput<D, I> {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            ForgeQueryDeclarationBridgeRoutingInput::enveloped(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            ForgeQueryDeclarationBridgeRoutingInput::deferred(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            ForgeQueryDeclarationBridgeRoutingInput::denied(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            ForgeQueryDeclarationBridgeRoutingInput::failed(envelope)
        }
    }
}

fn deny_non_success_mismatch<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    input: ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I> {
    match input {
        ForgeQueryDeclarationBridgeRoutingInput::Deferred(envelope) => {
            ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                ),
            )
        }
        ForgeQueryDeclarationBridgeRoutingInput::Denied(envelope) => {
            ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                ),
            )
        }
        ForgeQueryDeclarationBridgeRoutingInput::Failed(envelope) => {
            ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                ForgeQueryDeclarationBridgeRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::bridge_continuation_contract().map(|contract| contract.request()),
                    I::Family::bridge_continuation_contract().map(|contract| contract.family()),
                    ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                ),
            )
        }
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(_)
        | ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(_) => {
            unreachable!("covered envelopes use the covered-handle path")
        }
    }
}
