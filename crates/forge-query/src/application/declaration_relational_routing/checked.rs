use crate::application::{
    ForgeQueryDeclarationAspectFit, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryLowerAuthorityRouteFamily,
};

use super::{
    artifact::{
        ForgeQueryDeclarationRelationalRouting, ForgeQueryDeclarationRelationalRoutingClass,
    },
    aspect_gate::RelationalAuthorityAspectGate,
    contract::ForgeQueryDeclarationRelationalTruthRoutingSupportStatus,
    denial::{
        ForgeQueryDeclarationRelationalRoutingDeferred,
        ForgeQueryDeclarationRelationalRoutingDenialCause,
        ForgeQueryDeclarationRelationalRoutingDenied, ForgeQueryDeclarationRelationalRoutingFailed,
    },
    digest::derive_relational_routing_digest,
    explain::ForgeQueryDeclarationRelationalRoutingExplanation,
    handle_gate::{envelope_matches_handle, subject_matches_handle},
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
        ForgeQueryDeclarationRelationalRoutingInput::EnvelopeChecked(checked) => {
            forge_query_checked_declaration_relational_routing(lower_checked_input(checked))
        }
        ForgeQueryDeclarationRelationalRoutingInput::Enveloped(envelope) => route_enveloped(envelope),
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
                    I::Family::relational_truth_contract().map(|contract| contract.truth_claim()),
                    I::Family::relational_truth_contract()
                        .map(|contract| contract.authority_family()),
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
        ForgeQueryDeclarationRelationalRoutingInput::EnvelopeChecked(checked) => {
            forge_query_checked_declaration_relational_routing_on_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                support_status,
                lower_checked_input(checked),
            )
        }
        ForgeQueryDeclarationRelationalRoutingInput::Enveloped(envelope) => {
            if !envelope_matches_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                &envelope,
            ) {
                return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                    ForgeQueryDeclarationRelationalRoutingDenied::new(
                        envelope,
                        I::Family::relational_truth_contract().map(|contract| contract.truth_claim()),
                        I::Family::relational_truth_contract()
                            .map(|contract| contract.authority_family()),
                        ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch,
                    ),
                );
            }
            if support_status != ForgeQueryDeclarationRelationalTruthRoutingSupportStatus::Admitted
            {
                return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                    ForgeQueryDeclarationRelationalRoutingDenied::new(
                        envelope,
                        I::Family::relational_truth_contract().map(|contract| contract.truth_claim()),
                        I::Family::relational_truth_contract()
                            .map(|contract| contract.authority_family()),
                        ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalAuthorityUnavailable,
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
            forge_query_checked_declaration_relational_routing(other)
        }
    }
}

