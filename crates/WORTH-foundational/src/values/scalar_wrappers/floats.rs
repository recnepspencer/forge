use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalF32(pub u32);

impl CanonicalF32 {
    pub fn from_f32(value: f32) -> Self {
        Self::from_bits(value.to_bits())
    }

    pub fn from_bits(bits: u32) -> Self {
        if f32::from_bits(bits).is_nan() {
            Self(f32::NAN.to_bits())
        } else {
            Self(bits)
        }
    }

    pub fn bits(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalF64(pub u64);

impl CanonicalF64 {
    pub fn from_f64(value: f64) -> Self {
        Self::from_bits(value.to_bits())
    }

    pub fn from_bits(bits: u64) -> Self {
        if f64::from_bits(bits).is_nan() {
            Self(f64::NAN.to_bits())
        } else {
            Self(bits)
        }
    }

    pub fn bits(self) -> u64 {
        self.0
    }
}
