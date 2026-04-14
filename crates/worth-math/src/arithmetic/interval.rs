//! Interval arithmetic for Stage 1.5 evaluation (P2.1).
//!
//! DOMAIN: Conservative floating-point bounds for geometric predicates.
//! INVARIANTS: `lo <= hi` always. The true value is guaranteed to lie in `[lo, hi]`.
//! DEPENDENCIES: `sign` (TriSign).
//!
//! An `Interval` represents a range `[lo, hi]` that is guaranteed to contain
//! the true mathematical result. When the interval doesn't span zero, we can
//! certify the sign without resorting to exact arithmetic. This resolves
//! ~99% of cases that the fast f64 filter cannot.

use crate::sign::TriSign;
use std::ops::{Add, Mul, Neg, Sub};

/// Conservative floating-point interval `[lo, hi]`.
///
/// The true mathematical value is guaranteed to lie within `[lo, hi]`.
/// Arithmetic operations widen bounds conservatively to account for
/// IEEE 754 rounding. Sign determination succeeds when the interval
/// is entirely positive or entirely negative.
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    lo: f64,
    hi: f64,
}

/// Unit of least precision — the smallest representable difference
/// at a given magnitude. Used to conservatively widen interval bounds.
fn ulp(x: f64) -> f64 {
    let abs = x.abs();
    if abs == 0.0 {
        return f64::MIN_POSITIVE;
    }
    let bits = abs.to_bits();
    let next = f64::from_bits(bits + 1);
    next - abs
}

/// Widen a lower bound downward by one ULP (conservative).
fn widen_lo(x: f64) -> f64 {
    if x.is_infinite() || x.is_nan() {
        return x;
    }
    x - ulp(x)
}

/// Widen an upper bound upward by one ULP (conservative).
fn widen_hi(x: f64) -> f64 {
    if x.is_infinite() || x.is_nan() {
        return x;
    }
    x + ulp(x)
}

impl Interval {
    /// Create an interval from a single exact `f64` value.
    ///
    /// The resulting interval is `[v, v]` — zero width.
    pub fn from_f64(v: f64) -> Self {
        Self { lo: v, hi: v }
    }

    /// Create an interval with explicit bounds.
    ///
    /// Panics (debug) if `lo > hi`.
    pub fn from_bounds(lo: f64, hi: f64) -> Self {
        debug_assert!(lo <= hi, "Interval bounds inverted: {} > {}", lo, hi);
        Self { lo, hi }
    }

    /// Compute `a - b` as an interval, tracking rounding error.
    pub fn from_difference(a: f64, b: f64) -> Self {
        let diff = a - b;
        let err = ulp(diff);
        Self {
            lo: diff - err,
            hi: diff + err,
        }
    }

    /// The lower bound.
    pub fn lo(self) -> f64 {
        self.lo
    }

    /// The upper bound.
    pub fn hi(self) -> f64 {
        self.hi
    }

    /// The width of the interval (`hi - lo`).
    pub fn width(self) -> f64 {
        self.hi - self.lo
    }

    /// The midpoint of the interval.
    pub fn midpoint(self) -> f64 {
        (self.lo + self.hi) * 0.5
    }

    /// Whether the interval contains zero.
    pub fn contains_zero(self) -> bool {
        self.lo <= 0.0 && self.hi >= 0.0
    }

    /// Determine the sign of the interval.
    ///
    /// Returns `Some(Pos)` if entirely positive, `Some(Neg)` if entirely
    /// negative, `Some(Zero)` if the interval is exactly `[0, 0]`, or
    /// `None` if the interval spans zero (inconclusive).
    pub fn sign(self) -> Option<TriSign> {
        if self.lo > 0.0 {
            Some(TriSign::Pos)
        } else if self.hi < 0.0 {
            Some(TriSign::Neg)
        } else if self.lo == 0.0 && self.hi == 0.0 {
            Some(TriSign::Zero)
        } else {
            None
        }
    }

    /// Compute `a - b` with error bounds relative to `scale` (P2.4).
    ///
    /// At large coordinate magnitudes, `a - b` suffers catastrophic
    /// cancellation. This method widens bounds based on the scale of
    /// the operands rather than the difference, preventing false
    /// narrowing at extreme scales.
    pub fn from_scaled_difference(a: f64, b: f64, scale: f64) -> Self {
        let diff = a - b;
        let err_diff = ulp(diff);
        let err_scale = ulp(scale) * 2.0;
        let err = err_diff.max(err_scale);
        Self {
            lo: diff - err,
            hi: diff + err,
        }
    }

