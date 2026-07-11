use crate::{PhysicalGenerationOwner, PhysicalHeaderKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAuthenticityIdentity {
    header_kind: PhysicalHeaderKind,
    locality: PhysicalGenerationOwner,
    checked_byte_count: u64,
    checksum_value: u64,
    checksum_algorithm: &'static str,
}

impl PhysicalAuthenticityIdentity {
    pub const fn new(
        header_kind: PhysicalHeaderKind,
        locality: PhysicalGenerationOwner,
        checked_byte_count: u64,
        checksum_value: u64,
        checksum_algorithm: &'static str,
    ) -> Self {
        Self {
            header_kind,
            locality,
            checked_byte_count,
            checksum_value,
            checksum_algorithm,
        }
    }

    pub const fn header_kind(self) -> PhysicalHeaderKind {
        self.header_kind
    }

    pub const fn locality(self) -> PhysicalGenerationOwner {
        self.locality
    }

    pub const fn checked_byte_count(self) -> u64 {
        self.checked_byte_count
    }

    pub const fn checksum_value(self) -> u64 {
        self.checksum_value
    }

    pub const fn checksum_algorithm(self) -> &'static str {
        self.checksum_algorithm
    }
}
