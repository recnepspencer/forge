use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, DurablePhysicalRootManifest,
};

use crate::physical_runtime::{
    instance::PhysicalStoreInstanceParts, media_ownership::PhysicalMediaObserver,
    runtime::PhysicalRuntimeCore, AbortedRuntime, ClosedRuntime, RuntimeIdentity,
};

use super::super::lifecycle::record_observation::PhysicalRecordObserver;
use super::super::{
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, PhysicalRecordReader,
    RecordAllocationFrontier, RecordPublicationResidueObservation, RecordServingState,
};
use super::serving_health::ServingHealth;

#[cfg(feature = "certification-test-authority")]
#[path = "serving_runtime/certification.rs"]
mod certification;
mod physical_work;
mod record_writer;

pub struct ServingPhysicalRuntime {
    parts: PhysicalStoreInstanceParts,
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
        work_profile: crate::physical_runtime::PhysicalWorkProfileDeclaration,
    ) -> Result<Self, super::super::RecordServingAdmissionInspectionRequired> {
        match PhysicalStoreInstanceParts::from_record_admission(
            termination,
            media,
            core,
            bootstrap,
            allocation_frontier,
            frame_ports,
            work_profile,
        ) {
            Ok(parts) => Ok(Self { parts }),
            Err(failure) => {
                let (identity, terminal, cause) = failure.abort();
                Err(super::super::RecordServingAdmissionInspectionRequired::new(
                    identity,
                    terminal,
                    super::super::RecordBootstrapFailure::SignalConstruction(cause),
                ))
            }
        }
    }

    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.parts.core.runtime_identity()
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.parts.executor.record_serving_media().store_identity()
    }

    pub const fn observed_staging_residue(&self) -> bool {
        self.parts.publication_residue.staging_catalog_candidate()
    }

    pub const fn observed_non_authoritative_residue(&self) -> bool {
        !self.parts.publication_residue.is_empty()
    }

    pub const fn publication_residue(&self) -> RecordPublicationResidueObservation {
        self.parts.publication_residue
    }

    pub fn media_counters(&self) -> worth_store_physical_backend::MediaCounterSnapshot {
        self.parts.executor.record_serving_media().counters()
    }

    pub fn residency_counters(&self) -> worth_store_buffer_pool::PhysicalResidencyCounters {
        self.parts.frame_ports.counters()
    }

    pub fn drain_clean_residency(&self) -> u64 {
        self.parts.frame_ports.drain_unpinned_clean_frames()
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
            &self.parts.frame_ports,
            self.parts.executor.record_serving_media(),
            plan,
            adaptation,
        )?;
        if matches!(
            outcome,
            super::super::PhysicalScheduledWritebackOutcome::InspectionRequired(_)
                | super::super::PhysicalScheduledWritebackOutcome::ResidencyTerminal { .. }
        ) {
            self.parts.health.revoke();
        }
        Ok(outcome)
    }

    pub fn records(&self) -> PhysicalRecordReader<'_> {
        PhysicalRecordReader {
            media: self.parts.executor.record_serving_media(),
            format: self.parts.format,
            access: self.parts.access,
            current_root: &self.parts.current_root,
            health: &self.parts.health,
            lifecycle: self.parts.record_owner.reader(),
            frame_load: self.parts.frame_ports.loader(),
            frame_ports: &self.parts.frame_ports,
        }
    }

    pub fn records_mut(&mut self) -> PhysicalRecordWriter<'_> {
        PhysicalRecordWriter {
            media: self.parts.executor.record_serving_media(),
            format: self.parts.format,
            access: self.parts.access,
            current_root: &mut self.parts.current_root,
            free_space: &mut self.parts.free_space,
            allocation_frontier: &mut self.parts.allocation_frontier,
            publication_residue: &mut self.parts.publication_residue,
            health: &self.parts.health,
            _lease: self.parts.record_owner.writer(),
            frame_ports: &self.parts.frame_ports,
        }
    }

    pub fn observer(&self) -> PhysicalRecordObserver {
        let (lifecycle, lease) = self.parts.core.media_observation_parts();
        let media = PhysicalMediaObserver::for_record_serving(
            self.runtime_identity(),
            self.store_identity(),
            self.parts.executor.record_serving_media().mutation_owner(),
            self.parts.executor.record_serving_media().profile().clone(),
            self.parts
                .executor
                .record_serving_media()
                .counter_observer(),
            lifecycle,
            lease,
        );
        PhysicalRecordObserver::new(
            media,
            self.parts.record_owner.observer(),
            self.parts.format,
            self.parts.current_root.generation(),
            self.parts.publication_residue,
        )
    }

    pub fn close(self) -> super::super::ServingShutdownOutcome<ClosedRuntime> {
        self.parts.close()
    }

    pub fn abort(self) -> super::super::ServingShutdownOutcome<AbortedRuntime> {
        self.parts.abort()
    }
}
