use super::super::{
    publication_error, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
};
use super::identity::{
    SupportActionId, SupportActionPublicationState, SupportActionRecoveryDisposition,
};
use super::progression::ExecutedSupportAction;
use super::publication::SupportConsequenceEnvelope;
use crate::failure::StoreError;
use serde::{Deserialize, Serialize};

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
        self.validate_action_identity()?;
        self.validate_basis_linkage()?;
        self.validate_artifact_identity()?;
        self.validate_action_origin_posture()?;
        self.validate_publication_lifecycle()
    }

    fn validate_action_identity(&self) -> Result<(), StoreError> {
        if self.action_id.as_str().trim().is_empty() {
            return Err(publication_error(
                "subscription-support durable action records require non-empty action ids",
            ));
        }
        Ok(())
    }

    fn validate_basis_linkage(&self) -> Result<(), StoreError> {
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
        Ok(())
    }

    fn validate_artifact_identity(&self) -> Result<(), StoreError> {
        if self.artifact_id.as_str().trim().is_empty() {
            return Err(publication_error(
                "subscription-support durable action records require non-empty artifact ids",
            ));
        }
        Ok(())
    }

    fn validate_action_origin_posture(&self) -> Result<(), StoreError> {
        if self.action_origin == SubscriptionSupportActionOrigin::TierRecall
            && self.verdict == SubscriptionSupportOperationalVerdict::RebuildRequired
        {
            return Err(publication_error(
                "subscription-support durable action records may not claim rebuild-required tier recall posture",
            ));
        }
        Ok(())
    }

    fn validate_publication_lifecycle(&self) -> Result<(), StoreError> {
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
