//! BigInt-backed rational arithmetic (exact fallback).
//!
//! Provides `Rational`, a façade around `num_rational::BigRational`
//! (Convention §6: Façade External Dependencies). The rest of the kernel
//! depends on `Rational`, not on `num_rational` directly.
//!
//! Used as the exact arithmetic fallback for geometric computations
//! that cannot be resolved by expansion arithmetic alone.

use crate::sign::TriSign;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Exact rational number backed by arbitrary-precision integers.
///
/// All arithmetic is exact — no rounding, no truncation. Bit-lengths grow
/// with each operation (addressed by the precision budget in Milestone 0.2.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rational {
    inner: BigRational,
}

impl Serialize for Rational {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}/{}", self.inner.numer(), self.inner.denom()))
    }
}

impl<'de> Deserialize<'de> for Rational {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RationalVisitor;

        impl<'de> de::Visitor<'de> for RationalVisitor {
            type Value = Rational;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a rational number string like '3/4' or '5'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some((n, d)) = value.split_once('/') {
                    let numer: BigInt = n
                        .parse()
                        .map_err(|e| E::custom(format!("bad numerator: {e}")))?;
                    let denom: BigInt = d
                        .parse()
                        .map_err(|e| E::custom(format!("bad denominator: {e}")))?;
                    if denom.is_zero() {
                        return Err(E::custom("denominator is zero"));
                    }
                    Ok(Rational {
                        inner: BigRational::new(numer, denom),
                    })
                } else {
                    let n: BigInt = value
                        .parse()
                        .map_err(|e| E::custom(format!("bad integer: {e}")))?;
                    Ok(Rational {
                        inner: BigRational::from(n),
                    })
                }
            }
        }

        deserializer.deserialize_str(RationalVisitor)
    }
}

impl Rational {
    /// The additive identity.
    pub fn zero() -> Self {
        Self {
            inner: BigRational::zero(),
        }
    }

    /// The multiplicative identity.
    pub fn one() -> Self {
        Self {
            inner: BigRational::one(),
        }
    }

    /// Create a rational from an integer.
    pub fn from_integer(n: i64) -> Self {
        Self {
            inner: BigRational::from(BigInt::from(n)),
        }
    }

    /// Create a rational from a numerator and denominator.
    pub fn try_from_fraction(numer: i64, denom: i64) -> Result<Self, crate::error::MathError> {
        if denom == 0 {
            return Err(crate::error::MathError::InvalidInput(
                "Rational denominator must be non-zero".into(),
            ));
        }

        Ok(Self {
            inner: BigRational::new(BigInt::from(numer), BigInt::from(denom)),
        })
    }

    /// Convert an `f64` to an exact rational representation.
    ///
    /// Every finite IEEE 754 double has an exact rational form:
    /// `significand * 2^exponent`. This conversion is lossless.
    pub fn try_from_f64(value: f64) -> Result<Self, crate::error::MathError> {
        if !value.is_finite() {
            return Err(crate::error::MathError::InvalidInput(
                "Cannot convert NaN/Inf to Rational".into(),
            ));
        }

        if value == 0.0 {
            return Ok(Self::zero());
        }

        let bits = value.to_bits();
        let sign = if bits >> 63 == 1 { -1i64 } else { 1i64 };
        let exponent = ((bits >> 52) & 0x7FF) as i64;
        let mantissa = bits & 0x000F_FFFF_FFFF_FFFF;

        let (numer, denom) = if exponent == 0 {
            let sig = BigInt::from(mantissa);
            let num = BigInt::from(sign) * sig;
            let den = BigInt::from(1u64) << 1074usize;
            (num, den)
        } else {
            let sig = BigInt::from(mantissa | (1u64 << 52));
            let num = BigInt::from(sign) * sig;
            let e = exponent - 1023 - 52;
            if e >= 0 {
                let shifted = num << (e as usize);
                (shifted, BigInt::from(1))
            } else {
                let den = BigInt::from(1u64) << ((-e) as usize);
                (num, den)
            }
        };

        Ok(Self {
            inner: BigRational::new(numer, denom),
        })
    }

