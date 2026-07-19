#[cfg(test)]
use crate::application::WorthQueryDeclarationEnvelope;
use crate::application::{
    WorthQueryDeclarationBridgeRoutingChecked, WorthQueryDeclarationEnvelopeChecked,
    WorthQueryDeclarationInput, WorthQueryDeclarationRelationalRoutingChecked,
    WorthQueryDeclarationSignalCompatibilityChecked, WorthQueryDomainEntryMarker,
};

use super::super::contribution::{
    WorthQueryDeclarationEntryContributionEvidenceSet,
    WorthQueryDeclarationEntryContributionProofScope,
};
#[cfg(test)]
use super::{
    super::contribution::WorthQueryDeclarationEntryRetainedSubjectStrength,
    artifact::{
        WorthQueryDeclarationEntryInspectionBridgePosture,
        WorthQueryDeclarationEntryInspectionRelationalPosture,
        WorthQueryDeclarationEntryInspectionSignalPosture,
    },
    authority_posture::{normalized_bridge, normalized_relational, normalized_signal},
};

pub enum WorthQueryDeclarationEntryRetainedSubjectInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    EnvelopeChecked(WorthQueryDeclarationEnvelopeChecked<D, I>),
    RelationalRoutingChecked(WorthQueryDeclarationRelationalRoutingChecked<D, I>),
    BridgeRoutingChecked(WorthQueryDeclarationBridgeRoutingChecked<D, I>),
    SignalCompatibilityChecked(WorthQueryDeclarationSignalCompatibilityChecked<D, I>),
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEntryRetainedSubjectInput<D, I>
{
    pub fn envelope_checked(value: WorthQueryDeclarationEnvelopeChecked<D, I>) -> Self {
        Self::EnvelopeChecked(value)
    }
}

pub struct WorthQueryDeclarationEntryInspectionInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    subject: WorthQueryDeclarationEntryRetainedSubjectInput<D, I>,
    contribution_evidence: Option<WorthQueryDeclarationEntryContributionEvidenceSet>,
    contribution_scope: WorthQueryDeclarationEntryContributionProofScope,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEntryInspectionInput<D, I>
{
    pub fn envelope_checked(value: WorthQueryDeclarationEnvelopeChecked<D, I>) -> Self {
        Self::new(WorthQueryDeclarationEntryRetainedSubjectInput::EnvelopeChecked(value))
    }
    pub fn relational_routing_checked(
        value: WorthQueryDeclarationRelationalRoutingChecked<D, I>,
    ) -> Self {
        Self::new(WorthQueryDeclarationEntryRetainedSubjectInput::RelationalRoutingChecked(value))
    }
    pub fn bridge_routing_checked(value: WorthQueryDeclarationBridgeRoutingChecked<D, I>) -> Self {
        Self::new(WorthQueryDeclarationEntryRetainedSubjectInput::BridgeRoutingChecked(value))
    }
    pub fn signal_compatibility_checked(
        value: WorthQueryDeclarationSignalCompatibilityChecked<D, I>,
    ) -> Self {
        Self::new(WorthQueryDeclarationEntryRetainedSubjectInput::SignalCompatibilityChecked(value))
    }
    pub fn with_contribution_evidence(
        mut self,
        evidence: WorthQueryDeclarationEntryContributionEvidenceSet,
    ) -> Self {
        self.contribution_evidence = Some(evidence);
        self
    }
    pub fn with_admitted_plan_scope(
        mut self,
        plan: crate::runtime::WorthQueryAdmittedIntentPlan,
    ) -> Self {
        self.contribution_scope = self.contribution_scope.with_admitted_plan(plan);
        self
    }
    pub fn with_lower_runtime_boundary_scope(
        mut self,
        envelope: crate::runtime::WorthQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        self.contribution_scope = self
            .contribution_scope
            .with_lower_runtime_boundary(envelope);
        self
    }
    fn new(subject: WorthQueryDeclarationEntryRetainedSubjectInput<D, I>) -> Self {
        Self {
            subject,
            contribution_evidence: None,
            contribution_scope: WorthQueryDeclarationEntryContributionProofScope::default(),
        }
    }
}

#[cfg(test)]
pub(crate) struct NormalizedInspectionSubject<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    pub(crate) envelope: WorthQueryDeclarationEnvelope<D, I>,
    pub(crate) relational: Option<WorthQueryDeclarationEntryInspectionRelationalPosture>,
    pub(crate) bridge: Option<WorthQueryDeclarationEntryInspectionBridgePosture>,
    pub(crate) signal: Option<WorthQueryDeclarationEntryInspectionSignalPosture>,
    pub(crate) subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength,
}

#[cfg(test)]
pub(crate) fn normalized_subject<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    subject: WorthQueryDeclarationEntryInspectionInput<D, I>,
) -> (
    NormalizedInspectionSubject<D, I>,
    Option<WorthQueryDeclarationEntryContributionEvidenceSet>,
    WorthQueryDeclarationEntryContributionProofScope,
) {
    let normalized = normalize_retained_subject(subject.subject);
    (
        normalized,
        subject.contribution_evidence,
        subject.contribution_scope,
    )
}

#[cfg(test)]
pub(crate) fn normalize_retained_subject<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    subject: WorthQueryDeclarationEntryRetainedSubjectInput<D, I>,
) -> NormalizedInspectionSubject<D, I> {
    match subject {
        WorthQueryDeclarationEntryRetainedSubjectInput::EnvelopeChecked(checked) => match checked {
            WorthQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
                NormalizedInspectionSubject {
                    envelope,
                    relational: None,
                    bridge: None,
                    signal: None,
                    subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Envelope,
                }
            }
            WorthQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                NormalizedInspectionSubject {
                    envelope: envelope.into_envelope(),
                    relational: None,
                    bridge: None,
                    signal: None,
                    subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Envelope,
                }
            }
            WorthQueryDeclarationEnvelopeChecked::Denied(envelope) => NormalizedInspectionSubject {
                envelope: envelope.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Envelope,
            },
            WorthQueryDeclarationEnvelopeChecked::Failed(envelope) => NormalizedInspectionSubject {
                envelope: envelope.into_envelope(),
                relational: None,
                bridge: None,
                signal: None,
                subject_strength: WorthQueryDeclarationEntryRetainedSubjectStrength::Envelope,
            },
        },
        WorthQueryDeclarationEntryRetainedSubjectInput::RelationalRoutingChecked(checked) => {
            normalized_relational(checked)
        }
        WorthQueryDeclarationEntryRetainedSubjectInput::BridgeRoutingChecked(checked) => {
            normalized_bridge(checked)
        }
        WorthQueryDeclarationEntryRetainedSubjectInput::SignalCompatibilityChecked(checked) => {
            normalized_signal(checked)
        }
    }
}
