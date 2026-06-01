use crate::application::{
    bridge_authority_summary_from_publication, relational_authority_summary_from_publication,
    signal_authority_summary_from_publication, ForgeQueryDeclarationBridgeRoutingChecked,
    ForgeQueryDeclarationBridgeRoutingClass, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRelationalRoutingChecked, ForgeQueryDeclarationRelationalRoutingClass,
    ForgeQueryDeclarationSignalCompatibilityChecked, ForgeQueryDeclarationSignalCompatibilityClass,
    ForgeQueryDomainEntryMarker,
};

use super::{
    super::contribution::ForgeQueryDeclarationEntryRetainedSubjectStrength,
    artifact::{
        ForgeQueryDeclarationEntryInspectionBridgePosture,
        ForgeQueryDeclarationEntryInspectionRelationalPosture,
        ForgeQueryDeclarationEntryInspectionSignalPosture,
    },
    subject::NormalizedInspectionSubject,
};

pub(super) fn normalized_relational<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationRelationalRoutingChecked<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match checked {
        ForgeQueryDeclarationRelationalRoutingChecked::Routed(routing) => {
            let posture = ForgeQueryDeclarationEntryInspectionRelationalPosture {
                class: routing.class(),
                truth_claim: Some(routing.truth_claim()),
                authority_family: Some(routing.authority_family()),
                authority_aspect_summary:
                    crate::application::ForgeQueryDeclarationRelationalAuthorityAspectSummary::new(
                        routing.aspect_contract().clone(),
                        routing.aspect_coverage().clone(),
                        routing.aspect_coverage_basis(),
                        routing.aspect_fit(),
                        crate::application::authority_mismatch_from_fit(routing.aspect_fit()),
                    ),
                routing_digest: routing.relational_routing_digest().to_string(),
                denial_cause: None,
            };
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
                relational: Some(posture),
                bridge: None,
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Relational,
            }
        }
        ForgeQueryDeclarationRelationalRoutingChecked::Deferred(routing) => {
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Relational,
            }
        }
        ForgeQueryDeclarationRelationalRoutingChecked::Denied(routing) => {
            let denial_cause = routing.cause();
            let truth_claim = routing.truth_claim();
            let authority_family = routing.authority_family();
            let envelope = routing.into_envelope();
            let posture = ForgeQueryDeclarationEntryInspectionRelationalPosture {
                class: ForgeQueryDeclarationRelationalRoutingClass::ExclusiveRelationalTruth,
                truth_claim,
                authority_family,
                authority_aspect_summary: envelope_relational_summary(&envelope),
                routing_digest: "denied".to_string(),
                denial_cause: Some(denial_cause),
            };
            NormalizedInspectionSubject {
                envelope,
                relational: Some(posture),
                bridge: None,
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Relational,
            }
        }
        ForgeQueryDeclarationRelationalRoutingChecked::Failed(routing) => {
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Relational,
            }
        }
    }
}

pub(super) fn normalized_bridge<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationBridgeRoutingChecked<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match checked {
        ForgeQueryDeclarationBridgeRoutingChecked::Routed(routing) => {
            let posture = ForgeQueryDeclarationEntryInspectionBridgePosture {
                class: routing.class(),
                continuation_mode: Some(routing.continuation_request().mode()),
                truth_context: Some(routing.truth_context()),
                continuation_family: Some(routing.continuation_family()),
                authority_aspect_summary:
                    crate::application::ForgeQueryDeclarationBridgeAuthorityAspectSummary::new(
                        routing.aspect_contract().clone(),
                        routing.aspect_coverage().clone(),
                        routing.aspect_coverage_basis(),
                        routing.aspect_fit(),
                        crate::application::authority_mismatch_from_fit(routing.aspect_fit()),
                        routing.mapped_aspects().clone(),
                        crate::application::ForgeQueryDeclarationAspectCoverageBasis::BridgeMappedCoverage,
                        routing.mapping_fit(),
                    ),
                routing_digest: routing.bridge_routing_digest().to_string(),
                denial_cause: None,
            };
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
                relational: None,
                bridge: Some(posture),
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Bridge,
            }
        }
        ForgeQueryDeclarationBridgeRoutingChecked::Deferred(routing) => {
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Bridge,
            }
        }
        ForgeQueryDeclarationBridgeRoutingChecked::Denied(routing) => {
            let denial_cause = routing.cause();
            let continuation_request = routing.continuation_request();
            let continuation_family = routing.continuation_family();
            let envelope = routing.into_envelope();
            let posture = ForgeQueryDeclarationEntryInspectionBridgePosture {
                class: ForgeQueryDeclarationBridgeRoutingClass::ExclusiveBridgeContinuation,
                continuation_mode: continuation_request.map(|request| request.mode()),
                truth_context: continuation_request.map(|request| request.truth_context()),
                continuation_family,
                authority_aspect_summary: envelope_bridge_summary(&envelope),
                routing_digest: "denied".to_string(),
                denial_cause: Some(denial_cause),
            };
            NormalizedInspectionSubject {
                envelope,
                relational: None,
                bridge: Some(posture),
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Bridge,
            }
        }
        ForgeQueryDeclarationBridgeRoutingChecked::Failed(routing) => NormalizedInspectionSubject {
            envelope: routing.into_envelope(),
            relational: None,
            bridge: None,
            signal: None,
            subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Bridge,
        },
    }
}

