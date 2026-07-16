#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseReclaimAction {
    LeaseAcquired {
        slot: u32,
        generation: u64,
    },
    LeaseReleased {
        slot: u32,
        generation: u64,
    },
    LeaseRevoked {
        slot: u32,
        generation: u64,
    },
    LeaseExpiredWithoutAuthority {
        slot: u32,
        generation: u64,
    },
    OwnedCopyStabilized {
        slot: u32,
        generation: u64,
    },
    ReclaimAdmitted,
    ReclaimDeniedByLiveLease,
    IdentityReuseAdmitted {
        old_generation: u64,
        new_generation: u64,
    },
    IdentityReuseDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LeaseReclaimActionKind {
    LeaseAcquired,
    LeaseReleased,
    LeaseRevoked,
    LeaseExpiredWithoutAuthority,
    OwnedCopyStabilized,
    ReclaimAdmitted,
    ReclaimDeniedByLiveLease,
    IdentityReuseAdmitted,
    IdentityReuseDenied,
}

impl LeaseReclaimAction {
    pub const fn kind(self) -> LeaseReclaimActionKind {
        match self {
            Self::LeaseAcquired { .. } => LeaseReclaimActionKind::LeaseAcquired,
            Self::LeaseReleased { .. } => LeaseReclaimActionKind::LeaseReleased,
            Self::LeaseRevoked { .. } => LeaseReclaimActionKind::LeaseRevoked,
            Self::LeaseExpiredWithoutAuthority { .. } => {
                LeaseReclaimActionKind::LeaseExpiredWithoutAuthority
            }
            Self::OwnedCopyStabilized { .. } => LeaseReclaimActionKind::OwnedCopyStabilized,
            Self::ReclaimAdmitted => LeaseReclaimActionKind::ReclaimAdmitted,
            Self::ReclaimDeniedByLiveLease => LeaseReclaimActionKind::ReclaimDeniedByLiveLease,
            Self::IdentityReuseAdmitted { .. } => LeaseReclaimActionKind::IdentityReuseAdmitted,
            Self::IdentityReuseDenied => LeaseReclaimActionKind::IdentityReuseDenied,
        }
    }
}

impl LeaseReclaimActionKind {
    pub const fn all() -> [Self; 9] {
        [
            Self::LeaseAcquired,
            Self::LeaseReleased,
            Self::LeaseRevoked,
            Self::LeaseExpiredWithoutAuthority,
            Self::OwnedCopyStabilized,
            Self::ReclaimAdmitted,
            Self::ReclaimDeniedByLiveLease,
            Self::IdentityReuseAdmitted,
            Self::IdentityReuseDenied,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseReclaimDenial {
    LiveLeaseProtectsIdentity,
    ExpiryIsNotReleaseAuthority,
    GenerationDidNotAdvance,
    StaleLeaseGeneration,
}
