use crate::{LogSequenceNumber, WalTopologyDenial, WalTopologyDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WalLsnRange {
    start: LogSequenceNumber,
    end_exclusive: LogSequenceNumber,
}

impl WalLsnRange {
    pub fn new(
        start: LogSequenceNumber,
        end_exclusive: LogSequenceNumber,
    ) -> Result<Self, WalTopologyDenial> {
        if end_exclusive == start {
            return Err(WalTopologyDenial::new(WalTopologyDenialKind::EmptyRange));
        }
        if end_exclusive < start {
            return Err(WalTopologyDenial::new(WalTopologyDenialKind::InvertedRange));
        }
        Ok(Self {
            start,
            end_exclusive,
        })
    }

    pub const fn start(self) -> LogSequenceNumber {
        self.start
    }

    pub const fn end_exclusive(self) -> LogSequenceNumber {
        self.end_exclusive
    }

    pub const fn contains(self, lsn: LogSequenceNumber) -> bool {
        self.start.get() <= lsn.get() && lsn.get() < self.end_exclusive.get()
    }

    pub const fn is_contiguous_with(self, next: Self) -> bool {
        self.end_exclusive.get() == next.start.get()
    }

    pub const fn overlaps(self, next: Self) -> bool {
        self.start.get() < next.end_exclusive.get() && next.start.get() < self.end_exclusive.get()
    }
}

impl Ord for WalLsnRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start
            .cmp(&other.start)
            .then_with(|| self.end_exclusive.cmp(&other.end_exclusive))
    }
}

impl PartialOrd for WalLsnRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
