#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalHeaderReservedField {
    ChecksumSlot,
    RecoveryLsn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalHeaderReservedFields {
    checksum_slot: u32,
    recovery_lsn: u64,
}

impl PhysicalHeaderReservedFields {
    pub const fn new(checksum_slot: u32, recovery_lsn: u64) -> Self {
        Self {
            checksum_slot,
            recovery_lsn,
        }
    }

    pub const fn zeroed() -> Self {
        Self {
            checksum_slot: 0,
            recovery_lsn: 0,
        }
    }

    pub const fn checksum_slot(self) -> u32 {
        self.checksum_slot
    }

    pub const fn recovery_lsn(self) -> u64 {
        self.recovery_lsn
    }

    pub const fn misused_field(self) -> Option<PhysicalHeaderReservedField> {
        if self.checksum_slot != 0 {
            return Some(PhysicalHeaderReservedField::ChecksumSlot);
        }
        if self.recovery_lsn != 0 {
            return Some(PhysicalHeaderReservedField::RecoveryLsn);
        }
        None
    }
}
