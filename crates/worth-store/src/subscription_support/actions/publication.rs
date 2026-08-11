use super::super::{
    SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportOperationalVerdict,
};
use super::identity::{SupportActionId, SupportActionRecoveryDisposition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportActionPublicationWitness {
    pub(super) action_id: SupportActionId,
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
    pub(super) action_id: SupportActionId,
    pub(super) artifact_id: SubscriptionSupportArtifactId,
    pub(super) verdict: SubscriptionSupportOperationalVerdict,
    pub(super) action_origin: SubscriptionSupportActionOrigin,
    pub(super) recovery_disposition: SupportActionRecoveryDisposition,
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
    pub(super) envelope: SupportConsequenceEnvelope,
    pub(super) witness: SupportActionPublicationWitness,
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
