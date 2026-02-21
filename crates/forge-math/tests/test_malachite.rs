use malachite::Rational;
use std::convert::TryFrom;

#[test]
fn test_malachite_rational() {
    let r = Rational::from(3);
    println!("Rational from int: {}", r);
    
    let rf = Rational::try_from(3.14159);
    match rf {
        Ok(v) => println!("Rational from f64: {}", v),
        Err(e) => println!("Error converting f64: {:?}", e),
    }

    let a = Rational::from(10);
    let b = Rational::from(20);
    println!("Add: {}", a + b);
}
