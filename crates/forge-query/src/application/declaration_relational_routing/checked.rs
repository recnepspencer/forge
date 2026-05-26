use crate::application::{
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryLowerAuthorityRouteFamily,
};

use super::{
    artifact::{
        ForgeQueryDeclarationRelationalRouting, ForgeQueryDeclarationRelationalRoutingClass,
    },
    contract::ForgeQueryDeclarationRelationalTruthRoutingSupportStatus,
    denial::{
        ForgeQueryDeclarationRelationalRoutingDeferred,
        ForgeQueryDeclarationRelationalRoutingDenialCause,
        ForgeQueryDeclarationRelationalRoutingDenied, ForgeQueryDeclarationRelationalRoutingFailed,
    },
    digest::derive_relational_routing_digest,
    explain::ForgeQueryDeclarationRelationalRoutingExplanation,
    input::ForgeQueryDeclarationRelationalRoutingInput,
    lower::forge_query_lower_relational_binding,
};

pub enum ForgeQueryDeclarationRelationalRoutingChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Routed(ForgeQueryDeclarationRelationalRouting<D, I>),
    Deferred(ForgeQueryDeclarationRelationalRoutingDeferred<D, I>),
    Denied(ForgeQueryDeclarationRelationalRoutingDenied<D, I>),
    Failed(ForgeQueryDeclarationRelationalRoutingFailed<D, I>),
}

pub(crate) fn forge_query_checked_declaration_relational_routing<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationRelationalRoutingInput<D, I>,
) -> ForgeQueryDeclarationRelationalRoutingChecked<D, I> {
    match input {
        ForgeQueryDeclarationRelationalRoutingInput::EnvelopeChecked(checked) => match checked {
            ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                forge_query_checked_declaration_relational_routing(
                    ForgeQueryDeclarationRelationalRoutingInput::enveloped(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                forge_query_checked_declaration_relational_routing(
                    ForgeQueryDeclarationRelationalRoutingInput::deferred(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                forge_query_checked_declaration_relational_routing(
                    ForgeQueryDeclarationRelationalRoutingInput::denied(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                forge_query_checked_declaration_relational_routing(
                    ForgeQueryDeclarationRelationalRoutingInput::failed(envelope),
                )
            }
        },
        ForgeQueryDeclarationRelationalRoutingInput::Enveloped(envelope) => {
            let Some(contract) = I::Family::relational_truth_contract() else {
                return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                    ForgeQueryDeclarationRelationalRoutingDenied::new(
                        envelope,
                        ForgeQueryDeclarationRelationalRoutingDenialCause::UnsupportedRelationalTruthClaim,
                    ),
                );
            };
            let Some(route_plan) = envelope.route_plan() else {
                return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                    ForgeQueryDeclarationRelationalRoutingDenied::new(
                        envelope,
                        ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch,
                    ),
                );
            };
            if !route_plan
                .route_families()
                .contains(&ForgeQueryLowerAuthorityRouteFamily::Relational)
            {
                return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                    ForgeQueryDeclarationRelationalRoutingDenied::new(
                        envelope,
                        ForgeQueryDeclarationRelationalRoutingDenialCause::NonRelationalRoutePlan,
                    ),
                );
            }
            let mixed_origin = route_plan.route_count() > 1;
            let class = if mixed_origin {
                ForgeQueryDeclarationRelationalRoutingClass::MixedAuthorityRelationalTruth
            } else {
                ForgeQueryDeclarationRelationalRoutingClass::ExclusiveRelationalTruth
            };
            let (truth_claim, authority_family, binding) =
                forge_query_lower_relational_binding(&envelope, contract);
            let digest = derive_relational_routing_digest(
                &envelope,
                class,
                truth_claim,
                authority_family,
                binding.surface(),
                envelope.route_denial_cause(),
                envelope.receipt_denial_cause(),
            );
            let mut retained_truths = envelope.explain().retained_truths().to_vec();
            retained_truths.push(format!(
                "relational-truth-claim:{}",
                truth_claim.as_str()
            ));
            retained_truths.push(format!(
                "relational-authority-family:{}",
                authority_family.as_str()
            ));
            let explanation = ForgeQueryDeclarationRelationalRoutingExplanation::new(
                envelope.explain().crossing_posture(),
                truth_claim,
                authority_family,
                binding.surface(),
                retained_truths,
                envelope.explain().route_governing_reason().map(ToOwned::to_owned),
                envelope.route_denial_cause(),
                envelope.explain().receipt_governing_reason().to_string(),
                envelope.receipt_denial_cause(),
                envelope.evidence_origin(),
                mixed_origin,
            );
            ForgeQueryDeclarationRelationalRoutingChecked::Routed(
                ForgeQueryDeclarationRelationalRouting::new(
                    class,
                    truth_claim,
                    authority_family,
                    binding,
                    envelope,
                    digest,
                    explanation,
                ),
            )
        }
        ForgeQueryDeclarationRelationalRoutingInput::Deferred(envelope) => {
            let reason = envelope.reason();
            ForgeQueryDeclarationRelationalRoutingChecked::Deferred(
                ForgeQueryDeclarationRelationalRoutingDeferred::new(
                    envelope.into_envelope(),
                    reason,
                ),
            )
        }
        ForgeQueryDeclarationRelationalRoutingInput::Denied(envelope) => {
            ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                ForgeQueryDeclarationRelationalRoutingDenied::new(
                    envelope.into_envelope(),
                    ForgeQueryDeclarationRelationalRoutingDenialCause::EnvelopeNotCoveredForRelationalRouting,
                ),
            )
        }
        ForgeQueryDeclarationRelationalRoutingInput::Failed(envelope) => {
            let reason = envelope.reason();
            ForgeQueryDeclarationRelationalRoutingChecked::Failed(
                ForgeQueryDeclarationRelationalRoutingFailed::new(
                    envelope.into_envelope(),
                    reason,
                ),
            )
        }
    }
}

pub(crate) fn forge_query_checked_declaration_relational_routing_on_handle<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    support_status: ForgeQueryDeclarationRelationalTruthRoutingSupportStatus,
    input: ForgeQueryDeclarationRelationalRoutingInput<D, I>,
) -> ForgeQueryDeclarationRelationalRoutingChecked<D, I> {
    match input {
        ForgeQueryDeclarationRelationalRoutingInput::EnvelopeChecked(checked) => match checked {
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
                    ForgeQueryDeclarationRelationalRoutingInput::deferred(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                checked_non_success_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    ForgeQueryDeclarationRelationalRoutingInput::denied(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                checked_non_success_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    ForgeQueryDeclarationRelationalRoutingInput::failed(envelope),
                )
            }
        },
        ForgeQueryDeclarationRelationalRoutingInput::Enveloped(envelope) => {
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
    support_status: ForgeQueryDeclarationRelationalTruthRoutingSupportStatus,
    envelope: crate::application::ForgeQueryDeclarationEnvelope<D, I>,
) -> ForgeQueryDeclarationRelationalRoutingChecked<D, I> {
    if envelope.handle_identity_digest() != handle_identity_digest
        || envelope.operating_context_identity_digest() != operating_context_identity_digest
    {
        return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
            ForgeQueryDeclarationRelationalRoutingDenied::new(
                envelope,
                ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch,
            ),
        );
    }
    if support_status != ForgeQueryDeclarationRelationalTruthRoutingSupportStatus::Admitted {
        return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
            ForgeQueryDeclarationRelationalRoutingDenied::new(
                envelope,
                ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalAuthorityUnavailable,
            ),
        );
    }
    forge_query_checked_declaration_relational_routing(
        ForgeQueryDeclarationRelationalRoutingInput::enveloped(envelope),
    )
}

