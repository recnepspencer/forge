use crate::application::{
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::{
    checked::{
        ForgeQueryDeclarationSignalCompatibilityChecked,
        ForgeQueryDeclarationSignalCompatibilityInput,
    },
    denial::{
        ForgeQueryDeclarationSignalCompatibilityDenialCause,
        ForgeQueryDeclarationSignalCompatibilityDenied,
    },
};

pub(super) fn lower_checked_input<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationEnvelopeChecked<D, I>,
) -> ForgeQueryDeclarationSignalCompatibilityInput<D, I> {
    match checked {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
            ForgeQueryDeclarationSignalCompatibilityInput::deferred(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
            ForgeQueryDeclarationSignalCompatibilityInput::denied(envelope)
        }
        ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
            ForgeQueryDeclarationSignalCompatibilityInput::failed(envelope)
        }
    }
}

pub(super) fn deny_non_success_mismatch<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    input: ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> ForgeQueryDeclarationSignalCompatibilityChecked<D, I> {
    match input {
        ForgeQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => {
            ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                ForgeQueryDeclarationSignalCompatibilityDenied::new(
                    envelope.into_envelope(),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.execution_family()),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.required_basis_families().to_vec())
                        .unwrap_or_default(),
                    ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
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
                    ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
                ),
            )
        }
        ForgeQueryDeclarationSignalCompatibilityInput::Failed(envelope) => {
            ForgeQueryDeclarationSignalCompatibilityChecked::Denied(
                ForgeQueryDeclarationSignalCompatibilityDenied::new(
                    envelope.into_envelope(),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.execution_family()),
                    I::Family::signal_compatibility_contract()
                        .map(|contract| contract.required_basis_families().to_vec())
                        .unwrap_or_default(),
                    ForgeQueryDeclarationSignalCompatibilityDenialCause::SignalCompatibilityMismatch,
                ),
            )
        }
        ForgeQueryDeclarationSignalCompatibilityInput::Enveloped(_)
        | ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(_) => {
            unreachable!("non-success path only accepts non-success inputs")
        }
    }
}