pub(super) fn normalized_signal<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match checked {
        ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
            let posture = ForgeQueryDeclarationEntryInspectionSignalPosture {
                class: compatibility.class(),
                execution_family: Some(compatibility.execution_family()),
                basis_families: compatibility.basis_families().to_vec(),
                authority_aspect_summary:
                    crate::application::ForgeQueryDeclarationSignalAuthorityAspectSummary::new(
                        compatibility.aspect_contract().clone(),
                        compatibility.aspect_coverage().clone(),
                        compatibility.aspect_coverage_basis(),
                        compatibility.aspect_fit(),
                        crate::application::authority_mismatch_from_fit(compatibility.aspect_fit()),
                        compatibility.dependency_aspects().clone(),
                        compatibility.produced_aspects().clone(),
                    ),
                compatibility_digest: compatibility.signal_compatibility_digest().to_string(),
                denial_cause: None,
            };
            NormalizedInspectionSubject {
                envelope: compatibility.into_envelope(),
                relational: None,
                bridge: None,
                signal: Some(posture),
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Signal,
            }
        }
        ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(compatibility) => {
            NormalizedInspectionSubject {
                envelope: compatibility.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Signal,
            }
        }
        ForgeQueryDeclarationSignalCompatibilityChecked::Denied(compatibility) => {
            let denial_cause = compatibility.cause();
            let execution_family = compatibility.execution_family();
            let basis_families = compatibility.basis_families().to_vec();
            let envelope = compatibility.into_envelope();
            let posture = ForgeQueryDeclarationEntryInspectionSignalPosture {
                class: ForgeQueryDeclarationSignalCompatibilityClass::Denied,
                execution_family,
                basis_families,
                authority_aspect_summary: envelope_signal_summary(&envelope),
                compatibility_digest: "denied".to_string(),
                denial_cause: Some(denial_cause),
            };
            NormalizedInspectionSubject {
                envelope,
                relational: None,
                bridge: None,
                signal: Some(posture),
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Signal,
            }
        }
        ForgeQueryDeclarationSignalCompatibilityChecked::Failed(compatibility) => {
            NormalizedInspectionSubject {
                envelope: compatibility.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Signal,
            }
        }
    }
}

pub(crate) fn envelope_relational_summary<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> crate::application::ForgeQueryDeclarationRelationalAuthorityAspectSummary {
    relational_authority_summary_from_publication(
        envelope.aspect_contract(),
        envelope.aspect_publication(),
        I::Family::relational_truth_contract().as_ref(),
    )
}

pub(crate) fn envelope_bridge_summary<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> crate::application::ForgeQueryDeclarationBridgeAuthorityAspectSummary {
    bridge_authority_summary_from_publication(
        envelope.aspect_contract(),
        envelope.aspect_publication(),
        I::Family::bridge_continuation_contract().as_ref(),
    )
}

pub(crate) fn envelope_signal_summary<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
) -> crate::application::ForgeQueryDeclarationSignalAuthorityAspectSummary {
    signal_authority_summary_from_publication(
        envelope.aspect_contract(),
        envelope.aspect_publication(),
        I::Family::signal_compatibility_contract().as_ref(),
    )
}
