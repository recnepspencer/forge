//! 3D orientation predicate.
//!
//! DOMAIN: Determine if point `d` is above, on, or below the plane defined by `a, b, c`.
//! ALGORITHM: Shewchuk adaptive cascade (vendored from geometry-predicates).
//! DEPENDENCIES: `vendored`, `precision`, `CertifiedTriSign`.

use super::vendored;
use crate::arithmetic::precision::{build_target_description, PrecisionEscalation, PrecisionMode};
use crate::sign::{CertifiedTriSign, TriSign};

/// Input to [`orient3d`]: four 3D points.
pub type Orient3dInput = ([f64; 3], [f64; 3], [f64; 3], [f64; 3]);

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
///
/// Uses Shewchuk's adaptive cascade for exact sign determination.
/// Tracks which precision stage resolved the sign for observability.
pub fn orient3d(
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
    d: [f64; 3],
) -> Result<(CertifiedTriSign, PrecisionEscalation), crate::error::MathError> {
    let fast_det = compute_fast_determinant(a, b, c, d);
    let permanent = compute_permanent(a, b, c, d);
    let errbound = vendored::O3D_ERRBOUND_A * permanent;

    let float_sign = sign_of(fast_det);

    if fast_det > errbound || -fast_det > errbound {
        return Ok((
            CertifiedTriSign::new(float_sign),
            PrecisionEscalation {
                resolved_at: PrecisionMode::Float64,
                float_agreed: true,
                expansion_length: None,
                target_triple: build_target_description(),
                disagreement_magnitude: None,
                float_sign: Some(float_sign),
            },
        ));
    }

    let adaptive_det = vendored::orient3dadapt(a, b, c, d, permanent);
    let adaptive_sign = sign_of(adaptive_det);
    let float_agreed = float_sign == adaptive_sign;

    let disagreement_magnitude = if !float_agreed && fast_det != 0.0 {
        Some(fast_det.abs())
    } else {
        None
    };

    Ok((
        CertifiedTriSign::new(adaptive_sign),
        PrecisionEscalation {
            resolved_at: PrecisionMode::ExpansionB,
            float_agreed,
            expansion_length: Some(192),
            target_triple: build_target_description(),
            disagreement_magnitude,
            float_sign: Some(float_sign),
        },
    ))
}

/// Compute the f64 fast-path determinant (Shewchuk Stage A).
fn compute_fast_determinant(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) -> f64 {
    let adx = a[0] - d[0];
    let bdx = b[0] - d[0];
    let cdx = c[0] - d[0];
    let ady = a[1] - d[1];
    let bdy = b[1] - d[1];
    let cdy = c[1] - d[1];
    let adz = a[2] - d[2];
    let bdz = b[2] - d[2];
    let cdz = c[2] - d[2];
    adz * (bdx * cdy - cdx * bdy) + bdz * (cdx * ady - adx * cdy) + cdz * (adx * bdy - bdx * ady)
}

/// Compute the permanent (sum of absolute sub-products) for error bounding.
fn compute_permanent(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) -> f64 {
    let adx = a[0] - d[0];
    let bdx = b[0] - d[0];
    let cdx = c[0] - d[0];
    let ady = a[1] - d[1];
    let bdy = b[1] - d[1];
    let cdy = c[1] - d[1];
    let adz = a[2] - d[2];
    let bdz = b[2] - d[2];
    let cdz = c[2] - d[2];
    let bdxcdy = bdx * cdy;
    let cdxbdy = cdx * bdy;
    let cdxady = cdx * ady;
    let adxcdy = adx * cdy;
    let adxbdy = adx * bdy;
    let bdxady = bdx * ady;
    (vendored::abs(bdxcdy) + vendored::abs(cdxbdy)) * vendored::abs(adz)
        + (vendored::abs(cdxady) + vendored::abs(adxcdy)) * vendored::abs(bdz)
        + (vendored::abs(adxbdy) + vendored::abs(bdxady)) * vendored::abs(cdz)
}

fn sign_of(det: f64) -> TriSign {
    if det > 0.0 {
        TriSign::Pos
    } else if det < 0.0 {
        TriSign::Neg
    } else {
        TriSign::Zero
    }
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
        )
        .unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn orient3d_below_plane() {
        let (result, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, -1.0],
        )
        .unwrap();
        assert_eq!(result.sign(), TriSign::Pos);
    }

    #[test]
    fn orient3d_on_plane() {
        let (result, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.5, 0.5, 0.0],
        )
        .unwrap();
        assert_eq!(result.sign(), TriSign::Zero);
    }

    #[test]
    fn orient3d_near_coplanar() {
        let (result, _) = orient3d(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1e-300],
        )
        .unwrap();
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

    #[test]
    fn oracle_cross_validation_basic() {
        let cases = [
            (
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ),
            (
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.5, 0.5, 0.0],
            ),
        ];
        for (pa, pb, pc, pd) in cases {
            let our_det = super::super::vendored::orient3d(pa, pb, pc, pd);
            let oracle_det = geometry_predicates::orient3d(pa, pb, pc, pd);
            assert_eq!(
                our_det.signum(),
                oracle_det.signum(),
                "Oracle mismatch for orient3d"
            );
        }
    }
}
