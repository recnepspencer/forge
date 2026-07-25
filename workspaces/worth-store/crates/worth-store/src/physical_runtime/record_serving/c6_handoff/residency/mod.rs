use std::sync::{Arc, Weak};

use crate::physical_runtime::{
    instance::{
        PhysicalSchedulerAdmissionOwner, PhysicalStoreInstanceParts, PhysicalStoreWorkRuntime,
    },
    PhysicalWorkAdmission, PhysicalWorkExecution,
};

use super::{C6PhysicalWorkHandoffFailure, C6PhysicalWorkHandoffIdentity};
use crate::physical_runtime::record_serving::{
    residency::{frame_loading::CanonicalFrameReadSource, frame_ports::RecordFramePorts},
    CanonicalRecordReadPort, RecordWorkAdmission,
};

mod dirty_frame;
mod frame_load;
#[cfg(test)]
mod tests;
mod writeback;

pub use dirty_frame::C6AdmittedDirtyFrame;
pub use frame_load::{
    C6PhysicalFrameLease, C6PhysicalFrameReadFailure, C6PhysicalFrameWorkFailure,
};
pub use writeback::{
    C6AdmittedPhysicalWriteback, C6PhysicalWorkSettlement, C6PhysicalWritebackExecution,
    C6PhysicalWritebackReservation, C6PhysicalWritebackTransitionFailure,
    C6PreparedPhysicalWriteback, C6RetryablePhysicalWriteback,
};

/// The sealed C.6 capability for residency work inherited from one physical
/// Store instance.
///
/// It carries the existing pool, canonical read source, scheduler admission,
/// executor, and lifecycle generation. It does not construct or own any of
/// those authorities.
#[derive(Clone)]
pub struct C6PhysicalResidencyWork {
    pub(super) identity: C6PhysicalWorkHandoffIdentity,
    pub(super) runtime: Weak<PhysicalStoreWorkRuntime>,
    pub(super) execution: PhysicalWorkExecution,
    pub(super) scheduler: PhysicalSchedulerAdmissionOwner,
    pub(super) record: Arc<RecordWorkAdmission>,
    pub(super) frame_ports: RecordFramePorts,
    pub(super) frame_source: CanonicalFrameReadSource,
}

impl C6PhysicalResidencyWork {
    pub(in crate::physical_runtime) fn from_parts(
        parts: &PhysicalStoreInstanceParts,
        identity: C6PhysicalWorkHandoffIdentity,
    ) -> Self {
        let generation = parts.core.lifecycle_generation();
        let frame_source = CanonicalFrameReadSource::new(CanonicalRecordReadPort::new(
            &parts.work_runtime,
            generation,
            parts.work_admission,
            parts.scheduler_admission.clone(),
            Arc::clone(&parts.record_work),
        ));
        Self {
            identity,
            runtime: Arc::downgrade(&parts.work_runtime),
            execution: PhysicalStoreWorkRuntime::execution(&parts.work_runtime, generation),
            scheduler: parts.scheduler_admission.clone(),
            record: Arc::clone(&parts.record_work),
            frame_ports: parts.frame_ports.clone(),
            frame_source,
        }
    }

    pub fn counters(&self) -> worth_store_buffer_pool::PhysicalResidencyCounters {
        self.frame_ports.counters()
    }

    pub(super) fn require_current(
        &self,
        intent: &crate::physical_runtime::PhysicalWorkIntent,
    ) -> Result<(), C6PhysicalWorkHandoffFailure> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or(C6PhysicalWorkHandoffFailure::RuntimeReleased)?;
        PhysicalWorkAdmission::require_current(&runtime.submission, intent, &runtime.health)
            .map_err(C6PhysicalWorkHandoffFailure::PreEffect)
    }

    pub(super) fn require_writeback_intent<'a>(
        &self,
        intent: &'a crate::physical_runtime::PhysicalWorkIntent,
    ) -> Result<&'a worth_store_physical_format::RecordFrameCoordinate, C6PhysicalWorkHandoffFailure>
    {
        if !self.identity.admits(intent.identity())
            || intent.operation()
                != crate::physical_runtime::PhysicalWorkOperationFamily::ArtifactRangeWrite
        {
            return Err(C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity);
        }
        let [coordinate] = intent.scope().coordinates() else {
            return Err(C6PhysicalWorkHandoffFailure::CanonicalWritebackMismatch);
        };
        Ok(coordinate)
    }
}
