//! Three-stage filtered evaluation trait for geometric predicates.
//!
//! The [`FilteredEval`] trait defines a cascade of increasing precision:
//!
//! 1. **Stage 1 (f64):** Fast hardware arithmetic with Shewchuk-style error bounds.
//!    Resolves >95% of cases.
//! 2. **Stage 2 (Double-double):** Compensated arithmetic with ~106-bit precision.
//!    Resolves >99% of remaining cases.
//! 3. **Stage 3 (Rational):** Exact BigInt arithmetic. Resolves everything.
//!
//! Callers use [`FilteredEval::evaluate`] — they never pick a stage manually.
//!
//! # Reference
//!
//! Shewchuk, "Adaptive Precision Floating-Point Arithmetic and Fast Robust
//! Geometric Predicates," Discrete & Computational Geometry, 1997.

use crate::sign::{CertifiedTriSign, TriSign};

/// Three-stage filtered evaluation for geometric predicates.
///
/// Implementors define how to compute a predicate's sign at each precision
/// level. The [`evaluate`](FilteredEval::evaluate) method cascades through
/// stages, stopping as soon as the sign is resolved.
pub trait FilteredEval {
    /// The input type for this predicate.
    type Input;

    /// Stage 1: fast `f64` evaluation with error bounds.
    fn eval_f64(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError>;

    /// Stage 2: compensated double-double arithmetic (~106-bit precision).
    fn eval_double(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError>;

    /// Stage 3: exact rational arithmetic. Always resolves.
    fn eval_exact(&self, input: &Self::Input) -> Result<TriSign, crate::error::MathError>;

    /// Run the full filter cascade, returning a certified result.
    fn evaluate(&self, input: &Self::Input) -> Result<CertifiedTriSign, crate::error::MathError> {
        if let Some(sign) = self.eval_f64(input)? {
            return Ok(CertifiedTriSign::new(sign));
        }
        if let Some(sign) = self.eval_double(input)? {
            return Ok(CertifiedTriSign::new(sign));
        }
        Ok(CertifiedTriSign::new(self.eval_exact(input)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trivial predicate for testing the cascade: sign of a single number.
    struct SignOf;

    impl FilteredEval for SignOf {
        type Input = f64;

        fn eval_f64(&self, input: &f64) -> Result<Option<TriSign>, crate::error::MathError> {
            const TEST_TOLERANCE: f64 = 1e-10;
            if input.abs() <= TEST_TOLERANCE {
                return Ok(None);
            }
            if *input > 0.0 {
                Ok(Some(TriSign::Pos))
            } else {
                Ok(Some(TriSign::Neg))
            }
        }

        fn eval_double(&self, input: &f64) -> Result<Option<TriSign>, crate::error::MathError> {
            if *input == 0.0 {
                return Ok(None);
            }
            if *input > 0.0 {
                Ok(Some(TriSign::Pos))
            } else {
                Ok(Some(TriSign::Neg))
            }
        }

        fn eval_exact(&self, input: &f64) -> Result<TriSign, crate::error::MathError> {
            if *input > 0.0 {
                Ok(TriSign::Pos)
            } else if *input < 0.0 {
                Ok(TriSign::Neg)
            } else {
                Ok(TriSign::Zero)
            }
        }
    }

    #[test]
    fn cascade_resolves_at_stage1_for_clear_sign() {
        assert_eq!(SignOf.evaluate(&42.0).unwrap().sign(), TriSign::Pos);
    }

    #[test]
    fn cascade_resolves_at_stage1_for_clear_negative() {
        assert_eq!(SignOf.evaluate(&-100.0).unwrap().sign(), TriSign::Neg);
    }

    #[test]
    fn cascade_falls_through_to_stage2() {
        assert_eq!(SignOf.evaluate(&1e-15).unwrap().sign(), TriSign::Pos);
    }

    #[test]
    fn cascade_reaches_exact_for_zero() {
        assert_eq!(SignOf.evaluate(&0.0).unwrap().sign(), TriSign::Zero);
    }
}
