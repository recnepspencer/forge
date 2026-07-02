use super::{PhysicalReadIoPosture, StablePhysicalReadExecutionCounters};
use crate::{PhysicalReadPlanReleaseReceipt, StablePhysicalReadFoundationalEvidence};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StablePhysicalReadReceipt {
    read_plan_release: PhysicalReadPlanReleaseReceipt,
    counters: StablePhysicalReadExecutionCounters,
    io_posture: PhysicalReadIoPosture,
}

impl StablePhysicalReadReceipt {
    pub(crate) const fn new(
        read_plan_release: PhysicalReadPlanReleaseReceipt,
        counters: StablePhysicalReadExecutionCounters,
        io_posture: PhysicalReadIoPosture,
    ) -> Self {
        Self {
            read_plan_release,
            counters,
            io_posture,
        }
    }

    pub const fn read_plan_release(self) -> PhysicalReadPlanReleaseReceipt {
        self.read_plan_release
    }

    pub const fn counters(self) -> StablePhysicalReadExecutionCounters {
        self.counters
    }

    pub const fn io_posture(self) -> PhysicalReadIoPosture {
        self.io_posture
    }

    pub fn lower_to_foundational_evidence(&self) -> StablePhysicalReadFoundationalEvidence {
        StablePhysicalReadFoundationalEvidence::lower(self)
    }
}
