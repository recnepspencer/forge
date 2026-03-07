use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PayloadKey(u64);

impl PayloadKey {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl Display for PayloadKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "payload-{:016x}", self.0)
    }
}
