use super::{PhysicalLatchKey, PhysicalLatchMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatchAcquisitionDenial {
    EmptyPlan,
    DuplicateLatchKey(PhysicalLatchKey),
    ConflictingLatchMode {
        key: PhysicalLatchKey,
        first: PhysicalLatchMode,
        second: PhysicalLatchMode,
    },
    HierarchyInversion,
    UnorderedLockSet,
    ExecutionTimeLatchDiscovery(PhysicalLatchKey),
    UnauthorizedUpgrade(PhysicalLatchKey),
    CyclicPlan,
    WaitForGraphBoundExceeded,
}

pub type DeadlockPreventionDenial = LatchAcquisitionDenial;
