use super::super::{
    publication_error, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
};
use super::durable::SupportActionDurableRecord;
use super::identity::{
    SupportActionId, SupportActionPublicationState, SupportActionRecoveryDisposition,
};
use super::publication::{
    CompletedSupportProgramAction, SupportActionPublicationWitness, SupportConsequenceEnvelope,
};
use crate::failure::StoreError;
use serde::Serialize;

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
