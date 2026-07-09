use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalDate {
    pub days_from_unix_epoch: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalTime {
    pub nanos_since_midnight: u64,
}

impl CanonicalTime {
    pub const NANOS_PER_DAY: u64 = 86_400_000_000_000;

    pub fn new(nanos_since_midnight: u64) -> Option<Self> {
        if nanos_since_midnight < Self::NANOS_PER_DAY {
            Some(Self {
                nanos_since_midnight,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalTimestamp {
    pub micros_since_unix_epoch: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalTimestampTz {
    pub utc_micros_since_unix_epoch: i64,
    pub offset_minutes: i32,
}
