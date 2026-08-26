//! Caller-narrowable work limits for portable canonical-query re-admission.

const MAXIMUM_ENTRIES: u32 = 32_768;
const MAXIMUM_LOGICAL_BYTES: u64 = 4 * 1_024 * 1_024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPortableCanonicalQueryReadmissionLimits {
    maximum_entries: u32,
    maximum_logical_bytes: u64,
}

impl WorthQueryPortableCanonicalQueryReadmissionLimits {
    pub const DEFAULT: Self = Self {
        maximum_entries: MAXIMUM_ENTRIES,
        maximum_logical_bytes: MAXIMUM_LOGICAL_BYTES,
    };

    pub const fn new(maximum_entries: u32, maximum_logical_bytes: u64) -> Self {
        Self {
            maximum_entries,
            maximum_logical_bytes,
        }
    }

    pub const fn maximum_entries(self) -> u32 {
        self.maximum_entries
    }

    pub const fn maximum_logical_bytes(self) -> u64 {
        self.maximum_logical_bytes
    }

    pub(super) const fn narrowed(self) -> Self {
        Self {
            maximum_entries: if self.maximum_entries < Self::DEFAULT.maximum_entries {
                self.maximum_entries
            } else {
                Self::DEFAULT.maximum_entries
            },
            maximum_logical_bytes: if self.maximum_logical_bytes
                < Self::DEFAULT.maximum_logical_bytes
            {
                self.maximum_logical_bytes
            } else {
                Self::DEFAULT.maximum_logical_bytes
            },
        }
    }
}
