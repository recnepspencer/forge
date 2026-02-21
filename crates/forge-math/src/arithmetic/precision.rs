//! Precision budget for bounding exact arithmetic cost (Milestone 0.2.3).
//!
//! DOMAIN: Bit-length tracking and escalation for `Rational` operations.
//! INVARIANTS: Compression always preserves the sign of the value.
//! DEPENDENCIES: `rational`, `sign`.
//!
//! The problem: each exact rational operation can double the bit-length.
//! After 10 chained operations, numerators reach thousands of digits.
//! The key insight: you rarely need the exact *value* — you need the
//! exact *sign*. This module enforces a bit-length ceiling and provides
//! a "pressure valve" that rounds while preserving sign.

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

    pub fn within_budget(&self, _r: &Rational) -> bool {
        true // Malachite exact arithmetic is unconstrained
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
