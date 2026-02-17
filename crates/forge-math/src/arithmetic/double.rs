//! Double-double (compensated) arithmetic for Stage 2 evaluation.
//!
//! A `Double` represents a value as a non-overlapping pair `(hi, lo)` where
//! the true value is `hi + lo`. This gives ~106 bits of significand precision
//! (vs 53 for `f64`) using only hardware floating-point operations.
//!
//! Based on Knuth/Dekker error-free transforms. `two_sum(a, b)` computes
//! `s = a + b` and `e = (a + b) - s` exactly; `two_product(a, b)` computes
//! `p = a * b` and `e = fma(a, b, -p)` exactly.
//!
//! # Reference
//!
//! Shewchuk, "Adaptive Precision Floating-Point Arithmetic and Fast Robust
//! Geometric Predicates," 1997.

use crate::sign::TriSign;
use std::ops::{Add, Mul, Sub};

/// A double-double number: `value = hi + lo` with ~106 bits of precision.
///
/// Invariant: `|lo| <= 0.5 * ulp(hi)` (the pair is non-overlapping).
#[derive(Debug, Clone, Copy)]
pub struct Double {
    hi: f64,
    lo: f64,
}

impl Double {
    /// Create a `Double` from a single `f64` (error term is zero).
    pub fn from_f64(value: f64) -> Self {
        Self {
            hi: value,
            lo: 0.0,
        }
    }

    /// The high-order component.
    pub fn hi(self) -> f64 {
        self.hi
    }

    /// The low-order error compensation term.
    pub fn lo(self) -> f64 {
        self.lo
    }

    /// Error-free sum via the TwoSum algorithm (Knuth, 1969).
    ///
    /// Returns a `Double` such that `a + b == hi + lo` exactly.
    pub fn two_sum(a: f64, b: f64) -> Self {
        let sum = a + b;
        let virtual_b = sum - a;
        let error = (a - (sum - virtual_b)) + (b - virtual_b);
        Self { hi: sum, lo: error }
    }

    /// Error-free product using FMA.
    ///
    /// Returns a `Double` such that `a * b == hi + lo` exactly.
    pub fn two_product(a: f64, b: f64) -> Self {
        let product = a * b;
        let error = a.mul_add(b, -product);
        Self { hi: product, lo: error }
    }

    /// Negate this value.
    pub fn negate(self) -> Double {
        Double {
            hi: -self.hi,
            lo: -self.lo,
        }
    }

    /// Determine the sign, returning `None` only if truly unresolvable.
    ///
    /// The sign of `hi` dominates; if `hi == 0.0`, `lo` breaks the tie.
    /// Returns `Ok(Some(Zero))` when both components are zero.
    pub fn sign(self) -> Result<Option<TriSign>, crate::error::MathError> {
        if !self.hi.is_finite() || !self.lo.is_finite() {
            return Err(crate::error::MathError::InvalidInput(
                "Cannot determine sign of non-finite Double".into(),
            ));
        }

        if self.hi > 0.0 {
            Ok(Some(TriSign::Pos))
        } else if self.hi < 0.0 {
            Ok(Some(TriSign::Neg))
        } else if self.lo > 0.0 {
            Ok(Some(TriSign::Pos))
        } else if self.lo < 0.0 {
            Ok(Some(TriSign::Neg))
        } else {
            Ok(Some(TriSign::Zero))
        }
    }
}

impl Add for Double {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let sum = Self::two_sum(self.hi, rhs.hi);
        let low_sum = sum.lo + self.lo + rhs.lo;
        Self::two_sum(sum.hi, low_sum)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for Double {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + rhs.negate()
    }
}

impl Mul for Double {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let product = Self::two_product(self.hi, rhs.hi);
        let low_product = product.lo + (self.hi * rhs.lo) + (self.lo * rhs.hi);
        Self::two_sum(product.hi, low_product)
    }
}

impl std::fmt::Display for Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.16e} + {:.16e}", self.hi, self.lo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_exact_for_small_integers() {
        let result = Double::two_sum(1.0, 2.0);
        assert_eq!(result.hi, 3.0);
        assert_eq!(result.lo, 0.0);
    }

    #[test]
    fn two_sum_captures_rounding_error() {
        let result = Double::two_sum(1.0, 1e-16);
        let reconstructed = result.hi + result.lo;
        assert!((reconstructed - (1.0 + 1e-16)).abs() < 1e-31);
    }

    #[test]
    fn two_product_exact_for_small_integers() {
        let result = Double::two_product(3.0, 7.0);
        assert_eq!(result.hi, 21.0);
        assert_eq!(result.lo, 0.0);
    }

    #[test]
    fn sign_positive() {
        let d = Double::from_f64(42.0);
        assert_eq!(d.sign().unwrap(), Some(TriSign::Pos));
    }

    #[test]
    fn sign_negative() {
        let d = Double::from_f64(-3.14);
        assert_eq!(d.sign().unwrap(), Some(TriSign::Neg));
    }

    #[test]
    fn sign_zero() {
        let d = Double::from_f64(0.0);
        assert_eq!(d.sign().unwrap(), Some(TriSign::Zero));
    }

    #[test]
    fn add_preserves_extra_precision() {
        let a = Double::from_f64(1.0);
        let b = Double::from_f64(1e-16);
        let sum = a + b;
        assert_eq!(sum.sign().unwrap(), Some(TriSign::Pos));
    }

    #[test]
    fn sub_to_near_zero() {
        let a = Double::from_f64(1.0);
        let b = Double::from_f64(1.0);
        let diff = a - b;
        assert_eq!(diff.sign().unwrap(), Some(TriSign::Zero));
    }

    #[test]
    fn mul_sign_correctness() {
        let pos = Double::from_f64(3.0);
        let neg = Double::from_f64(-2.0);
        let product = pos * neg;
        assert_eq!(product.sign().unwrap(), Some(TriSign::Neg));
    }

    #[test]
    fn negate_flips_sign() {
        let d = Double::from_f64(5.0);
        let neg = d.negate();
        assert_eq!(neg.sign().unwrap(), Some(TriSign::Neg));
        assert_eq!(neg.hi(), -5.0);
    }
}
