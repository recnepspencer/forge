//! Vendored orient2d predicate orient2d.

use super::expansion::{
    fast_expansion_sum_zeroelim, two_diff, two_diff_tail, two_product, two_two_diff,
    two_two_product,
};

use super::parameters::{abs, PARAMS};

/// Approximate 2D orientation test. Non-robust version of [`orient2d`].
#[inline]
pub(in crate::predicates) fn orient2d_fast(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2]) -> f64 {
    let acx = pa[0] - pc[0];
    let bcx = pb[0] - pc[0];
    let acy = pa[1] - pc[1];
    let bcy = pb[1] - pc[1];
    acx * bcy - acy * bcx
}

#[inline]
pub(in crate::predicates) fn orient2d_exact(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2]) -> f64 {
    let [axby0, axby1] = two_product(pa[0], pb[1]);
    let [axcy0, axcy1] = two_product(pa[0], pc[1]);
    let aterms = two_two_diff(axby1, axby0, axcy1, axcy0);
    let [bxcy0, bxcy1] = two_product(pb[0], pc[1]);
    let [bxay0, bxay1] = two_product(pb[0], pa[1]);
    let bterms = two_two_diff(bxcy1, bxcy0, bxay1, bxay0);
    let [cxay0, cxay1] = two_product(pc[0], pa[1]);
    let [cxby0, cxby1] = two_product(pc[0], pb[1]);
    let cterms = two_two_diff(cxay1, cxay0, cxby1, cxby0);
    let mut v = [0.; 8];
    let vlength = fast_expansion_sum_zeroelim(&aterms, &bterms, &mut v);
    let mut w = [0.; 12];
    let wlength = fast_expansion_sum_zeroelim(&v[..vlength], &cterms, &mut w);
    w[wlength - 1]
}

#[inline]
pub(in crate::predicates) fn orient2d_slow(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2]) -> f64 {
    let [acxtail, acx] = two_diff(pa[0], pc[0]);
    let [acytail, acy] = two_diff(pa[1], pc[1]);
    let [bcxtail, bcx] = two_diff(pb[0], pc[0]);
    let [bcytail, bcy] = two_diff(pb[1], pc[1]);
    let axby = two_two_product(acx, acxtail, bcy, bcytail);
    let negate = -acy;
    let negatetail = -acytail;
    let bxay = two_two_product(bcx, bcxtail, negate, negatetail);
    let mut deter = [0.; 16];
    let deterlen = fast_expansion_sum_zeroelim(&axby, &bxay, &mut deter);
    deter[deterlen - 1]
}

#[inline]
pub(in crate::predicates) fn orient2dadapt(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
    detsum: f64,
) -> f64 {
    let acx = pa[0] - pc[0];
    let bcx = pb[0] - pc[0];
    let acy = pa[1] - pc[1];
    let bcy = pb[1] - pc[1];
    let [detlefttail, detleft] = two_product(acx, bcy);
    let [detrighttail, detright] = two_product(acy, bcx);
    let b = two_two_diff(detleft, detlefttail, detright, detrighttail);
    let mut det: f64 = b.iter().sum();
    let errbound = PARAMS.ccwerrbound_b * detsum;
    if det >= errbound || -det >= errbound {
        return det;
    }
    let acxtail = two_diff_tail(pa[0], pc[0], acx);
    let bcxtail = two_diff_tail(pb[0], pc[0], bcx);
    let acytail = two_diff_tail(pa[1], pc[1], acy);
    let bcytail = two_diff_tail(pb[1], pc[1], bcy);
    if acxtail == 0.0 && acytail == 0.0 && bcxtail == 0.0 && bcytail == 0.0 {
        return det;
    }
    let errbound = PARAMS.ccwerrbound_c * detsum + PARAMS.resulterrbound * abs(det);
    det += acx * bcytail + bcy * acxtail - (acy * bcxtail + bcx * acytail);
    if det >= errbound || -det >= errbound {
        return det;
    }
    let [s0, s1] = two_product(acxtail, bcy);
    let [t0, t1] = two_product(acytail, bcx);
    let u = two_two_diff(s1, s0, t1, t0);
    let mut c1: [f64; 8] = [0.; 8];
    let c1length = fast_expansion_sum_zeroelim(&b, &u, &mut c1);
    let [s0, s1] = two_product(acx, bcytail);
    let [t0, t1] = two_product(acy, bcxtail);
    let u = two_two_diff(s1, s0, t1, t0);
    let mut c2: [f64; 12] = [0.; 12];
    let c2length = fast_expansion_sum_zeroelim(&c1[..c1length], &u, &mut c2);
    let [s0, s1] = two_product(acxtail, bcytail);
    let [t0, t1] = two_product(acytail, bcxtail);
    let u = two_two_diff(s1, s0, t1, t0);
    let mut d: [f64; 16] = [0.; 16];
    let dlength = fast_expansion_sum_zeroelim(&c2[..c2length], &u, &mut d);
    d[dlength - 1]
}

#[inline]
pub(in crate::predicates) fn orient2d(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2]) -> f64 {
    let detleft = (pa[0] - pc[0]) * (pb[1] - pc[1]);
    let detright = (pa[1] - pc[1]) * (pb[0] - pc[0]);
    let det = detleft - detright;
    let detsum = if detleft > 0.0 {
        if detright <= 0.0 {
            return det;
        } else {
            detleft + detright
        }
    } else if detleft < 0.0 {
        if detright >= 0.0 {
            return det;
        } else {
            -detleft - detright
        }
    } else {
        return det;
    };
    let errbound = PARAMS.ccwerrbound_a * detsum;
    if det >= errbound || -det >= errbound {
        return det;
    }
    orient2dadapt(pa, pb, pc, detsum)
}
