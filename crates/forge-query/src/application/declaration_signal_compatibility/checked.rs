use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationEnvelopeDeferred, ForgeQueryDeclarationEnvelopeDenied,
    ForgeQueryDeclarationEnvelopeFailed, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQuerySignalCompatibilityPosture,
};

use super::{
    artifact::ForgeQueryDeclarationSignalCompatibility,
    contract::{
        ForgeQueryDeclarationSignalCompatibilitySupportRow,
        ForgeQueryDeclarationSignalCompatibilitySupportStatus,
    },
    denial::{
        ForgeQueryDeclarationSignalCompatibilityDeferred,
        ForgeQueryDeclarationSignalCompatibilityDenialCause,
        ForgeQueryDeclarationSignalCompatibilityDenied,
        ForgeQueryDeclarationSignalCompatibilityFailed,
    },
    digest::derive_signal_compatibility_digest,
    explain::ForgeQueryDeclarationSignalCompatibilityExplanation,
    lower::{derive_required_basis_families, derive_signal_execution_family},
};

pub enum ForgeQueryDeclarationSignalCompatibilityInput<
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
    ForgeQueryDeclarationSignalCompatibilityInput<D, I>
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

pub enum ForgeQueryDeclarationSignalCompatibilityChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Compatible(ForgeQueryDeclarationSignalCompatibility<D, I>),
    Deferred(ForgeQueryDeclarationSignalCompatibilityDeferred<D, I>),
    Denied(ForgeQueryDeclarationSignalCompatibilityDenied<D, I>),
    Failed(ForgeQueryDeclarationSignalCompatibilityFailed<D, I>),
}

