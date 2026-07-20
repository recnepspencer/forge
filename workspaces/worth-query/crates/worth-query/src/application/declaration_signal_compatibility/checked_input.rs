use crate::application::{
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::{
    checked::{
        WorthQueryDeclarationSignalCompatibilityChecked,
        WorthQueryDeclarationSignalCompatibilityInput,
    },
    denial::{
        WorthQueryDeclarationSignalCompatibilityDenialCause,
        WorthQueryDeclarationSignalCompatibilityDenied,
    },
};

pub(super) fn lower_checked_input<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryDeclarationEnvelopeChecked<D, I>,
) -> WorthQueryDeclarationSignalCompatibilityInput<D, I> {
    match checked {
        WorthQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            WorthQueryDeclarationSignalCompatibilityInput::enveloped(envelope)
        }
        WorthQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            WorthQueryDeclarationSignalCompatibilityInput::deferred(envelope)
        }
        WorthQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            WorthQueryDeclarationSignalCompatibilityInput::denied(envelope)
        }
        WorthQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            WorthQueryDeclarationSignalCompatibilityInput::failed(envelope)
        }
    }
}

pub(super) fn deny_non_success_mismatch<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    input: WorthQueryDeclarationSignalCompatibilityInput<D, I>,
) -> WorthQueryDeclarationSignalCompatibilityChecked<D, I> {
    match input {
        WorthQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => {
            WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                WorthQueryDeclarationSignalCompatibilityDenied::new(
                    envelope.into_envelope(),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.execution_family()),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.required_basis_families().to_vec())
                        .unwrap_or_default(),
                    WorthQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
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
                    WorthQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
                ),
            )
        }
        WorthQueryDeclarationSignalCompatibilityInput::Failed(envelope) => {
            WorthQueryDeclarationSignalCompatibilityChecked::Denied(
                WorthQueryDeclarationSignalCompatibilityDenied::new(
                    envelope.into_envelope(),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.execution_family()),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.required_basis_families().to_vec())
                        .unwrap_or_default(),
                    WorthQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
                ),
            )
        }
        WorthQueryDeclarationSignalCompatibilityInput::Enveloped(_)
        | WorthQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(_) => {
            unreachable!("non-success path only accepts non-success inputs")
        }
    }
}
