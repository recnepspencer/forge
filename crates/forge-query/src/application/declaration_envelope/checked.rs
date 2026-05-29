use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceipt,
    ForgeQueryDeclarationReceiptDenialCause, ForgeQueryDeclarationRoutePlan,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDomainEntryMarker,
};

use super::{
    artifact::ForgeQueryDeclarationEnvelope,
    class::{ForgeQueryDeclarationEnvelopeClass, ForgeQueryDeclarationEnvelopeEvidenceOrigin},
    denial::{
        ForgeQueryDeclarationEnvelopeDeferred, ForgeQueryDeclarationEnvelopeDenied,
        ForgeQueryDeclarationEnvelopeFailed, ForgeQueryDeclarationEnvelopeTerminalError,
    },
    digest::derive_envelope_digest,
    explain::ForgeQueryDeclarationEnvelopeExplanation,
    input::ForgeQueryDeclarationEnvelopeInput,
};

pub enum ForgeQueryDeclarationEnvelopeChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Enveloped(ForgeQueryDeclarationEnvelope<D, I>),
    Deferred(ForgeQueryDeclarationEnvelopeDeferred<D, I>),
    Denied(ForgeQueryDeclarationEnvelopeDenied<D, I>),
    Failed(ForgeQueryDeclarationEnvelopeFailed<D, I>),
}

pub(crate) fn forge_query_checked_declaration_envelope<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationEnvelopeInput<D, I>,
) -> ForgeQueryDeclarationEnvelopeChecked<D, I> {
    match input {
        ForgeQueryDeclarationEnvelopeInput::ReceiptChecked(checked) => match checked {
            crate::application::ForgeQueryDeclarationReceiptChecked::Issued(receipt) => {
                forge_query_checked_declaration_envelope(
                    ForgeQueryDeclarationEnvelopeInput::issued(receipt),
                )
            }
            crate::application::ForgeQueryDeclarationReceiptChecked::Deferred(receipt) => {
                forge_query_checked_declaration_envelope(
                    ForgeQueryDeclarationEnvelopeInput::deferred(receipt),
                )
            }
            crate::application::ForgeQueryDeclarationReceiptChecked::Denied(receipt) => {
                forge_query_checked_declaration_envelope(
                    ForgeQueryDeclarationEnvelopeInput::denied(receipt),
                )
            }
            crate::application::ForgeQueryDeclarationReceiptChecked::Failed(receipt) => {
                forge_query_checked_declaration_envelope(
                    ForgeQueryDeclarationEnvelopeInput::failed(receipt),
                )
            }
        },
        ForgeQueryDeclarationEnvelopeInput::IssuedReceipt(receipt) => {
            let envelope_digest = digest_for_receipt(
                &receipt,
                ForgeQueryDeclarationEnvelopeClass::CoveredCrossing,
                None,
                None,
            );
            let explanation = build_explanation(&receipt, None, None);
            let envelope =
                ForgeQueryDeclarationEnvelope::from_issued(receipt, envelope_digest, explanation);
            ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope)
        }
        ForgeQueryDeclarationEnvelopeInput::DeferredReceipt(receipt) => {
            let reason = receipt.reason();
            let envelope_digest = digest_for_receipt(
                receipt.receipt(),
                ForgeQueryDeclarationEnvelopeClass::DeferredCrossing,
                None,
                None,
            );
            let explanation = build_explanation(receipt.receipt(), None, None);
            let envelope =
                ForgeQueryDeclarationEnvelope::from_deferred(receipt, envelope_digest, explanation);
            ForgeQueryDeclarationEnvelopeChecked::Deferred(
                ForgeQueryDeclarationEnvelopeDeferred::new(envelope, reason),
            )
        }
        ForgeQueryDeclarationEnvelopeInput::DeniedReceipt(receipt) => {
            let route_cause = receipt.route_cause();
            let receipt_cause = receipt.receipt_cause();
            let reason = receipt.reason();
            let envelope_digest = digest_for_receipt(
                receipt.receipt(),
                ForgeQueryDeclarationEnvelopeClass::DeniedCrossing,
                route_cause,
                receipt_cause,
            );
            let explanation = build_explanation(receipt.receipt(), route_cause, receipt_cause);
            let envelope =
                ForgeQueryDeclarationEnvelope::from_denied(receipt, envelope_digest, explanation);
            ForgeQueryDeclarationEnvelopeChecked::Denied(ForgeQueryDeclarationEnvelopeDenied::new(
                envelope,
                route_cause,
                receipt_cause,
                reason,
            ))
        }
        ForgeQueryDeclarationEnvelopeInput::FailedReceipt(receipt) => {
            let reason = receipt.reason();
            let envelope_digest = digest_for_receipt(
                receipt.receipt(),
                ForgeQueryDeclarationEnvelopeClass::FailedCrossing,
                None,
                None,
            );
            let explanation = build_explanation(receipt.receipt(), None, None);
            let envelope =
                ForgeQueryDeclarationEnvelope::from_failed(receipt, envelope_digest, explanation);
            ForgeQueryDeclarationEnvelopeChecked::Failed(ForgeQueryDeclarationEnvelopeFailed::new(
                envelope, reason,
            ))
        }
    }
}

