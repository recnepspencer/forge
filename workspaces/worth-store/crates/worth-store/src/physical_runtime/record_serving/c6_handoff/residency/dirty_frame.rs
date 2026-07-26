use worth_store_physical_format::RecordFrameCoordinate;

use crate::physical_runtime::{PhysicalWorkIdentity, ReadyPhysicalWork};

use super::C6PhysicalResidencyWork;
use crate::physical_runtime::record_serving::c6_handoff::{
    C6PhysicalWorkHandoffFailure, C6PhysicalWorkHandoffIdentity,
};
use crate::physical_runtime::record_serving::residency::CertificationResidentFrame;

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
    pub fn admit_dirty_frame<F>(
        &self,
        ready: &ReadyPhysicalWork,
        lease: CertificationResidentFrame,
        fill: F,
    ) -> Result<C6AdmittedDirtyFrame, C6PhysicalWorkHandoffFailure>
    where
        F: FnOnce(&[u8], &mut [u8]),
    {
        let coordinate = *self.require_writeback_intent(ready.intent())?;
        self.require_current(ready.intent())?;
        if !lease.belongs_to(
            self.identity.store(),
            self.identity.runtime(),
            self.identity.generation(),
        ) || lease.coordinate() != coordinate
        {
            return Err(C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity);
        }
        let source_work_count = lease.physical_work_count();
        let first_source_work = lease.first_physical_work();
        let last_source_work = lease.last_physical_work();
        let allocation = self
            .frame_ports
            .begin_operation(
                worth_store_buffer_pool::PhysicalOperationAllocationScope::ForegroundWrite,
                std::num::NonZeroU64::new(u64::from(coordinate.length()))
                    .expect("a physical frame coordinate has nonzero length"),
            )
            .map_err(C6PhysicalWorkHandoffFailure::Residency)?;
        let (frame, _) = lease
            .into_dirty_candidate(&allocation, fill)
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
