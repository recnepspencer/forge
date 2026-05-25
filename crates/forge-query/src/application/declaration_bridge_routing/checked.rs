use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationEnvelopeDeferred, ForgeQueryDeclarationEnvelopeDenied,
    ForgeQueryDeclarationEnvelopeFailed, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryLowerAuthorityRouteFamily,
};

use super::{
    artifact::{ForgeQueryDeclarationBridgeRouting, ForgeQueryDeclarationBridgeRoutingClass},
    contract::ForgeQueryDeclarationBridgeRoutingSupportStatus,
    denial::{
        ForgeQueryDeclarationBridgeRoutingDeferred, ForgeQueryDeclarationBridgeRoutingDenialCause,
        ForgeQueryDeclarationBridgeRoutingDenied, ForgeQueryDeclarationBridgeRoutingFailed,
    },
    digest::derive_bridge_routing_digest,
    explain::ForgeQueryDeclarationBridgeRoutingExplanation,
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
        ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(checked) => match checked {
            ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                forge_query_checked_declaration_bridge_routing(
                    ForgeQueryDeclarationBridgeRoutingInput::enveloped(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                forge_query_checked_declaration_bridge_routing(
                    ForgeQueryDeclarationBridgeRoutingInput::deferred(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                forge_query_checked_declaration_bridge_routing(
                    ForgeQueryDeclarationBridgeRoutingInput::denied(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                forge_query_checked_declaration_bridge_routing(
                    ForgeQueryDeclarationBridgeRoutingInput::failed(envelope),
                )
            }
        },
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => {
            let Some(contract) = I::Family::bridge_continuation_contract() else {
                return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                    ForgeQueryDeclarationBridgeRoutingDenied::new(
                        envelope,
                        ForgeQueryDeclarationBridgeRoutingDenialCause::UnsupportedContinuationMode,
                    ),
                );
            };
            let Some(route_plan) = envelope.route_plan() else {
                return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                    ForgeQueryDeclarationBridgeRoutingDenied::new(
                        envelope,
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
                        ForgeQueryDeclarationBridgeRoutingDenialCause::NonBridgeRoutePlan,
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
            let explanation = ForgeQueryDeclarationBridgeRoutingExplanation::new(
                envelope.explain().crossing_posture(),
                continuation_request.mode(),
                continuation_request.truth_context(),
                continuation_family,
                binding.surface(),
                retained_truths,
                envelope.explain().route_governing_reason().map(ToOwned::to_owned),
                envelope.route_denial_cause(),
                envelope.explain().receipt_governing_reason().to_string(),
                envelope.receipt_denial_cause(),
                envelope.evidence_origin(),
                mixed_origin,
            );
            ForgeQueryDeclarationBridgeRoutingChecked::Routed(
                ForgeQueryDeclarationBridgeRouting::new(
                    class,
                    continuation_request,
                    continuation_family,
                    binding,
                    envelope,
                    digest,
                    explanation,
                ),
            )
        }
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
        ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(checked) => match checked {
            ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                checked_enveloped_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    support_status,
                    envelope,
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                checked_non_success_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    ForgeQueryDeclarationBridgeRoutingInput::deferred(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                checked_non_success_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    ForgeQueryDeclarationBridgeRoutingInput::denied(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                checked_non_success_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    ForgeQueryDeclarationBridgeRoutingInput::failed(envelope),
                )
            }
        },
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => {
            checked_enveloped_on_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                support_status,
                envelope,
            )
        }
        other => checked_non_success_on_handle(
            handle_identity_digest,
            operating_context_identity_digest,
            other,
        ),
    }
}

fn checked_enveloped_on_handle<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    support_status: ForgeQueryDeclarationBridgeRoutingSupportStatus,
    envelope: crate::application::ForgeQueryDeclarationEnvelope<D, I>,
) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I> {
    if envelope.handle_identity_digest() != handle_identity_digest
        || envelope.operating_context_identity_digest() != operating_context_identity_digest
    {
        return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
            ForgeQueryDeclarationBridgeRoutingDenied::new(
                envelope,
                ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
            ),
        );
    }
    if support_status != ForgeQueryDeclarationBridgeRoutingSupportStatus::Admitted {
        return ForgeQueryDeclarationBridgeRoutingChecked::Denied(
            ForgeQueryDeclarationBridgeRoutingDenied::new(
                envelope,
                ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeAuthorityUnavailable,
            ),
        );
    }
    forge_query_checked_declaration_bridge_routing(
        ForgeQueryDeclarationBridgeRoutingInput::enveloped(envelope),
    )
}

fn checked_non_success_on_handle<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    input: ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I> {
    if !subject_matches_handle(
        handle_identity_digest,
        operating_context_identity_digest,
        &input,
    ) {
        return match input {
            ForgeQueryDeclarationBridgeRoutingInput::Deferred(envelope) => {
                ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                    ForgeQueryDeclarationBridgeRoutingDenied::new(
                        envelope.into_envelope(),
                        ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                    ),
                )
            }
            ForgeQueryDeclarationBridgeRoutingInput::Denied(envelope) => {
                ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                    ForgeQueryDeclarationBridgeRoutingDenied::new(
                        envelope.into_envelope(),
                        ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                    ),
                )
            }
            ForgeQueryDeclarationBridgeRoutingInput::Failed(envelope) => {
                ForgeQueryDeclarationBridgeRoutingChecked::Denied(
                    ForgeQueryDeclarationBridgeRoutingDenied::new(
                        envelope.into_envelope(),
                        ForgeQueryDeclarationBridgeRoutingDenialCause::BridgeEnvelopeMismatch,
                    ),
                )
            }
            _ => unreachable!("covered envelopes use the covered-handle path"),
        };
    }
    forge_query_checked_declaration_bridge_routing(input)
}

fn subject_matches_handle<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    input: &ForgeQueryDeclarationBridgeRoutingInput<D, I>,
) -> bool {
    let envelope = match input {
        ForgeQueryDeclarationBridgeRoutingInput::Enveloped(envelope) => envelope,
        ForgeQueryDeclarationBridgeRoutingInput::Deferred(envelope) => envelope.envelope(),
        ForgeQueryDeclarationBridgeRoutingInput::Denied(envelope) => envelope.envelope(),
        ForgeQueryDeclarationBridgeRoutingInput::Failed(envelope) => envelope.envelope(),
        ForgeQueryDeclarationBridgeRoutingInput::EnvelopeChecked(_) => {
            unreachable!("checked input is lowered before handle matching")
        }
    };
    envelope.handle_identity_digest() == handle_identity_digest
        && envelope.operating_context_identity_digest() == operating_context_identity_digest
}
