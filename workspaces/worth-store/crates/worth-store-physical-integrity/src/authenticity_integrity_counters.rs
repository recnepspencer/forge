use crate::PreDecodeAdmissionCounters;
use worth_store_security::StoreAuthenticityCheckCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticityPolicyDecodeCounters {
    integrity: PreDecodeAdmissionCounters,
    authenticity: StoreAuthenticityCheckCounterSnapshot,
}

impl AuthenticityPolicyDecodeCounters {
    pub(crate) const fn new(
        integrity: PreDecodeAdmissionCounters,
        authenticity: StoreAuthenticityCheckCounterSnapshot,
    ) -> Self {
        Self {
            integrity,
            authenticity,
        }
    }

    pub const fn integrity(self) -> PreDecodeAdmissionCounters {
        self.integrity
    }

    pub const fn authenticity(self) -> StoreAuthenticityCheckCounterSnapshot {
        self.authenticity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthenticityRequiredDecodeCounters {
    integrity: PreDecodeAdmissionCounters,
    authenticity: StoreAuthenticityCheckCounterSnapshot,
    checksum_valid_authenticity_failed: u32,
    checksum_valid_authenticity_unavailable: u32,
    checksum_valid_authenticity_unsupported: u32,
}

impl AuthenticityRequiredDecodeCounters {
    pub(crate) const fn admitted(
        integrity: PreDecodeAdmissionCounters,
        authenticity: StoreAuthenticityCheckCounterSnapshot,
    ) -> Self {
        Self {
            integrity,
            authenticity,
            checksum_valid_authenticity_failed: 0,
            checksum_valid_authenticity_unavailable: 0,
            checksum_valid_authenticity_unsupported: 0,
        }
    }

    pub(crate) const fn denied(
        integrity: PreDecodeAdmissionCounters,
        authenticity: StoreAuthenticityCheckCounterSnapshot,
        failed: bool,
        unavailable: bool,
        unsupported: bool,
    ) -> Self {
        Self {
            integrity,
            authenticity,
            checksum_valid_authenticity_failed: failed as u32,
            checksum_valid_authenticity_unavailable: unavailable as u32,
            checksum_valid_authenticity_unsupported: unsupported as u32,
        }
    }

    pub const fn integrity(self) -> PreDecodeAdmissionCounters {
        self.integrity
    }

    pub const fn authenticity(self) -> StoreAuthenticityCheckCounterSnapshot {
        self.authenticity
    }

    pub const fn checksum_valid_authenticity_failed(self) -> u32 {
        self.checksum_valid_authenticity_failed
    }

    pub const fn checksum_valid_authenticity_unavailable(self) -> u32 {
        self.checksum_valid_authenticity_unavailable
    }

    pub const fn checksum_valid_authenticity_unsupported(self) -> u32 {
        self.checksum_valid_authenticity_unsupported
    }
}
