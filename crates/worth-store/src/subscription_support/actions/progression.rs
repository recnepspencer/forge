use super::super::{
    classification_error, publication_error, SubscriptionSupportActionOrigin,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
};
use super::identity::{SupportActionId, SupportActionRecoveryDisposition};
use super::publication::{
    PublishedSupportConsequence, SupportActionPublicationWitness, SupportConsequenceEnvelope,
};
use crate::failure::StoreError;
use serde::Serialize;

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
