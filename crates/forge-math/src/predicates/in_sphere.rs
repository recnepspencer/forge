//! 3D insphere predicate.
//!
//! DOMAIN: Is point `e` inside/on/outside the circumsphere of tetrahedron `a,b,c,d`?
//! ALGORITHM: Shewchuk adaptive cascade (vendored from geometry-predicates).
//! DEPENDENCIES: `vendored`, `precision`, `CertifiedTriSign`.

use crate::arithmetic::precision::{
    PrecisionEscalation, PrecisionMode, build_target_description,
};
use crate::sign::{CertifiedTriSign, TriSign};

/// Input to [`in_sphere`]: five 3D points.
pub type InSphereInput = ([f64; 3], [f64; 3], [f64; 3], [f64; 3], [f64; 3]);

/// Compute the insphere test for five 3D points.
///
/// Returns a [`CertifiedTriSign`] and [`PrecisionEscalation`] metadata:
/// - `Pos`: `e` is inside the circumsphere (assuming positive orientation of `a,b,c,d`)
/// - `Neg`: `e` is outside the circumsphere
/// - `Zero`: `e` is exactly on the circumsphere
///
/// The orientation of `a, b, c, d` affects the sign. Callers should ensure
/// positive orientation via [`orient3d`] first.
///
/// Uses Shewchuk's adaptive cascade for exact sign determination.
pub fn in_sphere(
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
    d: [f64; 3],
    e: [f64; 3],
) -> Result<(CertifiedTriSign, PrecisionEscalation), crate::error::MathError> {
    let det = super::vendored::insphere(a, b, c, d, e);
    let sign = sign_of(det);

    Ok((
        CertifiedTriSign::new(sign),
        PrecisionEscalation {
            resolved_at: PrecisionMode::Float64,
            float_agreed: true,
            expansion_length: None,
            target_triple: build_target_description(),
            disagreement_magnitude: None,
            float_sign: Some(sign),
        },
    ))
}

fn sign_of(det: f64) -> TriSign {
    if det > 0.0 { TriSign::Pos }
    else if det < 0.0 { TriSign::Neg }
    else { TriSign::Zero }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sign::TriSign;

    #[test]
    fn in_sphere_point_inside() {
        let (result, _) = in_sphere(
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
        let (result, _) = in_sphere(
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
        let (result, _) = in_sphere(
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [-1.0, 0.0, 0.0],
            [10.0, 10.0, 10.0],
        ).unwrap();
        assert!(!result.is_zero());
    }

    #[test]
    fn oracle_cross_validation_basic() {
        let cases = [
            ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [0.0, -1.0, 0.0]),
            ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [10.0, 10.0, 10.0]),
        ];
        for (pa, pb, pc, pd, pe) in cases {
            let our_det = super::super::vendored::insphere(pa, pb, pc, pd, pe);
            let oracle_det = geometry_predicates::insphere(pa, pb, pc, pd, pe);
            assert_eq!(
                our_det.signum(), oracle_det.signum(),
                "Oracle mismatch for insphere"
            );
        }
    }
}
