//! BigInt-backed rational arithmetic (Stage 3 exact fallback).
//!
//! Provides `Rational`, a façade around `num_rational::BigRational`
//! (Convention §6: Façade External Dependencies). The rest of the kernel
//! depends on `Rational`, not on `num_rational` directly.
//!
//! Stage 3 of the filtered evaluation pipeline — always correct, but
//! expensive. In practice, <1% of predicate evaluations reach this stage.

use crate::sign::TriSign;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Exact rational number backed by arbitrary-precision integers.
///
/// All arithmetic is exact — no rounding, no truncation. Bit-lengths grow
/// with each operation (addressed by the precision budget in Milestone 0.2.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rational {
    inner: BigRational,
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
            inner: BigRational::from_integer(BigInt::from(n)),
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

        let (sign, significand, exponent) = extract_ieee754_components(value);
        let signed_significand = BigInt::from(sign) * significand;
        let result = assemble_rational(signed_significand, exponent);
        Ok(Self { inner: result })
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
        let numerator: f64 = self.inner.numer().to_string().parse().unwrap_or(f64::MAX);
        let denominator: f64 = self.inner.denom().to_string().parse().unwrap_or(1.0);
        numerator / denominator
    }

    /// The number of bits in the numerator.
    pub fn numer_bit_length(&self) -> u32 {
        bit_length_of(self.inner.numer())
    }

    /// The number of bits in the denominator.
    pub fn denom_bit_length(&self) -> u32 {
        bit_length_of(self.inner.denom())
    }

    /// Total bit-length: `max(numer_bits, denom_bits)`.
    ///
    /// Used by the precision budget (Milestone 0.2.3) to detect runaway growth.
    pub fn bit_length(&self) -> u32 {
        self.numer_bit_length().max(self.denom_bit_length())
    }

    /// Compress this rational to fit within `target_bits`.
    ///
    /// Reduces bit-length by right-shifting the numerator and denominator
    /// until both fit within the target. **Sign is always preserved.**
    /// Zero values pass through unchanged.
    ///
    /// This is the "pressure valve" for Milestone 0.2.3: when exact
    /// arithmetic causes bit-lengths to explode, compress the value
    /// while keeping the sign intact (the sign is what matters for
    /// predicate evaluation).
    pub fn compress(&self, target_bits: u32) -> Self {
        if self.inner.is_zero() {
            return self.clone();
        }

        let numer_bits = self.numer_bit_length();
        let denom_bits = self.denom_bit_length();
        let max_bits = numer_bits.max(denom_bits);

        if max_bits <= target_bits {
            return self.clone();
        }

        let shift = max_bits - target_bits;

        let shifted_numer = self.inner.numer() >> (shift as usize);
        let shifted_denom = self.inner.denom() >> (shift as usize);

        if shifted_denom == BigInt::from(0) {
            let sign_val = if self.inner.is_positive() { 1i64 } else { -1i64 };
            return Self::from_integer(sign_val);
        }

        if shifted_numer == BigInt::from(0) {
            let sign_val = if self.inner.is_positive() { 1i64 } else { -1i64 };
            return Self::from_integer(sign_val);
        }

        Self {
            inner: BigRational::new(shifted_numer, shifted_denom),
        }
    }

    /// Returns whether this rational is exactly zero.
    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    /// Absolute value of this rational.
    pub fn abs(&self) -> Self {
        Self {
            inner: self.inner.abs(),
        }
    }

    /// Negate this rational.
    pub fn negate(&self) -> Self {
        Self {
            inner: -&self.inner,
        }
    }
}

/// Extract sign, significand, and exponent from an IEEE 754 `f64`.
///
/// Handles both normal and subnormal representations.
/// Returns `(sign, significand, exponent)` where sign is `1` or `-1`.
fn extract_ieee754_components(value: f64) -> (i64, BigInt, i32) {
    let bits = value.to_bits();
    let sign = if bits >> 63 == 1 { -1i64 } else { 1i64 };
    let raw_exponent = ((bits >> 52) & 0x7FF) as i32;
    let mantissa = bits & 0x000F_FFFF_FFFF_FFFF;

    let (significand, exponent) = if raw_exponent == 0 {
        (BigInt::from(mantissa), 1 - 1023 - 52)
    } else {
        let sig = BigInt::from(1u64 << 52 | mantissa);
        (sig, raw_exponent - 1023 - 52)
    };

    (sign, significand, exponent)
}

/// Scale a signed significand by `2^exponent` to produce a `BigRational`.
fn assemble_rational(signed_significand: BigInt, exponent: i32) -> BigRational {
    if exponent >= 0 {
        let power = BigInt::from(1u64) << (exponent as u32);
        BigRational::from_integer(signed_significand * power)
    } else {
        let power = BigInt::from(1u64) << ((-exponent) as u32);
        BigRational::new(signed_significand, power)
    }
}

/// Count the number of bits needed to represent a `BigInt` (ignoring sign).
fn bit_length_of(n: &BigInt) -> u32 {
    let (_, digits) = n.to_u64_digits();
    if digits.is_empty() {
        return 0;
    }
    let most_significant_digit = match digits.last() {
        Some(&d) => d,
        None => return 0,
    };
    let leading_bits = 64 - most_significant_digit.leading_zeros();
    ((digits.len() - 1) as u32) * 64 + leading_bits
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

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            inner: -self.inner,
        }
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
mod tests {
    use super::*;

    #[test]
    fn zero_has_zero_sign() {
        assert_eq!(Rational::zero().sign(), TriSign::Zero);
    }

    #[test]
    fn positive_integer_has_positive_sign() {
        assert_eq!(Rational::from_integer(42).sign(), TriSign::Pos);
    }

    #[test]
    fn negative_integer_has_negative_sign() {
        assert_eq!(Rational::from_integer(-7).sign(), TriSign::Neg);
    }

    #[test]
    fn fraction_arithmetic() {
        let half = Rational::try_from_fraction(1, 2).unwrap();
        let third = Rational::try_from_fraction(1, 3).unwrap();
        let sum = &half + &third;
        assert_eq!(sum, Rational::try_from_fraction(5, 6).unwrap());
    }

    #[test]
    fn f64_exact_conversion_integer() {
        assert_eq!(Rational::try_from_f64(3.0).unwrap(), Rational::from_integer(3));
    }

    #[test]
    fn f64_exact_conversion_half() {
        assert_eq!(Rational::try_from_f64(0.5).unwrap(), Rational::try_from_fraction(1, 2).unwrap());
    }

    #[test]
    fn f64_exact_conversion_negative() {
        assert_eq!(Rational::try_from_f64(-2.5).unwrap(), Rational::try_from_fraction(-5, 2).unwrap());
    }

    #[test]
    fn f64_zero_converts_to_rational_zero() {
        let r = Rational::try_from_f64(0.0).unwrap();
        assert!(r.is_zero());
        assert_eq!(r.sign(), TriSign::Zero);
    }

    #[test]
    fn subtraction_to_zero_gives_zero_sign() {
        let a = Rational::try_from_f64(1.0 / 3.0).unwrap();
        let b = a.clone();
        assert_eq!((a - b).sign(), TriSign::Zero);
    }

    #[test]
    fn bit_length_small_value() {
        assert!(Rational::from_integer(255).numer_bit_length() <= 8);
    }

    #[test]
    fn bit_length_grows_with_operations() {
        let a = Rational::try_from_f64(1.0 / 3.0).unwrap();
        let b = Rational::try_from_f64(1.0 / 7.0).unwrap();
        let product = &a * &b;
        assert!(product.bit_length() >= a.bit_length().min(b.bit_length()));
    }
}
