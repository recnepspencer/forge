use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, DurablePhysicalRootManifest,
};

use crate::physical_runtime::{
    media_ownership::{MediaShutdownOutcome, PhysicalMediaObserver},
    runtime::PhysicalRuntimeCore,
    AbortedRuntime, ClosedRuntime, RuntimeIdentity,
};

use super::super::{
    lifecycle::record_lifecycle::RecordServingOwner,
    lifecycle::record_observation::PhysicalRecordObserver,
};
use super::super::{
    publication::append, AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy,
    AdmittedRecordPlacementPolicy, PhysicalRecordReader, PublishedRecordBatch,
    RecordAllocationFrontier, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
    RecordPublicationResidueObservation, RecordServingState,
};
use super::serving_health::ServingHealth;

#[cfg(feature = "certification-test-authority")]
#[path = "serving_runtime/certification.rs"]
mod certification;

pub struct ServingPhysicalRuntime {
    termination: crate::physical_runtime::lifecycle::LifecycleTerminationGuard,
    owner: RecordServingOwner,
    media: QualifiedFilesystemMedia,
    core: PhysicalRuntimeCore,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
    current_root: DurablePhysicalRootManifest,
    free_space: worth_store_physical_format::DurableFreeSpaceManifestHeader,
    allocation_frontier: RecordAllocationFrontier,
    publication_residue: RecordPublicationResidueObservation,
    health: ServingHealth,
    frame_ports: super::super::residency::frame_ports::RecordFramePorts,
}

pub struct PhysicalRecordWriter<'runtime> {
    media: &'runtime QualifiedFilesystemMedia,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
    current_root: &'runtime mut DurablePhysicalRootManifest,
    free_space: &'runtime mut worth_store_physical_format::DurableFreeSpaceManifestHeader,
    allocation_frontier: &'runtime mut RecordAllocationFrontier,
    publication_residue: &'runtime mut RecordPublicationResidueObservation,
    health: &'runtime ServingHealth,
    _lease: super::super::lifecycle::record_lifecycle::RecordWriterLease,
    frame_ports: &'runtime super::super::residency::frame_ports::RecordFramePorts,
}