fn route_enveloped<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    envelope: crate::application::ForgeQueryDeclarationEnvelope<D, I>,
) -> ForgeQueryDeclarationRelationalRoutingChecked<D, I> {
    let Some(contract) = I::Family::relational_truth_contract() else {
        return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
            ForgeQueryDeclarationRelationalRoutingDenied::new(
                envelope,
                None,
                None,
                ForgeQueryDeclarationRelationalRoutingDenialCause::UnsupportedRelationalTruthClaim,
            ),
        );
    };
    let denied_truth_claim = Some(contract.truth_claim());
    let denied_authority_family = Some(contract.authority_family());
    let Some(route_plan) = envelope.route_plan() else {
        return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
            ForgeQueryDeclarationRelationalRoutingDenied::new(
                envelope,
                denied_truth_claim,
                denied_authority_family,
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
                denied_truth_claim,
                denied_authority_family,
                ForgeQueryDeclarationRelationalRoutingDenialCause::NonRelationalRoutePlan,
            ),
        );
    }

    let authority_aspects =
        RelationalAuthorityAspectGate::from_envelope(&envelope, contract.required_aspects());
    match authority_aspects.fit() {
        ForgeQueryDeclarationAspectFit::Conflict => {
            return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                ForgeQueryDeclarationRelationalRoutingDenied::new(
                    envelope,
                    denied_truth_claim,
                    denied_authority_family,
                    ForgeQueryDeclarationRelationalRoutingDenialCause::AspectConflict,
                ),
            );
        }
        ForgeQueryDeclarationAspectFit::MissingRequired => {
            return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                ForgeQueryDeclarationRelationalRoutingDenied::new(
                    envelope,
                    denied_truth_claim,
                    denied_authority_family,
                    ForgeQueryDeclarationRelationalRoutingDenialCause::MissingRequiredAspect,
                ),
            );
        }
        ForgeQueryDeclarationAspectFit::Partial => {
            return ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                ForgeQueryDeclarationRelationalRoutingDenied::new(
                    envelope,
                    denied_truth_claim,
                    denied_authority_family,
                    ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalAspectGap,
                ),
            );
        }
        ForgeQueryDeclarationAspectFit::Exact
        | ForgeQueryDeclarationAspectFit::CompatibleSuperset => {}
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
        authority_aspects.contract(),
        authority_aspects.coverage(),
        authority_aspects.coverage_basis(),
        authority_aspects.fit(),
        envelope.route_denial_cause(),
        envelope.receipt_denial_cause(),
    );
    let mut retained_truths = envelope.explain().retained_truths().to_vec();
    retained_truths.push(format!("relational-truth-claim:{}", truth_claim.as_str()));
    retained_truths.push(format!(
        "relational-authority-family:{}",
        authority_family.as_str()
    ));
    retained_truths.push(format!(
        "relational-aspect-fit:{:?}",
        authority_aspects.fit()
    ));
    retained_truths.push(format!(
        "relational-aspect-coverage-basis:{:?}",
        authority_aspects.coverage_basis()
    ));
    let explanation = ForgeQueryDeclarationRelationalRoutingExplanation::new(
        envelope.explain().crossing_posture(),
        truth_claim,
        authority_family,
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
    ForgeQueryDeclarationRelationalRoutingChecked::Routed(
        ForgeQueryDeclarationRelationalRouting::new(
            class,
            truth_claim,
            authority_family,
            binding,
            authority_aspects.contract().clone(),
            authority_aspects.coverage().clone(),
            authority_aspects.coverage_basis(),
            authority_aspects.fit(),
            envelope,
            digest,
            explanation,
        ),
    )
}

fn lower_checked_input<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    checked: ForgeQueryDeclarationEnvelopeChecked<D, I>,
) -> ForgeQueryDeclarationRelationalRoutingInput<D, I> {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            ForgeQueryDeclarationRelationalRoutingInput::enveloped(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            ForgeQueryDeclarationRelationalRoutingInput::deferred(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            ForgeQueryDeclarationRelationalRoutingInput::denied(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            ForgeQueryDeclarationRelationalRoutingInput::failed(envelope)
        }
    }
}

fn deny_non_success_mismatch<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    input: ForgeQueryDeclarationRelationalRoutingInput<D, I>,
) -> ForgeQueryDeclarationRelationalRoutingChecked<D, I> {
    match input {
        ForgeQueryDeclarationRelationalRoutingInput::Deferred(envelope) => {
            ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                ForgeQueryDeclarationRelationalRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::relational_truth_contract().map(|contract| contract.truth_claim()),
                    I::Family::relational_truth_contract()
                        .map(|contract| contract.authority_family()),
                    ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch,
                ),
            )
        }
        ForgeQueryDeclarationRelationalRoutingInput::Denied(envelope) => {
            ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                ForgeQueryDeclarationRelationalRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::relational_truth_contract().map(|contract| contract.truth_claim()),
                    I::Family::relational_truth_contract()
                        .map(|contract| contract.authority_family()),
                    ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch,
                ),
            )
        }
        ForgeQueryDeclarationRelationalRoutingInput::Failed(envelope) => {
            ForgeQueryDeclarationRelationalRoutingChecked::Denied(
                ForgeQueryDeclarationRelationalRoutingDenied::new(
                    envelope.into_envelope(),
                    I::Family::relational_truth_contract().map(|contract| contract.truth_claim()),
                    I::Family::relational_truth_contract()
                        .map(|contract| contract.authority_family()),
                    ForgeQueryDeclarationRelationalRoutingDenialCause::RelationalEnvelopeMismatch,
                ),
            )
        }
        ForgeQueryDeclarationRelationalRoutingInput::Enveloped(_)
        | ForgeQueryDeclarationRelationalRoutingInput::EnvelopeChecked(_) => {
            unreachable!("covered envelopes use the covered-handle path")
        }
    }
}
