use crate::application::{
    WorthQueryDeclarationAspectFit, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationEnvelopeDeferred,
    WorthQueryDeclarationEnvelopeDenied, WorthQueryDeclarationEnvelopeFailed,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
    WorthQuerySignalCompatibilityPosture,
};

use super::{
    artifact::WorthQueryDeclarationSignalCompatibility,
    aspect_gate::SignalAuthorityAspectGate,
    checked_input::{deny_non_success_mismatch, lower_checked_input},
    contract::{
        WorthQueryDeclarationSignalCompatibilitySupportRow,
        WorthQueryDeclarationSignalCompatibilitySupportStatus,
    },
    denial::{
        WorthQueryDeclarationSignalCompatibilityDeferred,
        WorthQueryDeclarationSignalCompatibilityDenialCause,
        WorthQueryDeclarationSignalCompatibilityDenied,
        WorthQueryDeclarationSignalCompatibilityFailed,
    },
    digest::derive_signal_compatibility_digest,
    explain::WorthQueryDeclarationSignalCompatibilityExplanation,
    handle_gate::{envelope_matches_handle, subject_matches_handle},
    lower::{derive_required_basis_families, derive_signal_execution_family},
};

pub enum WorthQueryDeclarationSignalCompatibilityInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Enveloped(WorthQueryDeclarationEnvelope<D, I>),
    Deferred(WorthQueryDeclarationEnvelopeDeferred<D, I>),
    Denied(WorthQueryDeclarationEnvelopeDenied<D, I>),
    Failed(WorthQueryDeclarationEnvelopeFailed<D, I>),
    EnvelopeChecked(WorthQueryDeclarationEnvelopeChecked<D, I>),
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationSignalCompatibilityInput<D, I>
{
    pub fn enveloped(envelope: WorthQueryDeclarationEnvelope<D, I>) -> Self {
        Self::Enveloped(envelope)
    }

    pub fn deferred(envelope: WorthQueryDeclarationEnvelopeDeferred<D, I>) -> Self {
        Self::Deferred(envelope)
    }

    pub fn denied(envelope: WorthQueryDeclarationEnvelopeDenied<D, I>) -> Self {
        Self::Denied(envelope)
    }

    pub fn failed(envelope: WorthQueryDeclarationEnvelopeFailed<D, I>) -> Self {
        Self::Failed(envelope)
    }

    pub fn envelope_checked(checked: WorthQueryDeclarationEnvelopeChecked<D, I>) -> Self {
        Self::EnvelopeChecked(checked)
    }
}

pub enum WorthQueryDeclarationSignalCompatibilityChecked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Compatible(WorthQueryDeclarationSignalCompatibility<D, I>),
    Deferred(WorthQueryDeclarationSignalCompatibilityDeferred<D, I>),
    Denied(WorthQueryDeclarationSignalCompatibilityDenied<D, I>),
    Failed(WorthQueryDeclarationSignalCompatibilityFailed<D, I>),
}

pub(crate) fn worth_query_checked_declaration_signal_compatibility<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    input: WorthQueryDeclarationSignalCompatibilityInput<D, I>,
) -> WorthQueryDeclarationSignalCompatibilityChecked<D, I> {
    match input {
        WorthQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(checked) => {
            worth_query_checked_declaration_signal_compatibility(lower_checked_input(checked))
        }
        WorthQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => {
            checked_enveloped(envelope)
        }
        WorthQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => {
            let reason = envelope.reason();
            WorthQueryDeclarationSignalCompatibilityChecked::Deferred(
                WorthQueryDeclarationSignalCompatibilityDeferred::new(
                    envelope.into_envelope(),
                    reason,
                ),
            )
        }
        WorthQueryDeclarationSignalCompatibilityInput::Denied(envelope) => {
            WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                WorthQueryDeclarationSignalCompatibilityDenied::new(
                    envelope.into_envelope(),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.execution_family()),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.required_basis_families().to_vec())
                        .unwrap_or_default(),
                    WorthQueryDeclarationSignalCompatibilityDenialCause::EnvelopeNotCoveredForSignalCompatibility,
                ),
            )
        }
        WorthQueryDeclarationSignalCompatibilityInput::Failed(envelope) => {
            let reason = envelope.reason();
            WorthQueryDeclarationSignalCompatibilityChecked::Failed(
                WorthQueryDeclarationSignalCompatibilityFailed::new(
                    envelope.into_envelope(),
                    reason,
                ),
            )
        }
    }
}

