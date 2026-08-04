//! Vendored incircle predicate evaluation.

use super::adaptive::incircleadapt;

use super::super::parameters::{abs, PARAMS};

#[inline]
pub(in crate::predicates) fn incircle(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
    pd: [f64; 2],
) -> f64 {
    let adx = pa[0] - pd[0];
    let bdx = pb[0] - pd[0];
    let cdx = pc[0] - pd[0];
    let ady = pa[1] - pd[1];
    let bdy = pb[1] - pd[1];
    let cdy = pc[1] - pd[1];
    let bdxcdy = bdx * cdy;
    let cdxbdy = cdx * bdy;
    let alift = adx * adx + ady * ady;
    let cdxady = cdx * ady;
    let adxcdy = adx * cdy;
    let blift = bdx * bdx + bdy * bdy;
    let adxbdy = adx * bdy;
    let bdxady = bdx * ady;
    let clift = cdx * cdx + cdy * cdy;
    let det = alift * (bdxcdy - cdxbdy) + blift * (cdxady - adxcdy) + clift * (adxbdy - bdxady);
    let permanent = (abs(bdxcdy) + abs(cdxbdy)) * alift
        + (abs(cdxady) + abs(adxcdy)) * blift
        + (abs(adxbdy) + abs(bdxady)) * clift;
    let errbound = PARAMS.iccerrbound_a * permanent;
    if det > errbound || -det > errbound {
        return det;
    }
    incircleadapt(pa, pb, pc, pd, permanent)
}
