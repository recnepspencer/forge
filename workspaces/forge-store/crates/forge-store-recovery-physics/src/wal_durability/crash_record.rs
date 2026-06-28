use std::marker::PhantomData;

use forge_store_physical_backend::{BackendDurabilityProfile, BackendDurabilityProfileId};

use super::{AcknowledgmentPrecondition, WalDurabilityCrashBasis};

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
    pub fn from_unacknowledged_durable_precondition(
        precondition: AcknowledgmentPrecondition<P>,
    ) -> Self {
        Self {
            profile: PhantomData,
            basis: WalDurabilityCrashBasis::from_append_receipt(precondition.into_receipt()),
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

    pub(crate) fn into_basis(self) -> WalDurabilityCrashBasis {
        self.basis
    }
}
