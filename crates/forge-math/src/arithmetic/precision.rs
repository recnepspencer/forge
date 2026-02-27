//! Precision tracking for geometric predicates and exact arithmetic.
//!
//! DOMAIN: Precision mode reporting, escalation metadata, and bit-length budgets.
//! INVARIANTS: PrecisionMode ordering reflects increasing computational cost.
//! DEPENDENCIES: `rational`, `sign`.
//!
//! This module provides:
//! - [`PrecisionMode`]: Which precision stage resolved a predicate.
//! - [`PrecisionEscalation`]: Full metadata about how a predicate was evaluated.
//! - [`PrecisionBudget`]: Bit-length tracking for exact `Rational` operations.

use serde::{Deserialize, Serialize};

use crate::sign::TriSign;

/// Which precision stage resolved a predicate evaluation.
///
/// Ordered by increasing computational cost; `Float64` is cheapest,
/// `ExactRational` is most expensive. This ordering is used by
/// divergence detection to identify the "hardest" decision in a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PrecisionMode {
    /// Shewchuk Stage A: standard IEEE 754 with error bounds.
    Float64,
    /// Shewchuk Stage B: first adaptive expansion refinement.
    ExpansionB,
    /// Shewchuk Stage C: full expansion with tail corrections.
    ExpansionC,
    /// Exact rational arithmetic (BigInt). Used as fallback for
    /// non-predicate computations (e.g. exact vertex positions).
    ExactRational,
}

impl std::fmt::Display for PrecisionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrecisionMode::Float64 => write!(f, "Float64"),
            PrecisionMode::ExpansionB => write!(f, "ExpansionB"),
            PrecisionMode::ExpansionC => write!(f, "ExpansionC"),
            PrecisionMode::ExactRational => write!(f, "ExactRational"),
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
    /// Number of f64 components in the expansion that resolved the result.
    /// `None` if resolved at `Float64` stage (no expansion needed).
    pub expansion_length: Option<usize>,
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

    /// Number of expansion components, if expansion was used.
    pub fn get_expansion_length(&self) -> Option<usize> {
        self.expansion_length
    }

    /// The compilation target triple (e.g. "aarch64-apple-darwin").
    pub fn get_target_triple(&self) -> &str {
        &self.target_triple
    }
}

impl std::fmt::Display for PrecisionEscalation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "resolved_at={}, float_agreed={}",
            self.resolved_at, self.float_agreed
        )?;
        if let Some(len) = self.expansion_length {
            write!(f, ", expansion_len={}", len)?;
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PRECISION BUDGET (bit-length tracking for Rational operations)
// ═══════════════════════════════════════════════════════════════════════════

use crate::arithmetic::rational::Rational;

/// Tracks bit-length across a chain of exact operations.
///
/// When a `Rational` exceeds the configured threshold, the budget
/// records an [`EscalationEvent`] and can compress the value back
/// to manageable size while preserving its sign.
#[derive(Debug, Clone)]
pub struct PrecisionBudget {
    threshold: u32,
    escalations: Vec<EscalationEvent>,
}

/// A recorded precision escalation (Doctrine D2 traceability).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscalationEvent {
    /// Bit-length before compression.
    pub bit_length_before: u32,
    /// Bit-length after compression.
    pub bit_length_after: u32,
    /// The threshold that was exceeded.
    pub threshold: u32,
    /// Whether the sign was preserved through compression.
    pub sign_preserved: bool,
}

impl PrecisionBudget {
    /// Create a budget with the given bit-length threshold.
    pub fn new(threshold: u32) -> Self {
        Self {
            threshold,
            escalations: Vec::new(),
        }
    }

    /// Create a budget with the default 512-bit threshold.
    pub fn default_budget() -> Self {
        Self::new(512)
    }

    /// The configured bit-length threshold.
    pub fn threshold(&self) -> u32 {
        self.threshold
    }

    /// Whether the given rational fits within the bit-length budget.
    pub fn within_budget(&self, r: &Rational) -> bool {
        r.bit_length() <= self.threshold
    }

    /// Enforce the budget: if the value exceeds the threshold, compress it.
    ///
    /// Returns the (possibly compressed) value. Records an [`EscalationEvent`]
    /// if compression occurred. Sign is always preserved.
    pub fn enforce(&mut self, r: Rational) -> Rational {
        if self.within_budget(&r) {
            return r;
        }

        let bit_length_before = r.bit_length();
        let sign_before = r.sign();
        let compressed = r.compress(self.threshold);
        let bit_length_after = compressed.bit_length();
        let sign_after = compressed.sign();

        self.escalations.push(EscalationEvent {
            bit_length_before,
            bit_length_after,
            threshold: self.threshold,
            sign_preserved: sign_before == sign_after,
        });

        compressed
    }

    /// All escalation events recorded by this budget.
    pub fn escalations(&self) -> &[EscalationEvent] {
        &self.escalations
    }

    /// Number of escalation events.
    pub fn escalation_count(&self) -> usize {
        self.escalations.len()
    }
}

impl Default for PrecisionBudget {
    fn default() -> Self {
        Self::default_budget()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_rational_within_budget() {
        let budget = PrecisionBudget::new(512);
        let r = Rational::from_integer(42);
        assert!(budget.within_budget(&r));
    }

    #[test]
    fn enforce_does_not_compress_small_values() {
        let mut budget = PrecisionBudget::new(512);
        let r = Rational::from_integer(42);
        let result = budget.enforce(r.clone());
        assert_eq!(result, r);
        assert_eq!(budget.escalation_count(), 0);
    }

    #[test]
    fn default_budget_is_512_bits() {
        let budget = PrecisionBudget::default();
        assert_eq!(budget.threshold(), 512);
    }
}
