use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, RecordArtifactFile, RecordFrameCoordinate,
};

use crate::physical_runtime::{
    instance::{PhysicalExecutionCall, PhysicalStoreInstanceParts, PhysicalStoreWorkRuntime},
    record_serving::CanonicalRecordReadPort,
    LifecycleGeneration, PhysicalWorkExecution, RuntimeIdentity,
};

use super::{
    super::{frame_loading::CanonicalFrameReadSource, ServingFrameResidency},
    CertificationFrameReadFailure, CertificationFrameWorkFailure, CertificationResidentFrame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CertificationResidencyBinding {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
}

impl CertificationResidencyBinding {
    pub(super) fn matches(
        self,
        store: StableStoreIdentity,
        runtime: RuntimeIdentity,
        generation: LifecycleGeneration,
    ) -> bool {
        self.store == store && self.runtime == runtime && self.generation == generation
    }
}

/// Certification-only observation and fault-driving access to physical
/// residency.
///
/// This capability exists to prove the ordinary Store composition boundary. It
/// cannot be constructed outside the serving runtime and is absent unless
/// certification authority is enabled.
#[derive(Clone)]
pub struct PhysicalResidencyCertification {
    binding: CertificationResidencyBinding,
    execution: PhysicalWorkExecution,
    residency: ServingFrameResidency,
}

impl PhysicalResidencyCertification {
    pub(in crate::physical_runtime) fn from_parts(parts: &PhysicalStoreInstanceParts) -> Self {
        let generation = parts.core.lifecycle_generation();
        Self {
            binding: CertificationResidencyBinding {
                store: parts
                    .work_runtime
                    .executor
                    .record_serving_media()
                    .store_identity(),
                runtime: parts.core.runtime_identity(),
                generation,
            },
            execution: PhysicalStoreWorkRuntime::execution(&parts.work_runtime, generation),
            residency: ServingFrameResidency::new(
                parts.residency.ports().clone(),
                CanonicalFrameReadSource::new(CanonicalRecordReadPort::new(
                    &parts.work_runtime,
                    generation,
                    parts.work_admission,
                    parts.scheduler_admission.clone(),
                    std::sync::Arc::clone(&parts.record_work),
                )),
            ),
        }
    }

    pub fn pin_exact(
        &self,
        coordinate: RecordFrameCoordinate,
    ) -> Result<CertificationResidentFrame, CertificationFrameReadFailure> {
        let _call = self.admit_frame_call()?;
        let allocation = self
            .residency
            .begin_operation(
                worth_store_buffer_pool::PhysicalOperationAllocationScope::ForegroundRead,
                std::num::NonZeroU64::new(u64::from(coordinate.length()))
                    .expect("a physical frame coordinate has nonzero length"),
            )
            .map_err(CertificationFrameReadFailure::Residency)?;
        let frame = self
            .residency
            .load_exact(&allocation, coordinate)
            .map_err(CertificationFrameReadFailure::from)?;
        Ok(CertificationResidentFrame::bind(
            self.binding,
            coordinate,
            frame,
        ))
    }

    pub fn pin_bounded(
        &self,
        artifact: RecordArtifactFile,
        limit: u32,
    ) -> Result<CertificationResidentFrame, CertificationFrameReadFailure> {
        let _call = self.admit_frame_call()?;
        let allocation = self
            .residency
            .begin_operation(
                worth_store_buffer_pool::PhysicalOperationAllocationScope::ForegroundRead,
                std::num::NonZeroU64::new(u64::from(limit))
                    .ok_or(CertificationFrameReadFailure::AccessLimitExceeded)?,
            )
            .map_err(CertificationFrameReadFailure::Residency)?;
        let frame = self
            .residency
            .load_bounded(&allocation, artifact, limit)
            .map_err(CertificationFrameReadFailure::from)?;
        let coordinate = RecordFrameCoordinate::new(artifact, 0, frame.len() as u32)
            .ok_or(CertificationFrameReadFailure::InvalidCoordinate)?;
        Ok(CertificationResidentFrame::bind(
            self.binding,
            coordinate,
            frame,
        ))
    }

    pub fn counters(&self) -> worth_store_buffer_pool::PhysicalResidencyCounters {
        self.residency.counters()
    }

    fn admit_frame_call(&self) -> Result<PhysicalExecutionCall, CertificationFrameReadFailure> {
        self.execution.admit_call().map_err(|failure| {
            CertificationFrameReadFailure::PhysicalWork(CertificationFrameWorkFailure::PreEffect(
                failure,
            ))
        })
    }
}
