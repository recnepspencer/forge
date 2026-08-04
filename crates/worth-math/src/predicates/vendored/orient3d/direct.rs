//! Vendored orient3d predicate direct.

use super::super::expansion::{
    fast_expansion_sum_zeroelim, scale_expansion_zeroelim, two_diff, two_product, two_two_diff,
    two_two_product,
};

/// Approximate 3D orientation test. Non-robust version of [`orient3d`].
#[inline]
pub(in crate::predicates) fn orient3d_fast(
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
    adx * (bdy * cdz - bdz * cdy) + bdx * (cdy * adz - cdz * ady) + cdx * (ady * bdz - adz * bdy)
}

#[inline]
pub(in crate::predicates) fn orient3d_exact(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
) -> f64 {
    let [axby0, axby1] = two_product(pa[0], pb[1]);
    let [bxay0, bxay1] = two_product(pb[0], pa[1]);
    let ab = two_two_diff(axby1, axby0, bxay1, bxay0);
    let [bxcy0, bxcy1] = two_product(pb[0], pc[1]);
    let [cxby0, cxby1] = two_product(pc[0], pb[1]);
    let bc = two_two_diff(bxcy1, bxcy0, cxby1, cxby0);
    let [cxdy0, cxdy1] = two_product(pc[0], pd[1]);
    let [dxcy0, dxcy1] = two_product(pd[0], pc[1]);
    let cd = two_two_diff(cxdy1, cxdy0, dxcy1, dxcy0);
    let [dxay0, dxay1] = two_product(pd[0], pa[1]);
    let [axdy0, axdy1] = two_product(pa[0], pd[1]);
    let da = two_two_diff(dxay1, dxay0, axdy1, axdy0);
    let [axcy0, axcy1] = two_product(pa[0], pc[1]);
    let [cxay0, cxay1] = two_product(pc[0], pa[1]);
    let mut ac = two_two_diff(axcy1, axcy0, cxay1, cxay0);
    let [bxdy0, bxdy1] = two_product(pb[0], pd[1]);
    let [dxby0, dxby1] = two_product(pd[0], pb[1]);
    let mut bd = two_two_diff(bxdy1, bxdy0, dxby1, dxby0);

    let mut temp8 = [0.; 8];
    let mut abc = [0.; 12];
    let mut bcd = [0.; 12];
    let mut cda = [0.; 12];
    let mut dab = [0.; 12];
    let mut adet = [0.; 24];
    let mut bdet = [0.; 24];
    let mut cdet = [0.; 24];
    let mut ddet = [0.; 24];
    let mut abdet = [0.; 48];
    let mut cddet = [0.; 48];
    let mut deter = [0.; 96];

    let templen = fast_expansion_sum_zeroelim(&cd, &da, &mut temp8);
    let cdalen = fast_expansion_sum_zeroelim(&temp8[..templen], &ac, &mut cda);
    let templen = fast_expansion_sum_zeroelim(&da, &ab, &mut temp8);
    let dablen = fast_expansion_sum_zeroelim(&temp8[..templen], &bd, &mut dab);
    bd.iter_mut().for_each(|x| *x = -*x);
    ac.iter_mut().for_each(|x| *x = -*x);
    let templen = fast_expansion_sum_zeroelim(&ab, &bc, &mut temp8);
    let abclen = fast_expansion_sum_zeroelim(&temp8[..templen], &ac, &mut abc);
    let templen = fast_expansion_sum_zeroelim(&bc, &cd, &mut temp8);
    let bcdlen = fast_expansion_sum_zeroelim(&temp8[..templen], &bd, &mut bcd);
    let alen = scale_expansion_zeroelim(&bcd[..bcdlen], pa[2], &mut adet);
    let blen = scale_expansion_zeroelim(&cda[..cdalen], -pb[2], &mut bdet);
    let clen = scale_expansion_zeroelim(&dab[..dablen], pc[2], &mut cdet);
    let dlen = scale_expansion_zeroelim(&abc[..abclen], -pd[2], &mut ddet);
    let ablen = fast_expansion_sum_zeroelim(&adet[..alen], &bdet[..blen], &mut abdet);
    let cdlen = fast_expansion_sum_zeroelim(&cdet[..clen], &ddet[..dlen], &mut cddet);
    let deterlen = fast_expansion_sum_zeroelim(&abdet[..ablen], &cddet[..cdlen], &mut deter);
    deter[deterlen - 1]
}

