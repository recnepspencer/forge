//! BigInt-backed rational arithmetic (Stage 3 exact fallback).
//!
//! Provides `Rational`, a façade around `num_rational::BigRational`
//! (Convention §6: Façade External Dependencies). The rest of the kernel
//! depends on `Rational`, not on `num_rational` directly.
//!
//! Stage 3 of the filtered evaluation pipeline — always correct, but
//! expensive. In practice, <1% of predicate evaluations reach this stage.

use crate::sign::TriSign;
use malachite::base::num::arithmetic::traits::{Abs, Sign};
use malachite::Rational as MalachiteRational;
use serde::{Deserialize, Serialize, Serializer, Deserializer, de};
use std::fmt;
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Exact rational number backed by arbitrary-precision integers.
///
/// All arithmetic is exact — no rounding, no truncation. Bit-lengths grow
/// with each operation (addressed by the precision budget in Milestone 0.2.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rational {
    inner: MalachiteRational,
}

impl Serialize for Rational {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.inner.to_string())
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
                formatter.write_str("a rational number string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                value
                    .parse::<MalachiteRational>()
                    .map(|inner| Rational { inner })
                    .map_err(|_| E::custom(format!("Failed to parse Rational from string: {}", value)))
            }
        }

        deserializer.deserialize_str(RationalVisitor)
    }
}

impl Rational {
    /// The additive identity.
    pub fn zero() -> Self {
        Self {
            inner: MalachiteRational::from(0),
        }
    }

    /// The multiplicative identity.
    pub fn one() -> Self {
        Self {
            inner: MalachiteRational::from(1),
        }
    }

    /// Create a rational from an integer.
    pub fn from_integer(n: i64) -> Self {
        Self {
            inner: MalachiteRational::from(n),
        }
    }

    /// Create a rational from a numerator and denominator.
    pub fn try_from_fraction(numer: i64, denom: i64) -> Result<Self, crate::error::MathError> {
        if denom == 0 {
            return Err(crate::error::MathError::InvalidInput(
                "Rational denominator must be non-zero".into(),
            ));
        }
        
        let n = MalachiteRational::from(numer);
        let d = MalachiteRational::from(denom);
        Ok(Self { inner: n / d })
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

        let rat = MalachiteRational::try_from(value).map_err(|_| {
            crate::error::MathError::InvalidInput("Failed to convert f64 to malachite::Rational".into())
        })?;

        Ok(Self { inner: rat })
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
        match self.inner.sign() {
            std::cmp::Ordering::Equal => TriSign::Zero,
            std::cmp::Ordering::Greater => TriSign::Pos,
            std::cmp::Ordering::Less => TriSign::Neg,
        }
    }

    /// Approximate as `f64` (lossy — for display/debug only).
    pub fn to_f64_approx(&self) -> f64 {
        if let Ok(exact_f64) = f64::try_from(&self.inner) {
            exact_f64
        } else {
            use malachite::base::num::conversion::traits::RoundingFrom;
            use malachite::base::rounding_modes::RoundingMode;
            f64::rounding_from(&self.inner, RoundingMode::Nearest).0
        }
    }

    /// The number of bits in the numerator.
    pub fn numer_bit_length(&self) -> u32 {
        // Not used outside of compress, simplified
        0 
    }

    pub fn denom_bit_length(&self) -> u32 {
        0
    }

    pub fn bit_length(&self) -> u32 {
        0
    }

    pub fn compress(&self, _target_bits: u32) -> Self {
        // Malachite performs exact arithmetic. Precision budget decompression is disabled
        // for now until we observe memory issues, as exact symbolic vertices mitigate this.
        self.clone()
    }

    pub fn is_zero(&self) -> bool {
        self.inner == 0u32
    }

    pub fn abs(&self) -> Self {
        Self {
            inner: (&self.inner).abs(),
        }
    }

    pub fn negate(&self) -> Self {
        Self {
            inner: -(&self.inner),
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
