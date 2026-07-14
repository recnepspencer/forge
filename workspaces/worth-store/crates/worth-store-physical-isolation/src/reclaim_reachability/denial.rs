use crate::{
    HazardLeaseGeneration, HazardLeaseKind, HazardLeaseSlot, PhysicalReadProtectedFootprintBasis,
    RootEpoch,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReclaimDenial {
    MissingCandidateReachability,
    CandidateDoesNotMatchExecutedReachability {
        executed: PhysicalReadProtectedFootprintBasis,
        candidate: PhysicalReadProtectedFootprintBasis,
    },
    CandidateRootDoesNotMatchExecutedReachability {
        executed: RootEpoch,
        candidate: RootEpoch,
    },
    BackendResidueIsNotReachabilityAuthority,
    CurrentRootAbsenceIsNotReachabilityAuthority,
    RawReaderHandleScanIsNotReachabilityAuthority,
    CopiedReadPlanFieldsAreNotReachabilityAuthority,
    LeaseExpiryIsNotReclaimAuthority {
        slot: HazardLeaseSlot,
    },
    BlockedByLiveHazardLease {
        slot: HazardLeaseSlot,
        generation: HazardLeaseGeneration,
        kind: HazardLeaseKind,
        overlapping_ranges: u64,
    },
}

pub const fn reject_backend_residue_as_reclaim_authority() -> ReclaimDenial {
    ReclaimDenial::BackendResidueIsNotReachabilityAuthority
}

pub const fn reject_current_root_absence_as_reclaim_authority() -> ReclaimDenial {
    ReclaimDenial::CurrentRootAbsenceIsNotReachabilityAuthority
}

pub const fn reject_raw_reader_handle_scan_as_reclaim_authority() -> ReclaimDenial {
    ReclaimDenial::RawReaderHandleScanIsNotReachabilityAuthority
}

pub const fn reject_copied_read_plan_fields_as_reclaim_authority() -> ReclaimDenial {
    ReclaimDenial::CopiedReadPlanFieldsAreNotReachabilityAuthority
}

pub const fn reject_lease_expiry_as_reclaim_authority(slot: HazardLeaseSlot) -> ReclaimDenial {
    ReclaimDenial::LeaseExpiryIsNotReclaimAuthority { slot }
}
