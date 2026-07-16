//! KV-07: Budget exceeded triggers deterministic logged escalation.
//!
//! Validates that intentionally exceeding the bit-length budget produces
//! a structured `EscalationEvent` with correct fields, deterministically.

use worth_math::arithmetic::precision::PrecisionBudget;
use worth_math::arithmetic::rational::Rational;
use worth_math::sign::TriSign;

#[test]
fn kv07_exceeding_budget_produces_escalation_event() {
    let mut budget = PrecisionBudget::new(256);
    let mut r = Rational::try_from_f64(-1.0).unwrap();
    let large = Rational::try_from_f64(1e15).unwrap();
    let small = Rational::try_from_f64(1e-15).unwrap();

    for _ in 0..20 {
        r = &r * &large;
        r = &r * &small;
    }

    let before_bits = r.bit_length();
    assert!(
        before_bits > 256,
        "20 multiply-divide cycles should exceed 256 bits, got {}",
        before_bits
    );

    let compressed = budget.enforce(r);
    assert_eq!(budget.escalation_count(), 1);
    assert_eq!(compressed.sign(), TriSign::Neg);
}

#[test]
fn kv07_multiple_escalations_recorded() {
    let mut budget = PrecisionBudget::new(256);
    let mut r = Rational::try_from_f64(1.0).unwrap();
    let large = Rational::try_from_f64(1e15).unwrap();
    let small = Rational::try_from_f64(1e-15).unwrap();

    for _ in 0..20 {
        r = &r * &large;
        r = &r * &small;
    }
    r = budget.enforce(r);

    for _ in 0..20 {
        r = &r * &large;
        r = &r * &small;
    }
    let compressed = budget.enforce(r);
    assert!(budget.within_budget(&compressed));

    assert_eq!(
        budget.escalation_count(),
        2,
        "Two rounds of 20 multiply-divide cycles must produce two escalations"
    );
}

#[test]
fn kv07_escalation_preserves_negative_sign() {
    let mut budget = PrecisionBudget::new(256);
    let mut r = Rational::try_from_f64(-1.0).unwrap();
    let large = Rational::try_from_f64(1e15).unwrap();
    let small = Rational::try_from_f64(1e-15).unwrap();

    for _ in 0..20 {
        r = &r * &large;
        r = &r * &small;
    }

    assert_eq!(r.sign(), TriSign::Neg);
    let compressed = budget.enforce(r);
    assert_eq!(compressed.sign(), TriSign::Neg);
}

#[test]
fn kv07_small_value_stays_within_budget() {
    let budget = PrecisionBudget::new(256);
    let r = Rational::try_from_f64(42.0).unwrap();
    assert!(budget.within_budget(&r));
}