pub(crate) fn worth_query_checked_declaration_signal_compatibility_on_handle<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    support_rows: &[WorthQueryDeclarationSignalCompatibilitySupportRow],
    input: WorthQueryDeclarationSignalCompatibilityInput<D, I>,
) -> WorthQueryDeclarationSignalCompatibilityChecked<D, I> {
    match input {
        WorthQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(checked) => {
            worth_query_checked_declaration_signal_compatibility_on_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                support_rows,
                lower_checked_input(checked),
            )
        }
        WorthQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => {
            if !envelope_matches_handle(
                handle_identity_digest,
                operating_context_identity_digest,
                &envelope,
            ) {
                return WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                    WorthQueryDeclarationSignalCompatibilityDenied::new(
                        envelope,
                        I::Family::signal_compatibility_contract()
                            .map(|contract| contract.execution_family()),
                        I::Family::signal_compatibility_contract()
                            .map(|contract| contract.required_basis_families().to_vec())
                            .unwrap_or_default(),
                        WorthQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
                    ),
                );
            }

            match checked_enveloped(envelope) {
                WorthQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
                    let matched_rows = support_rows
                        .iter()
                        .filter(|row| row.execution_family() == compatibility.execution_family())
                        .collect::<Vec<_>>();

                    if !matched_rows.is_empty()
                        && matched_rows.iter().any(|row| {
                            row.status()
                                == WorthQueryDeclarationSignalCompatibilitySupportStatus::Admitted
                        })
                    {
                        WorthQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility)
                    } else if let Some(row) = matched_rows.iter().find(|row| {
                        row.status()
                            == WorthQueryDeclarationSignalCompatibilitySupportStatus::Deferred
                    }) {
                        WorthQueryDeclarationSignalCompatibilityChecked::Deferred(
                            WorthQueryDeclarationSignalCompatibilityDeferred::new(
                                compatibility.into_envelope(),
                                row.reason(),
                            ),
                        )
                    } else {
                        let execution_family = compatibility.execution_family();
                        let basis_families = compatibility.basis_families().to_vec();
                        WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                            WorthQueryDeclarationSignalCompatibilityDenied::new(
                                compatibility.into_envelope(),
                                Some(execution_family),
                                basis_families,
                                WorthQueryDeclarationSignalCompatibilityDenialCause::SignalBasisMismatch,
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
            worth_query_checked_declaration_signal_compatibility(other)
        }
    }
}

fn checked_enveloped<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
    envelope: WorthQueryDeclarationEnvelope<D, I>,
) -> WorthQueryDeclarationSignalCompatibilityChecked<D, I> {
    match I::Family::taxonomy().signal_compatibility() {
        WorthQuerySignalCompatibilityPosture::Deferred => {
            WorthQueryDeclarationSignalCompatibilityChecked::Deferred(
                WorthQueryDeclarationSignalCompatibilityDeferred::new(
                    envelope,
                    "signal compatibility for this family remains explicitly deferred",
                ),
            )
        }
        WorthQuerySignalCompatibilityPosture::NotCompatible => {
            WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                WorthQueryDeclarationSignalCompatibilityDenied::new(
                    envelope,
                    None,
                    Vec::new(),
                    WorthQueryDeclarationSignalCompatibilityDenialCause::SignalFamilyUnsupported,
                ),
            )
        }
        WorthQuerySignalCompatibilityPosture::Compatible => {
            let Some(contract) = I::Family::signal_compatibility_contract() else {
                return WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                    WorthQueryDeclarationSignalCompatibilityDenied::new(
                        envelope,
                        None,
                        Vec::new(),
                        WorthQueryDeclarationSignalCompatibilityDenialCause::SignalExecutionFamilyUnavailable,
                    ),
                );
            };
            let denied_execution_family = Some(contract.execution_family());
            let denied_basis_families = contract.required_basis_families().to_vec();
            let aspect_gate = SignalAuthorityAspectGate::from_envelope(&envelope, &contract);
            match aspect_gate.fit() {
                WorthQueryDeclarationAspectFit::Conflict => {
                    return WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                        WorthQueryDeclarationSignalCompatibilityDenied::new(
                            envelope,
                            denied_execution_family,
                            denied_basis_families.clone(),
                            WorthQueryDeclarationSignalCompatibilityDenialCause::AspectConflict,
                        ),
                    );
                }
                WorthQueryDeclarationAspectFit::MissingRequired => {
                    return WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                        WorthQueryDeclarationSignalCompatibilityDenied::new(
                            envelope,
                            denied_execution_family,
                            denied_basis_families.clone(),
                            WorthQueryDeclarationSignalCompatibilityDenialCause::MissingRequiredAspect,
                        ),
                    );
                }
                WorthQueryDeclarationAspectFit::Partial => {
                    return WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                        WorthQueryDeclarationSignalCompatibilityDenied::new(
                            envelope,
                            denied_execution_family,
                            denied_basis_families.clone(),
                            WorthQueryDeclarationSignalCompatibilityDenialCause::AuthorityAspectGap,
                        ),
                    );
                }
                WorthQueryDeclarationAspectFit::Exact
                | WorthQueryDeclarationAspectFit::CompatibleSuperset => {}
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
            let explanation = WorthQueryDeclarationSignalCompatibilityExplanation::new(
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
            WorthQueryDeclarationSignalCompatibilityChecked::Compatible(
                WorthQueryDeclarationSignalCompatibility::new(
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