pub(crate) fn forge_query_declaration_envelope_terminal_from_receipt_terminal<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    error: crate::application::ForgeQueryDeclarationReceiptTerminalError<D, I>,
) -> ForgeQueryDeclarationEnvelopeTerminalError<D, I> {
    match error {
        crate::application::ForgeQueryDeclarationReceiptTerminalError::Deferred(receipt) => {
            match forge_query_checked_declaration_envelope(
                ForgeQueryDeclarationEnvelopeInput::deferred(receipt),
            ) {
                ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                    ForgeQueryDeclarationEnvelopeTerminalError::Deferred(envelope)
                }
                _ => panic!("deferred receipt should lower into deferred envelope"),
            }
        }
        crate::application::ForgeQueryDeclarationReceiptTerminalError::Denied(receipt) => {
            match forge_query_checked_declaration_envelope(
                ForgeQueryDeclarationEnvelopeInput::denied(receipt),
            ) {
                ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                    ForgeQueryDeclarationEnvelopeTerminalError::Denied(envelope)
                }
                _ => panic!("denied receipt should lower into denied envelope"),
            }
        }
        crate::application::ForgeQueryDeclarationReceiptTerminalError::Failed(receipt) => {
            match forge_query_checked_declaration_envelope(
                ForgeQueryDeclarationEnvelopeInput::failed(receipt),
            ) {
                ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                    ForgeQueryDeclarationEnvelopeTerminalError::Failed(envelope)
                }
                _ => panic!("failed receipt should lower into failed envelope"),
            }
        }
    }
}

fn digest_for_receipt<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    receipt: &ForgeQueryDeclarationReceipt<D, I>,
    class: ForgeQueryDeclarationEnvelopeClass,
    route_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
) -> CanonicalDerivedDigest {
    let evidence = receipt.foundational_evidence();
    let version = evidence
        .subject()
        .canonical_declaration()
        .version()
        .foundational()
        .clone();
    derive_envelope_digest(
        version,
        receipt.handle_identity_digest(),
        receipt.operating_context_identity_digest(),
        receipt.declaration_family_key(),
        receipt.declaration_digest(),
        receipt.progression_digest(),
        receipt.route_plan_digest(),
        &canonical_digest_token(receipt.receipt_digest()),
        class,
        ForgeQueryDeclarationEnvelopeEvidenceOrigin::from_foundational_class(evidence.class()),
        route_cause,
        receipt_cause,
    )
}

fn build_explanation<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    receipt: &ForgeQueryDeclarationReceipt<D, I>,
    route_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
) -> ForgeQueryDeclarationEnvelopeExplanation {
    let evidence = receipt.foundational_evidence();
    let evidence_origin =
        ForgeQueryDeclarationEnvelopeEvidenceOrigin::from_foundational_class(evidence.class());
    let mut retained_truths = receipt.explain().retained_truths().to_vec();
    retained_truths.push(format!(
        "receipt:{}",
        canonical_digest_token(receipt.receipt_digest())
    ));
    retained_truths.push(format!("evidence-origin:{}", evidence_origin.as_str()));
    if let Some(route_plan) = receipt.route_plan() {
        retained_truths.push(format!("route-plan:{}", route_plan.route_plan_digest()));
    }

    ForgeQueryDeclarationEnvelopeExplanation::new(
        receipt.explain().crossing_posture(),
        evidence_origin,
        receipt.explain().route_reference().map(ToOwned::to_owned),
        retained_truths,
        receipt.route_plan().map(route_reason_from_plan),
        route_cause,
        receipt.explain().governing_reason().to_string(),
        receipt_cause,
    )
}

fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}

fn route_reason_from_plan<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    route_plan: &ForgeQueryDeclarationRoutePlan<D, I>,
) -> String {
    let mut parts = vec![route_plan.explain().route_contract_reason().to_string()];
    parts.extend(route_plan.explain().route_segment_reasons().iter().cloned());
    if let Some(intent_reason) = route_plan.explain().intent_reason() {
        parts.push(intent_reason.to_string());
    }
    parts.join("; ")
}
