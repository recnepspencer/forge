//! Vendored orient3d predicate evaluation.

use super::adaptive::orient3dadapt;

use super::super::parameters::{abs, PARAMS};

#[inline]
pub(in crate::predicates) fn orient3d(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
) -> f64 {
    let adx = pa[0] - pd[0];
    let bdx = pb[0] - pd[0];
    let cdx = pc[0] - pd[0];
    let ady = pa[1] - pd[1];
    let bdy = pb[1] - pd[1];
    let cdy = pc[1] - pd[1];
    let adz = pa[2] - pd[2];
    let bdz = pb[2] - pd[2];
    let cdz = pc[2] - pd[2];
    let bdxcdy = bdx * cdy;
    let cdxbdy = cdx * bdy;
    let cdxady = cdx * ady;
    let adxcdy = adx * cdy;
    let adxbdy = adx * bdy;
    let bdxady = bdx * ady;
    let det = adz * (bdxcdy - cdxbdy) + bdz * (cdxady - adxcdy) + cdz * (adxbdy - bdxady);
    let permanent = (abs(bdxcdy) + abs(cdxbdy)) * abs(adz)
        + (abs(cdxady) + abs(adxcdy)) * abs(bdz)
        + (abs(adxbdy) + abs(bdxady)) * abs(cdz);
    let errbound = PARAMS.o3derrbound_a * permanent;
    if det > errbound || -det > errbound {
        return det;
    }
    orient3dadapt(pa, pb, pc, pd, permanent)
}
