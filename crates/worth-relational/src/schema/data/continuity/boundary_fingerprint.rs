use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaBoundaryFingerprint(pub [u8; 32]);

impl SchemaBoundaryFingerprint {
    pub const ZERO: Self = Self([0; 32]);

    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}
