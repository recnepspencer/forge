use crate::application::{
    ForgeQueryDeclarationBridgeContinuationFamily, ForgeQueryDeclarationBridgeContinuationMode,
    ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationBridgeRoutingClass,
    ForgeQueryDeclarationBridgeTruthContext, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRelationalAuthorityFamily, ForgeQueryDeclarationRelationalRoutingChecked,
    ForgeQueryDeclarationRelationalRoutingClass, ForgeQueryDeclarationRelationalTruthClaim,
    ForgeQueryDeclarationSignalCompatibilityChecked, ForgeQueryDeclarationSignalCompatibilityClass,
    ForgeQueryDeclarationSignalExecutionFamily, ForgeQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

use super::{
    super::contribution::{
        ForgeQueryDeclarationEntryContributionEvidenceSet,
        ForgeQueryDeclarationEntryContributionProofScope,
        ForgeQueryDeclarationEntryRetainedSubjectStrength,
    },
    artifact::{
        ForgeQueryDeclarationEntryInspectionBridgePosture,
        ForgeQueryDeclarationEntryInspectionRelationalPosture,
        ForgeQueryDeclarationEntryInspectionSignalPosture,
    },
};

pub enum ForgeQueryDeclarationEntryRetainedSubjectInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    EnvelopeChecked(ForgeQueryDeclarationEnvelopeChecked<D, I>),
    RelationalRoutingChecked(ForgeQueryDeclarationRelationalRoutingChecked<D, I>),
    BridgeRoutingChecked(ForgeQueryDeclarationBridgeRoutingChecked<D, I>),
    SignalCompatibilityChecked(ForgeQueryDeclarationSignalCompatibilityChecked<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryRetainedSubjectInput<D, I>
{
    pub fn envelope_checked(value: ForgeQueryDeclarationEnvelopeChecked<D, I>) -> Self {
        Self::EnvelopeChecked(value)
    }
    pub fn relational_routing_checked(
        value: ForgeQueryDeclarationRelationalRoutingChecked<D, I>,
    ) -> Self {
        Self::RelationalRoutingChecked(value)
    }
    pub fn bridge_routing_checked(value: ForgeQueryDeclarationBridgeRoutingChecked<D, I>) -> Self {
        Self::BridgeRoutingChecked(value)
    }
    pub fn signal_compatibility_checked(
        value: ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
    ) -> Self {
        Self::SignalCompatibilityChecked(value)
    }
}

pub struct ForgeQueryDeclarationEntryInspectionInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    subject: ForgeQueryDeclarationEntryRetainedSubjectInput<D, I>,
    contribution_evidence: Option<ForgeQueryDeclarationEntryContributionEvidenceSet>,
    contribution_scope: ForgeQueryDeclarationEntryContributionProofScope,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryInspectionInput<D, I>
{
    pub fn envelope_checked(value: ForgeQueryDeclarationEnvelopeChecked<D, I>) -> Self {
        Self::new(ForgeQueryDeclarationEntryRetainedSubjectInput::EnvelopeChecked(value))
    }
    pub fn relational_routing_checked(
        value: ForgeQueryDeclarationRelationalRoutingChecked<D, I>,
    ) -> Self {
        Self::new(ForgeQueryDeclarationEntryRetainedSubjectInput::RelationalRoutingChecked(value))
    }
    pub fn bridge_routing_checked(value: ForgeQueryDeclarationBridgeRoutingChecked<D, I>) -> Self {
        Self::new(ForgeQueryDeclarationEntryRetainedSubjectInput::BridgeRoutingChecked(value))
    }
    pub fn signal_compatibility_checked(
        value: ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
    ) -> Self {
        Self::new(ForgeQueryDeclarationEntryRetainedSubjectInput::SignalCompatibilityChecked(value))
    }
    pub fn with_contribution_evidence(
        mut self,
        evidence: ForgeQueryDeclarationEntryContributionEvidenceSet,
    ) -> Self {
        self.contribution_evidence = Some(evidence);
        self
    }
    pub fn with_admitted_plan_scope(
        mut self,
        plan: crate::runtime::ForgeQueryAdmittedIntentPlan,
    ) -> Self {
        self.contribution_scope = self.contribution_scope.with_admitted_plan(plan);
        self
    }
    pub fn with_lower_runtime_boundary_scope(
        mut self,
        envelope: crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        self.contribution_scope = self
            .contribution_scope
            .with_lower_runtime_boundary(envelope);
        self
    }
    fn new(subject: ForgeQueryDeclarationEntryRetainedSubjectInput<D, I>) -> Self {
        Self {
            subject,
            contribution_evidence: None,
            contribution_scope: ForgeQueryDeclarationEntryContributionProofScope::default(),
        }
    }
}

pub(crate) struct NormalizedInspectionSubject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    pub(crate) envelope: ForgeQueryDeclarationEnvelope<D, I>,
    pub(crate) relational: Option<ForgeQueryDeclarationEntryInspectionRelationalPosture>,
    pub(crate) bridge: Option<ForgeQueryDeclarationEntryInspectionBridgePosture>,
    pub(crate) signal: Option<ForgeQueryDeclarationEntryInspectionSignalPosture>,
    pub(crate) subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength,
}

pub(crate) fn normalized_subject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    subject: ForgeQueryDeclarationEntryInspectionInput<D, I>,
) -> (
    NormalizedInspectionSubject<D, I>,
    Option<ForgeQueryDeclarationEntryContributionEvidenceSet>,
    ForgeQueryDeclarationEntryContributionProofScope,
) {
    let normalized = normalize_retained_subject(subject.subject);
    (
        normalized,
        subject.contribution_evidence,
        subject.contribution_scope,
    )
}

