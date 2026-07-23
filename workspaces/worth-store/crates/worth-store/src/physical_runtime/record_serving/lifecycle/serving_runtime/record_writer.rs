use super::PhysicalRecordWriter;
use crate::physical_runtime::record_serving::{
    publication::append, AdmittedRecordPlacementPolicy, PublishedRecordBatch, RecordAppendBatch,
    RecordAppendDenial, RecordAppendError,
};

impl PhysicalRecordWriter<'_> {
    pub fn append_batch(
        &mut self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        self.append_batch_with_capacity_transition(
            batch,
            placement,
            append::ManifestCapacityTransition::PreserveCurrent,
        )
    }

    pub fn append_batch_reconstructing_manifest_capacity(
        &mut self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        self.append_batch_with_capacity_transition(
            batch,
            placement,
            append::ManifestCapacityTransition::ReconstructToRequested,
        )
    }

    fn append_batch_with_capacity_transition(
        &mut self,
        batch: RecordAppendBatch,
        placement: AdmittedRecordPlacementPolicy,
        capacity_transition: append::ManifestCapacityTransition,
    ) -> Result<PublishedRecordBatch, RecordAppendError> {
        if self.health.requires_inspection() {
            return Err(RecordAppendError::Denied(
                RecordAppendDenial::ServingRequiresInspection,
            ));
        }
        if !placement.admits(self.format) {
            return Err(RecordAppendError::Denied(
                RecordAppendDenial::PlacementFormatMismatch,
            ));
        }
        batch
            .preflight(self.access)
            .map_err(RecordAppendError::Denied)?;
        crate::physical_runtime::record_serving::planning::batch_placement::preflight_placement(
            self.format,
            placement,
            &batch,
        )?;
        let operation_bytes = crate::physical_runtime::record_serving::planning::batch_placement::append_operation_allocation_bytes(
            self.format,
            placement,
            &batch,
        );
        let _allocation = self
            .frame_ports
            .begin_operation(
                worth_store_buffer_pool::OperationAllocationScope::ForegroundWrite,
                operation_bytes,
            )
            .map_err(|reason| {
                RecordAppendError::Denied(RecordAppendDenial::ResidencyUnavailable(reason))
            })?;
        match append::append(
            append::RecordAppendExecutionContext {
                media: self.media,
                format: self.format,
                access: self.access,
                current_root: self.current_root,
                current_free_space: self.free_space,
                allocation_frontier: self.allocation_frontier,
                placement,
                frame_ports: self.frame_ports,
                capacity_transition,
            },
            batch,
        ) {
            Ok((published, successor, free_space)) => {
                *self.current_root = successor;
                *self.free_space = free_space;
                Ok(published)
            }
            Err(RecordAppendError::Unpublished(failure)) => {
                *self.publication_residue = self.publication_residue.merge(failure.residue());
                if failure.requires_inspection() {
                    self.health.revoke();
                }
                Err(RecordAppendError::Unpublished(failure))
            }
            Err(error @ RecordAppendError::Indeterminate(failure)) => {
                *self.publication_residue = failure.residue();
                self.health.revoke();
                Err(error)
            }
            Err(RecordAppendError::StreamFailed(failure)) => {
                if failure.requires_inspection() {
                    self.health.revoke();
                }
                Err(RecordAppendError::StreamFailed(failure))
            }
            Err(RecordAppendError::Denied(denial)) => {
                self.health.observe_append_denial(denial);
                Err(RecordAppendError::Denied(denial))
            }
        }
    }
}
