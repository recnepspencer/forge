use super::ServingPhysicalRuntime;

impl ServingPhysicalRuntime {
    pub fn certification_physical_work_courtroom_binding(
        &self,
    ) -> crate::physical_runtime::record_serving::PhysicalWorkCourtroomBinding {
        let identity = crate::physical_runtime::record_serving::C6PhysicalWorkHandoffIdentity::new(
            self.store_identity(),
            self.runtime_identity(),
            self.parts.core.lifecycle_generation(),
        );
        crate::physical_runtime::record_serving::PhysicalWorkCourtroomBinding::new(
            identity,
            self.physical_work_observer(),
        )
    }

    pub fn certification_begin_lifecycle_termination(&self) {
        self.parts.termination.begin_for_certification();
    }

    pub fn certification_stale_physical_work_execution(
        &self,
    ) -> crate::physical_runtime::PhysicalWorkExecution {
        let generation = self
            .parts
            .core
            .lifecycle_generation()
            .certification_predecessor();
        crate::physical_runtime::instance::PhysicalStoreWorkRuntime::execution(
            &self.parts.work_runtime,
            generation,
        )
    }

    pub fn certification_cross_settle_physical_writes(
        &self,
        first: crate::physical_runtime::PhysicalExecutorCommand,
        second: crate::physical_runtime::PhysicalExecutorCommand,
    ) -> Result<
        [crate::physical_runtime::PhysicalWorkEffectFate; 2],
        crate::physical_runtime::PhysicalWorkPreEffectDenial,
    > {
        self.parts
            .work_runtime
            .certification_cross_settle_physical_writes(first, second)
    }

    pub fn certification_pause_physical_command_shards_after_lock(
        &self,
    ) -> crate::physical_runtime::certification::CertificationPhysicalSubmissionPauseGate {
        self.parts
            .work_runtime
            .submission
            .pause_after_command_shard_lock_for_certification()
    }

    pub fn certification_pause_physical_signal_after_dequeue(
        &self,
    ) -> crate::physical_runtime::certification::CertificationPhysicalSignalPauseGate {
        self.parts
            .work_runtime
            .signal
            .pause_after_dequeue_for_certification()
    }

    pub fn certification_pause_physical_execution_at(
        &self,
        checkpoint: crate::physical_runtime::certification::
            CertificationPhysicalExecutionCheckpoint,
    ) -> crate::physical_runtime::certification::CertificationPhysicalExecutionPauseGate {
        self.parts
            .work_runtime
            .executor
            .pause_at_for_certification(checkpoint)
    }

    pub fn certification_fail_next_physical_signal_abandonment(&self) {
        self.parts
            .work_runtime
            .signal
            .fail_next_abandonment_for_certification();
    }

    pub fn certification_physical_signal_route_depth(
        &self,
        route: crate::physical_runtime::PhysicalSignalAspectBindingDigest,
    ) -> Option<usize> {
        self.parts
            .work_runtime
            .signal
            .route_depth_for_certification(route)
    }

    pub fn certification_publication_dependencies(
        &self,
    ) -> Vec<crate::physical_runtime::certification::PhysicalPublicationDependencyObservation> {
        self.parts
            .work_runtime
            .signal
            .publication_dependencies_for_certification()
            .expect("the serving Signal owner remains available")
    }

    pub fn certification_require_serving_inspection(&self) {
        self.parts.work_runtime.health.revoke();
    }

    pub fn certification_publication_summary(
        &self,
    ) -> Result<
        crate::physical_runtime::record_serving::PhysicalRecordPublicationSummary,
        crate::physical_runtime::record_serving::RecordCanonicalObservationDenial,
    > {
        let (root, free_space) = self.parts.publication.planning_snapshot();
        crate::physical_runtime::record_serving::evidence::canonical_observation::observe_runtime_topology(
            crate::physical_runtime::record_serving::evidence::canonical_observation::RuntimeTopologySource {
                media: self.parts.work_runtime.executor.record_serving_media(),
                frame_load: self.parts.frame_ports.loader(),
                format: self.parts.format,
                access: self.parts.access,
                root: &root,
                free_space: &free_space,
            },
        )
    }

    pub fn certification_frame_port_observer(
        &self,
    ) -> crate::physical_runtime::record_serving::FramePortCounterObserver {
        self.parts.frame_ports.observer()
    }

    pub fn certification_reject_next_candidate_publication_after_physical_write(&self) {
        self.parts.frame_ports.reject_next_candidate_publication();
    }

    pub fn certification_reject_next_catalog_eligibility_join(&self) {
        self.parts
            .publication
            .reject_next_catalog_eligibility_join();
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
}