pub(crate) fn normalize_retained_subject<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    subject: ForgeQueryDeclarationEntryRetainedSubjectInput<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match subject {
        ForgeQueryDeclarationEntryRetainedSubjectInput::EnvelopeChecked(checked) => match checked {
            ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                NormalizedInspectionSubject {
                    envelope,
                    relational: None,
                    bridge: None,
                    signal: None,
                    subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Envelope,
                }
            }
            ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                NormalizedInspectionSubject {
                    envelope: envelope.into_envelope(),
                    relational: None,
                    bridge: None,
                    signal: None,
                    subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Envelope,
                }
            }
            ForgeQueryDeclarationEnvelopeChecked::Denied(envelope) => NormalizedInspectionSubject {
                envelope: envelope.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Envelope,
            },
            ForgeQueryDeclarationEnvelopeChecked::Failed(envelope) => NormalizedInspectionSubject {
                envelope: envelope.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength::Envelope,
            },
        },
        ForgeQueryDeclarationEntryRetainedSubjectInput::RelationalRoutingChecked(checked) => {
            normalized_relational(checked)
        }
        ForgeQueryDeclarationEntryRetainedSubjectInput::BridgeRoutingChecked(checked) => {
            normalized_bridge(checked)
        }
        ForgeQueryDeclarationEntryRetainedSubjectInput::SignalCompatibilityChecked(checked) => {
            normalized_signal(checked)
        }
    }
}

fn normalized_relational<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    checked: ForgeQueryDeclarationRelationalRoutingChecked<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match checked {
        ForgeQueryDeclarationRelationalRoutingChecked::Routed(routing) => {
            let posture = ForgeQueryDeclarationEntryInspectionRelationalPosture {
                class: routing.class(),
                truth_claim: routing.truth_claim(),
                authority_family: routing.authority_family(),
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
            let posture = ForgeQueryDeclarationEntryInspectionRelationalPosture {
                class: ForgeQueryDeclarationRelationalRoutingClass::ExclusiveRelationalTruth,
                truth_claim: ForgeQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth,
                authority_family: ForgeQueryDeclarationRelationalAuthorityFamily::Runtime,
                routing_digest: "denied".to_string(),
                denial_cause: Some(routing.cause()),
            };
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
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

fn normalized_bridge<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    checked: ForgeQueryDeclarationBridgeRoutingChecked<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match checked {
        ForgeQueryDeclarationBridgeRoutingChecked::Routed(routing) => {
            let posture = ForgeQueryDeclarationEntryInspectionBridgePosture {
                class: routing.class(),
                continuation_mode: routing.continuation_request().mode(),
                truth_context: routing.truth_context(),
                continuation_family: routing.continuation_family(),
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
            let posture = ForgeQueryDeclarationEntryInspectionBridgePosture {
                class: ForgeQueryDeclarationBridgeRoutingClass::ExclusiveBridgeContinuation,
                continuation_mode: ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
                truth_context: ForgeQueryDeclarationBridgeTruthContext::Current,
                continuation_family: ForgeQueryDeclarationBridgeContinuationFamily::RuntimeRoute,
                routing_digest: "denied".to_string(),
                denial_cause: Some(routing.cause()),
            };
            NormalizedInspectionSubject {
                envelope: routing.into_envelope(),
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

fn normalized_signal<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    checked: ForgeQueryDeclarationSignalCompatibilityChecked<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match checked {
        ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
            let posture = ForgeQueryDeclarationEntryInspectionSignalPosture {
                class: compatibility.class(),
                execution_family: compatibility.execution_family(),
                basis_families: compatibility.basis_families().to_vec(),
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
            let posture = ForgeQueryDeclarationEntryInspectionSignalPosture {
                class: ForgeQueryDeclarationSignalCompatibilityClass::Denied,
                execution_family:
                    ForgeQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution,
                basis_families: vec![BasisFamily::CurrentHead],
                compatibility_digest: "denied".to_string(),
                denial_cause: Some(compatibility.cause()),
            };
            NormalizedInspectionSubject {
                envelope: compatibility.into_envelope(),
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
