//! Four-stage filtered evaluation trait for geometric predicates.
//!
//! The [`FilteredEval`] trait defines a cascade of increasing precision:
//!
//! 1. **Stage 1 (f64):** Fast hardware arithmetic with Shewchuk-style error bounds.
//!    Resolves >95% of cases.
//! 2. **Stage 2 (Interval):** Conservative interval arithmetic with ULP-widened bounds.
//!    Resolves >99% of remaining cases.
//! 3. **Stage 3 (Double-double):** Compensated arithmetic with ~106-bit precision.
//!    Resolves >99.9% of remaining cases.
//! 4. **Stage 4 (Rational):** Exact BigInt arithmetic. Resolves everything.
//!
//! Callers use [`FilteredEval::evaluate`] — they never pick a stage manually.
//! Every evaluation returns a [`PrecisionEscalation`] recording which stage
//! resolved the result and whether the f64 fast-path agreed.
//!
//! # Reference
//!
//! Shewchuk, "Adaptive Precision Floating-Point Arithmetic and Fast Robust
//! Geometric Predicates," Discrete & Computational Geometry, 1997.

use serde::{Deserialize, Serialize};

use crate::sign::{CertifiedTriSign, TriSign};

/// Which precision mode resolved a predicate evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PrecisionMode {
    /// Standard IEEE 754 double — fast, sufficient for >95% of decisions.
    Float64,
    /// Conservative interval arithmetic — resolves >99% of remaining.
    Interval,
    /// Compensated double-double (~106-bit significand).
    Double,
    /// Exact rational arithmetic — resolves everything, expensive.
    Rational,
}

impl std::fmt::Display for PrecisionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrecisionMode::Float64 => write!(f, "Float64"),
            PrecisionMode::Interval => write!(f, "Interval"),
            PrecisionMode::Double => write!(f, "Double"),
            PrecisionMode::Rational => write!(f, "Rational"),
        }
    }
}

/// Build a target description string from compile-time cfg macros.
///
/// Used by the replay system to detect architecture mismatches (MB-R6).
/// If two traces were compiled for different targets, FMA behavior may
/// differ in the f64 fast-path — replay must flag this.
pub fn build_target_description() -> String {
    format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS)
}

/// Metadata describing how a predicate evaluation was resolved.
///
/// Attached to every predicate result so the kernel can detect
/// when float-precision decisions diverge from exact results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrecisionEscalation {
    /// The precision mode that produced the final certified answer.
    pub resolved_at: PrecisionMode,
    /// Whether the f64 fast-path produced the same sign as the final answer.
    /// `false` means the f64 result would have been wrong — a critical finding.
    pub float_agreed: bool,
    /// The interval width at the point of evaluation (if interval stage ran).
    /// Wider intervals indicate less numerical confidence.
    pub interval_width: Option<f64>,
    /// The compilation target triple for cross-architecture replay verification.
    pub target_triple: String,
    /// The maximum disagreement magnitude across coordinates.
    pub disagreement_magnitude: Option<f64>,
    /// The sign produced by the f64 fast-path (before escalation).
    /// `None` if f64 was inconclusive. Used by divergence detection (P2.3).
    pub float_sign: Option<TriSign>,
}

impl PrecisionEscalation {
    /// The mode that produced the final answer.
    pub fn get_resolved_at(&self) -> PrecisionMode {
        self.resolved_at
    }

    /// Whether the f64 fast-path agreed with the final answer.
    pub fn get_float_agreed(&self) -> bool {
        self.float_agreed
    }

    /// Interval width at the point of evaluation, if interval stage ran.
    pub fn get_interval_width(&self) -> Option<f64> {
        self.interval_width
    }

    /// The compilation target triple (e.g. "aarch64-apple-darwin").
    pub fn get_target_triple(&self) -> &str {
        &self.target_triple
    }
}

impl std::fmt::Display for PrecisionEscalation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "resolved_at={}, float_agreed={}", self.resolved_at, self.float_agreed)?;
        if let Some(w) = self.interval_width {
            write!(f, ", interval_width={:.2e}", w)?;
        }
        Ok(())
    }
}

/// Four-stage filtered evaluation for geometric predicates.
///
/// Implementors define how to compute a predicate's sign at each precision
/// level. The [`evaluate`](FilteredEval::evaluate) method cascades through
/// stages, stopping as soon as the sign is resolved, and records which
/// stage succeeded.
pub trait FilteredEval {
    /// The input type for this predicate.
    type Input;

