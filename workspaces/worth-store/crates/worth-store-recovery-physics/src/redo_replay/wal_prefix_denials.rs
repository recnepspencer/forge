use crate::{LogSequenceNumber, WalLsnRange, WalSegmentGeneration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TornWalTailClassification {
    valid_prefix: WalLsnRange,
    torn_lsn: LogSequenceNumber,
}

impl TornWalTailClassification {
    pub(crate) const fn new(valid_prefix: WalLsnRange, torn_lsn: LogSequenceNumber) -> Self {
        Self {
            valid_prefix,
            torn_lsn,
        }
    }

    pub const fn valid_prefix(self) -> WalLsnRange {
        self.valid_prefix
    }

    pub const fn torn_lsn(self) -> LogSequenceNumber {
        self.torn_lsn
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiddleWalCorruptionDenial {
    corrupted_lsn: LogSequenceNumber,
    acknowledged_range: WalLsnRange,
}

impl MiddleWalCorruptionDenial {
    pub(crate) const fn new(
        corrupted_lsn: LogSequenceNumber,
        acknowledged_range: WalLsnRange,
    ) -> Self {
        Self {
            corrupted_lsn,
            acknowledged_range,
        }
    }

    pub const fn corrupted_lsn(self) -> LogSequenceNumber {
        self.corrupted_lsn
    }

    pub const fn acknowledged_range(self) -> WalLsnRange {
        self.acknowledged_range
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingAcknowledgedWalRangeDenial {
    missing_range: WalLsnRange,
    acknowledged_range: WalLsnRange,
}

impl MissingAcknowledgedWalRangeDenial {
    pub(crate) const fn new(missing_range: WalLsnRange, acknowledged_range: WalLsnRange) -> Self {
        Self {
            missing_range,
            acknowledged_range,
        }
    }

    pub const fn missing_range(self) -> WalLsnRange {
        self.missing_range
    }

    pub const fn acknowledged_range(self) -> WalLsnRange {
        self.acknowledged_range
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaleWalGenerationDenial {
    lsn: LogSequenceNumber,
    expected: WalSegmentGeneration,
    observed: WalSegmentGeneration,
}

impl StaleWalGenerationDenial {
    pub(crate) const fn new(
        lsn: LogSequenceNumber,
        expected: WalSegmentGeneration,
        observed: WalSegmentGeneration,
    ) -> Self {
        Self {
            lsn,
            expected,
            observed,
        }
    }

    pub const fn lsn(self) -> LogSequenceNumber {
        self.lsn
    }

    pub const fn expected(self) -> WalSegmentGeneration {
        self.expected
    }

    pub const fn observed(self) -> WalSegmentGeneration {
        self.observed
    }
}
