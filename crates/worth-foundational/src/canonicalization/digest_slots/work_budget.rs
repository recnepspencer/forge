#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalDigestWorkBudget {
    maximum_entry_count: u32,
    maximum_encoded_bytes: usize,
}

impl CanonicalDigestWorkBudget {
    const STANDARD_MAXIMUM_ENTRY_COUNT: u32 = 4_096;
    const STANDARD_MAXIMUM_ENCODED_BYTES: usize = 4 * 1_024 * 1_024;

    pub const fn new(maximum_entry_count: u32, maximum_encoded_bytes: usize) -> Option<Self> {
        if maximum_entry_count == 0 || maximum_encoded_bytes == 0 {
            None
        } else {
            Some(Self {
                maximum_entry_count,
                maximum_encoded_bytes,
            })
        }
    }

    pub const fn maximum_entry_count(self) -> u32 {
        self.maximum_entry_count
    }

    pub const fn maximum_encoded_bytes(self) -> usize {
        self.maximum_encoded_bytes
    }

    pub const fn standard() -> Self {
        Self {
            maximum_entry_count: Self::STANDARD_MAXIMUM_ENTRY_COUNT,
            maximum_encoded_bytes: Self::STANDARD_MAXIMUM_ENCODED_BYTES,
        }
    }
}
