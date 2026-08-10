use super::Rational;
use crate::sign::TriSign;

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
    assert_eq!(
        Rational::try_from_f64(3.0).unwrap(),
        Rational::from_integer(3)
    );
}

#[test]
fn f64_exact_conversion_half() {
    assert_eq!(
        Rational::try_from_f64(0.5).unwrap(),
        Rational::try_from_fraction(1, 2).unwrap()
    );
}

#[test]
fn f64_exact_conversion_negative() {
    assert_eq!(
        Rational::try_from_f64(-2.5).unwrap(),
        Rational::try_from_fraction(-5, 2).unwrap()
    );
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

#[test]
fn serde_round_trip() {
    let r = Rational::try_from_fraction(3, 7).unwrap();
    let json = serde_json::to_string(&r).unwrap();
    let r2: Rational = serde_json::from_str(&json).unwrap();
    assert_eq!(r, r2);
}

#[test]
fn to_f64_approx_accuracy() {
    let r = Rational::try_from_fraction(1, 3).unwrap();
    let approx = r.to_f64_approx();
    assert!((approx - 1.0 / 3.0).abs() < 1e-15);
}
