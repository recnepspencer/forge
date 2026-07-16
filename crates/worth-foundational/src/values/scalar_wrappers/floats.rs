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

    /// Returns the canonical native value. NaN payloads have already been
    /// normalized by construction, so consumers do not need to reinterpret
    /// representation bits to use the value.
    pub fn as_f32(self) -> f32 {
        f32::from_bits(self.0)
    }

    pub(crate) fn is_canonical(self) -> bool {
        !f32::from_bits(self.0).is_nan() || self.0 == f32::NAN.to_bits()
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

    pub fn as_f64(self) -> f64 {
        f64::from_bits(self.0)
    }

    pub(crate) fn is_canonical(self) -> bool {
        !f64::from_bits(self.0).is_nan() || self.0 == f64::NAN.to_bits()
    }
}
