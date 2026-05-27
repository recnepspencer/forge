use crate::application::{
    checked_route_plan_from_progressed_with_profile, ForgeQueryAdmittedConfiguredDomainHandle,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationSignalCompatibilityChecked,
    ForgeQueryDeclarationSignalCompatibilityInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::identity::hash_parts;
use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use super::{
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationSubject,
};

pub(super) fn checked_and_linked_from_subject<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    subject: ForgeQuerySignalCompatibilityOrchestrationSubject<D, I>,
) -> (
    ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
    ForgeQueryBindingLinkedArtifacts,
) {
    match subject {
        ForgeQuerySignalCompatibilityOrchestrationSubject::SignalCompatibility(subject) => {
            let linked = linked_from_signal_subject(&subject);
            (handle.signal_compatibility_checked(subject), linked)
        }
        ForgeQuerySignalCompatibilityOrchestrationSubject::Progressed {
            progression,
            route_intent,
        } => {
            let base_linked = ForgeQueryBindingLinkedArtifacts::new()
                .with_declaration_digest(canonical_digest_token(
                    progression.canonical_declaration().declaration_digest(),
                ))
                .with_progression_digest(progression.progression_digest());
            let route_checked = checked_route_plan_from_progressed_with_profile(
                handle,
                progression,
                route_intent,
                FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
            );
            let receipt_checked = handle.receipt_routes_checked(
                crate::application::ForgeQueryDeclarationReceiptInput::route_checked(route_checked),
            );
            let envelope_checked = handle.envelope_routes_checked(
                crate::application::ForgeQueryDeclarationEnvelopeInput::receipt_checked(
                    receipt_checked,
                ),
            );
            let checked = handle.signal_compatibility_checked(
                ForgeQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked),
            );
            let linked = linked_from_signal_checked(&checked, base_linked);
            (checked, linked)
        }
    }
}

pub(super) fn handle_alignment_outcome<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    subject: &ForgeQuerySignalCompatibilityOrchestrationSubject<D, I>,
) -> Option<ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>> {
    match subject {
        ForgeQuerySignalCompatibilityOrchestrationSubject::SignalCompatibility(subject) => {
            let envelope = subject_envelope(subject);
            if envelope.operating_context_identity_digest()
                != handle.operating_context_identity_digest()
            {
                Some(
                    ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(
                        "the retained signal subject was admitted in a different operating context"
                            .to_string(),
                    ),
                )
            } else if envelope.handle_identity_digest() != handle.handle_identity_digest() {
                Some(
                    ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(
                        "the retained signal subject was admitted on a different configured domain handle"
                            .to_string(),
                    ),
                )
            } else {
                None
            }
        }
        ForgeQuerySignalCompatibilityOrchestrationSubject::Progressed { progression, .. } => {
            if progression.operating_context_identity_digest()
                != handle.operating_context_identity_digest()
            {
                Some(ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(
                    "the retained declaration progression was admitted in a different operating context"
                        .to_string(),
                ))
            } else if progression.canonical_declaration().handle_identity_digest()
                != handle.handle_identity_digest()
            {
                Some(ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongHandle(
                    "the retained declaration progression was admitted on a different configured domain handle"
                        .to_string(),
                ))
            } else {
                None
            }
        }
    }
}

pub(super) fn linked_from_orchestration_subject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    subject: &ForgeQuerySignalCompatibilityOrchestrationSubject<D, I>,
) -> ForgeQueryBindingLinkedArtifacts {
    match subject {
        ForgeQuerySignalCompatibilityOrchestrationSubject::SignalCompatibility(subject) => {
            linked_from_signal_subject(subject)
        }
        ForgeQuerySignalCompatibilityOrchestrationSubject::Progressed { progression, .. } => {
            ForgeQueryBindingLinkedArtifacts::new()
                .with_declaration_digest(canonical_digest_token(
                    progression.canonical_declaration().declaration_digest(),
                ))
                .with_progression_digest(progression.progression_digest())
        }
    }
}

pub(super) fn orchestration_digest(
    kind: &str,
    linked: &ForgeQueryBindingLinkedArtifacts,
    outcome: &str,
) -> String {
    hash_parts(&[
        "forge_query_signal_compatibility_orchestration_v1".to_string(),
        kind.to_string(),
        format!("outcome:{outcome}"),
        format!("linked:{linked:?}"),
    ])
}

fn linked_from_signal_subject<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    subject: &ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> ForgeQueryBindingLinkedArtifacts {
    linked_from_envelope(
        subject_envelope(subject),
        ForgeQueryBindingLinkedArtifacts::new(),
    )
}

fn subject_envelope<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    subject: &ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
) -> &crate::application::ForgeQueryDeclarationEnvelope<D, I> {
    match subject {
        ForgeQueryDeclarationSignalCompatibilityInput::Enveloped(envelope) => envelope,
        ForgeQueryDeclarationSignalCompatibilityInput::Deferred(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::Denied(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::Failed(envelope) => envelope.envelope(),
        ForgeQueryDeclarationSignalCompatibilityInput::EnvelopeChecked(checked) => match checked {
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                envelope
            }
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                envelope.envelope()
            }
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => {
                envelope.envelope()
            }
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => {
                envelope.envelope()
            }
        },
    }
}

fn linked_from_signal_checked<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    checked: &ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
    base: ForgeQueryBindingLinkedArtifacts,
) -> ForgeQueryBindingLinkedArtifacts {
    match checked {
        ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
            linked_from_envelope(compatibility.envelope(), base)
        }
        ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(deferred) => {
            linked_from_envelope(deferred.envelope(), base)
        }
        ForgeQueryDeclarationSignalCompatibilityChecked::Denied(denied) => {
            linked_from_envelope(denied.envelope(), base)
        }
        ForgeQueryDeclarationSignalCompatibilityChecked::Failed(failed) => {
            linked_from_envelope(failed.envelope(), base)
        }
    }
}

fn linked_from_envelope<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    envelope: &crate::application::ForgeQueryDeclarationEnvelope<D, I>,
    mut linked: ForgeQueryBindingLinkedArtifacts,
) -> ForgeQueryBindingLinkedArtifacts {
    linked = linked
        .with_declaration_digest(envelope.declaration_digest())
        .with_receipt_digest(canonical_digest_token(envelope.receipt_digest()))
        .with_envelope_digest(canonical_digest_token(envelope.envelope_digest()));
    if let Some(progression_digest) = envelope.progression_digest() {
        linked = linked.with_progression_digest(progression_digest);
    }
    if let Some(route_plan_digest) = envelope.route_plan_digest() {
        linked = linked.with_route_plan_digest(route_plan_digest);
    }
    linked
}

fn canonical_digest_token(digest: &forge_foundational::facade::CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
