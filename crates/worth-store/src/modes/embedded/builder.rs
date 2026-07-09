use super::{
    checkpoint_envelopes::{
        ClassifiedEmbeddedCheckpointEnvelope, EmbeddedCheckpointClassification,
        ExternalRuntimeCommitEnvelope, VerifiedExternalRuntimeCommitEnvelope,
    },
    checkpoint_types::{IntoVerifiedEmbeddedCheckpoint, VerifiedEmbeddedCheckpoint},
    persistence::EmbeddedCheckpointPersistenceReceipt,
};
use crate::{
    evidence::{OperatingModeLane, PersistedModeLaneEvidence},
    facade::{WORTHStore, WORTHStoreBuilder},
    failure::StoreError,
    modes::lifecycle::{EmbeddedModeConstructionPlan, ExternalArtifactIntakeCapabilityProof},
};

#[cfg(test)]
use super::checkpoint_envelopes::ExternalRuntimeCheckpointEnvelope;

#[derive(Debug)]
pub struct EmbeddedModeBuilder {
    construction: EmbeddedModeConstructionPlan,
}

impl EmbeddedModeBuilder {
    pub(crate) fn new(store_builder: WORTHStoreBuilder) -> Self {
        Self {
            construction: EmbeddedModeConstructionPlan::new(store_builder),
        }
    }

    pub fn build(self) -> Result<EmbeddedStoreHandle, StoreError> {
        let (store_builder, capability) = self.construction.into_parts();
        let store = store_builder.build()?;
        store.record_embedded_mode_selection();
        Ok(EmbeddedStoreHandle { store, capability })
    }
}

#[derive(Debug)]
pub struct EmbeddedStoreHandle {
    store: WORTHStore,
    capability: ExternalArtifactIntakeCapabilityProof,
}

impl EmbeddedStoreHandle {
    pub fn admit_external_checkpoint<K, C, I>(
        &self,
        checkpoint: I,
    ) -> Result<VerifiedEmbeddedCheckpoint<K, C>, StoreError>
    where
        I: IntoVerifiedEmbeddedCheckpoint<K, C>,
    {
        checkpoint.into_verified()
    }

    pub fn persist_external_commit(
        &mut self,
        external: ExternalRuntimeCommitEnvelope,
    ) -> Result<crate::PersistedAuthoritativeCommit, StoreError> {
        let _capability = self.capability;
        let verified = VerifiedExternalRuntimeCommitEnvelope::verify(external)?;
        self.store.record_external_commit_intake();
        self.store.append_runtime_envelope(verified.into_envelope())
    }

    pub fn persist_external_checkpoint<K, C>(
        &mut self,
        checkpoint: VerifiedEmbeddedCheckpoint<K, C>,
    ) -> Result<EmbeddedCheckpointPersistenceReceipt, StoreError> {
        self.persist_verified_external_checkpoint(checkpoint.envelope)
    }

    fn persist_verified_external_checkpoint(
        &mut self,
        checkpoint: ClassifiedEmbeddedCheckpointEnvelope,
    ) -> Result<EmbeddedCheckpointPersistenceReceipt, StoreError> {
        let _capability = self.capability;
        self.store.record_external_checkpoint_intake();
        let mut contained_commit_ids = Vec::with_capacity(checkpoint.contained_commits().len());
        for commit in checkpoint.contained_commits() {
            let persisted = self.store.append_runtime_envelope(commit.clone())?;
            contained_commit_ids.push(persisted.envelope().commit.commit_id);
        }
        let receipt_checkpoint_id = checkpoint.checkpoint_id.clone();
        let receipt_classification: EmbeddedCheckpointClassification =
            checkpoint.classification.clone().into();
        let record = checkpoint.into_record(contained_commit_ids.clone());
        self.store.persist_embedded_checkpoint_record(record)?;
        Ok(EmbeddedCheckpointPersistenceReceipt {
            checkpoint_id: receipt_checkpoint_id,
            contained_commit_ids,
            classification: receipt_classification,
        })
    }

    #[cfg(test)]
    pub(crate) fn persist_external_checkpoint_unchecked(
        &mut self,
        checkpoint: ExternalRuntimeCheckpointEnvelope,
    ) -> Result<EmbeddedCheckpointPersistenceReceipt, StoreError> {
        let _capability = self.capability;
        self.store.record_external_checkpoint_intake();
        let checkpoint = match ClassifiedEmbeddedCheckpointEnvelope::classify(checkpoint) {
            Ok(checkpoint) => checkpoint,
            Err(error) => {
                if matches!(
                    error.kind(),
                    crate::StoreErrorKind::EmbeddedCheckpointAuthorityViolation
                ) {
                    self.store.record_embedded_checkpoint_authority_rejection();
                    self.store.record_mode_misuse_rejection();
                }
                return Err(error);
            }
        };
        self.persist_verified_external_checkpoint(checkpoint)
    }

    pub fn fetch_persisted_checkpoint(
        &self,
        checkpoint_id: &str,
    ) -> Result<crate::PersistedEmbeddedCheckpoint, StoreError> {
        self.store
            .fetch_embedded_checkpoint(crate::EmbeddedCheckpointFetchRequest::new(checkpoint_id))
    }

    pub fn store(&self) -> &WORTHStore {
        &self.store
    }

    pub fn milestone_2_lane_evidence(&self) -> PersistedModeLaneEvidence {
        self.store
            .milestone_2_lane_evidence(OperatingModeLane::Embedded)
    }
}
