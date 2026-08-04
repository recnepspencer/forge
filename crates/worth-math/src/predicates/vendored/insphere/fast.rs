//! Vendored insphere predicate fast.

/// Approximate 3D insphere test. Non-robust version of [`insphere`].
#[inline]
pub(in crate::predicates) fn insphere_fast(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
    pe: [f64; 3],
) -> f64 {
    let aex = pa[0] - pe[0];
    let bex = pb[0] - pe[0];
    let cex = pc[0] - pe[0];
    let dex = pd[0] - pe[0];
    let aey = pa[1] - pe[1];
    let bey = pb[1] - pe[1];
    let cey = pc[1] - pe[1];
    let dey = pd[1] - pe[1];
    let aez = pa[2] - pe[2];
    let bez = pb[2] - pe[2];
    let cez = pc[2] - pe[2];
    let dez = pd[2] - pe[2];
    let ab = aex * bey - bex * aey;
    let bc = bex * cey - cex * bey;
    let cd = cex * dey - dex * cey;
    let da = dex * aey - aex * dey;
    let ac = aex * cey - cex * aey;
    let bd = bex * dey - dex * bey;
    let abc = aez * bc - bez * ac + cez * ab;
    let bcd = bez * cd - cez * bd + dez * bc;
    let cda = cez * da + dez * ac + aez * cd;
    let dab = dez * ab + aez * bd + bez * da;
    let alift = aex * aex + aey * aey + aez * aez;
    let blift = bex * bex + bey * bey + bez * bez;
    let clift = cex * cex + cey * cey + cez * cez;
    let dlift = dex * dex + dey * dey + dez * dez;
    dlift * abc - clift * dab + (blift * cda - alift * bcd)
}