impl ServingPhysicalRuntime {
    pub(in crate::physical_runtime::record_serving) fn from_admission(
        termination: crate::physical_runtime::lifecycle::LifecycleTerminationGuard,
        media: QualifiedFilesystemMedia,
        core: PhysicalRuntimeCore,
        bootstrap: RecordServingState,
        allocation_frontier: RecordAllocationFrontier,
        frame_ports: super::super::residency::frame_ports::RecordFramePorts,
    ) -> Self {
        Self {
            termination,
            owner: RecordServingOwner::new(),
            media,
            core,
            format: bootstrap.format,
            access: bootstrap.access,
            current_root: bootstrap.current_root,
            free_space: bootstrap.free_space,
            allocation_frontier,
            publication_residue: bootstrap.publication_residue,
            health: ServingHealth::new(!bootstrap.publication_residue.is_empty()),
            frame_ports,
        }
    }

    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.core.runtime_identity()
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.media.store_identity()
    }

    pub const fn observed_staging_residue(&self) -> bool {
        self.publication_residue.staging_catalog_candidate()
    }

    pub const fn observed_non_authoritative_residue(&self) -> bool {
        !self.publication_residue.is_empty()
    }

    pub const fn publication_residue(&self) -> RecordPublicationResidueObservation {
        self.publication_residue
    }

    pub fn media_counters(&self) -> worth_store_physical_backend::MediaCounterSnapshot {
        self.media.counters()
    }

    pub fn residency_counters(&self) -> worth_store_buffer_pool::PhysicalResidencyCounters {
        self.frame_ports.counters()
    }

    pub fn drain_clean_residency(&self) -> u64 {
        self.frame_ports.drain_unpinned_clean_frames()
    }

    pub fn admit_physical_scheduler_capability(
        &self,
        requirement: worth_store_io_scheduler::IoSchedulerBackendCapabilityRequirement,
    ) -> Result<
        worth_store_io_scheduler::IoSchedulerBackendCapabilityAdmission,
        worth_store_io_scheduler::IoSchedulerBackendCapabilityDenial,
    > {
        let claim = self
            .media
            .scheduler_capability_claim(
                requirement.capability_kind(),
                requirement.required_evidence(),
            )
            .map_err(
                worth_store_io_scheduler::IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied,
            )?;
        worth_store_io_scheduler::admit_backend_capability_for_scheduler_qualified_claim(
            claim,
            requirement,
        )
    }

    pub fn execute_scheduled_writeback(
        &self,
        plan: worth_store_io_scheduler::QueueExecutionReadyPlan,
        adaptation: worth_store_physical_backend::BackendQueueExecutionAdaptation,
    ) -> Result<
        super::super::PhysicalScheduledWritebackOutcome,
        super::super::PhysicalScheduledWritebackAdmissionDenial,
    > {
        let outcome = super::super::residency::scheduled_writeback::execute_store_writeback(
            &self.frame_ports,
            &self.media,
            plan,
            adaptation,
        )?;
        if matches!(
            outcome,
            super::super::PhysicalScheduledWritebackOutcome::InspectionRequired(_)
                | super::super::PhysicalScheduledWritebackOutcome::ResidencyTerminal { .. }
        ) {
            self.health.revoke();
        }
        Ok(outcome)
    }

    pub fn records(&self) -> PhysicalRecordReader<'_> {
        PhysicalRecordReader {
            media: &self.media,
            format: self.format,
            access: self.access,
            current_root: &self.current_root,
            health: &self.health,
            lifecycle: self.owner.reader(),
            frame_load: self.frame_ports.loader(),
            frame_ports: &self.frame_ports,
        }
    }

    pub fn records_mut(&mut self) -> PhysicalRecordWriter<'_> {
        PhysicalRecordWriter {
            media: &self.media,
            format: self.format,
            access: self.access,
            current_root: &mut self.current_root,
            free_space: &mut self.free_space,
            allocation_frontier: &mut self.allocation_frontier,
            publication_residue: &mut self.publication_residue,
            health: &self.health,
            _lease: self.owner.writer(),
            frame_ports: &self.frame_ports,
        }
    }

    pub fn observer(&self) -> PhysicalRecordObserver {
        let (lifecycle, lease) = self.core.media_observation_parts();
        let media = PhysicalMediaObserver::for_record_serving(
            self.runtime_identity(),
            self.store_identity(),
            self.media.mutation_owner(),
            self.media.profile().clone(),
            self.media.counter_observer(),
            lifecycle,
            lease,
        );
        PhysicalRecordObserver::new(
            media,
            self.owner.observer(),
            self.format,
            self.current_root.generation(),
            self.publication_residue,
        )
    }

    pub fn close(self) -> super::super::ServingShutdownOutcome<ClosedRuntime> {
        let Self {
            termination,
            media,
            core,
            owner,
            format: _,
            access: _,
            current_root: _,
            free_space: _,
            allocation_frontier: _,
            publication_residue,
            health,
            frame_ports,
        } = self;
        let residency = frame_ports.close();
        drop(termination);
        let counters = owner.observer();
        drop(owner);
        let records = super::super::RecordServingTerminalObservation::new(
            health.requires_inspection()
                || !publication_residue.is_empty()
                || residency.requires_inspection(),
            publication_residue,
            counters.snapshot(),
        );
        let release = media.close();
        super::super::ServingShutdownOutcome::new(
            MediaShutdownOutcome::new(core.close(), release),
            records,
            residency,
        )
    }

    pub fn abort(self) -> super::super::ServingShutdownOutcome<AbortedRuntime> {
        let Self {
            termination,
            media,
            core,
            owner,
            format: _,
            access: _,
            current_root: _,
            free_space: _,
            allocation_frontier: _,
            publication_residue,
            health,
            frame_ports,
        } = self;
        let residency = frame_ports.close();
        drop(termination);
        let counters = owner.observer();
        drop(owner);
        let records = super::super::RecordServingTerminalObservation::new(
            health.requires_inspection()
                || !publication_residue.is_empty()
                || residency.requires_inspection(),
            publication_residue,
            counters.snapshot(),
        );
        let release = media.close();
        super::super::ServingShutdownOutcome::new(
            MediaShutdownOutcome::new(core.abort(), release),
            records,
            residency,
        )
    }
}

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
        super::super::planning::batch_placement::preflight_placement(
            self.format,
            placement,
            &batch,
        )?;
        let operation_bytes =
            super::super::planning::batch_placement::append_operation_allocation_bytes(
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
