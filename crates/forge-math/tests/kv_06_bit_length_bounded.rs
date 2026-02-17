//! KV-06: Sequential exact operations keep bit-length bounded.
//!
//! Validates that applying `PrecisionBudget::enforce` after each operation
//! prevents runaway bit-growth while preserving signs.

use forge_math::precision::PrecisionBudget;
use forge_math::rational::Rational;

#[test]
fn kv06_multiplications_stay_bounded() {
    let mut budget = PrecisionBudget::new(256);
    let mut r = Rational::try_from_f64(std::f64::consts::PI).unwrap();
    let val = Rational::try_from_f64(std::f64::consts::E).unwrap();

    r = &r * &val;
    r = budget.enforce(r);
    assert!(r.bit_length() <= 256);

    r = &r * &val;
    r = budget.enforce(r);
    assert!(r.bit_length() <= 256);
}

#[test]
fn kv06_divisions_stay_bounded() {
    let mut budget = PrecisionBudget::new(256);
    let mut r = Rational::try_from_f64(1.0 / 3.0).unwrap();
    let val = Rational::try_from_f64(1.0 / 7.0).unwrap();

    r = &r * &val;
    r = budget.enforce(r);
    assert!(r.bit_length() <= 256);

    r = &r * &val;
    r = budget.enforce(r);
    assert!(r.bit_length() <= 256);
}

#[test]
fn kv06_sign_preserved_across_compressions() {
    let mut budget = PrecisionBudget::new(128);
    let mut r = Rational::try_from_f64(-std::f64::consts::PI).unwrap();
    let val = Rational::try_from_f64(std::f64::consts::E).unwrap();

    let sign_before = r.sign();
    r = &r * &val;
    r = budget.enforce(r);
    assert_eq!(sign_before, r.sign());

    r = &r * &val;
    r = budget.enforce(r);
    assert_eq!(sign_before, r.sign());

    r = budget.enforce(r);
    assert!(budget.within_budget(&r));

    r = &r * &Rational::try_from_f64(std::f64::consts::PI).unwrap();
    r = budget.enforce(r);
    assert_eq!(sign_before, r.sign());
}

#[test]
fn kv06_small_values_never_trigger_escalation() {
    let mut budget = PrecisionBudget::new(512);
    let r = Rational::from_integer(42);
    let result = budget.enforce(r.clone());
    assert_eq!(result, r);
    assert_eq!(budget.escalation_count(), 0);
}

#[test]
fn kv06_zero_survives_compression() {
    let mut budget = PrecisionBudget::new(64);
    let r = Rational::zero();
    let result = budget.enforce(r);
    assert!(result.is_zero());
    assert_eq!(budget.escalation_count(), 0);
}
