#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogSequenceNumber {
    value: u64,
}

impl LogSequenceNumber {
    pub const GENESIS: Self = Self { value: 0 };

    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn get(self) -> u64 {
        self.value
    }
}
