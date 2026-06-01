use crate::application::{
    ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationRelationalRoutingChecked, ForgeQueryDeclarationSignalCompatibilityChecked,
    ForgeQueryDomainEntryMarker,
};

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
    authority_posture::{normalized_bridge, normalized_relational, normalized_signal},
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
