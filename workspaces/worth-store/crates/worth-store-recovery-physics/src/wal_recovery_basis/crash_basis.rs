use std::marker::PhantomData;

use worth_store_physical_backend::{
    BackendDurabilityProfile, BackendDurabilityProfileId, WalDurabilityBarrierSet,
};

use crate::{WalLsnRange, WalSegmentGeneration, WalSegmentId};

use super::{WalDurabilityObservation, WalDurabilityObservationBasis, WalFrameDigest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalDurabilityCrashBasis {
    basis: WalDurabilityObservationBasis,
}

impl WalDurabilityCrashBasis {
    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.basis.profile_id()
    }

    pub const fn segment_id(&self) -> WalSegmentId {
        self.basis.segment_id()
    }

    pub const fn generation(&self) -> WalSegmentGeneration {
        self.basis.generation()
    }

    pub const fn lsn_range(&self) -> WalLsnRange {
        self.basis.lsn_range()
    }

    pub fn frame_digest(&self) -> &WalFrameDigest {
        self.basis.frame_digest()
    }

    pub const fn required_barriers(&self) -> WalDurabilityBarrierSet {
        self.basis.required_barriers()
    }

    pub const fn completed_barriers(&self) -> WalDurabilityBarrierSet {
        self.basis.completed_barriers()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalDurabilityCrashRecord<P: BackendDurabilityProfile> {
    profile: PhantomData<P>,
    basis: WalDurabilityCrashBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReopenedWalDurabilityCrashRecord<P: BackendDurabilityProfile> {
    profile: PhantomData<P>,
    basis: WalDurabilityCrashBasis,
}

impl<P: BackendDurabilityProfile> WalDurabilityCrashRecord<P> {
    pub fn from_unobserved_durability(observation: WalDurabilityObservation<P>) -> Self {
        Self {
            profile: PhantomData,
            basis: WalDurabilityCrashBasis {
                basis: observation.into_basis(),
            },
        }
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.basis.profile_id()
    }

    pub const fn crash_basis(&self) -> &WalDurabilityCrashBasis {
        &self.basis
    }

    pub fn reopen_for_recovery(self) -> ReopenedWalDurabilityCrashRecord<P> {
        ReopenedWalDurabilityCrashRecord {
            profile: PhantomData,
            basis: self.basis,
        }
    }
}

impl<P: BackendDurabilityProfile> ReopenedWalDurabilityCrashRecord<P> {
    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.basis.profile_id()
    }

    pub const fn crash_basis(&self) -> &WalDurabilityCrashBasis {
        &self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalDurabilityCrashPosture<P: BackendDurabilityProfile> {
    DurableWalAvailableForRecovery {
        profile: PhantomData<P>,
        basis: WalDurabilityCrashBasis,
    },
}

impl<P: BackendDurabilityProfile> WalDurabilityCrashPosture<P> {
    pub fn from_reopened_durability_record(record: ReopenedWalDurabilityCrashRecord<P>) -> Self {
        Self::DurableWalAvailableForRecovery {
            profile: PhantomData,
            basis: record.basis,
        }
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.crash_basis().profile_id()
    }

    pub const fn crash_basis(&self) -> &WalDurabilityCrashBasis {
        match self {
            Self::DurableWalAvailableForRecovery { basis, .. } => basis,
        }
    }

    pub const fn is_replayable_after_crash(&self) -> bool {
        matches!(self, Self::DurableWalAvailableForRecovery { .. })
    }
}
