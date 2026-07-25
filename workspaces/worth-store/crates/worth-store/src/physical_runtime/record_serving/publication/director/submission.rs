use std::sync::Weak;

use super::RecordPublicationDirector;
use crate::physical_runtime::record_serving::{
    publication::append::ManifestCapacityTransition, AdmittedRecordPlacementPolicy,
    PublishedRecordBatch, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
};

#[derive(Clone)]
pub struct PhysicalRecordSubmission {
    director: Weak<RecordPublicationDirector>,
}

pub struct PreparedRecordAppend {
    director: Weak<RecordPublicationDirector>,
    batch: RecordAppendBatch,
    placement: AdmittedRecordPlacementPolicy,
    capacity_transition: ManifestCapacityTransition,
}

impl PhysicalRecordSubmission {
    pub(super) const fn new(director: Weak<RecordPublicationDirector>) -> Self {
        Self { director }
    }

    pub fn prepare_append(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<PreparedRecordAppend, RecordAppendError> {
        self.prepare_with_capacity_transition(
            batch,
            placement,
            ManifestCapacityTransition::PreserveCurrent,
        )
    }

    pub fn append_batch(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        self.prepare_append(batch, placement)?.publish()
    }

    pub fn append_batch_reconstructing_manifest_capacity(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        self.prepare_with_capacity_transition(
            batch,
            placement,
            ManifestCapacityTransition::ReconstructToRequested,
        )?
        .publish()
    }

    fn prepare_with_capacity_transition(
        &self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
        capacity_transition: ManifestCapacityTransition,
    ) -> Result<PreparedRecordAppend, RecordAppendError> {
        let director = self.director.upgrade().ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublicationAuthorityReleased,
        ))?;
        director.preflight(&batch, placement, capacity_transition)?;
        Ok(PreparedRecordAppend {
            director: self.director.clone(),
            batch,
            placement,
            capacity_transition,
        })
    }
}

impl PreparedRecordAppend {
    pub fn publish(self) -> Result<PublishedRecordBatch, RecordAppendError> {
        let director = self.director.upgrade().ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublicationAuthorityReleased,
        ))?;
        director.publish(self.batch, self.placement, self.capacity_transition)
    }
}
