//! Insphere predicate.

use crate::filter::FilteredEval;
use crate::rational::Rational;
use crate::sign::{CertifiedTriSign, TriSign};
use super::IN_SPHERE_ERR_BOUND_A;

/// Insphere predicate: is `e` inside, outside, or on the circumsphere of `a, b, c, d`?
struct InSpherePredicate;

/// Input to [`in_sphere`]: five 3D points.
pub type InSphereInput = ([f64; 3], [f64; 3], [f64; 3], [f64; 3], [f64; 3]);

impl FilteredEval for InSpherePredicate {
    type Input = InSphereInput;

    fn eval_f64(&self, input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError> {
        let (a, b, c, d, e) = input;

        let aex = a[0] - e[0];
        let bex = b[0] - e[0];
        let cex = c[0] - e[0];
        let dex = d[0] - e[0];
        let aey = a[1] - e[1];
        let bey = b[1] - e[1];
        let cey = c[1] - e[1];
        let dey = d[1] - e[1];
        let aez = a[2] - e[2];
        let bez = b[2] - e[2];
        let cez = c[2] - e[2];
        let dez = d[2] - e[2];

        let aexbey = aex * bey;
        let bexaey = bex * aey;
        let ab = aexbey - bexaey;
        let bexcey = bex * cey;
        let cexbey = cex * bey;
        let bc = bexcey - cexbey;
        let cexdey = cex * dey;
        let dexcey = dex * cey;
        let cd = cexdey - dexcey;
        let dexaey = dex * aey;
        let aexdey = aex * dey;
        let da = dexaey - aexdey;
        let aexcey = aex * cey;
        let cexaey = cex * aey;
        let ac = aexcey - cexaey;
        let bexdey = bex * dey;
        let dexbey = dex * bey;
        let bd = bexdey - dexbey;

        let abc = aez * bc - bez * ac + cez * ab;
        let bcd = bez * cd - cez * bd + dez * bc;
        let cda = cez * da + dez * ac + aez * cd;
        let dab = dez * ab + aez * bd + bez * da;

        let alift = aex * aex + aey * aey + aez * aez;
        let blift = bex * bex + bey * bey + bez * bez;
        let clift = cex * cex + cey * cey + cez * cez;
        let dlift = dex * dex + dey * dey + dez * dez;

        let det = (dlift * abc - clift * dab) + (blift * cda - alift * bcd);

        if !det.is_finite() {
            return Err(crate::error::MathError::InvalidInput(
                "Non-finite determinant in in_sphere".into(),
            ));
        }

        let abc_bound = aez.abs() * (bexcey.abs() + cexbey.abs())
            + bez.abs() * (aexcey.abs() + cexaey.abs())
            + cez.abs() * (aexbey.abs() + bexaey.abs());
        let bcd_bound = bez.abs() * (cexdey.abs() + dexcey.abs())
            + cez.abs() * (bexdey.abs() + dexbey.abs())
            + dez.abs() * (bexcey.abs() + cexbey.abs());
        let cda_bound = cez.abs() * (dexaey.abs() + aexdey.abs())
            + dez.abs() * (aexcey.abs() + cexaey.abs())
            + aez.abs() * (cexdey.abs() + dexcey.abs());
        let dab_bound = dez.abs() * (aexbey.abs() + bexaey.abs())
            + aez.abs() * (bexdey.abs() + dexbey.abs())
            + bez.abs() * (dexaey.abs() + aexdey.abs());

        let det_bound = alift * bcd_bound + blift * cda_bound
            + clift * dab_bound + dlift * abc_bound;
        let err = IN_SPHERE_ERR_BOUND_A * det_bound;

        if det > err {
            Ok(Some(TriSign::Pos))
        } else if det < -err {
            Ok(Some(TriSign::Neg))
        } else {
            Ok(None)
        }
    }

    fn eval_double(&self, _input: &Self::Input) -> Result<Option<TriSign>, crate::error::MathError> {
        // TODO(optimization): implement double-double insphere if profiling
        // shows Stage 3 is called too often.
        Ok(None)
    }

    fn eval_exact(&self, input: &Self::Input) -> Result<TriSign, crate::error::MathError> {
        let (a, b, c, d, e) = input;

        let aex = Rational::try_from_f64(a[0])? - Rational::try_from_f64(e[0])?;
        let bex = Rational::try_from_f64(b[0])? - Rational::try_from_f64(e[0])?;
        let cex = Rational::try_from_f64(c[0])? - Rational::try_from_f64(e[0])?;
        let dex = Rational::try_from_f64(d[0])? - Rational::try_from_f64(e[0])?;
        let aey = Rational::try_from_f64(a[1])? - Rational::try_from_f64(e[1])?;
        let bey = Rational::try_from_f64(b[1])? - Rational::try_from_f64(e[1])?;
        let cey = Rational::try_from_f64(c[1])? - Rational::try_from_f64(e[1])?;
        let dey = Rational::try_from_f64(d[1])? - Rational::try_from_f64(e[1])?;
        let aez = Rational::try_from_f64(a[2])? - Rational::try_from_f64(e[2])?;
        let bez = Rational::try_from_f64(b[2])? - Rational::try_from_f64(e[2])?;
        let cez = Rational::try_from_f64(c[2])? - Rational::try_from_f64(e[2])?;
        let dez = Rational::try_from_f64(d[2])? - Rational::try_from_f64(e[2])?;

        let ab = &aex * &bey - &bex * &aey;
        let bc = &bex * &cey - &cex * &bey;
        let cd = &cex * &dey - &dex * &cey;
        let da = &dex * &aey - &aex * &dey;
        let ac = &aex * &cey - &cex * &aey;
        let bd = &bex * &dey - &dex * &bey;

        let abc = &aez * &bc - &bez * &ac + &cez * &ab;
        let bcd = &bez * &cd - &cez * &bd + &dez * &bc;
        let cda = &cez * &da + &dez * &ac + &aez * &cd;
        let dab = &dez * &ab + &aez * &bd + &bez * &da;

        let alift = &aex * &aex + &aey * &aey + &aez * &aez;
        let blift = &bex * &bex + &bey * &bey + &bez * &bez;
        let clift = &cex * &cex + &cey * &cey + &cez * &cez;
        let dlift = &dex * &dex + &dey * &dey + &dez * &dez;

        let det = (&dlift * &abc - &clift * &dab) + (&blift * &cda - &alift * &bcd);
        Ok(det.sign())
    }
}

/// Compute the insphere test for five 3D points.
///
/// Returns a [`CertifiedTriSign`]:
/// - `Pos`: `e` is inside the circumsphere (assuming positive orientation of `a,b,c,d`)
/// - `Neg`: `e` is outside the circumsphere
/// - `Zero`: `e` is exactly on the circumsphere
///
/// The orientation of `a, b, c, d` affects the sign. Callers should ensure
/// positive orientation via [`orient3d`] first.
pub fn in_sphere(
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
    d: [f64; 3],
    e: [f64; 3],
) -> Result<CertifiedTriSign, crate::error::MathError> {
    InSpherePredicate.evaluate(&(a, b, c, d, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sign::TriSign;

    #[test]
    fn in_sphere_point_inside() {
        let result = in_sphere(
            [1.0, 0.0, -0.707],
            [-1.0, 0.0, -0.707],
            [0.0, 1.0, 0.707],
            [0.0, -1.0, 0.707],
            [0.0, 0.0, 0.0],
        ).unwrap();
        assert!(!result.is_zero());
    }

    #[test]
    fn in_sphere_point_on_sphere() {
        let result = in_sphere(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [-1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
        ).unwrap();
        assert_eq!(result.sign(), TriSign::Zero);
    }

    #[test]
    fn in_sphere_point_outside() {
        let result = in_sphere(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [-1.0, 0.0, 0.0],
            [10.0, 10.0, 10.0],
        ).unwrap();
        assert!(!result.is_zero());
    }
}
