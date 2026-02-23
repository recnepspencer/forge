//! 2D orientation predicate.
//!
//! DOMAIN: Sign of the 2×2 determinant `(b-a) × (c-a)`.
//! ALGORITHM: Shewchuk adaptive cascade (vendored from geometry-predicates).
//! DEPENDENCIES: `vendored`, `precision`, `CertifiedTriSign`.

use crate::arithmetic::precision::{
    PrecisionEscalation, PrecisionMode, build_target_description,
};
use crate::sign::{CertifiedTriSign, TriSign};
use super::vendored;

/// Compute the 2D orientation of three points.
///
/// Returns a [`CertifiedTriSign`] and [`PrecisionEscalation`] metadata:
/// - `Pos`: counter-clockwise (pa, pb, pc)
/// - `Neg`: clockwise
/// - `Zero`: exactly collinear
///
/// This is the sign of the determinant:
/// ```text
/// | ax-cx  ay-cy |
/// | bx-cx  by-cy |
/// ```
///
/// Uses Shewchuk's adaptive cascade for exact sign determination
/// with minimal arithmetic work. Tracks which precision stage resolved.
pub fn orient2d(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
) -> Result<(CertifiedTriSign, PrecisionEscalation), crate::error::MathError> {
    let detleft = (pa[0] - pc[0]) * (pb[1] - pc[1]);
    let detright = (pa[1] - pc[1]) * (pb[0] - pc[0]);
    let det = detleft - detright;

    let detsum = if detleft > 0.0 {
        if detright <= 0.0 {
            return Ok(make_float64_result(det));
        }
        detleft + detright
    } else if detleft < 0.0 {
        if detright >= 0.0 {
            return Ok(make_float64_result(det));
        }
        -detleft - detright
    } else {
        return Ok(make_float64_result(det));
    };

    let errbound = vendored::CCW_ERRBOUND_A * detsum;
    if det >= errbound || -det >= errbound {
        return Ok(make_float64_result(det));
    }

    let adaptive_det = vendored::orient2dadapt(pa, pb, pc, detsum);
    let float_sign = sign_of(det);
    let adaptive_sign = sign_of(adaptive_det);
    let float_agreed = float_sign == adaptive_sign;

    let disagreement_magnitude = if !float_agreed && det != 0.0 {
        Some(det.abs())
    } else {
        None
    };

    Ok((
        CertifiedTriSign::new(adaptive_sign),
        PrecisionEscalation {
            resolved_at: PrecisionMode::ExpansionB,
            float_agreed,
            expansion_length: Some(16),
            target_triple: build_target_description(),
            disagreement_magnitude,
            float_sign: Some(float_sign),
        },
    ))
}

/// Build a Float64-resolved result from a determinant that passed the error bound.
fn make_float64_result(det: f64) -> (CertifiedTriSign, PrecisionEscalation) {
    let sign = sign_of(det);
    (
        CertifiedTriSign::new(sign),
        PrecisionEscalation {
            resolved_at: PrecisionMode::Float64,
            float_agreed: true,
            expansion_length: None,
            target_triple: build_target_description(),
            disagreement_magnitude: None,
            float_sign: Some(sign),
        },
    )
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
    fn orient2d_counter_clockwise() {
        let (result, _) = orient2d([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]).unwrap();
        assert_eq!(result.sign(), TriSign::Pos);
    }

    #[test]
    fn orient2d_clockwise() {
        let (result, _) = orient2d([0.0, 0.0], [0.0, 1.0], [1.0, 0.0]).unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn orient2d_collinear() {
        assert_eq!(
            orient2d([0.0, 0.0], [1.0, 1.0], [2.0, 2.0]).unwrap().0.sign(),
            TriSign::Zero
        );
    }

    #[test]
    fn orient2d_collinear_on_x_axis() {
        assert_eq!(
            orient2d([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]).unwrap().0.sign(),
            TriSign::Zero
        );
    }

    #[test]
    fn orient2d_collinear_on_y_axis() {
        assert_eq!(
            orient2d([0.0, 0.0], [0.0, 1.0], [0.0, 2.0]).unwrap().0.sign(),
            TriSign::Zero
        );
    }

    #[test]
    fn orient2d_near_collinear_positive() {
        let (result, _) = orient2d([0.0, 0.0], [1.0, 0.0], [0.5, 1e-15]).unwrap();
        assert_eq!(result.sign(), TriSign::Pos);
    }

    #[test]
    fn orient2d_near_collinear_negative() {
        let (result, _) = orient2d([0.0, 0.0], [1.0, 0.0], [0.5, -1e-15]).unwrap();
        assert_eq!(result.sign(), TriSign::Neg);
    }

    #[test]
    fn orient2d_is_deterministic() {
        let a = [0.1, 0.2];
        let b = [0.3, 0.4];
        let c = [0.5, 0.7];
        assert_eq!(
            orient2d(a, b, c).unwrap().0.sign(),
            orient2d(a, b, c).unwrap().0.sign()
        );
    }

    #[test]
    fn oracle_cross_validation_basic() {
        let cases = [
            ([0.0, 0.0], [1.0, 0.0], [0.0, 1.0]),
            ([0.0, 0.0], [0.0, 1.0], [1.0, 0.0]),
            ([0.0, 0.0], [1.0, 1.0], [2.0, 2.0]),
            ([1.0, 1.0], [2.0, 3.0], [4.0, 5.0]),
        ];
        for (pa, pb, pc) in cases {
            let our_det = super::super::vendored::orient2d(pa, pb, pc);
            let oracle_det = geometry_predicates::orient2d(pa, pb, pc);
            assert_eq!(
                our_det.signum(), oracle_det.signum(),
                "Oracle mismatch for orient2d({pa:?}, {pb:?}, {pc:?}): ours={our_det}, oracle={oracle_det}"
            );
        }
    }
}
