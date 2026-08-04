//! Vendored insphere predicate exact.

use super::super::expansion::{
    fast_expansion_sum_zeroelim, scale_expansion_zeroelim, two_product, two_two_diff,
};

#[inline]
pub(in crate::predicates) fn insphere_exact(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
    pe: [f64; 3],
) -> f64 {
    let mut temp8a = [0.; 8];
    let mut temp8b = [0.; 8];
    let mut temp16 = [0.; 16];
    let mut abc = [0.; 24];
    let mut bcd = [0.; 24];
    let mut cde = [0.; 24];
    let mut dea = [0.; 24];
    let mut eab = [0.; 24];
    let mut abd = [0.; 24];
    let mut bce = [0.; 24];
    let mut cda = [0.; 24];
    let mut deb = [0.; 24];
    let mut eac = [0.; 24];
    let mut temp48a = [0.; 48];
    let mut temp48b = [0.; 48];
    let mut abcd = [0.; 96];
    let mut bcde = [0.; 96];
    let mut cdea = [0.; 96];
    let mut deab = [0.; 96];
    let mut eabc = [0.; 96];
    let mut temp192 = [0.; 192];
    let mut det384x = [0.; 384];
    let mut det384y = [0.; 384];
    let mut det384z = [0.; 384];
    let mut detxy = [0.; 768];
    let mut adet = [0.; 1152];
    let mut bdet = [0.; 1152];
    let mut cdet = [0.; 1152];
    let mut ddet = [0.; 1152];
    let mut edet = [0.; 1152];
    let mut abdet = [0.; 2304];
    let mut cddet = [0.; 2304];
    let mut cdedet = [0.; 3456];
    let [axby0, axby1] = two_product(pa[0], pb[1]);
    let [bxay0, bxay1] = two_product(pb[0], pa[1]);
    let ab = two_two_diff(axby1, axby0, bxay1, bxay0);
    let [bxcy0, bxcy1] = two_product(pb[0], pc[1]);
    let [cxby0, cxby1] = two_product(pc[0], pb[1]);
    let bc = two_two_diff(bxcy1, bxcy0, cxby1, cxby0);
    let [cxdy0, cxdy1] = two_product(pc[0], pd[1]);
    let [dxcy0, dxcy1] = two_product(pd[0], pc[1]);
    let cd = two_two_diff(cxdy1, cxdy0, dxcy1, dxcy0);
    let [dxey0, dxey1] = two_product(pd[0], pe[1]);
    let [exdy0, exdy1] = two_product(pe[0], pd[1]);
    let de = two_two_diff(dxey1, dxey0, exdy1, exdy0);
    let [exay0, exay1] = two_product(pe[0], pa[1]);
    let [axey0, axey1] = two_product(pa[0], pe[1]);
    let ea = two_two_diff(exay1, exay0, axey1, axey0);
    let [axcy0, axcy1] = two_product(pa[0], pc[1]);
    let [cxay0, cxay1] = two_product(pc[0], pa[1]);
    let ac = two_two_diff(axcy1, axcy0, cxay1, cxay0);
    let [bxdy0, bxdy1] = two_product(pb[0], pd[1]);
    let [dxby0, dxby1] = two_product(pd[0], pb[1]);
    let bd = two_two_diff(bxdy1, bxdy0, dxby1, dxby0);
    let [cxey0, cxey1] = two_product(pc[0], pe[1]);
    let [excy0, excy1] = two_product(pe[0], pc[1]);
    let ce = two_two_diff(cxey1, cxey0, excy1, excy0);
    let [dxay0, dxay1] = two_product(pd[0], pa[1]);
    let [axdy0, axdy1] = two_product(pa[0], pd[1]);
    let da = two_two_diff(dxay1, dxay0, axdy1, axdy0);
    let [exby0, exby1] = two_product(pe[0], pb[1]);
    let [bxey0, bxey1] = two_product(pb[0], pe[1]);
    let eb = two_two_diff(exby1, exby0, bxey1, bxey0);
    let temp8alen = scale_expansion_zeroelim(&bc, pa[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&ac, -pb[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&ab, pc[2], &mut temp8a);
    let abclen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut abc);
    let temp8alen = scale_expansion_zeroelim(&cd, pb[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&bd, -pc[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&bc, pd[2], &mut temp8a);
    let bcdlen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut bcd);
    let temp8alen = scale_expansion_zeroelim(&de, pc[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&ce, -pd[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&cd, pe[2], &mut temp8a);
    let cdelen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut cde);
    let temp8alen = scale_expansion_zeroelim(&ea, pd[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&da, -pe[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&de, pa[2], &mut temp8a);
    let dealen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut dea);
    let temp8alen = scale_expansion_zeroelim(&ab, pe[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&eb, -pa[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&ea, pb[2], &mut temp8a);
    let eablen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut eab);
    let temp8alen = scale_expansion_zeroelim(&bd, pa[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&da, pb[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&ab, pd[2], &mut temp8a);
    let abdlen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut abd);
    let temp8alen = scale_expansion_zeroelim(&ce, pb[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&eb, pc[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&bc, pe[2], &mut temp8a);
    let bcelen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut bce);
    let temp8alen = scale_expansion_zeroelim(&da, pc[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&ac, pd[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&cd, pa[2], &mut temp8a);
    let cdalen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut cda);
    let temp8alen = scale_expansion_zeroelim(&eb, pd[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&bd, pe[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&de, pb[2], &mut temp8a);
    let deblen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut deb);
    let temp8alen = scale_expansion_zeroelim(&ac, pe[2], &mut temp8a);
    let temp8blen = scale_expansion_zeroelim(&ce, pa[2], &mut temp8b);
    let temp16len =
        fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp8b[..temp8blen], &mut temp16);
    let temp8alen = scale_expansion_zeroelim(&ea, pc[2], &mut temp8a);
    let eaclen = fast_expansion_sum_zeroelim(&temp8a[..temp8alen], &temp16[..temp16len], &mut eac);
    let temp48alen = fast_expansion_sum_zeroelim(&cde[..cdelen], &bce[..bcelen], &mut temp48a);
    let temp48blen = fast_expansion_sum_zeroelim(&deb[..deblen], &bcd[..bcdlen], &mut temp48b);
    temp48b[..temp48blen].iter_mut().for_each(|x| *x = -*x);
    let bcdelen =
        fast_expansion_sum_zeroelim(&temp48a[..temp48alen], &temp48b[..temp48blen], &mut bcde);
    let xlen = scale_expansion_zeroelim(&bcde[..bcdelen], pa[0], &mut temp192);
    let xlen = scale_expansion_zeroelim(&temp192[..xlen], pa[0], &mut det384x);
    let ylen = scale_expansion_zeroelim(&bcde[..bcdelen], pa[1], &mut temp192);
    let ylen = scale_expansion_zeroelim(&temp192[..ylen], pa[1], &mut det384y);
    let zlen = scale_expansion_zeroelim(&bcde[..bcdelen], pa[2], &mut temp192);
    let zlen = scale_expansion_zeroelim(&temp192[..zlen], pa[2], &mut det384z);
    let xylen = fast_expansion_sum_zeroelim(&det384x[..xlen], &det384y[..ylen], &mut detxy);
    let alen = fast_expansion_sum_zeroelim(&detxy[..xylen], &det384z[..zlen], &mut adet);
    let temp48alen = fast_expansion_sum_zeroelim(&dea[..dealen], &cda[..cdalen], &mut temp48a);
    let temp48blen = fast_expansion_sum_zeroelim(&eac[..eaclen], &cde[..cdelen], &mut temp48b);
    temp48b[..temp48blen].iter_mut().for_each(|x| *x = -*x);
    let cdealen =
        fast_expansion_sum_zeroelim(&temp48a[..temp48alen], &temp48b[..temp48blen], &mut cdea);
    let xlen = scale_expansion_zeroelim(&cdea[..cdealen], pb[0], &mut temp192);
    let xlen = scale_expansion_zeroelim(&temp192[..xlen], pb[0], &mut det384x);
    let ylen = scale_expansion_zeroelim(&cdea[..cdealen], pb[1], &mut temp192);
    let ylen = scale_expansion_zeroelim(&temp192[..ylen], pb[1], &mut det384y);
    let zlen = scale_expansion_zeroelim(&cdea[..cdealen], pb[2], &mut temp192);
    let zlen = scale_expansion_zeroelim(&temp192[..zlen], pb[2], &mut det384z);
    let xylen = fast_expansion_sum_zeroelim(&det384x[..xlen], &det384y[..ylen], &mut detxy);
    let blen = fast_expansion_sum_zeroelim(&detxy[..xylen], &det384z[..zlen], &mut bdet);
    let temp48alen = fast_expansion_sum_zeroelim(&eab[..eablen], &deb[..deblen], &mut temp48a);
    let temp48blen = fast_expansion_sum_zeroelim(&abd[..abdlen], &dea[..dealen], &mut temp48b);
    temp48b[..temp48blen].iter_mut().for_each(|x| *x = -*x);
    let deablen =
        fast_expansion_sum_zeroelim(&temp48a[..temp48alen], &temp48b[..temp48blen], &mut deab);
    let xlen = scale_expansion_zeroelim(&deab[..deablen], pc[0], &mut temp192);
    let xlen = scale_expansion_zeroelim(&temp192[..xlen], pc[0], &mut det384x);
    let ylen = scale_expansion_zeroelim(&deab[..deablen], pc[1], &mut temp192);
    let ylen = scale_expansion_zeroelim(&temp192[..ylen], pc[1], &mut det384y);
    let zlen = scale_expansion_zeroelim(&deab[..deablen], pc[2], &mut temp192);
    let zlen = scale_expansion_zeroelim(&temp192[..zlen], pc[2], &mut det384z);
    let xylen = fast_expansion_sum_zeroelim(&det384x[..xlen], &det384y[..ylen], &mut detxy);
    let clen = fast_expansion_sum_zeroelim(&detxy[..xylen], &det384z[..zlen], &mut cdet);
    let temp48alen = fast_expansion_sum_zeroelim(&abc[..abclen], &eac[..eaclen], &mut temp48a);
    let temp48blen = fast_expansion_sum_zeroelim(&bce[..bcelen], &eab[..eablen], &mut temp48b);
    temp48b[..temp48blen].iter_mut().for_each(|x| *x = -*x);
    let eabclen =
        fast_expansion_sum_zeroelim(&temp48a[..temp48alen], &temp48b[..temp48blen], &mut eabc);
    let xlen = scale_expansion_zeroelim(&eabc[..eabclen], pd[0], &mut temp192);
    let xlen = scale_expansion_zeroelim(&temp192[..xlen], pd[0], &mut det384x);
    let ylen = scale_expansion_zeroelim(&eabc[..eabclen], pd[1], &mut temp192);
    let ylen = scale_expansion_zeroelim(&temp192[..ylen], pd[1], &mut det384y);
    let zlen = scale_expansion_zeroelim(&eabc[..eabclen], pd[2], &mut temp192);
    let zlen = scale_expansion_zeroelim(&temp192[..zlen], pd[2], &mut det384z);
    let xylen = fast_expansion_sum_zeroelim(&det384x[..xlen], &det384y[..ylen], &mut detxy);
    let dlen = fast_expansion_sum_zeroelim(&detxy[..xylen], &det384z[..zlen], &mut ddet);
    let temp48alen = fast_expansion_sum_zeroelim(&bcd[..bcdlen], &abd[..abdlen], &mut temp48a);
    let temp48blen = fast_expansion_sum_zeroelim(&cda[..cdalen], &abc[..abclen], &mut temp48b);
    temp48b[..temp48blen].iter_mut().for_each(|x| *x = -*x);
    let abcdlen =
        fast_expansion_sum_zeroelim(&temp48a[..temp48alen], &temp48b[..temp48blen], &mut abcd);
    let xlen = scale_expansion_zeroelim(&abcd[..abcdlen], pe[0], &mut temp192);
    let xlen = scale_expansion_zeroelim(&temp192[..xlen], pe[0], &mut det384x);
    let ylen = scale_expansion_zeroelim(&abcd[..abcdlen], pe[1], &mut temp192);
    let ylen = scale_expansion_zeroelim(&temp192[..ylen], pe[1], &mut det384y);
    let zlen = scale_expansion_zeroelim(&abcd[..abcdlen], pe[2], &mut temp192);
    let zlen = scale_expansion_zeroelim(&temp192[..zlen], pe[2], &mut det384z);
    let xylen = fast_expansion_sum_zeroelim(&det384x[..xlen], &det384y[..ylen], &mut detxy);
    let elen = fast_expansion_sum_zeroelim(&detxy[..xylen], &det384z[..zlen], &mut edet);
    let ablen = fast_expansion_sum_zeroelim(&adet[..alen], &bdet[..blen], &mut abdet);
    let cdlen = fast_expansion_sum_zeroelim(&cdet[..clen], &ddet[..dlen], &mut cddet);
    let cdelen = fast_expansion_sum_zeroelim(&cddet[..cdlen], &edet[..elen], &mut cdedet);
    let mut deter = [0.; 5760];
    let deterlen = fast_expansion_sum_zeroelim(&abdet[..ablen], &cdedet[..cdelen], &mut deter);
    deter[deterlen - 1]
}
