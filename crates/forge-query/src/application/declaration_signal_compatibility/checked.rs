use crate::application::{
    ForgeQueryDeclarationAspectFit, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeDeferred,
    ForgeQueryDeclarationEnvelopeDenied, ForgeQueryDeclarationEnvelopeFailed,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQuerySignalCompatibilityPosture,
};

use super::{
    artifact::ForgeQueryDeclarationSignalCompatibility,
    aspect_gate::SignalAuthorityAspectGate,
    checked_input::{deny_non_success_mismatch, lower_checked_input},
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
    handle_gate::{envelope_matches_handle, subject_matches_handle},
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
        ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(checked) => {
            forge_query_checked_declaration_signal_compatibility(lower_checked_input(checked))
        }
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
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.execution_family()),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.required_basis_families().to_vec())
                        .unwrap_or_default(),
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
        ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(checked) => {
            forge_query_checked_declaration_signal_compatibility_on_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                support_rows,
                lower_checked_input(checked),
            )
        }
        ForgeQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => {
            if !envelope_matches_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                &envelope,
            ) {
                return ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                    ForgeQueryDeclarationSignalCompatibilityDenied::new(
                        envelope,
                        I::Family::signal_compatibility_contract()
                            .map(|contract| contract.execution_family()),
                        I::Family::signal_compatibility_contract()
                            .map(|contract| contract.required_basis_families().to_vec())
                            .unwrap_or_default(),
                        ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
                    ),
                );
            }

            match checked_enveloped(envelope) {
                ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
                    let matched_rows = support_rows
                        .iter()
                        .filter(|row| row.execution_family() == compatibility.execution_family())
                        .collect::<Vec<_>>();

                    if !matched_rows.is_empty()
                        && matched_rows.iter().any(|row| {
                            row.status()
                                == ForgeQueryDeclarationSignalCompatibilitySupportStatus::Admitted
                        })
                    {
                        ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility)
                    } else if let Some(row) = matched_rows.iter().find(|row| {
                        row.status()
                            == ForgeQueryDeclarationSignalCompatibilitySupportStatus::Deferred
                    }) {
                        ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(
                            ForgeQueryDeclarationSignalCompatibilityDeferred::new(
                                compatibility.into_envelope(),
                                row.reason(),
                            ),
                        )
                    } else {
                        let execution_family = compatibility.execution_family();
                        let basis_families = compatibility.basis_families().to_vec();
                        ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                            ForgeQueryDeclarationSignalCompatibilityDenied::new(
                                compatibility.into_envelope(),
                                Some(execution_family),
                                basis_families,
                                ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalBasisMismatch,
                            ),
                        )
                    }
                }
                other => other,
            }
        }
        other => {
            if !subject_matches_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                &other,
            ) {
                return deny_non_success_mismatch(other);
            }
            forge_query_checked_declaration_signal_compatibility(other)
        }
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
                    None,
                    Vec::new(),
                    ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalFamilyUnsupported,
                ),
            )
        }
        ForgeQuerySignalCompatibilityPosture::Compatible => {
            let Some(contract) = I::Family::signal_compatibility_contract() else {
                return ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                    ForgeQueryDeclarationSignalCompatibilityDenied::new(
                        envelope,
                        None,
                        Vec::new(),
                        ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalExecutionFamilyUnavailable,
                    ),
                );
            };
            let denied_execution_family = Some(contract.execution_family());
            let denied_basis_families = contract.required_basis_families().to_vec();
            let aspect_gate = SignalAuthorityAspectGate::from_envelope(&envelope, &contract);
            match aspect_gate.fit() {
                ForgeQueryDeclarationAspectFit::Conflict => {
                    return ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                        ForgeQueryDeclarationSignalCompatibilityDenied::new(
                            envelope,
                            denied_execution_family,
                            denied_basis_families.clone(),
                            ForgeQueryDeclarationSignalCompatibilityDenialCause::AspectConflict,
                        ),
                    );
                }
                ForgeQueryDeclarationAspectFit::MissingRequired => {
                    return ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                        ForgeQueryDeclarationSignalCompatibilityDenied::new(
                            envelope,
                            denied_execution_family,
                            denied_basis_families.clone(),
                            ForgeQueryDeclarationSignalCompatibilityDenialCause::MissingRequiredAspect,
                        ),
                    );
                }
                ForgeQueryDeclarationAspectFit::Partial => {
                    return ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                        ForgeQueryDeclarationSignalCompatibilityDenied::new(
                            envelope,
                            denied_execution_family,
                            denied_basis_families.clone(),
                            ForgeQueryDeclarationSignalCompatibilityDenialCause::AuthorityAspectGap,
                        ),
                    );
                }
                ForgeQueryDeclarationAspectFit::Exact
                | ForgeQueryDeclarationAspectFit::CompatibleSuperset => {}
            }

            let execution_family = derive_signal_execution_family(&envelope, &contract);
            let basis_families = derive_required_basis_families(&contract);
            let future_projection = envelope
                .route_plan()
                .expect("covered signal compatibility envelopes retain route-plan truth")
                .future_projection()
                .clone();
            let digest = derive_signal_compatibility_digest(
                &envelope,
                execution_family,
                &basis_families,
                I::Family::taxonomy().signal_compatibility(),
                aspect_gate.authority_contract(),
                aspect_gate.coverage(),
                aspect_gate.coverage_basis(),
                aspect_gate.fit(),
                aspect_gate.dependency_aspects(),
                aspect_gate.produced_aspects(),
                &future_projection,
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
            retained_truths.push(format!("signal-aspect-fit:{:?}", aspect_gate.fit()));
            retained_truths.extend(future_projection.retained_facts());
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
                    aspect_gate.authority_contract().clone(),
                    aspect_gate.coverage().clone(),
                    aspect_gate.coverage_basis(),
                    aspect_gate.fit(),
                    aspect_gate.dependency_aspects().clone(),
                    aspect_gate.produced_aspects().clone(),
                    future_projection,
                    envelope,
                    digest,
                    explanation,
                ),
            )
        }
    }
}
