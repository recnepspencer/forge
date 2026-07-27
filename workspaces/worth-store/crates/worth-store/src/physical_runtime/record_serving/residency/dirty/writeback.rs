use std::sync::{Arc, Weak};

use worth_store_buffer_pool::PhysicalWritebackClaim;
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

use crate::physical_runtime::work::PhysicalWorkAdmissionAuthority;
use crate::physical_runtime::{
    instance::{PhysicalSchedulerAdmissionOwner, PhysicalStoreWorkRuntime},
    PhysicalMutationSubmission, PhysicalWorkExecution, ReadyPhysicalWork,
    ResourceAdmittedPhysicalWork,
};

use super::AdmittedDirtyFrame;
use crate::physical_runtime::record_serving::{
    residency::frame_ports::RecordFramePorts, RecordWorkAdmission,
};

mod admission;
mod execution;
mod progression;

#[derive(Clone)]
pub(in crate::physical_runtime::record_serving) struct FrameWritebackPort {
    pub(super) runtime: Weak<PhysicalStoreWorkRuntime>,
    pub(super) execution: PhysicalWorkExecution,
    pub(super) submission: PhysicalMutationSubmission,
    pub(super) physical: PhysicalWorkAdmissionAuthority,
    pub(super) scheduler: PhysicalSchedulerAdmissionOwner,
    pub(super) record: Arc<RecordWorkAdmission>,
    pub(super) frame_ports: RecordFramePorts,
}

#[must_use = "prepared writeback must request readiness or retain its dirty authority"]
pub struct PreparedPhysicalWriteback {
    pub(super) receipt: crate::physical_runtime::PhysicalWorkSubmissionReceipt,
    pub(super) claim: PhysicalWritebackClaim,
    pub(super) dirty: AdmittedDirtyFrame,
    pub(super) durability: ArtifactRangeWriteDurabilityRequirement,
}

#[must_use = "ready writeback must enter scheduler admission or retain dirty authority"]
pub struct ReadyPhysicalWriteback {
    pub(super) ready: ReadyPhysicalWork,
    pub(super) claim: PhysicalWritebackClaim,
    pub(super) dirty: AdmittedDirtyFrame,
    pub(super) durability: ArtifactRangeWriteDurabilityRequirement,
}

#[must_use = "admitted writeback must execute or deliberately retain dirty authority"]
pub struct AdmittedPhysicalWriteback {
    pub(super) work: ResourceAdmittedPhysicalWork,
    pub(super) claim: PhysicalWritebackClaim,
    pub(super) dirty: AdmittedDirtyFrame,
}

impl FrameWritebackPort {
    pub(in crate::physical_runtime::record_serving) fn new(
        runtime: Weak<PhysicalStoreWorkRuntime>,
        execution: PhysicalWorkExecution,
        submission: PhysicalMutationSubmission,
        physical: PhysicalWorkAdmissionAuthority,
        scheduler: PhysicalSchedulerAdmissionOwner,
        record: Arc<RecordWorkAdmission>,
        frame_ports: RecordFramePorts,
    ) -> Self {
        Self {
            runtime,
            execution,
            submission,
            physical,
            scheduler,
            record,
            frame_ports,
        }
    }
}

impl PreparedPhysicalWriteback {
    pub const fn identity(&self) -> crate::physical_runtime::PhysicalWorkIdentity {
        self.receipt.identity()
    }
}

impl ReadyPhysicalWriteback {
    #[cfg(feature = "certification-test-authority")]
    pub const fn identity(&self) -> crate::physical_runtime::PhysicalWorkIdentity {
        self.ready.intent().identity()
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn consumer_handle(&self) -> crate::physical_runtime::PhysicalWorkConsumerHandle {
        self.ready.consumer_handle()
    }
}

impl AdmittedPhysicalWriteback {
    #[cfg(feature = "certification-test-authority")]
    pub const fn identity(&self) -> crate::physical_runtime::PhysicalWorkIdentity {
        self.work.intent().identity()
    }
}
