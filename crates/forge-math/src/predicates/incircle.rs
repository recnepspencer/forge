//! 2D incircle predicate.
//!
//! DOMAIN: Is point `d` inside/on/outside the circumcircle of triangle `a, b, c`?
//! ALGORITHM: Shewchuk adaptive cascade (vendored from geometry-predicates).
//! DEPENDENCIES: `vendored`, `precision`, `CertifiedTriSign`.

use crate::arithmetic::precision::{build_target_description, PrecisionEscalation, PrecisionMode};
use crate::sign::{CertifiedTriSign, TriSign};

/// Compute the 2D incircle test for four points.
///
/// Returns a [`CertifiedTriSign`] and [`PrecisionEscalation`] metadata:
/// - `Pos`: `d` is inside the circumcircle of `a, b, c` (assuming CCW orientation)
/// - `Neg`: `d` is outside the circumcircle
/// - `Zero`: `d` is exactly on the circumcircle (cocircular)
///
/// The points `a, b, c` must be in counterclockwise order, or the sign
/// of the result will be reversed.
///
/// Uses Shewchuk's adaptive cascade for exact sign determination.
pub fn incircle(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
    pd: [f64; 2],
) -> Result<(CertifiedTriSign, PrecisionEscalation), crate::error::MathError> {
    let det = super::vendored::incircle(pa, pb, pc, pd);
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
    fn incircle_point_inside() {
        let (result, _) = incircle([0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.1, 0.1]).unwrap();
        assert_eq!(result.sign(), TriSign::Pos);
    }

    #[test]
    fn incircle_point_outside() {
        let (result, _) = incircle([0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [10.0, 10.0]).unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn incircle_cocircular() {
        let (result, _) = incircle([1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0]).unwrap();
        assert_eq!(result.sign(), TriSign::Zero);
    }

    #[test]
    fn oracle_cross_validation_basic() {
        let cases = [
            ([0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [0.1, 0.1]),
            ([0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [10.0, 10.0]),
            ([1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0]),
        ];
        for (pa, pb, pc, pd) in cases {
            let our_det = super::super::vendored::incircle(pa, pb, pc, pd);
            let oracle_det = geometry_predicates::incircle(pa, pb, pc, pd);
            assert_eq!(
                our_det.signum(),
                oracle_det.signum(),
                "Oracle mismatch for incircle({pa:?}, {pb:?}, {pc:?}, {pd:?})"
            );
        }
    }

    #[test]
    fn incircle_is_deterministic() {
        let a = [0.1, 0.2];
        let b = [0.3, 0.4];
        let c = [0.7, 0.1];
        let d = [0.2, 0.3];
        assert_eq!(
            incircle(a, b, c, d).unwrap().0.sign(),
            incircle(a, b, c, d).unwrap().0.sign()
        );
    }
}
