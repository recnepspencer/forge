use malachite::Rational;
use malachite::num::conversion::traits::RoundingFrom;
use malachite::rounding_modes::RoundingMode;
fn main() {
    let r = 0.8660254037844386f64;
    let rat = Rational::try_from(r).unwrap();
    let d = &rat * &rat;
    let f = f64::rounding_from(&d, RoundingMode::Nearest).0;
    println!("f: {:?}", f);
}
