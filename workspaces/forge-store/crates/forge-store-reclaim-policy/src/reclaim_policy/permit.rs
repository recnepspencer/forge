#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReclaimPermit {
    permits: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReclaimPermitDenial {
    ZeroReclaimPermit,
}

impl ReclaimPermit {
    pub const fn new(permits: u32) -> Result<Self, ReclaimPermitDenial> {
        if permits == 0 {
            return Err(ReclaimPermitDenial::ZeroReclaimPermit);
        }
        Ok(Self { permits })
    }

    pub const fn permits(self) -> u32 {
        self.permits
    }
}