    /// Whether the interval bounds are near the subnormal range.
    ///
    /// Returns `true` when bounds approach `f64::MIN_POSITIVE`,
    /// indicating potential underflow in subsequent operations.
    pub fn is_underflow_risk(self) -> bool {
        let threshold = f64::MIN_POSITIVE * 1e10;
        (self.lo.abs() < threshold && self.lo != 0.0)
            || (self.hi.abs() < threshold && self.hi != 0.0)
    }

    /// Whether the interval bounds are near `f64::MAX`.
    ///
    /// Returns `true` when bounds exceed 1e300, indicating
    /// potential overflow in subsequent operations.
    pub fn is_overflow_risk(self) -> bool {
        self.lo.abs() > 1e300 || self.hi.abs() > 1e300
    }
}

impl Add for Interval {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            lo: widen_lo(self.lo + rhs.lo),
            hi: widen_hi(self.hi + rhs.hi),
        }
    }
}

impl Sub for Interval {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            lo: widen_lo(self.lo - rhs.hi),
            hi: widen_hi(self.hi - rhs.lo),
        }
    }
}

impl Mul for Interval {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let products = [
            self.lo * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.lo,
            self.hi * rhs.hi,
        ];
        let lo = products.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = products.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self {
            lo: widen_lo(lo),
            hi: widen_hi(hi),
        }
    }
}

impl Neg for Interval {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            lo: -self.hi,
            hi: -self.lo,
        }
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.6e}, {:.6e}]", self.lo, self.hi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_f64_zero_width() {
        let iv = Interval::from_f64(42.0);
        assert_eq!(iv.lo(), 42.0);
        assert_eq!(iv.hi(), 42.0);
        assert_eq!(iv.width(), 0.0);
    }

    #[test]
    fn sign_positive() {
        let iv = Interval::from_bounds(1.0, 2.0);
        assert_eq!(iv.sign(), Some(TriSign::Pos));
    }

    #[test]
    fn sign_negative() {
        let iv = Interval::from_bounds(-3.0, -1.0);
        assert_eq!(iv.sign(), Some(TriSign::Neg));
    }

    #[test]
    fn sign_zero() {
        let iv = Interval::from_f64(0.0);
        assert_eq!(iv.sign(), Some(TriSign::Zero));
    }

    #[test]
    fn sign_inconclusive() {
        let iv = Interval::from_bounds(-1e-15, 1e-15);
        assert_eq!(iv.sign(), None);
    }

    #[test]
    fn add_containment() {
        let a = Interval::from_f64(1.0);
        let b = Interval::from_f64(2.0);
        let sum = a + b;
        assert!(sum.lo() <= 3.0);
        assert!(sum.hi() >= 3.0);
    }

    #[test]
    fn sub_containment() {
        let a = Interval::from_f64(5.0);
        let b = Interval::from_f64(3.0);
        let diff = a - b;
        assert!(diff.lo() <= 2.0);
        assert!(diff.hi() >= 2.0);
    }

    #[test]
    fn mul_containment() {
        let a = Interval::from_f64(3.0);
        let b = Interval::from_f64(7.0);
        let product = a * b;
        assert!(product.lo() <= 21.0);
        assert!(product.hi() >= 21.0);
    }

    #[test]
    fn mul_mixed_signs() {
        let a = Interval::from_bounds(-2.0, 3.0);
        let b = Interval::from_bounds(-1.0, 4.0);
        let product = a * b;
        assert!(product.lo() <= -8.0);
        assert!(product.hi() >= 12.0);
    }

    #[test]
    fn neg_flips_bounds() {
        let iv = Interval::from_bounds(1.0, 5.0);
        let neg = -iv;
        assert_eq!(neg.lo(), -5.0);
        assert_eq!(neg.hi(), -1.0);
    }

    #[test]
    fn contains_zero_spanning() {
        assert!(Interval::from_bounds(-1.0, 1.0).contains_zero());
        assert!(!Interval::from_bounds(1.0, 2.0).contains_zero());
        assert!(!Interval::from_bounds(-2.0, -1.0).contains_zero());
    }

    #[test]
    fn from_difference_contains_true_value() {
        let a = 1.0;
        let b = 1.0 - 1e-16;
        let iv = Interval::from_difference(a, b);
        let true_diff = a - b;
        assert!(iv.lo() <= true_diff);
        assert!(iv.hi() >= true_diff);
    }
}
