//! Exact geometric predicates on integer grid coordinates.
//!
//! DOMAIN: Orientation tests for vertices quantized to a fixed-precision
//! integer grid (EMBER algorithm). All arithmetic is exact in i128 —
//! no epsilon, no filter cascade, no BigInt allocation.
//!
//! INVARIANTS: Input coordinates must fit in 30 bits (±2^30). The 3D
//! orientation determinant of 30-bit inputs requires at most 94 bits,
//! which fits exactly in i128 (128 bits).

use crate::sign::{CertifiedTriSign, TriSign};

/// Exact 3D orientation predicate on integer grid coordinates.
///
/// Computes the sign of the 4×4 determinant:
/// ```text
/// | ax-dx  ay-dy  az-dz |
/// | bx-dx  by-dy  bz-dz |
/// | cx-dx  cy-dy  cz-dz |
/// ```
///
/// Returns `Pos` if D is below the plane defined by A, B, C (CCW from above),
/// `Neg` if above, and `Zero` if coplanar.
///
/// **Exact**: 30-bit inputs → 94-bit determinant → fits in i128. Zero error.
pub fn orient3d_grid(pa: [i64; 3], pb: [i64; 3], pc: [i64; 3], pd: [i64; 3]) -> CertifiedTriSign {
    let adx = (pa[0] - pd[0]) as i128;
    let ady = (pa[1] - pd[1]) as i128;
    let adz = (pa[2] - pd[2]) as i128;

    let bdx = (pb[0] - pd[0]) as i128;
    let bdy = (pb[1] - pd[1]) as i128;
    let bdz = (pb[2] - pd[2]) as i128;

    let cdx = (pc[0] - pd[0]) as i128;
    let cdy = (pc[1] - pd[1]) as i128;
    let cdz = (pc[2] - pd[2]) as i128;

    let det = adx * (bdy * cdz - bdz * cdy) - ady * (bdx * cdz - bdz * cdx)
        + adz * (bdx * cdy - bdy * cdx);

    CertifiedTriSign::new(TriSign::from_i128(det))
}

/// Exact 2D orientation predicate on integer grid coordinates.
///
/// Computes the sign of:
/// ```text
/// | ax-cx  ay-cy |
/// | bx-cx  by-cy |
/// ```
///
/// **Exact**: 30-bit inputs → 62-bit determinant → fits in i128.
pub fn orient2d_grid(pa: [i64; 2], pb: [i64; 2], pc: [i64; 2]) -> CertifiedTriSign {
    let acx = (pa[0] - pc[0]) as i128;
    let acy = (pa[1] - pc[1]) as i128;
    let bcx = (pb[0] - pc[0]) as i128;
    let bcy = (pb[1] - pc[1]) as i128;

    let det = acx * bcy - acy * bcx;

    CertifiedTriSign::new(TriSign::from_i128(det))
}

/// Classify a grid point relative to a plane defined by three grid points.
///
/// Returns the orientation of `point` relative to the plane through `a`, `b`, `c`.
pub fn classify_point_grid(
    a: [i64; 3],
    b: [i64; 3],
    c: [i64; 3],
    point: [i64; 3],
) -> CertifiedTriSign {
    orient3d_grid(a, b, c, point)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sign::TriSign;

    #[test]
    fn coplanar_points_return_zero() {
        let a = [0, 0, 0];
        let b = [1, 0, 0];
        let c = [0, 1, 0];
        let d = [1, 1, 0];
        assert_eq!(orient3d_grid(a, b, c, d).sign(), TriSign::Zero);
    }

    #[test]
    fn point_above_plane_returns_neg() {
        let a = [0, 0, 0];
        let b = [1, 0, 0];
        let c = [0, 1, 0];
        let d = [0, 0, 1];
        assert_eq!(orient3d_grid(a, b, c, d).sign(), TriSign::Neg);
    }

    #[test]
    fn point_below_plane_returns_pos() {
        let a = [0, 0, 0];
        let b = [1, 0, 0];
        let c = [0, 1, 0];
        let d = [0, 0, -1];
        assert_eq!(orient3d_grid(a, b, c, d).sign(), TriSign::Pos);
    }

    #[test]
    fn large_30bit_coordinates() {
        let max = 1i64 << 30;
        let a = [max, 0, 0];
        let b = [0, max, 0];
        let c = [0, 0, max];
        let d = [0, 0, 0];
        let result = orient3d_grid(a, b, c, d);
        assert_eq!(result.sign(), TriSign::Pos);
    }

    #[test]
    fn orient2d_ccw() {
        let a = [0, 0];
        let b = [1, 0];
        let c = [0, 1];
        assert_eq!(orient2d_grid(a, b, c).sign(), TriSign::Pos);
    }

    #[test]
    fn orient2d_collinear() {
        let a = [0, 0];
        let b = [1, 1];
        let c = [2, 2];
        assert_eq!(orient2d_grid(a, b, c).sign(), TriSign::Zero);
    }
}
