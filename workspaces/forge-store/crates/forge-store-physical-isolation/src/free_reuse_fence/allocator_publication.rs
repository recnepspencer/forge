use crate::{PhysicalOrderingContract, PhysicalOrderingSite};

use super::FreeReuseFenceDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocatorPublicationReceipt {
    ordering: PhysicalOrderingContract,
}

impl AllocatorPublicationReceipt {
    pub fn from_ordering(ordering: PhysicalOrderingContract) -> Result<Self, FreeReuseFenceDenial> {
        let ordering = ordering
            .require_site(PhysicalOrderingSite::AllocatorPublication)
            .map_err(|_| FreeReuseFenceDenial::AllocatorPublicationOrderingNotCrashStable)?;
        Ok(Self { ordering })
    }

    pub const fn ordering(self) -> PhysicalOrderingContract {
        self.ordering
    }
}
