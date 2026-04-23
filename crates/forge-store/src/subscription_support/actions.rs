use super::{
    classification_error, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportActionRecoveryDisposition {
    NotInterrupted,
    InterruptedBeforePublication,
    PublishedConsequenceRecovered,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
