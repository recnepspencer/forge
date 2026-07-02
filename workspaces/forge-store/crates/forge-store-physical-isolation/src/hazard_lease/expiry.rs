use super::{
    HazardLeaseDenial, HazardLeaseGeneration, HazardLeaseReleaseReceipt, HazardLeaseSlot,
    OwnedCopyStableReadReceipt, ReadHandleRevocationReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseExpiryPosture {
    ExpiredWithoutAuthority {
        slot: HazardLeaseSlot,
        generation: HazardLeaseGeneration,
    },
    Released(HazardLeaseReleaseReceipt),
    Revoked(ReadHandleRevocationReceipt),
    OwnedCopyStable(OwnedCopyStableReadReceipt),
}

impl LeaseExpiryPosture {
    pub const fn expired_without_authority(
        slot: HazardLeaseSlot,
        generation: HazardLeaseGeneration,
    ) -> Self {
        Self::ExpiredWithoutAuthority { slot, generation }
    }

    pub const fn from_release(receipt: HazardLeaseReleaseReceipt) -> Self {
        Self::Released(receipt)
    }

    pub const fn from_revocation(receipt: ReadHandleRevocationReceipt) -> Self {
        Self::Revoked(receipt)
    }

    pub const fn from_owned_copy(receipt: OwnedCopyStableReadReceipt) -> Self {
        Self::OwnedCopyStable(receipt)
    }

    pub const fn require_reclaim_authority(self) -> Result<(), HazardLeaseDenial> {
        match self {
            Self::ExpiredWithoutAuthority { slot, .. } => {
                Err(HazardLeaseDenial::ExpiredLeaseWithoutReleaseRevocationOrOwnedCopy { slot })
            }
            Self::Released(_) | Self::Revoked(_) | Self::OwnedCopyStable(_) => Ok(()),
        }
    }
}