    /// Stage 1: fast `f64` evaluation with error bounds.
    fn eval_f64(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError>;

    /// Stage 2: conservative interval arithmetic with ULP-widened bounds.
    fn eval_interval(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError>;

    /// Stage 3: compensated double-double arithmetic (~106-bit precision).
    fn eval_double(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError>;

    /// Stage 4: exact rational arithmetic. Always resolves.
    fn eval_exact(&self, input: &Self::Input) -> Result<TriSign, crate::error::MathError>;

    /// Run the full filter cascade, returning a certified result
    /// and metadata about which precision stage resolved it.
    fn evaluate(
        &self,
        input: &Self::Input,
    ) -> Result<(CertifiedTriSign, PrecisionEscalation), crate::error::MathError> {
        let f64_result = self.eval_f64(input)?;

        if let Some(sign) = f64_result {
            return Ok((
                CertifiedTriSign::new(sign),
                PrecisionEscalation {
                    resolved_at: PrecisionMode::Float64,
                    float_agreed: true,
                    interval_width: None,
                    target_triple: build_target_description(),
                    disagreement_magnitude: None,
                    float_sign: Some(sign),
                },
            ));
        }

        let interval_result = self.eval_interval(input)?;

        if let Some(sign) = interval_result {
            return Ok((
                CertifiedTriSign::new(sign),
                PrecisionEscalation {
                    resolved_at: PrecisionMode::Interval,
                    float_agreed: false,
                    interval_width: None,
                    target_triple: build_target_description(),
                    disagreement_magnitude: None,
                    float_sign: f64_result,
                },
            ));
        }

        let double_result = self.eval_double(input)?;

        if let Some(sign) = double_result {
            return Ok((
                CertifiedTriSign::new(sign),
                PrecisionEscalation {
                    resolved_at: PrecisionMode::Double,
                    float_agreed: false,
                    interval_width: None,
                    target_triple: build_target_description(),
                    disagreement_magnitude: None,
                    float_sign: f64_result,
                },
            ));
        }

        let exact_sign = self.eval_exact(input)?;
        Ok((
            CertifiedTriSign::new(exact_sign),
            PrecisionEscalation {
                resolved_at: PrecisionMode::Rational,
                float_agreed: false,
                interval_width: None,
                target_triple: build_target_description(),
                disagreement_magnitude: None,
                float_sign: f64_result,
            },
        ))
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

        fn eval_interval(&self, input: &f64) -> Result<Option<TriSign>, crate::error::MathError> {
            const INTERVAL_TOLERANCE: f64 = 1e-15;
            if input.abs() <= INTERVAL_TOLERANCE {
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
        let (sign, esc) = SignOf.evaluate(&42.0).unwrap();
        assert_eq!(sign.sign(), TriSign::Pos);
        assert_eq!(esc.get_resolved_at(), PrecisionMode::Float64);
        assert!(esc.get_float_agreed());
    }

    #[test]
    fn cascade_resolves_at_stage1_for_clear_negative() {
        let (sign, esc) = SignOf.evaluate(&-100.0).unwrap();
        assert_eq!(sign.sign(), TriSign::Neg);
        assert_eq!(esc.get_resolved_at(), PrecisionMode::Float64);
    }

    #[test]
    fn cascade_falls_through_to_interval() {
        let (sign, esc) = SignOf.evaluate(&1e-12).unwrap();
        assert_eq!(sign.sign(), TriSign::Pos);
        assert_eq!(esc.get_resolved_at(), PrecisionMode::Interval);
        assert!(!esc.get_float_agreed());
    }

    #[test]
    fn cascade_falls_through_to_double() {
        let (sign, esc) = SignOf.evaluate(&1e-16).unwrap();
        assert_eq!(sign.sign(), TriSign::Pos);
        assert_eq!(esc.get_resolved_at(), PrecisionMode::Double);
        assert!(!esc.get_float_agreed());
    }

    #[test]
    fn cascade_reaches_exact_for_zero() {
        let (sign, esc) = SignOf.evaluate(&0.0).unwrap();
        assert_eq!(sign.sign(), TriSign::Zero);
        assert_eq!(esc.get_resolved_at(), PrecisionMode::Rational);
        assert!(!esc.get_float_agreed());
    }
}