pub(crate) fn forge_query_checked_declaration_signal_compatibility<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> ForgeQueryDeclarationSignalCompatibilityChecked<D, I> {
    match input {
        ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(checked) => match checked {
            ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                forge_query_checked_declaration_signal_compatibility(
                    ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                forge_query_checked_declaration_signal_compatibility(
                    ForgeQueryDeclarationSignalCompatibilityInput::deferred(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                forge_query_checked_declaration_signal_compatibility(
                    ForgeQueryDeclarationSignalCompatibilityInput::denied(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                forge_query_checked_declaration_signal_compatibility(
                    ForgeQueryDeclarationSignalCompatibilityInput::failed(envelope),
                )
            }
        },
        ForgeQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => {
            checked_enveloped(envelope)
        }
        ForgeQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => {
            let reason = envelope.reason();
            ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(
                ForgeQueryDeclarationSignalCompatibilityDeferred::new(
                    envelope.into_envelope(),
                    reason,
                ),
            )
        }
        ForgeQueryDeclarationSignalCompatibilityInput::Denied(envelope) => {
            ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                ForgeQueryDeclarationSignalCompatibilityDenied::new(
                    envelope.into_envelope(),
                    ForgeQueryDeclarationSignalCompatibilityDenialCause::EnvelopeNotCoveredForSignalCompatibility,
                ),
            )
        }
        ForgeQueryDeclarationSignalCompatibilityInput::Failed(envelope) => {
            let reason = envelope.reason();
            ForgeQueryDeclarationSignalCompatibilityChecked::Failed(
                ForgeQueryDeclarationSignalCompatibilityFailed::new(
                    envelope.into_envelope(),
                    reason,
                ),
            )
        }
    }
}

pub(crate) fn forge_query_checked_declaration_signal_compatibility_on_handle<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    support_rows: &[ForgeQueryDeclarationSignalCompatibilitySupportRow],
    input: ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> ForgeQueryDeclarationSignalCompatibilityChecked<D, I> {
    match input {
        ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(checked) => match checked {
            ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                checked_enveloped_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    support_rows,
                    envelope,
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                checked_non_success_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    ForgeQueryDeclarationSignalCompatibilityInput::deferred(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                checked_non_success_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    ForgeQueryDeclarationSignalCompatibilityInput::denied(envelope),
                )
            }
            ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                checked_non_success_on_handle(
                    handle_identity_digest,
                    operating_context_identity_digest,
                    ForgeQueryDeclarationSignalCompatibilityInput::failed(envelope),
                )
            }
        },
        ForgeQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => {
            checked_enveloped_on_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                support_rows,
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

fn checked_enveloped<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
) -> ForgeQueryDeclarationSignalCompatibilityChecked<D, I> {
    match I::Family::taxonomy().signal_compatibility() {
        ForgeQuerySignalCompatibilityPosture::Deferred => {
            ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(
                ForgeQueryDeclarationSignalCompatibilityDeferred::new(
                    envelope,
                    "signal compatibility for this family remains explicitly deferred",
                ),
            )
        }
        ForgeQuerySignalCompatibilityPosture::NotCompatible => {
            ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                ForgeQueryDeclarationSignalCompatibilityDenied::new(
                    envelope,
                    ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalFamilyUnsupported,
                ),
            )
        }
        ForgeQuerySignalCompatibilityPosture::Compatible => {
            let Some(contract) = I::Family::signal_compatibility_contract() else {
                return ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                    ForgeQueryDeclarationSignalCompatibilityDenied::new(
                        envelope,
                        ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalExecutionFamilyUnavailable,
                    ),
                );
            };
            let execution_family = derive_signal_execution_family(&envelope, contract);
            let basis_families = derive_required_basis_families(contract);
            let digest = derive_signal_compatibility_digest(
                &envelope,
                execution_family,
                &basis_families,
                ForgeQuerySignalCompatibilityPosture::Compatible,
                envelope.route_denial_cause(),
                envelope.receipt_denial_cause(),
            );
            let mut retained_truths = envelope.explain().retained_truths().to_vec();
            retained_truths.push(format!(
                "primary-authority:{}",
                I::Family::taxonomy().primary_authority_family().as_str()
            ));
            retained_truths.push(format!(
                "signal-execution-family:{}",
                execution_family.as_str()
            ));
            retained_truths.push(format!(
                "required-basis-families:{}",
                basis_families
                    .iter()
                    .map(|family| family.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            ));
            let explanation = ForgeQueryDeclarationSignalCompatibilityExplanation::new(
                "compatible with later signal-backed derived execution",
                execution_family,
                basis_families.clone(),
                retained_truths,
                envelope
                    .explain()
                    .route_governing_reason()
                    .map(ToOwned::to_owned),
                envelope.route_denial_cause(),
                envelope.explain().receipt_governing_reason().to_string(),
                envelope.receipt_denial_cause(),
                envelope.evidence_origin(),
            );
            ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(
                ForgeQueryDeclarationSignalCompatibility::new(
                    I::Family::taxonomy().primary_authority_family(),
                    execution_family,
                    basis_families,
                    envelope,
                    digest,
                    explanation,
                ),
            )
        }
    }
}

fn checked_enveloped_on_handle<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    support_rows: &[ForgeQueryDeclarationSignalCompatibilitySupportRow],
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
) -> ForgeQueryDeclarationSignalCompatibilityChecked<D, I> {
    if envelope.handle_identity_digest() != handle_identity_digest
        || envelope.operating_context_identity_digest() != operating_context_identity_digest
    {
        return ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
            ForgeQueryDeclarationSignalCompatibilityDenied::new(
                envelope,
                ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
            ),
        );
    }

    match forge_query_checked_declaration_signal_compatibility(
        ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
    ) {
        ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
            if compatibility.basis_families().iter().all(|basis_family| {
                support_rows.iter().any(|row| {
                    row.execution_family() == compatibility.execution_family()
                        && row.basis_family() == *basis_family
                        && row.status()
                            == ForgeQueryDeclarationSignalCompatibilitySupportStatus::Admitted
                })
            }) {
                ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility)
            } else {
                ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                    ForgeQueryDeclarationSignalCompatibilityDenied::new(
                        compatibility.into_envelope(),
                        ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalBasisMismatch,
                    ),
                )
            }
        }
        other => other,
    }
}

fn checked_non_success_on_handle<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    input: ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> ForgeQueryDeclarationSignalCompatibilityChecked<D, I> {
    if !subject_matches_handle(
        handle_identity_digest,
        operating_context_identity_digest,
        &input,
    ) {
        return match input {
            ForgeQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => {
                ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                    ForgeQueryDeclarationSignalCompatibilityDenied::new(
                        envelope.into_envelope(),
                        ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
                    ),
                )
            }
            ForgeQueryDeclarationSignalCompatibilityInput::Denied(envelope) => {
                ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                    ForgeQueryDeclarationSignalCompatibilityDenied::new(
                        envelope.into_envelope(),
                        ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
                    ),
                )
            }
            ForgeQueryDeclarationSignalCompatibilityInput::Failed(envelope) => {
                ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                    ForgeQueryDeclarationSignalCompatibilityDenied::new(
                        envelope.into_envelope(),
                        ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
                    ),
                )
            }
            ForgeQueryDeclarationSignalCompatibilityInput::Enveloped(_)
            | ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(_) => {
                unreachable!("non-success path only accepts non-success inputs")
            }
        };
    }
    forge_query_checked_declaration_signal_compatibility(input)
}

fn subject_matches_handle<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    input: &ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> bool {
    let envelope = match input {
        ForgeQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => envelope,
        ForgeQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::Denied(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::Failed(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(_) => {
            unreachable!("checked inputs are lowered before subject matching")
        }
    };
    envelope.handle_identity_digest() == handle_identity_digest
        && envelope.operating_context_identity_digest() == operating_context_identity_digest
}