fn checked_non_success_on_handle<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    input: ForgeQueryDeclarationRelationalRoutingInput<D, I>,
) -> ForgeQueryDeclarationRelationalRoutingChecked<D, I> {
    if !subject_matches_handle(
        handle_identity_digest,
        operating_context_identity_digest,
        &input,
    ) {
        return match input {
            ForgeQueryDeclarationRelationalRoutingInput::Deferred(envelope) => {
                ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                    ForgeQueryDeclarationRelationalRoutingDenied::new(
                        envelope.into_envelope(),
                        ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch,
                    ),
                )
            }
            ForgeQueryDeclarationRelationalRoutingInput::Denied(envelope) => {
                ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                    ForgeQueryDeclarationRelationalRoutingDenied::new(
                        envelope.into_envelope(),
                        ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch,
                    ),
                )
            }
            ForgeQueryDeclarationRelationalRoutingInput::Failed(envelope) => {
                ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                    ForgeQueryDeclarationRelationalRoutingDenied::new(
                        envelope.into_envelope(),
                        ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch,
                    ),
                )
            }
            _ => unreachable!("covered envelopes use the covered-handle path"),
        };
    }
    forge_query_checked_declaration_relational_routing(input)
}

fn subject_matches_handle<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    input: &ForgeQueryDeclarationRelationalRoutingInput<D, I>,
) -> bool {
    let envelope = match input {
        ForgeQueryDeclarationRelationalRoutingInput::Enveloped(envelope) => envelope,
        ForgeQueryDeclarationRelationalRoutingInput::Deferred(envelope) => envelope.envelope(),
        ForgeQueryDeclarationRelationalRoutingInput::Denied(envelope) => envelope.envelope(),
        ForgeQueryDeclarationRelationalRoutingInput::Failed(envelope) => envelope.envelope(),
        ForgeQueryDeclarationRelationalRoutingInput::EnvelopeChecked(_) => {
            unreachable!("checked input is lowered before handle matching")
        }
    };
    envelope.handle_identity_digest() == handle_identity_digest
        && envelope.operating_context_identity_digest() == operating_context_identity_digest
}
