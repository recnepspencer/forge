//! 3D orientation predicate.
//!
//! DOMAIN: Determine if point `d` is above, on, or below the plane defined by `a, b, c`.
//! INVARIANTS: Exact arithmetic always resolves. Filtered stages never lie.
//! DEPENDENCIES: `Double`, `Interval`, `Rational`, `FilteredEval`, `CertifiedTriSign`.

use crate::arithmetic::double::Double;
use crate::arithmetic::filter::{FilteredEval, PrecisionEscalation};
use crate::arithmetic::interval::Interval;
use crate::arithmetic::rational::Rational;
use crate::sign::{CertifiedTriSign, TriSign};
use super::ORIENT3D_ERR_BOUND_A;

/// 3D orientation predicate: sign of the 3×3 determinant.
///
/// Returns `Pos` if `d` is above the plane `abc` (using the right-hand rule),
/// `Neg` if below, `Zero` if exactly on the plane.
pub(crate) struct Orient3dPredicate;

/// Input to [`orient3d`]: four 3D points.
pub type Orient3dInput = ([f64; 3], [f64; 3], [f64; 3], [f64; 3]);

impl FilteredEval for Orient3dPredicate {
    type Input = Orient3dInput;

    fn eval_f64(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError> {
        let (a, b, c, d) = input;

        let adx = a[0] - d[0];
        let bdx = b[0] - d[0];
        let cdx = c[0] - d[0];
        let ady = a[1] - d[1];
        let bdy = b[1] - d[1];
        let cdy = c[1] - d[1];
        let adz = a[2] - d[2];
        let bdz = b[2] - d[2];
        let cdz = c[2] - d[2];

        let det = adx * (bdy * cdz - bdz * cdy)
                - bdx * (ady * cdz - adz * cdy)
                + cdx * (ady * bdz - adz * bdy);

        let permanent = (adx.abs() * (bdy.abs() * cdz.abs() + bdz.abs() * cdy.abs())
            + bdx.abs() * (ady.abs() * cdz.abs() + adz.abs() * cdy.abs())
            + cdx.abs() * (ady.abs() * bdz.abs() + adz.abs() * bdy.abs()));

        let err = ORIENT3D_ERR_BOUND_A * permanent;

        if !det.is_finite() {
            return Err(crate::error::MathError::InvalidInput(
                "Non-finite determinant in orient3d".into(),
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

    fn eval_interval(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError> {
        let (a, b, c, d) = input;

        let adx = Interval::from_difference(a[0], d[0]);
        let bdx = Interval::from_difference(b[0], d[0]);
        let cdx = Interval::from_difference(c[0], d[0]);
        let ady = Interval::from_difference(a[1], d[1]);
        let bdy = Interval::from_difference(b[1], d[1]);
        let cdy = Interval::from_difference(c[1], d[1]);
        let adz = Interval::from_difference(a[2], d[2]);
        let bdz = Interval::from_difference(b[2], d[2]);
        let cdz = Interval::from_difference(c[2], d[2]);

        let det = adx * (bdy * cdz - bdz * cdy)
                - bdx * (ady * cdz - adz * cdy)
                + cdx * (ady * bdz - adz * bdy);

        Ok(det.sign())
    }

    fn eval_double(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError> {
        let (a, b, c, d) = input;

        let adx = Double::two_sum(a[0], -d[0]);
        let bdx = Double::two_sum(b[0], -d[0]);
        let cdx = Double::two_sum(c[0], -d[0]);
        let ady = Double::two_sum(a[1], -d[1]);
        let bdy = Double::two_sum(b[1], -d[1]);
        let cdy = Double::two_sum(c[1], -d[1]);
        let adz = Double::two_sum(a[2], -d[2]);
        let bdz = Double::two_sum(b[2], -d[2]);
        let cdz = Double::two_sum(c[2], -d[2]);

        let det = adx * (bdy * cdz - bdz * cdy)
                - bdx * (ady * cdz - adz * cdy)
                + cdx * (ady * bdz - adz * bdy);

        det.sign()
    }

    fn eval_exact(&self, input: &Self::Input) -> Result<TriSign, crate::error::MathError> {
        let (a, b, c, d) = input;

        let adx = Rational::try_from_f64(a[0])? - Rational::try_from_f64(d[0])?;
        let bdx = Rational::try_from_f64(b[0])? - Rational::try_from_f64(d[0])?;
        let cdx = Rational::try_from_f64(c[0])? - Rational::try_from_f64(d[0])?;
        let ady = Rational::try_from_f64(a[1])? - Rational::try_from_f64(d[1])?;
        let bdy = Rational::try_from_f64(b[1])? - Rational::try_from_f64(d[1])?;
        let cdy = Rational::try_from_f64(c[1])? - Rational::try_from_f64(d[1])?;
        let adz = Rational::try_from_f64(a[2])? - Rational::try_from_f64(d[2])?;
        let bdz = Rational::try_from_f64(b[2])? - Rational::try_from_f64(d[2])?;
        let cdz = Rational::try_from_f64(c[2])? - Rational::try_from_f64(d[2])?;

        let det = &adx * &(&bdy * &cdz - &bdz * &cdy)
                - &bdx * &(&ady * &cdz - &adz * &cdy)
                + &cdx * &(&ady * &bdz - &adz * &bdy);

        Ok(det.sign())
    }
}

/// Compute the 3D orientation of four points.
///
/// Returns a [`CertifiedTriSign`] and [`PrecisionEscalation`] metadata:
/// - `Pos`: `d` is above the plane defined by `a, b, c` (right-hand rule)
/// - `Neg`: `d` is below the plane
/// - `Zero`: `d` is exactly on the plane
///
/// This is the sign of the 3×3 determinant:
/// ```text
/// | ax-dx  ay-dy  az-dz |
/// | bx-dx  by-dy  bz-dz |
/// | cx-dx  cy-dy  cz-dz |
/// ```
pub fn orient3d(
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
    d: [f64; 3],
) -> Result<(CertifiedTriSign, PrecisionEscalation), crate::error::MathError> {
    Orient3dPredicate.evaluate(&(a, b, c, d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sign::TriSign;

    #[test]
    fn orient3d_above_plane() {
        let (result, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ).unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn orient3d_below_plane() {
        let (result, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, -1.0],
        ).unwrap();
        assert_eq!(result.sign(), TriSign::Pos);
    }

    #[test]
    fn orient3d_on_plane() {
        let (result, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.5, 0.5, 0.0],
        ).unwrap();
        assert_eq!(result.sign(), TriSign::Zero);
    }

    #[test]
    fn orient3d_near_coplanar() {
        let (result, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1e-300],
        ).unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn orient3d_is_deterministic() {
        let a = [0.1, 0.2, 0.3];
        let b = [0.4, 0.5, 0.6];
        let c = [0.7, 0.8, 1.0];
        let d = [0.3, 0.6, 0.9];
        assert_eq!(
            orient3d(a, b, c, d).unwrap().0.sign(),
            orient3d(a, b, c, d).unwrap().0.sign()
        );
    }
}
