use worth_store_physical_format::RecordFrameCoordinate;

use crate::physical_runtime::PhysicalWorkIdentity;

use super::super::PhysicalFrameReadFailure;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSpeculativeReadFailure {
    Frame(PhysicalFrameReadFailure),
    OutcomeAllocationUnavailable,
    HitCreatedPhysicalWork {
        coordinate: RecordFrameCoordinate,
        observed_count: u64,
    },
    CoalescedConsumerCreatedPhysicalWork {
        coordinate: RecordFrameCoordinate,
        observed_count: u64,
    },
    CanonicalMissWorkIdentityMismatch {
        coordinate: RecordFrameCoordinate,
        observed_count: u64,
        first: Option<PhysicalWorkIdentity>,
        last: Option<PhysicalWorkIdentity>,
    },
}
