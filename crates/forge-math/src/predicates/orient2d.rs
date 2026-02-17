//! 2D orientation predicate.

use crate::double::Double;
use crate::filter::FilteredEval;
use crate::rational::Rational;
use crate::sign::{CertifiedTriSign, TriSign};
use super::ORIENT2D_ERR_BOUND_A;

/// 2D orientation predicate: sign of the cross-product `(b-a) × (c-a)`.
///
/// Returns `Pos` for counter-clockwise, `Neg` for clockwise, `Zero` for collinear.
struct Orient2dPredicate;

/// Input to [`orient2d`]: three 2D points.
pub type Orient2dInput = ([f64; 2], [f64; 2], [f64; 2]);

impl FilteredEval for Orient2dPredicate {
    type Input = Orient2dInput;

    fn eval_f64(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError> {
        let (a, b, c) = input;

        let acx = a[0] - c[0];
        let bcx = b[0] - c[0];
        let acy = a[1] - c[1];
        let bcy = b[1] - c[1];

        let det = acx * bcy - acy * bcx;
        let det_bound = acx.abs() * bcy.abs() + acy.abs() * bcx.abs();
        let err = ORIENT2D_ERR_BOUND_A * det_bound;

        if !det.is_finite() {
            return Err(crate::error::MathError::InvalidInput(
                "Non-finite determinant in orient2d".into(),
            ));
        }

        if det > err {
            Ok(Some(TriSign::Pos))
        } else if det < -err {
            Ok(Some(TriSign::Neg))
        } else {
            Ok(None)
        }
    }

    fn eval_double(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError> {
        let (a, b, c) = input;

        let acx = Double::two_sum(a[0], -c[0]);
        let bcx = Double::two_sum(b[0], -c[0]);
        let acy = Double::two_sum(a[1], -c[1]);
        let bcy = Double::two_sum(b[1], -c[1]);

        let det = (acx * bcy) - (acy * bcx);
        det.sign()
    }

    fn eval_exact(&self, input: &Self::Input) -> Result<TriSign, crate::error::MathError> {
        let (a, b, c) = input;

        let acx = Rational::try_from_f64(a[0])? - Rational::try_from_f64(c[0])?;
        let bcx = Rational::try_from_f64(b[0])? - Rational::try_from_f64(c[0])?;
        let acy = Rational::try_from_f64(a[1])? - Rational::try_from_f64(c[1])?;
        let bcy = Rational::try_from_f64(b[1])? - Rational::try_from_f64(c[1])?;

        let det = &acx * &bcy - &acy * &bcx;
        Ok(det.sign())
    }
}

/// Compute the 2D orientation of three points.
///
/// Returns a [`CertifiedTriSign`]:
/// - `Pos`: counter-clockwise
/// - `Neg`: clockwise
/// - `Zero`: exactly collinear
///
/// This is the sign of the 2×2 determinant:
/// ```text
/// | ax-cx  ay-cy |
/// | bx-cx  by-cy |
/// ```
pub fn orient2d(
    a: [f64; 2],
    b: [f64; 2],
    c: [f64; 2],
) -> Result<CertifiedTriSign, crate::error::MathError> {
    Orient2dPredicate.evaluate(&(a, b, c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sign::TriSign;

    #[test]
    fn orient2d_counter_clockwise() {
        let result = orient2d([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]).unwrap();
        assert_eq!(result.sign(), TriSign::Pos);
    }

    #[test]
    fn orient2d_clockwise() {
        let result = orient2d([0.0, 0.0], [0.0, 1.0], [1.0, 0.0]).unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn orient2d_collinear() {
        assert_eq!(
            orient2d([0.0, 0.0], [1.0, 1.0], [2.0, 2.0]).unwrap().sign(),
            TriSign::Zero
        );
    }

    #[test]
    fn orient2d_collinear_on_x_axis() {
        assert_eq!(
            orient2d([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]).unwrap().sign(),
            TriSign::Zero
        );
    }

    #[test]
    fn orient2d_collinear_on_y_axis() {
        assert_eq!(
            orient2d([0.0, 0.0], [0.0, 1.0], [0.0, 2.0]).unwrap().sign(),
            TriSign::Zero
        );
    }

    #[test]
    fn orient2d_near_collinear_positive() {
        let result = orient2d([0.0, 0.0], [1.0, 0.0], [0.5, 1e-15]).unwrap();
        assert_eq!(result.sign(), TriSign::Pos);
    }

    #[test]
    fn orient2d_near_collinear_negative() {
        let result = orient2d([0.0, 0.0], [1.0, 0.0], [0.5, -1e-15]).unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn orient2d_is_deterministic() {
        let a = [0.1, 0.2];
        let b = [0.3, 0.4];
        let c = [0.5, 0.7];
        assert_eq!(
            orient2d(a, b, c).unwrap().sign(),
            orient2d(a, b, c).unwrap().sign()
        );
    }
}
