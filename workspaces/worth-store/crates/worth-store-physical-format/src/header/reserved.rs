#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalHeaderReservedField {
    ChecksumSlot,
    ReservedTail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalHeaderReservedFields {
    checksum_slot: u32,
    tail: [u8; 8],
}

impl PhysicalHeaderReservedFields {
    pub const fn new(checksum_slot: u32, tail: [u8; 8]) -> Self {
        Self {
            checksum_slot,
            tail,
        }
    }

    pub const fn zeroed() -> Self {
        Self {
            checksum_slot: 0,
            tail: [0; 8],
        }
    }

    pub const fn checksum_slot(self) -> u32 {
        self.checksum_slot
    }

    pub const fn misused_field(self) -> Option<PhysicalHeaderReservedField> {
        if self.checksum_slot != 0 {
            return Some(PhysicalHeaderReservedField::ChecksumSlot);
        }
        if self.tail[0] != 0
            || self.tail[1] != 0
            || self.tail[2] != 0
            || self.tail[3] != 0
            || self.tail[4] != 0
            || self.tail[5] != 0
            || self.tail[6] != 0
            || self.tail[7] != 0
        {
            return Some(PhysicalHeaderReservedField::ReservedTail);
        }
        None
    }
}