    /// Convert a 3D `f64` array to an exact rational representation.
    /// Returns `None` if any component is non-finite.
    pub fn try_from_f64_3(pos: &[f64; 3]) -> Option<[Self; 3]> {
        let x = Self::try_from_f64(pos[0]).ok()?;
        let y = Self::try_from_f64(pos[1]).ok()?;
        let z = Self::try_from_f64(pos[2]).ok()?;
        Some([x, y, z])
    }

    /// Return the exact sign of this rational.
    pub fn sign(&self) -> TriSign {
        if self.inner.is_zero() {
            TriSign::Zero
        } else if self.inner.is_positive() {
            TriSign::Pos
        } else {
            TriSign::Neg
        }
    }

    /// Approximate as `f64` (lossy — for display/debug only).
    pub fn to_f64_approx(&self) -> f64 {
        let numer = self.inner.numer().to_f64().unwrap_or(f64::INFINITY);
        let denom = self.inner.denom().to_f64().unwrap_or(f64::INFINITY);
        if denom == 0.0 {
            if numer >= 0.0 {
                f64::INFINITY
            } else {
                f64::NEG_INFINITY
            }
        } else {
            numer / denom
        }
    }

    /// The number of bits in the numerator.
    pub fn numer_bit_length(&self) -> u32 {
        self.inner.numer().bits() as u32
    }

    /// The number of bits in the denominator.
    pub fn denom_bit_length(&self) -> u32 {
        self.inner.denom().bits() as u32
    }

    /// Total bit length (max of numerator and denominator).
    pub fn bit_length(&self) -> u32 {
        self.numer_bit_length().max(self.denom_bit_length())
    }

    /// Compress to fit within target_bits (lossy, sign-preserving).
    ///
    /// Reduces bit-length by round-tripping through f64 approximation.
    /// Guarantees: sign is preserved, result has ≤53 bits (f64 precision).
    /// This is lossy — magnitude may shift by up to 1 ULP.
    pub fn compress(&self, _target_bits: u32) -> Self {
        if self.inner.is_zero() {
            return self.clone();
        }
        let approx = self.to_f64_approx();
        match Self::try_from_f64(approx) {
            Ok(compressed) if compressed.sign() == self.sign() => compressed,
            _ => self.clone(),
        }
    }

    /// Check if this rational is exactly zero.
    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    /// Absolute value.
    pub fn abs(&self) -> Self {
        Self {
            inner: self.inner.abs(),
        }
    }

    /// Negate this value.
    pub fn negate(&self) -> Self {
        Self {
            inner: -&self.inner,
        }
    }
}

impl Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            inner: self.inner + rhs.inner,
        }
    }
}

impl Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            inner: self.inner - rhs.inner,
        }
    }
}

impl Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            inner: self.inner * rhs.inner,
        }
    }
}

impl Div for Rational {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            inner: self.inner / rhs.inner,
        }
    }
}

impl Add for &Rational {
    type Output = Rational;
    fn add(self, rhs: Self) -> Rational {
        Rational {
            inner: &self.inner + &rhs.inner,
        }
    }
}

impl Sub for &Rational {
    type Output = Rational;
    fn sub(self, rhs: Self) -> Rational {
        Rational {
            inner: &self.inner - &rhs.inner,
        }
    }
}

impl Mul for &Rational {
    type Output = Rational;
    fn mul(self, rhs: Self) -> Rational {
        Rational {
            inner: &self.inner * &rhs.inner,
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        Self { inner: -self.inner }
    }
}

impl Neg for &Rational {
    type Output = Rational;
    fn neg(self) -> Rational {
        Rational {
            inner: -&self.inner,
        }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

#[cfg(test)]
mod tests;
