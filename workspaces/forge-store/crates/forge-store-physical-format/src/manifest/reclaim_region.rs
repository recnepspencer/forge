use crate::PhysicalReference;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReclaimRegion {
    reference: PhysicalReference,
    byte_len: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReclaimRegionDenial {
    EmptyRegion,
}

impl PhysicalReclaimRegion {
    pub const fn new(
        reference: PhysicalReference,
        byte_len: u32,
    ) -> Result<Self, PhysicalReclaimRegionDenial> {
        if byte_len == 0 {
            return Err(PhysicalReclaimRegionDenial::EmptyRegion);
        }
        Ok(Self {
            reference,
            byte_len,
        })
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn byte_len(self) -> u32 {
        self.byte_len
    }
}
