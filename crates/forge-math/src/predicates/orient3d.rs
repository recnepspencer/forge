//! 3D orientation predicate.

use crate::double::Double;
use crate::filter::FilteredEval;
use crate::rational::Rational;
use crate::sign::{CertifiedTriSign, TriSign};
use super::ORIENT3D_ERR_BOUND_A;

/// 3D orientation predicate: is `d` above, below, or on the plane through `a, b, c`?
struct Orient3dPredicate;

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

        let det_bound = adx.abs() * (bdy.abs() * cdz.abs() + bdz.abs() * cdy.abs())
            + bdx.abs() * (ady.abs() * cdz.abs() + adz.abs() * cdy.abs())
            + cdx.abs() * (ady.abs() * bdz.abs() + adz.abs() * bdy.abs());

        let err = ORIENT3D_ERR_BOUND_A * det_bound;

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

        let m1 = bdy * cdz - bdz * cdy;
        let m2 = ady * cdz - adz * cdy;
        let m3 = ady * bdz - adz * bdy;

        let det = adx * m1 - bdx * m2 + cdx * m3;
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

        let m1 = &bdy * &cdz - &bdz * &cdy;
        let m2 = &ady * &cdz - &adz * &cdy;
        let m3 = &ady * &bdz - &adz * &bdy;

        let det = &adx * &m1 - &bdx * &m2 + &cdx * &m3;
        Ok(det.sign())
    }
}

/// Compute the 3D orientation of four points.
///
/// Returns a [`CertifiedTriSign`]:
/// - `Pos` / `Neg`: `d` is above / below the oriented plane through `a, b, c`
/// - `Zero`: `d` is exactly coplanar
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
) -> Result<CertifiedTriSign, crate::error::MathError> {
    Orient3dPredicate.evaluate(&(a, b, c, d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sign::TriSign;

    #[test]
    fn orient3d_above_plane() {
        let result = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ).unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn orient3d_below_plane() {
        let result = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, -1.0],
        ).unwrap();
        assert_eq!(result.sign(), TriSign::Pos);
    }

    #[test]
    fn orient3d_coplanar() {
        let result = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ).unwrap();
        assert_eq!(result.sign(), TriSign::Zero);
    }

    #[test]
    fn orient3d_near_coplanar_above() {
        let result = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.5, 0.5, 1e-15],
        ).unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn orient3d_is_deterministic() {
        let a = [0.1, 0.2, 0.3];
        let b = [0.4, 0.5, 0.6];
        let c = [0.7, 0.8, 1.0];
        let d = [0.0, 0.0, 0.0];
        assert_eq!(
            orient3d(a, b, c, d).unwrap().sign(),
            orient3d(a, b, c, d).unwrap().sign()
        );
    }
}
