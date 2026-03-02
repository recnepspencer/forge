//! Simulation of Simplicity (SoS) predicates for degenerate geometry.
//!
//! DOMAIN: Exact tie-breaking for the common degenerate cases in point
//!         classification — when orient2d or orient3d returns exactly zero.
//!
//! ALGORITHM: Edelsbrunner–Mücke §4. The query point P is mathematically
//! perturbed by P(ε) = (P_x+ε³, P_y+ε¹, P_z+ε²). Because ε is infinitesimal
//! no floating-point arithmetic is done with it — if the base predicate result
//! is exactly zero we read the sign off the ε-polynomial coefficients.
//!
//! DEPENDENCIES: forge-math (orient2d, TriSign).

use forge_core::KernelError;
use forge_math::predicates::orient2d;
use forge_math::sign::TriSign;

/// Orient2d tie-breaker for P(ε) = (P_y + ε¹, P_z + ε²) in the YZ plane.
///
/// Called only when `orient2d(a, b, p_yz)` is exactly zero.
/// `a` and `b` are YZ coordinates `[y, z]` of the edge endpoints.
pub fn sos_orient2d_tiebreak(a: [f64; 2], b: [f64; 2]) -> TriSign {
    let delta1 = a[1] - b[1];
    if delta1 > 0.0 {
        return TriSign::Pos;
    }
    if delta1 < 0.0 {
        return TriSign::Neg;
    }

    let delta2 = b[0] - a[0];
    if delta2 > 0.0 {
        return TriSign::Pos;
    }
    if delta2 < 0.0 {
        return TriSign::Neg;
    }

    TriSign::Zero
}

/// Full orient3d tie-breaker for P(ε) = (P_x + ε³, P_y + ε¹, P_z + ε²).
///
/// Called only when `orient3d(a, b, c, p)` is exactly zero.
pub fn sos_orient3d_tiebreak(
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
) -> Result<TriSign, KernelError> {
    let (o_xz, _) = orient2d([a[0], a[2]], [b[0], b[2]], [c[0], c[2]]).map_err(|e| {
        KernelError::InternalError {
            message: e.to_string(),
            context: None,
        }
    })?;
    if o_xz.sign() != TriSign::Zero {
        return Ok(o_xz.sign());
    }

    let (o_xy, _) = orient2d([a[0], a[1]], [b[0], b[1]], [c[0], c[1]]).map_err(|e| {
        KernelError::InternalError {
            message: e.to_string(),
            context: None,
        }
    })?;
    if o_xy.sign() != TriSign::Zero {
        return Ok(match o_xy.sign() {
            TriSign::Pos => TriSign::Neg,
            TriSign::Neg => TriSign::Pos,
            TriSign::Zero => TriSign::Zero,
        });
    }

    let (o_yz, _) = orient2d([a[1], a[2]], [b[1], b[2]], [c[1], c[2]]).map_err(|e| {
        KernelError::InternalError {
            message: e.to_string(),
            context: None,
        }
    })?;
    Ok(match o_yz.sign() {
        TriSign::Pos => TriSign::Neg,
        TriSign::Neg => TriSign::Pos,
        TriSign::Zero => TriSign::Zero,
    })
}

/// Winding-number contribution of a single YZ-plane edge for query point (py, pz).
///
/// Uses the SoS perturbation P_z + ε² so that `az == pz` is treated as
/// A being strictly above the scanline. Returns +1, −1, or 0.
pub fn sos_edge_crossing_yz(
    py: f64,
    pz: f64,
    ay: f64,
    az: f64,
    by: f64,
    bz: f64,
) -> Result<i32, KernelError> {
    let a_above = az > pz;
    let b_above = bz > pz;

    if a_above == b_above {
        return Ok(0);
    }

    let (raw_orient, _) =
        orient2d([ay, az], [by, bz], [py, pz]).map_err(|e| KernelError::InternalError {
            message: e.to_string(),
            context: None,
        })?;

    let sign = if raw_orient.sign() != TriSign::Zero {
        raw_orient.sign()
    } else {
        sos_orient2d_tiebreak([ay, az], [by, bz])
    };

    if !a_above && sign == TriSign::Pos {
        return Ok(1);
    }
    if a_above && sign == TriSign::Neg {
        return Ok(-1);
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sos_orient2d_tiebreak_nonzero_delta1() {
        assert_eq!(sos_orient2d_tiebreak([0.0, 2.0], [0.0, 0.0]), TriSign::Pos);
        assert_eq!(sos_orient2d_tiebreak([0.0, 0.0], [0.0, 2.0]), TriSign::Neg);
    }

    #[test]
    fn sos_orient2d_tiebreak_delta2_fallback() {
        assert_eq!(sos_orient2d_tiebreak([1.0, 0.0], [2.0, 0.0]), TriSign::Pos);
        assert_eq!(sos_orient2d_tiebreak([2.0, 0.0], [1.0, 0.0]), TriSign::Neg);
    }

    #[test]
    fn sos_edge_no_crossing_same_side() {
        let result = sos_edge_crossing_yz(0.0, 0.0, 0.0, 1.0, 1.0, 2.0).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn sos_edge_upward_crossing() {
        let collinear = sos_edge_crossing_yz(0.0, 0.0, 0.0, -1.0, 0.0, 1.0).unwrap();
        assert_eq!(
            collinear, 0,
            "Collinear upward edge: SoS yields Neg, no +1 crossing"
        );

        let upward = sos_edge_crossing_yz(-1.0, 0.0, 0.0, -1.0, 0.0, 1.0).unwrap();
        assert_eq!(upward, 1, "P to the left of upward edge → +1");

        let downward = sos_edge_crossing_yz(-1.0, 0.0, 0.0, 1.0, 0.0, -1.0).unwrap();
        assert_eq!(downward, -1, "P to the right of downward edge → -1");
    }
}
