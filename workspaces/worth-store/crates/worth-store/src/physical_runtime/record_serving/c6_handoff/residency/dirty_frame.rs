use worth_store_physical_format::RecordFrameCoordinate;

use crate::physical_runtime::{PhysicalWorkIdentity, ReadyPhysicalWork};

use super::{C6PhysicalFrameLease, C6PhysicalResidencyWork};
use crate::physical_runtime::record_serving::c6_handoff::{
    C6PhysicalWorkHandoffFailure, C6PhysicalWorkHandoffIdentity,
};

#[derive(Debug)]
#[must_use = "dirty admission must advance through canonical writeback or be explicitly discarded"]
pub struct C6AdmittedDirtyFrame {
    pub(super) handoff: C6PhysicalWorkHandoffIdentity,
    pub(super) identity: PhysicalWorkIdentity,
    pub(super) coordinate: RecordFrameCoordinate,
    pub(super) frame: worth_store_buffer_pool::DirtyPhysicalFrame,
    source_work_count: u64,
    first_source_work: Option<PhysicalWorkIdentity>,
    last_source_work: Option<PhysicalWorkIdentity>,
}

#[derive(Clone, Copy)]
pub(super) struct C6DirtyFrameBinding {
    handoff: C6PhysicalWorkHandoffIdentity,
    identity: PhysicalWorkIdentity,
    coordinate: RecordFrameCoordinate,
    source_work_count: u64,
    first_source_work: Option<PhysicalWorkIdentity>,
    last_source_work: Option<PhysicalWorkIdentity>,
}

impl C6PhysicalResidencyWork {
    pub fn admit_dirty_frame(
        &self,
        ready: &ReadyPhysicalWork,
        lease: C6PhysicalFrameLease,
        bytes: Vec<u8>,
    ) -> Result<C6AdmittedDirtyFrame, C6PhysicalWorkHandoffFailure> {
        let coordinate = *self.require_writeback_intent(ready.intent())?;
        self.require_current(ready.intent())?;
        if lease.handoff_identity() != self.identity || lease.coordinate() != coordinate {
            return Err(C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity);
        }
        let source_work_count = lease.physical_work_count();
        let first_source_work = lease.first_physical_work();
        let last_source_work = lease.last_physical_work();
        let (frame, _) = lease
            .into_dirty_candidate(bytes)
            .map_err(C6PhysicalWorkHandoffFailure::Residency)?;
        Ok(C6AdmittedDirtyFrame {
            handoff: self.identity,
            identity: ready.intent().identity(),
            coordinate,
            frame,
            source_work_count,
            first_source_work,
            last_source_work,
        })
    }

    pub fn discard_dirty_frame(
        &self,
        dirty: C6AdmittedDirtyFrame,
    ) -> Result<(), C6PhysicalWorkHandoffFailure> {
        if dirty.handoff != self.identity {
            return Err(C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity);
        }
        dirty
            .frame
            .discard_candidate()
            .map_err(C6PhysicalWorkHandoffFailure::Residency)
    }
}

impl C6AdmittedDirtyFrame {
    pub const fn identity(&self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn coordinate(&self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub const fn source_physical_work_count(&self) -> u64 {
        self.source_work_count
    }

    pub const fn first_source_physical_work(&self) -> Option<PhysicalWorkIdentity> {
        self.first_source_work
    }

    pub const fn last_source_physical_work(&self) -> Option<PhysicalWorkIdentity> {
        self.last_source_work
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        C6DirtyFrameBinding,
        worth_store_buffer_pool::DirtyPhysicalFrame,
    ) {
        (
            C6DirtyFrameBinding {
                handoff: self.handoff,
                identity: self.identity,
                coordinate: self.coordinate,
                source_work_count: self.source_work_count,
                first_source_work: self.first_source_work,
                last_source_work: self.last_source_work,
            },
            self.frame,
        )
    }

    pub(super) fn from_parts(
        binding: C6DirtyFrameBinding,
        frame: worth_store_buffer_pool::DirtyPhysicalFrame,
    ) -> Self {
        Self {
            handoff: binding.handoff,
            identity: binding.identity,
            coordinate: binding.coordinate,
            frame,
            source_work_count: binding.source_work_count,
            first_source_work: binding.first_source_work,
            last_source_work: binding.last_source_work,
        }
    }
}
