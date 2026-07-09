use super::{
    classification_error, publication_error, SubscriptionSupportActionOrigin,
    SubscriptionSupportArtifactId, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict,
};
use crate::failure::StoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SupportActionId(String);

impl SupportActionId {
    pub fn new(value: impl Into<String>) -> Result<Self, StoreError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(classification_error(
                "subscription-support action ids must be non-empty",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportActionRecoveryDisposition {
    NotInterrupted,
    InterruptedBeforePublication,
    PublishedConsequenceRecovered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportActionPublicationState {
    PendingPublication,
    InterruptedBeforePublication,
    PublishedConsequence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RawSupportProgramAction {
    action_id: SupportActionId,
    basis: SubscriptionSupportOperationalBasis,
    planned_verdict: SubscriptionSupportOperationalVerdict,
}

impl RawSupportProgramAction {
    pub fn new(
        action_id: SupportActionId,
        basis: SubscriptionSupportOperationalBasis,
        planned_verdict: SubscriptionSupportOperationalVerdict,
    ) -> Result<Self, StoreError> {
        if basis.action_origin() == SubscriptionSupportActionOrigin::TierRecall
            && planned_verdict == SubscriptionSupportOperationalVerdict::RebuildRequired
        {
            return Err(classification_error(
                "tier-recall support actions may not claim rebuild-required maintenance posture",
            ));
        }
        Ok(Self {
            action_id,
            basis,
            planned_verdict,
        })
    }

    pub fn plan(self) -> PlannedSupportAction {
        PlannedSupportAction {
            action_id: self.action_id,
            basis: self.basis,
            planned_verdict: self.planned_verdict,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlannedSupportAction {
    action_id: SupportActionId,
    basis: SubscriptionSupportOperationalBasis,
    planned_verdict: SubscriptionSupportOperationalVerdict,
}

impl PlannedSupportAction {
    pub fn verify(self) -> ProofCheckedSupportAction {
        ProofCheckedSupportAction {
            action_id: self.action_id,
            basis: self.basis,
            planned_verdict: self.planned_verdict,
        }
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProofCheckedSupportAction {
    action_id: SupportActionId,
    basis: SubscriptionSupportOperationalBasis,
    planned_verdict: SubscriptionSupportOperationalVerdict,
}

impl ProofCheckedSupportAction {
    pub fn execute(self) -> ExecutedSupportAction {
        ExecutedSupportAction {
            action_id: self.action_id,
            basis: self.basis,
            planned_verdict: self.planned_verdict,
            disposition: SupportActionRecoveryDisposition::NotInterrupted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExecutedSupportAction {
    action_id: SupportActionId,
    basis: SubscriptionSupportOperationalBasis,
    planned_verdict: SubscriptionSupportOperationalVerdict,
    disposition: SupportActionRecoveryDisposition,
}

impl ExecutedSupportAction {
    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.basis
    }

    pub fn planned_verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.planned_verdict
    }

    pub(crate) fn publication_envelope_header_bytes(&self) -> Result<u64, StoreError> {
        let envelope = SupportConsequenceEnvelope {
            action_id: self.action_id.clone(),
            artifact_id: self.basis.artifact_id().clone(),
            verdict: self.planned_verdict,
            action_origin: self.basis.action_origin(),
            recovery_disposition: self.disposition,
        };
        let encoded = serde_json::to_vec(&envelope).map_err(|err| {
            publication_error(format!(
                "subscription-support action envelope must serialize before publication: {err}"
            ))
        })?;
        Ok(encoded.len() as u64)
    }

    pub(crate) fn publish(self) -> PublishedSupportConsequence {
        let envelope = SupportConsequenceEnvelope {
            action_id: self.action_id.clone(),
            artifact_id: self.basis.artifact_id().clone(),
            verdict: self.planned_verdict,
            action_origin: self.basis.action_origin(),
            recovery_disposition: self.disposition,
        };
        PublishedSupportConsequence {
            envelope,
            witness: SupportActionPublicationWitness {
                action_id: self.action_id,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportActionPublicationWitness {
    action_id: SupportActionId,
}

impl SupportActionPublicationWitness {
    #[allow(dead_code)]
    pub(crate) fn new(action_id: SupportActionId) -> Self {
        Self { action_id }
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportConsequenceEnvelope {
    action_id: SupportActionId,
    artifact_id: SubscriptionSupportArtifactId,
    verdict: SubscriptionSupportOperationalVerdict,
    action_origin: SubscriptionSupportActionOrigin,
    recovery_disposition: SupportActionRecoveryDisposition,
}

impl SupportConsequenceEnvelope {
    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn action_origin(&self) -> SubscriptionSupportActionOrigin {
        self.action_origin
    }

    pub fn recovery_disposition(&self) -> SupportActionRecoveryDisposition {
        self.recovery_disposition
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublishedSupportConsequence {
    envelope: SupportConsequenceEnvelope,
    witness: SupportActionPublicationWitness,
}

impl PublishedSupportConsequence {
    pub fn complete(self) -> CompletedSupportProgramAction {
        CompletedSupportProgramAction {
            envelope: self.envelope,
            witness: self.witness,
        }
    }

    pub fn envelope(&self) -> &SupportConsequenceEnvelope {
        &self.envelope
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompletedSupportProgramAction {
    envelope: SupportConsequenceEnvelope,
    witness: SupportActionPublicationWitness,
}

impl CompletedSupportProgramAction {
    #[allow(dead_code)]
    pub(crate) fn new(
        envelope: SupportConsequenceEnvelope,
        witness: SupportActionPublicationWitness,
    ) -> Self {
        Self { envelope, witness }
    }

    pub fn envelope(&self) -> &SupportConsequenceEnvelope {
        &self.envelope
    }

    pub fn publication_witness(&self) -> &SupportActionPublicationWitness {
        &self.witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportActionDurableRecord {
    action_id: SupportActionId,
    basis: SubscriptionSupportOperationalBasis,
    artifact_id: SubscriptionSupportArtifactId,
    verdict: SubscriptionSupportOperationalVerdict,
    action_origin: SubscriptionSupportActionOrigin,
    publication_state: SupportActionPublicationState,
    published_envelope: Option<SupportConsequenceEnvelope>,
}

impl SupportActionDurableRecord {
    pub(crate) fn from_executed(action: &ExecutedSupportAction) -> Self {
        Self {
            action_id: action.action_id().clone(),
            basis: action.basis().clone(),
            artifact_id: action.basis().artifact_id().clone(),
            verdict: action.planned_verdict(),
            action_origin: action.basis().action_origin(),
            publication_state: SupportActionPublicationState::PendingPublication,
            published_envelope: None,
        }
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.basis
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn action_origin(&self) -> SubscriptionSupportActionOrigin {
        self.action_origin
    }

    pub fn publication_state(&self) -> SupportActionPublicationState {
        self.publication_state
    }

    pub fn published_envelope(&self) -> Option<&SupportConsequenceEnvelope> {
        self.published_envelope.as_ref()
    }

    pub(crate) fn storage_key(&self) -> String {
        self.action_id.as_str().to_string()
    }

    pub(crate) fn mark_interrupted_before_publication(&mut self) {
        self.publication_state = SupportActionPublicationState::InterruptedBeforePublication;
        self.published_envelope = None;
    }

    pub(crate) fn mark_published_consequence(
        &mut self,
        envelope: SupportConsequenceEnvelope,
    ) -> Result<(), StoreError> {
        if envelope.action_id() != &self.action_id
            || envelope.artifact_id() != &self.artifact_id
            || envelope.verdict() != self.verdict
            || envelope.action_origin() != self.action_origin
        {
            return Err(publication_error(
                "subscription-support published consequence envelope drifted from its durable action record",
            ));
        }
        self.publication_state = SupportActionPublicationState::PublishedConsequence;
        self.published_envelope = Some(envelope);
        Ok(())
    }

    pub(crate) fn validate(&self) -> Result<(), StoreError> {
        if self.action_id.as_str().trim().is_empty() {
            return Err(publication_error(
                "subscription-support durable action records require non-empty action ids",
            ));
        }
        if self.basis.artifact_id() != &self.artifact_id {
            return Err(publication_error(
                "subscription-support durable action basis drifted from the durable action artifact id",
            ));
        }
        if self.basis.action_origin() != self.action_origin {
            return Err(publication_error(
                "subscription-support durable action basis drifted from the durable action origin",
            ));
        }
        if self.artifact_id.as_str().trim().is_empty() {
            return Err(publication_error(
                "subscription-support durable action records require non-empty artifact ids",
            ));
        }
        if self.action_origin == SubscriptionSupportActionOrigin::TierRecall
            && self.verdict == SubscriptionSupportOperationalVerdict::RebuildRequired
        {
            return Err(publication_error(
                "subscription-support durable action records may not claim rebuild-required tier recall posture",
            ));
        }
        match self.publication_state {
            SupportActionPublicationState::PendingPublication
            | SupportActionPublicationState::InterruptedBeforePublication => {
                if self.published_envelope.is_some() {
                    return Err(publication_error(
                        "subscription-support interrupted or pending action records must not carry a published envelope",
                    ));
                }
            }
            SupportActionPublicationState::PublishedConsequence => {
                let Some(envelope) = &self.published_envelope else {
                    return Err(publication_error(
                        "subscription-support published action records require a durable envelope",
                    ));
                };
                if envelope.action_id() != &self.action_id
                    || envelope.artifact_id() != &self.artifact_id
                    || envelope.verdict() != self.verdict
                    || envelope.action_origin() != self.action_origin
                {
                    return Err(publication_error(
                        "subscription-support durable action record envelope drifted from its indexed action identity",
                    ));
                }
                if envelope.recovery_disposition()
                    != SupportActionRecoveryDisposition::NotInterrupted
                {
                    return Err(publication_error(
                        "subscription-support durable published envelopes may not persist recovered or interrupted disposition",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportActionPublicationRecoveryReport {
    action_id: SupportActionId,
    translation_basis: SubscriptionSupportOperationalBasis,
    artifact_id: SubscriptionSupportArtifactId,
    verdict: SubscriptionSupportOperationalVerdict,
    action_origin: SubscriptionSupportActionOrigin,
    recovery_disposition: SupportActionRecoveryDisposition,
    completed_action: Option<CompletedSupportProgramAction>,
}

impl SubscriptionSupportActionPublicationRecoveryReport {
    pub(crate) fn from_durable_record(
        record: &SupportActionDurableRecord,
    ) -> Result<Self, StoreError> {
        let (recovery_disposition, completed_action) = match record.publication_state() {
            SupportActionPublicationState::PendingPublication
            | SupportActionPublicationState::InterruptedBeforePublication => (
                SupportActionRecoveryDisposition::InterruptedBeforePublication,
                None,
            ),
            SupportActionPublicationState::PublishedConsequence => {
                let envelope = record
                    .published_envelope()
                    .cloned()
                    .ok_or_else(|| {
                        publication_error(
                            "subscription-support published recovery requires a durable consequence envelope",
                        )
                    })?;
                let recovered_envelope = SupportConsequenceEnvelope {
                    recovery_disposition:
                        SupportActionRecoveryDisposition::PublishedConsequenceRecovered,
                    ..envelope
                };
                (
                    SupportActionRecoveryDisposition::PublishedConsequenceRecovered,
                    Some(CompletedSupportProgramAction::new(
                        recovered_envelope.clone(),
                        SupportActionPublicationWitness::new(
                            recovered_envelope.action_id().clone(),
                        ),
                    )),
                )
            }
        };
        Ok(Self {
            action_id: record.action_id().clone(),
            translation_basis: record.basis().clone(),
            artifact_id: record.artifact_id().clone(),
            verdict: record.verdict(),
            action_origin: record.action_origin(),
            recovery_disposition,
            completed_action,
        })
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn translation_basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.translation_basis
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn action_origin(&self) -> SubscriptionSupportActionOrigin {
        self.action_origin
    }

    pub fn recovery_disposition(&self) -> SupportActionRecoveryDisposition {
        self.recovery_disposition
    }

    pub fn completed_action(&self) -> Option<&CompletedSupportProgramAction> {
        self.completed_action.as_ref()
    }
}
