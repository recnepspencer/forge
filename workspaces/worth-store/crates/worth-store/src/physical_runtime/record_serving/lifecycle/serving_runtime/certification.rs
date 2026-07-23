use super::ServingPhysicalRuntime;

impl ServingPhysicalRuntime {
    pub fn certification_begin_lifecycle_termination(&self) {
        self.parts.termination.begin_for_certification();
    }

    pub fn certification_pause_physical_command_shards_after_lock(
        &self,
    ) -> crate::physical_runtime::certification::CertificationPhysicalSubmissionPauseGate {
        self.parts
            .work_submission
            .pause_after_command_shard_lock_for_certification()
    }

    pub fn certification_fail_physical_signal_worker(&self) {
        self.parts.signal_owner.fail_worker_for_certification();
    }

    pub fn certification_require_serving_inspection(&self) {
        self.parts.health.revoke();
    }

    pub fn certification_publication_summary(
        &self,
    ) -> Result<
        crate::physical_runtime::record_serving::PhysicalRecordPublicationSummary,
        crate::physical_runtime::record_serving::RecordCanonicalObservationDenial,
    > {
        crate::physical_runtime::record_serving::evidence::canonical_observation::observe_runtime_topology(
            crate::physical_runtime::record_serving::evidence::canonical_observation::RuntimeTopologySource {
                media: self.parts.executor.record_serving_media(),
                frame_load: self.parts.frame_ports.loader(),
                format: self.parts.format,
                access: self.parts.access,
                root: &self.parts.current_root,
                free_space: &self.parts.free_space,
            },
        )
    }

    pub fn certification_frame_port_observer(
        &self,
    ) -> crate::physical_runtime::record_serving::FramePortCounterObserver {
        self.parts.frame_ports.observer()
    }

    pub fn certification_admit_dirty_frame(
        &self,
        coordinate: worth_store_physical_format::RecordFrameCoordinate,
        bytes: Vec<u8>,
    ) -> Result<(), worth_store_buffer_pool::PhysicalResidencyDenial> {
        self.parts
            .frame_ports
            .admit_dirty_for_certification(coordinate, bytes)
    }

    pub fn certification_writeback_declaration(
        &self,
        coordinate: worth_store_physical_format::RecordFrameCoordinate,
        grouping: worth_store_buffer_pool::BufferPoolQueueGroupingScope,
        flush_epoch: u64,
        resource_shape: worth_store_contracts::QueueProducerResourceShape,
    ) -> Result<
        worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration,
        worth_store_buffer_pool::PhysicalResidencyDenial,
    > {
        self.parts
            .frame_ports
            .writeback_declaration_for_certification(
                coordinate,
                grouping,
                flush_epoch,
                resource_shape,
            )
    }

    #[cfg(test)]
    pub(crate) fn certification_read_artifact_range(
        &self,
        coordinate: worth_store_physical_format::RecordFrameCoordinate,
    ) -> Vec<u8> {
        let mut bytes = vec![0; coordinate.length() as usize];
        crate::physical_runtime::record_serving::residency::artifact_tree::PhysicalRecordArtifactTree::new(
            self.parts.executor.record_serving_media(),
        )
        .read_exact_at(coordinate.artifact(), coordinate.offset(), &mut bytes)
        .expect("certification artifact range must remain readable");
        bytes
    }
}
