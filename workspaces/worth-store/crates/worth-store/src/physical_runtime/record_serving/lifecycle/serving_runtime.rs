use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{
    instance::{PhysicalStoreInstanceFoundation, PhysicalStoreInstanceParts},
    media_ownership::PhysicalMediaObserver,
    AbortedRuntime, ClosedRuntime, RuntimeIdentity,
};

use super::super::lifecycle::record_observation::PhysicalRecordObserver;
use super::super::{PhysicalRecordReader, RecordPublicationResidueObservation};

#[cfg(feature = "certification-test-authority")]
#[path = "serving_runtime/certification.rs"]
mod certification;
mod physical_work;

pub struct ServingPhysicalRuntime {
    parts: PhysicalStoreInstanceParts,
}

impl ServingPhysicalRuntime {
    pub(in crate::physical_runtime::record_serving) fn from_admission(
        foundation: PhysicalStoreInstanceFoundation,
    ) -> Result<Self, super::super::RecordServingAdmissionInspectionRequired> {
        match PhysicalStoreInstanceParts::from_record_admission(foundation) {
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

    pub fn store_identity(&self) -> StableStoreIdentity {
        self.parts
            .work_runtime
            .executor
            .record_serving_media()
            .store_identity()
    }

    pub fn observed_staging_residue(&self) -> bool {
        self.parts.publication.residue().staging_catalog_candidate()
    }

    pub fn observed_non_authoritative_residue(&self) -> bool {
        !self.parts.publication.residue().is_empty()
    }

    pub fn publication_residue(&self) -> RecordPublicationResidueObservation {
        self.parts.publication.residue()
    }

    pub fn media_counters(&self) -> worth_store_physical_backend::MediaCounterSnapshot {
        self.parts
            .work_runtime
            .executor
            .record_serving_media()
            .counters()
    }

    pub fn residency_counters(&self) -> worth_store_buffer_pool::PhysicalResidencyCounters {
        self.parts.frame_ports.counters()
    }

    pub fn drain_clean_residency(&self) -> u64 {
        self.parts.frame_ports.drain_unpinned_clean_frames()
    }

    pub fn physical_residency_writeback_command(
        &self,
        work: crate::physical_runtime::ResourceAdmittedPhysicalWork,
    ) -> Result<
        crate::physical_runtime::PhysicalExecutorCommand,
        super::super::PhysicalScheduledWritebackAdmissionDenial,
    > {
        self.parts.work_runtime.health.permit().map_err(|_| {
            super::super::PhysicalScheduledWritebackAdmissionDenial::ServingRequiresInspection
        })?;
        if work.intent().operation()
            != crate::physical_runtime::PhysicalWorkOperationFamily::ArtifactRangeWrite
        {
            return Err(
                super::super::PhysicalScheduledWritebackAdmissionDenial::CanonicalWorkMismatch,
            );
        }
        let [coordinate] = work.intent().scope().coordinates() else {
            return Err(
                super::super::PhysicalScheduledWritebackAdmissionDenial::CanonicalWorkMismatch,
            );
        };
        let claim = self
            .parts
            .frame_ports
            .claim_writeback(*coordinate)
            .map_err(super::super::PhysicalScheduledWritebackAdmissionDenial::Residency)?;
        super::super::residency::scheduled_writeback::PhysicalScheduledWriteback::validate(
            &claim,
            work.queue_plan(),
        )?;
        Ok(crate::physical_runtime::PhysicalExecutorCommand::residency_writeback(work, claim))
    }

    pub fn bind_physical_residency_writeback_retry(
        &self,
        retry: crate::physical_runtime::PhysicalRetryCommand,
        work: crate::physical_runtime::ResourceAdmittedPhysicalWork,
    ) -> Result<
        crate::physical_runtime::PhysicalExecutorCommand,
        super::super::PhysicalScheduledWritebackAdmissionDenial,
    > {
        retry
            .admit_residency_retry(&work)
            .map_err(super::super::PhysicalScheduledWritebackAdmissionDenial::Retry)?;
        self.physical_residency_writeback_command(work)
    }

    pub fn records(&self) -> PhysicalRecordReader {
        let port = super::super::CanonicalRecordReadPort::new(
            &self.parts.work_runtime,
            self.parts.core.lifecycle_generation(),
            self.parts.work_admission,
            self.parts.scheduler_admission.clone(),
            self.parts.record_work.clone(),
        );
        PhysicalRecordReader {
            store: self.store_identity(),
            format: self.parts.format,
            access: self.parts.access,
            current_root: self.parts.publication.current_root(),
            runtime: std::sync::Arc::downgrade(&self.parts.work_runtime),
            lifecycle: self.parts.record_owner.reader(),
            frame_ports: self.parts.frame_ports.clone(),
            source: super::super::residency::frame_loading::CanonicalFrameReadSource::new(port),
        }
    }

    pub fn c6_physical_work_handoff(&self) -> super::super::C6PhysicalWorkHandoff {
        super::super::C6PhysicalWorkHandoff::from_parts(&self.parts, self.records())
    }

    pub fn record_submission(&self) -> super::super::PhysicalRecordSubmission {
        super::super::RecordPublicationDirector::submission(&self.parts.publication)
    }

    pub fn observer(&self) -> PhysicalRecordObserver {
        let (lifecycle, lease) = self.parts.core.media_observation_parts();
        let media = PhysicalMediaObserver::for_record_serving(
            self.runtime_identity(),
            self.store_identity(),
            self.parts
                .work_runtime
                .executor
                .record_serving_media()
                .mutation_owner(),
            self.parts
                .work_runtime
                .executor
                .record_serving_media()
                .profile()
                .clone(),
            self.parts
                .work_runtime
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
            self.parts.publication.current_root().generation(),
            self.parts.publication.residue(),
        )
    }

    pub fn close(self) -> super::super::ServingShutdownOutcome<ClosedRuntime> {
        self.close_plan().execute().into_shutdown()
    }

    pub fn close_plan(self) -> crate::physical_runtime::PhysicalStoreClosePlan {
        crate::physical_runtime::PhysicalStoreClosePlan::new(self.parts)
    }

    pub fn abort(self) -> super::super::ServingShutdownOutcome<AbortedRuntime> {
        self.abort_with_evidence().into_shutdown()
    }

    pub fn abort_with_evidence(self) -> crate::physical_runtime::PhysicalStoreAbortOutcome {
        crate::physical_runtime::PhysicalStoreAbortOutcome::execute(self.parts)
    }
}
