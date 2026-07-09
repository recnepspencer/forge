use crate::application::{
    bridge_authority_summary_from_publication, relational_authority_summary_from_publication,
    signal_authority_summary_from_publication, WorthQueryDeclarationBridgeRoutingChecked,
    WorthQueryDeclarationBridgeRoutingClass, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationRelationalRoutingChecked, WorthQueryDeclarationRelationalRoutingClass,
    WorthQueryDeclarationSignalCompatibilityChecked, WorthQueryDeclarationSignalCompatibilityClass,
    WorthQueryDomainEntryMarker,
};

use super::{
    super::contribution::WorthQueryDeclarationEntryRetainedSubjectStrength,
    artifact::{
        WorthQueryDeclarationEntryInspectionBridgePosture,
        WorthQueryDeclarationEntryInspectionRelationalPosture,
        WorthQueryDeclarationEntryInspectionSignalPosture,
    },
    subject::NormalizedInspectionSubject,
};

pub(super) fn normalized_relational<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryDeclarationRelationalRoutingChecked<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match checked {
        WorthQueryDeclarationRelationalRoutingChecked::Routed(routing) => {
            let posture = WorthQueryDeclarationEntryInspectionRelationalPosture {
                class: routing.class(),
                truth_claim: Some(routing.truth_claim()),
                authority_family: Some(routing.authority_family()),
                authority_aspect_summary:
                    crate::application::WorthQueryDeclarationRelationalAuthorityAspectSummary::new(
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
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Relational,
            }
        }
        WorthQueryDeclarationRelationalRoutingChecked::Deferred(routing) => {
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Relational,
            }
        }
        WorthQueryDeclarationRelationalRoutingChecked::Denied(routing) => {
            let denial_cause = routing.cause();
            let truth_claim = routing.truth_claim();
            let authority_family = routing.authority_family();
            let envelope = routing.into_envelope();
            let posture = WorthQueryDeclarationEntryInspectionRelationalPosture {
                class: WorthQueryDeclarationRelationalRoutingClass::ExclusiveRelationalTruth,
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
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Relational,
            }
        }
        WorthQueryDeclarationRelationalRoutingChecked::Failed(routing) => {
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Relational,
            }
        }
    }
}

pub(super) fn normalized_bridge<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryDeclarationBridgeRoutingChecked<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match checked {
        WorthQueryDeclarationBridgeRoutingChecked::Routed(routing) => {
            let posture = WorthQueryDeclarationEntryInspectionBridgePosture {
                class: routing.class(),
                continuation_mode: Some(routing.continuation_request().mode()),
                truth_context: Some(routing.truth_context()),
                continuation_family: Some(routing.continuation_family()),
                authority_aspect_summary:
                    crate::application::WorthQueryDeclarationBridgeAuthorityAspectSummary::new(
                        routing.aspect_contract().clone(),
                        routing.aspect_coverage().clone(),
                        routing.aspect_coverage_basis(),
                        routing.aspect_fit(),
                        crate::application::authority_mismatch_from_fit(routing.aspect_fit()),
                        routing.mapped_aspects().clone(),
                        crate::application::WorthQueryDeclarationAspectCoverageBasis::BridgeMappedCoverage,
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
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Bridge,
            }
        }
        WorthQueryDeclarationBridgeRoutingChecked::Deferred(routing) => {
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Bridge,
            }
        }
        WorthQueryDeclarationBridgeRoutingChecked::Denied(routing) => {
            let denial_cause = routing.cause();
            let continuation_request = routing.continuation_request();
            let continuation_family = routing.continuation_family();
            let envelope = routing.into_envelope();
            let posture = WorthQueryDeclarationEntryInspectionBridgePosture {
                class: WorthQueryDeclarationBridgeRoutingClass::ExclusiveBridgeContinuation,
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
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Bridge,
            }
        }
        WorthQueryDeclarationBridgeRoutingChecked::Failed(routing) => NormalizedInspectionSubject {
            envelope: routing.into_envelope(),
            relational: None,
            bridge: None,
            signal: None,
            subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Bridge,
        },
    }
}

pub(super) fn normalized_signal<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryDeclarationSignalCompatibilityChecked<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match checked {
        WorthQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
            let posture = WorthQueryDeclarationEntryInspectionSignalPosture {
                class: compatibility.class(),
                execution_family: Some(compatibility.execution_family()),
                basis_families: compatibility.basis_families().to_vec(),
                authority_aspect_summary:
                    crate::application::WorthQueryDeclarationSignalAuthorityAspectSummary::new(
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
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Signal,
            }
        }
        WorthQueryDeclarationSignalCompatibilityChecked::Deferred(compatibility) => {
            NormalizedInspectionSubject {
                envelope: compatibility.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Signal,
            }
        }
        WorthQueryDeclarationSignalCompatibilityChecked::Denied(compatibility) => {
            let denial_cause = compatibility.cause();
            let execution_family = compatibility.execution_family();
            let basis_families = compatibility.basis_families().to_vec();
            let envelope = compatibility.into_envelope();
            let posture = WorthQueryDeclarationEntryInspectionSignalPosture {
                class: WorthQueryDeclarationSignalCompatibilityClass::Denied,
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
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Signal,
            }
        }
        WorthQueryDeclarationSignalCompatibilityChecked::Failed(compatibility) => {
            NormalizedInspectionSubject {
                envelope: compatibility.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Signal,
            }
        }
    }
}

pub(crate) fn envelope_relational_summary<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
) -> crate::application::WorthQueryDeclarationRelationalAuthorityAspectSummary {
    relational_authority_summary_from_publication(
        envelope.aspect_contract(),
        envelope.aspect_publication(),
        I::Family::relational_truth_contract().as_ref(),
    )
}

pub(crate) fn envelope_bridge_summary<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
) -> crate::application::WorthQueryDeclarationBridgeAuthorityAspectSummary {
    bridge_authority_summary_from_publication(
        envelope.aspect_contract(),
        envelope.aspect_publication(),
        I::Family::bridge_continuation_contract().as_ref(),
    )
}

pub(crate) fn envelope_signal_summary<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    envelope: &WorthQueryDeclarationEnvelope<D, I>,
) -> crate::application::WorthQueryDeclarationSignalAuthorityAspectSummary {
    signal_authority_summary_from_publication(
        envelope.aspect_contract(),
        envelope.aspect_publication(),
        I::Family::signal_compatibility_contract().as_ref(),
    )
}
