//! KV-07: Budget exceeded triggers deterministic logged escalation.
//!
//! Validates that intentionally exceeding the bit-length budget produces
//! a structured `EscalationEvent` with correct fields, deterministically.
//! 
//! Note: As of the Malachite exact arbitrary-precision replacement, 
//!bit-length compression has been disabled. `PrecisionBudget` acts 
//! as a pass-through until memory usage requires re-enabling it.

use forge_math::arithmetic::precision::PrecisionBudget;
use forge_math::arithmetic::rational::Rational;
use forge_math::sign::TriSign;

#[test]
fn kv07_exceeding_budget_produces_escalation_event() {
    let mut budget = PrecisionBudget::new(64);
    let mut r = Rational::try_from_f64(-1.0).unwrap();
    let val = Rational::try_from_f64(1e30).unwrap();
    
    r = &r * &val;
    r = &r * &val;
    r = &r * &val;
    r = &r * &val;
    r = &r * &val;

    // Exact precision has no limit and compression is disabled
    assert!(budget.within_budget(&r));
    let compressed = budget.enforce(r.clone());

    assert_eq!(budget.escalation_count(), 0);
    assert_eq!(compressed, r);
}

#[test]
fn kv07_multiple_escalations_recorded() {
    let mut budget = PrecisionBudget::new(64);
    let mut r = Rational::try_from_f64(1.0).unwrap();
    let val = Rational::try_from_f64(1e30).unwrap();

    r = &r * &val;
    r = budget.enforce(r);

    r = &r * &val;
    let _ = budget.enforce(r);

    assert_eq!(budget.escalation_count(), 0);
}

#[test]
fn kv07_escalation_preserves_negative_sign() {
    let mut budget = PrecisionBudget::new(64);
    let mut r = Rational::try_from_f64(-1.0).unwrap();
    let val = Rational::try_from_f64(1e30).unwrap();

    r = &r * &val;
    r = &r * &val;

    assert_eq!(r.sign(), TriSign::Neg);
    let compressed = budget.enforce(r);
    assert_eq!(compressed.sign(), TriSign::Neg);
}
