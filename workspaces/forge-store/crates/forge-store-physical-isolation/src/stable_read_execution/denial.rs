use super::StablePhysicalReadExecutionCounters;
use crate::{PhysicalByteGuardDenial, PhysicalByteGuardScope, PhysicalReadPlanAdmissionDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalReadExecutionDenial {
    ReadPlanDenied(PhysicalReadPlanAdmissionDenial),
    ByteGuardDenied(PhysicalByteGuardDenial),
    GuardScopeNotInPlan {
        scope: PhysicalByteGuardScope,
        counters: StablePhysicalReadExecutionCounters,
    },
    ByteGuardScopeMismatch {
        admitted: PhysicalByteGuardScope,
        observed: PhysicalByteGuardScope,
    },
    HiddenStructuralLatchIoWithoutDeclaredCost {
        counters: StablePhysicalReadExecutionCounters,
    },
}

impl From<PhysicalReadPlanAdmissionDenial> for PhysicalReadExecutionDenial {
    fn from(denial: PhysicalReadPlanAdmissionDenial) -> Self {
        Self::ReadPlanDenied(denial)
    }
}

impl From<PhysicalByteGuardDenial> for PhysicalReadExecutionDenial {
    fn from(denial: PhysicalByteGuardDenial) -> Self {
        Self::ByteGuardDenied(denial)
    }
}
