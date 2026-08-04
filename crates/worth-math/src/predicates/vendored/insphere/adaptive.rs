//! Vendored insphere predicate adaptive.

use super::exact::insphere_exact;

use super::super::expansion::{
    fast_expansion_sum_zeroelim, scale_expansion_zeroelim, two_diff_tail, two_product, two_two_diff,
};

use super::super::parameters::{abs, PARAMS};

#[inline]
pub(in crate::predicates) fn insphereadapt(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
    pe: [f64; 3],
    permanent: f64,
) -> f64 {
    let mut temp8a: [f64; 8] = [0.; 8];
    let mut temp8b: [f64; 8] = [0.; 8];
    let mut temp8c: [f64; 8] = [0.; 8];
    let mut temp16: [f64; 16] = [0.; 16];
    let mut temp24: [f64; 24] = [0.; 24];
    let mut temp48: [f64; 48] = [0.; 48];
    let mut xdet: [f64; 96] = [0.; 96];
    let mut ydet: [f64; 96] = [0.; 96];
    let mut zdet: [f64; 96] = [0.; 96];
    let mut xydet: [f64; 192] = [0.; 192];
    let mut adet: [f64; 288] = [0.; 288];
    let mut bdet: [f64; 288] = [0.; 288];
    let mut cdet: [f64; 288] = [0.; 288];
    let mut ddet: [f64; 288] = [0.; 288];
    let mut abdet: [f64; 576] = [0.; 576];
    let mut cddet: [f64; 576] = [0.; 576];
    let mut fin1: [f64; 1152] = [0.; 1152];
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
    let [aexbey0, aexbey1] = two_product(aex, bey);
    let [bexaey0, bexaey1] = two_product(bex, aey);
    let ab = two_two_diff(aexbey1, aexbey0, bexaey1, bexaey0);
    let [bexcey0, bexcey1] = two_product(bex, cey);
    let [cexbey0, cexbey1] = two_product(cex, bey);
    let bc = two_two_diff(bexcey1, bexcey0, cexbey1, cexbey0);
    let [cexdey0, cexdey1] = two_product(cex, dey);
    let [dexcey0, dexcey1] = two_product(dex, cey);
    let cd = two_two_diff(cexdey1, cexdey0, dexcey1, dexcey0);
    let [dexaey0, dexaey1] = two_product(dex, aey);
    let [aexdey0, aexdey1] = two_product(aex, dey);
    let da = two_two_diff(dexaey1, dexaey0, aexdey1, aexdey0);
    let [aexcey0, aexcey1] = two_product(aex, cey);
    let [cexaey0, cexaey1] = two_product(cex, aey);
    let ac = two_two_diff(aexcey1, aexcey0, cexaey1, cexaey0);
    let [bexdey0, bexdey1] = two_product(bex, dey);
    let [dexbey0, dexbey1] = two_product(dex, bey);
    let bd = two_two_diff(bexdey1, bexdey0, dexbey1, dexbey0);
    let temp8alen = scale_expansion_zeroelim(&cd, bez, &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&bd, -cez, &mut temp8b);
    let temp8clen = scale_expansion_zeroelim(&bc, dez, &mut temp8c);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp24len =
        fast_expansion_sum_zeroelim(&temp8c[..temp8clen], &temp16[..temp16len], &mut temp24);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], aex, &mut temp48);
    let xlen = scale_expansion_zeroelim(&temp48[..temp48len], -aex, &mut xdet);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], aey, &mut temp48);
    let ylen = scale_expansion_zeroelim(&temp48[..temp48len], -aey, &mut ydet);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], aez, &mut temp48);
    let zlen = scale_expansion_zeroelim(&temp48[..temp48len], -aez, &mut zdet);
    let xylen = fast_expansion_sum_zeroelim(&xdet[..xlen], &ydet[..ylen], &mut xydet);
    let alen = fast_expansion_sum_zeroelim(&xydet[..xylen], &zdet[..zlen], &mut adet);
    let temp8alen = scale_expansion_zeroelim(&da, cez, &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&ac, dez, &mut temp8b);
    let temp8clen = scale_expansion_zeroelim(&cd, aez, &mut temp8c);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp24len =
        fast_expansion_sum_zeroelim(&temp8c[..temp8clen], &temp16[..temp16len], &mut temp24);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], bex, &mut temp48);
    let xlen = scale_expansion_zeroelim(&temp48[..temp48len], bex, &mut xdet);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], bey, &mut temp48);
    let ylen = scale_expansion_zeroelim(&temp48[..temp48len], bey, &mut ydet);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], bez, &mut temp48);
    let zlen = scale_expansion_zeroelim(&temp48[..temp48len], bez, &mut zdet);
    let xylen = fast_expansion_sum_zeroelim(&xdet[..xlen], &ydet[..ylen], &mut xydet);
    let blen = fast_expansion_sum_zeroelim(&xydet[..xylen], &zdet[..zlen], &mut bdet);
    let temp8alen = scale_expansion_zeroelim(&ab, dez, &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&bd, aez, &mut temp8b);
    let temp8clen = scale_expansion_zeroelim(&da, bez, &mut temp8c);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp24len =
        fast_expansion_sum_zeroelim(&temp8c[..temp8clen], &temp16[..temp16len], &mut temp24);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], cex, &mut temp48);
    let xlen = scale_expansion_zeroelim(&temp48[..temp48len], -cex, &mut xdet);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], cey, &mut temp48);
    let ylen = scale_expansion_zeroelim(&temp48[..temp48len], -cey, &mut ydet);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], cez, &mut temp48);
    let zlen = scale_expansion_zeroelim(&temp48[..temp48len], -cez, &mut zdet);
    let xylen = fast_expansion_sum_zeroelim(&xdet[..xlen], &ydet[..ylen], &mut xydet);
    let clen = fast_expansion_sum_zeroelim(&xydet[..xylen], &zdet[..zlen], &mut cdet);
    let temp8alen = scale_expansion_zeroelim(&bc, aez, &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&ac, -bez, &mut temp8b);
    let temp8clen = scale_expansion_zeroelim(&ab, cez, &mut temp8c);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp24len =
        fast_expansion_sum_zeroelim(&temp8c[..temp8clen], &temp16[..temp16len], &mut temp24);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], dex, &mut temp48);
    let xlen = scale_expansion_zeroelim(&temp48[..temp48len], dex, &mut xdet);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], dey, &mut temp48);
    let ylen = scale_expansion_zeroelim(&temp48[..temp48len], dey, &mut ydet);
    let temp48len = scale_expansion_zeroelim(&temp24[..temp24len], dez, &mut temp48);
    let zlen = scale_expansion_zeroelim(&temp48[..temp48len], dez, &mut zdet);
    let xylen = fast_expansion_sum_zeroelim(&xdet[..xlen], &ydet[..ylen], &mut xydet);
    let dlen = fast_expansion_sum_zeroelim(&xydet[..xylen], &zdet[..zlen], &mut ddet);
    let ablen = fast_expansion_sum_zeroelim(&adet[..alen], &bdet[..blen], &mut abdet);
    let cdlen = fast_expansion_sum_zeroelim(&cdet[..clen], &ddet[..dlen], &mut cddet);
    let finlength = fast_expansion_sum_zeroelim(&abdet[..ablen], &cddet[..cdlen], &mut fin1);
    let mut det: f64 = fin1[..finlength].iter().sum();
    let errbound = PARAMS.isperrbound_b * permanent;
    if det >= errbound || -det >= errbound {
        return det;
    }
    let aextail = two_diff_tail(pa[0], pe[0], aex);
    let aeytail = two_diff_tail(pa[1], pe[1], aey);
    let aeztail = two_diff_tail(pa[2], pe[2], aez);
    let bextail = two_diff_tail(pb[0], pe[0], bex);
    let beytail = two_diff_tail(pb[1], pe[1], bey);
    let beztail = two_diff_tail(pb[2], pe[2], bez);
    let cextail = two_diff_tail(pc[0], pe[0], cex);
    let ceytail = two_diff_tail(pc[1], pe[1], cey);
    let ceztail = two_diff_tail(pc[2], pe[2], cez);
    let dextail = two_diff_tail(pd[0], pe[0], dex);
    let deytail = two_diff_tail(pd[1], pe[1], dey);
    let deztail = two_diff_tail(pd[2], pe[2], dez);
    if aextail == 0.0
        && aeytail == 0.0
        && aeztail == 0.0
        && bextail == 0.0
        && beytail == 0.0
        && beztail == 0.0
        && cextail == 0.0
        && ceytail == 0.0
        && ceztail == 0.0
        && dextail == 0.0
        && deytail == 0.0
        && deztail == 0.0
    {
        return det;
    }
    let errbound = PARAMS.isperrbound_c * permanent + PARAMS.resulterrbound * abs(det);
    let abeps = aex * beytail + bey * aextail - (aey * bextail + bex * aeytail);
    let bceps = bex * ceytail + cey * bextail - (bey * cextail + cex * beytail);
    let cdeps = cex * deytail + dey * cextail - (cey * dextail + dex * ceytail);
    let daeps = dex * aeytail + aey * dextail - (dey * aextail + aex * deytail);
    let aceps = aex * ceytail + cey * aextail - (aey * cextail + cex * aeytail);
    let bdeps = bex * deytail + dey * bextail - (bey * dextail + dex * beytail);
    det += (bex * bex + bey * bey + bez * bez)
        * (cez * daeps
            + dez * aceps
            + aez * cdeps
            + (ceztail * da[3] + deztail * ac[3] + aeztail * cd[3]))
        + (dex * dex + dey * dey + dez * dez)
            * (aez * bceps - bez * aceps
                + cez * abeps
                + (aeztail * bc[3] - beztail * ac[3] + ceztail * ab[3]))
        - ((aex * aex + aey * aey + aez * aez)
            * (bez * cdeps - cez * bdeps
                + dez * bceps
                + (beztail * cd[3] - ceztail * bd[3] + deztail * bc[3]))
            + (cex * cex + cey * cey + cez * cez)
                * (dez * abeps
                    + aez * bdeps
                    + bez * daeps
                    + (deztail * ab[3] + aeztail * bd[3] + beztail * da[3])))
        + 2.0
            * ((bex * bextail + bey * beytail + bez * beztail)
                * (cez * da[3] + dez * ac[3] + aez * cd[3])
                + (dex * dextail + dey * deytail + dez * deztail)
                    * (aez * bc[3] - bez * ac[3] + cez * ab[3])
                - ((aex * aextail + aey * aeytail + aez * aeztail)
                    * (bez * cd[3] - cez * bd[3] + dez * bc[3])
                    + (cex * cextail + cey * ceytail + cez * ceztail)
                        * (dez * ab[3] + aez * bd[3] + bez * da[3])));
    if det >= errbound || -det >= errbound {
        return det;
    }
    insphere_exact(pa, pb, pc, pd, pe)
}
