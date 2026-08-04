use std::marker::PhantomData;

#[cfg(feature = "certification-test-authority")]
use super::{
    AdversarialLostFlushProfile, AdversarialReorderedFlushProfile, BackendDurabilitySupport,
    MmapFlushNotDurabilityCertifiedProfile, PosixFileFsyncDirFsyncProfile,
    SimulatedStrictDurableProfile, WindowsFlushFileBuffersProfile,
};
use super::{BackendDurabilityProfile, BackendDurabilityProfileId, WalDurabilityBarrier};

#[cfg(feature = "certification-test-authority")]
mod authority_sealed {
    pub trait Sealed<P> {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendDurabilityBarrierDenialKind {
    UnsupportedDurabilityCapability,
    AdversarialLostFlush,
    BarrierNotRequiredByProfile,
    BarrierNotCompleted,
    ProfileMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendDurabilityBarrierDenial {
    profile_id: BackendDurabilityProfileId,
    barrier: WalDurabilityBarrier,
    kind: BackendDurabilityBarrierDenialKind,
}

impl BackendDurabilityBarrierDenial {
    #[cfg(feature = "certification-test-authority")]
    pub(crate) const fn new<P: BackendDurabilityProfile>(
        barrier: WalDurabilityBarrier,
        kind: BackendDurabilityBarrierDenialKind,
    ) -> Self {
        Self {
            profile_id: P::ID,
            barrier,
            kind,
        }
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.profile_id
    }

    pub const fn barrier(&self) -> WalDurabilityBarrier {
        self.barrier
    }

    pub const fn kind(&self) -> BackendDurabilityBarrierDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalDurabilityBarrierReceipt<P: BackendDurabilityProfile, S> {
    profile: PhantomData<P>,
    scope: S,
    barrier: WalDurabilityBarrier,
}

impl<P: BackendDurabilityProfile, S> WalDurabilityBarrierReceipt<P, S> {
    #[cfg(feature = "certification-test-authority")]
    pub(crate) const fn from_executed_scope(scope: S, barrier: WalDurabilityBarrier) -> Self {
        Self {
            profile: PhantomData,
            scope,
            barrier,
        }
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        P::ID
    }

    pub const fn barrier(&self) -> WalDurabilityBarrier {
        self.barrier
    }

    pub const fn scope(&self) -> &S {
        &self.scope
    }
}

#[cfg(feature = "certification-test-authority")]
pub trait BackendDurabilityBarrierAuthority<P: BackendDurabilityProfile>:
    authority_sealed::Sealed<P> + Copy + Clone + Eq + 'static
{
    fn certify_completed_barrier<S>(
        self,
        scope: S,
        barrier: WalDurabilityBarrier,
    ) -> Result<WalDurabilityBarrierReceipt<P, S>, BackendDurabilityBarrierDenial>
    where
        S: Clone + Eq,
    {
        certify_profile_barrier::<P, S>(scope, barrier)
    }
}

#[cfg(feature = "certification-test-authority")]
fn certify_profile_barrier<P: BackendDurabilityProfile, S: Clone + Eq>(
    scope: S,
    barrier: WalDurabilityBarrier,
) -> Result<WalDurabilityBarrierReceipt<P, S>, BackendDurabilityBarrierDenial> {
    match P::SUPPORT {
        BackendDurabilitySupport::Certified => certify_required_barrier::<P, S>(scope, barrier),
        BackendDurabilitySupport::UnsupportedDurabilityCapability => {
            Err(BackendDurabilityBarrierDenial::new::<P>(
                barrier,
                BackendDurabilityBarrierDenialKind::UnsupportedDurabilityCapability,
            ))
        }
        BackendDurabilitySupport::AdversarialLostFlush => {
            Err(BackendDurabilityBarrierDenial::new::<P>(
                barrier,
                BackendDurabilityBarrierDenialKind::AdversarialLostFlush,
            ))
        }
    }
}

#[cfg(feature = "certification-test-authority")]
fn certify_required_barrier<P: BackendDurabilityProfile, S: Clone + Eq>(
    scope: S,
    barrier: WalDurabilityBarrier,
) -> Result<WalDurabilityBarrierReceipt<P, S>, BackendDurabilityBarrierDenial> {
    if P::REQUIRED_BARRIERS.contains(barrier) {
        Ok(WalDurabilityBarrierReceipt::from_executed_scope(
            scope, barrier,
        ))
    } else {
        Err(BackendDurabilityBarrierDenial::new::<P>(
            barrier,
            BackendDurabilityBarrierDenialKind::BarrierNotRequiredByProfile,
        ))
    }
}

#[cfg(feature = "certification-test-authority")]
macro_rules! define_barrier_authority {
    ($authority:ident, $profile:ty) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct $authority;

        impl $authority {
            pub const fn new() -> Self {
                Self
            }
        }

        impl authority_sealed::Sealed<$profile> for $authority {}

        impl BackendDurabilityBarrierAuthority<$profile> for $authority {}
    };
}

#[cfg(feature = "certification-test-authority")]
define_barrier_authority!(
    SimulatedStrictDurabilityAuthority,
    SimulatedStrictDurableProfile
);
#[cfg(feature = "certification-test-authority")]
define_barrier_authority!(
    PosixFileFsyncDirFsyncAuthority,
    PosixFileFsyncDirFsyncProfile
);
#[cfg(feature = "certification-test-authority")]
define_barrier_authority!(
    WindowsFlushFileBuffersAuthority,
    WindowsFlushFileBuffersProfile
);
#[cfg(feature = "certification-test-authority")]
define_barrier_authority!(
    MmapFlushNotDurabilityCertifiedAuthority,
    MmapFlushNotDurabilityCertifiedProfile
);
#[cfg(feature = "certification-test-authority")]
define_barrier_authority!(AdversarialLostFlushAuthority, AdversarialLostFlushProfile);
#[cfg(feature = "certification-test-authority")]
define_barrier_authority!(
    AdversarialReorderedFlushAuthority,
    AdversarialReorderedFlushProfile
);