#[inline]
pub(in crate::predicates) fn orient3d_slow(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
) -> f64 {
    let mut temp16: [f64; 16] = [0.; 16];
    let mut temp32: [f64; 32] = [0.; 32];
    let mut temp32t: [f64; 32] = [0.; 32];
    let mut adet: [f64; 64] = [0.; 64];
    let mut bdet: [f64; 64] = [0.; 64];
    let mut cdet: [f64; 64] = [0.; 64];
    let mut abdet: [f64; 128] = [0.; 128];
    let mut deter: [f64; 192] = [0.; 192];
    let [adxtail, adx] = two_diff(pa[0], pd[0]);
    let [adytail, ady] = two_diff(pa[1], pd[1]);
    let [adztail, adz] = two_diff(pa[2], pd[2]);
    let [bdxtail, bdx] = two_diff(pb[0], pd[0]);
    let [bdytail, bdy] = two_diff(pb[1], pd[1]);
    let [bdztail, bdz] = two_diff(pb[2], pd[2]);
    let [cdxtail, cdx] = two_diff(pc[0], pd[0]);
    let [cdytail, cdy] = two_diff(pc[1], pd[1]);
    let [cdztail, cdz] = two_diff(pc[2], pd[2]);
    let axby = two_two_product(adx, adxtail, bdy, bdytail);
    let negate = -ady;
    let negatetail = -adytail;
    let bxay = two_two_product(bdx, bdxtail, negate, negatetail);
    let bxcy = two_two_product(bdx, bdxtail, cdy, cdytail);
    let negate = -bdy;
    let negatetail = -bdytail;
    let cxby = two_two_product(cdx, cdxtail, negate, negatetail);
    let cxay = two_two_product(cdx, cdxtail, ady, adytail);
    let negate = -cdy;
    let negatetail = -cdytail;
    let axcy = two_two_product(adx, adxtail, negate, negatetail);
    let temp16len = fast_expansion_sum_zeroelim(&bxcy, &cxby, &mut temp16);
    let temp32len = scale_expansion_zeroelim(&temp16[..temp16len], adz, &mut temp32);
    let temp32tlen = scale_expansion_zeroelim(&temp16[..temp16len], adztail, &mut temp32t);
    let alen = fast_expansion_sum_zeroelim(&temp32[..temp32len], &temp32t[..temp32tlen], &mut adet);
    let temp16len = fast_expansion_sum_zeroelim(&cxay, &axcy, &mut temp16);
    let temp32len = scale_expansion_zeroelim(&temp16[..temp16len], bdz, &mut temp32);
    let temp32tlen = scale_expansion_zeroelim(&temp16[..temp16len], bdztail, &mut temp32t);
    let blen = fast_expansion_sum_zeroelim(&temp32[..temp32len], &temp32t[..temp32tlen], &mut bdet);
    let temp16len = fast_expansion_sum_zeroelim(&axby, &bxay, &mut temp16);
    let temp32len = scale_expansion_zeroelim(&temp16[..temp16len], cdz, &mut temp32);
    let temp32tlen = scale_expansion_zeroelim(&temp16[..temp16len], cdztail, &mut temp32t);
    let clen = fast_expansion_sum_zeroelim(&temp32[..temp32len], &temp32t[..temp32tlen], &mut cdet);
    let ablen = fast_expansion_sum_zeroelim(&adet[..alen], &bdet[..blen], &mut abdet);
    let deterlen = fast_expansion_sum_zeroelim(&abdet[..ablen], &cdet[..clen], &mut deter);
    deter[deterlen - 1]
}
