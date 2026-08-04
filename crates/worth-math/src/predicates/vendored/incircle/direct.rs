//! Vendored incircle predicate direct.

use super::super::expansion::{
    fast_expansion_sum_zeroelim, scale_expansion_zeroelim, two_diff, two_product, two_two_diff,
    two_two_product,
};

/// Approximate 2D incircle test. Non-robust version of [`incircle`].
#[inline]
pub(in crate::predicates) fn incircle_fast(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
    pd: [f64; 2],
) -> f64 {
    let adx = pa[0] - pd[0];
    let ady = pa[1] - pd[1];
    let bdx = pb[0] - pd[0];
    let bdy = pb[1] - pd[1];
    let cdx = pc[0] - pd[0];
    let cdy = pc[1] - pd[1];
    let abdet = adx * bdy - bdx * ady;
    let bcdet = bdx * cdy - cdx * bdy;
    let cadet = cdx * ady - adx * cdy;
    let alift = adx * adx + ady * ady;
    let blift = bdx * bdx + bdy * bdy;
    let clift = cdx * cdx + cdy * cdy;
    alift * bcdet + blift * cadet + clift * abdet
}

#[inline]
pub(in crate::predicates) fn incircle_exact(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
    pd: [f64; 2],
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
    let templen = fast_expansion_sum_zeroelim(&cd, &da, &mut temp8);
    let mut cda = [0.; 12];
    let cdalen = fast_expansion_sum_zeroelim(&temp8[..templen], &ac, &mut cda);
    let templen = fast_expansion_sum_zeroelim(&da, &ab, &mut temp8);
    let mut dab = [0.; 12];
    let dablen = fast_expansion_sum_zeroelim(&temp8[..templen], &bd, &mut dab);
    bd.iter_mut().for_each(|x| *x = -*x);
    ac.iter_mut().for_each(|x| *x = -*x);
    let templen = fast_expansion_sum_zeroelim(&ab, &bc, &mut temp8);
    let mut abc = [0.; 12];
    let abclen = fast_expansion_sum_zeroelim(&temp8[..templen], &ac, &mut abc);
    let templen = fast_expansion_sum_zeroelim(&bc, &cd, &mut temp8);
    let mut bcd = [0.; 12];
    let bcdlen = fast_expansion_sum_zeroelim(&temp8[..templen], &bd, &mut bcd);
    let mut det24x = [0.; 24];
    let xlen = scale_expansion_zeroelim(&bcd[..bcdlen], pa[0], &mut det24x);
    let mut det48x = [0.; 48];
    let xlen = scale_expansion_zeroelim(&det24x[..xlen], pa[0], &mut det48x);
    let mut det24y = [0.; 24];
    let ylen = scale_expansion_zeroelim(&bcd[..bcdlen], pa[1], &mut det24y);
    let mut det48y = [0.; 48];
    let ylen = scale_expansion_zeroelim(&det24y[..ylen], pa[1], &mut det48y);
    let mut adet = [0.; 96];
    let alen = fast_expansion_sum_zeroelim(&det48x[..xlen], &det48y[..ylen], &mut adet);
    let xlen = scale_expansion_zeroelim(&cda[..cdalen], pb[0], &mut det24x);
    let xlen = scale_expansion_zeroelim(&det24x[..xlen], -pb[0], &mut det48x);
    let ylen = scale_expansion_zeroelim(&cda[..cdalen], pb[1], &mut det24y);
    let ylen = scale_expansion_zeroelim(&det24y[..ylen], -pb[1], &mut det48y);
    let mut bdet = [0.; 96];
    let blen = fast_expansion_sum_zeroelim(&det48x[..xlen], &det48y[..ylen], &mut bdet);
    let xlen = scale_expansion_zeroelim(&dab[..dablen], pc[0], &mut det24x);
    let xlen = scale_expansion_zeroelim(&det24x[..xlen], pc[0], &mut det48x);
    let ylen = scale_expansion_zeroelim(&dab[..dablen], pc[1], &mut det24y);
    let ylen = scale_expansion_zeroelim(&det24y[..ylen], pc[1], &mut det48y);
    let mut cdet = [0.; 96];
    let clen = fast_expansion_sum_zeroelim(&det48x[..xlen], &det48y[..ylen], &mut cdet);
    let xlen = scale_expansion_zeroelim(&abc[..abclen], pd[0], &mut det24x);
    let xlen = scale_expansion_zeroelim(&det24x[..xlen], -pd[0], &mut det48x);
    let ylen = scale_expansion_zeroelim(&abc[..abclen], pd[1], &mut det24y);
    let ylen = scale_expansion_zeroelim(&det24y[..ylen], -pd[1], &mut det48y);
    let mut ddet = [0.; 96];
    let dlen = fast_expansion_sum_zeroelim(&det48x[..xlen], &det48y[..ylen], &mut ddet);
    let mut abdet = [0.; 192];
    let ablen = fast_expansion_sum_zeroelim(&adet[..alen], &bdet[..blen], &mut abdet);
    let mut cddet = [0.; 192];
    let cdlen = fast_expansion_sum_zeroelim(&cdet[..clen], &ddet[..dlen], &mut cddet);
    let mut deter = [0.; 384];
    let deterlen = fast_expansion_sum_zeroelim(&abdet[..ablen], &cddet[..cdlen], &mut deter);
    deter[deterlen - 1]
}

#[inline]
pub(in crate::predicates) fn incircle_slow(
    pa: [f64; 2],
    pb: [f64; 2],
    pc: [f64; 2],
    pd: [f64; 2],
) -> f64 {
    let [adxtail, adx] = two_diff(pa[0], pd[0]);
    let [adytail, ady] = two_diff(pa[1], pd[1]);
    let [bdxtail, bdx] = two_diff(pb[0], pd[0]);
    let [bdytail, bdy] = two_diff(pb[1], pd[1]);
    let [cdxtail, cdx] = two_diff(pc[0], pd[0]);
    let [cdytail, cdy] = two_diff(pc[1], pd[1]);
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
    let mut temp16 = [0.; 16];
    let temp16len = fast_expansion_sum_zeroelim(&bxcy, &cxby, &mut temp16);
    let mut detx = [0.; 32];
    let xlen = scale_expansion_zeroelim(&temp16[..temp16len], adx, &mut detx);
    let mut detxx = [0.; 64];
    let xxlen = scale_expansion_zeroelim(&detx[..xlen], adx, &mut detxx);
    let mut detxt = [0.; 32];
    let xtlen = scale_expansion_zeroelim(&temp16[..temp16len], adxtail, &mut detxt);
    let mut detxxt = [0.; 64];
    let xxtlen = scale_expansion_zeroelim(&detxt[..xtlen], adx, &mut detxxt);
    detxxt[..xxtlen].iter_mut().for_each(|x| *x *= 2.0);
    let mut detxtxt = [0.; 64];
    let xtxtlen = scale_expansion_zeroelim(&detxt[..xtlen], adxtail, &mut detxtxt);
    let mut x1 = [0.; 128];
    let x1len = fast_expansion_sum_zeroelim(&detxx[..xxlen], &detxxt[..xxtlen], &mut x1);
    let mut x2 = [0.; 192];
    let x2len = fast_expansion_sum_zeroelim(&x1[..x1len], &detxtxt[..xtxtlen], &mut x2);
    let mut dety = [0.; 32];
    let ylen = scale_expansion_zeroelim(&temp16[..temp16len], ady, &mut dety);
    let mut detyy = [0.; 64];
    let yylen = scale_expansion_zeroelim(&dety[..ylen], ady, &mut detyy);
    let mut detyt = [0.; 32];
    let ytlen = scale_expansion_zeroelim(&temp16[..temp16len], adytail, &mut detyt);
    let mut detyyt = [0.; 64];
    let yytlen = scale_expansion_zeroelim(&detyt[..ytlen], ady, &mut detyyt);
    detyyt[..yytlen].iter_mut().for_each(|x| *x *= 2.0);
    let mut detytyt = [0.; 64];
    let ytytlen = scale_expansion_zeroelim(&detyt[..ytlen], adytail, &mut detytyt);
    let mut y1 = [0.; 128];
    let y1len = fast_expansion_sum_zeroelim(&detyy[..yylen], &detyyt[..yytlen], &mut y1);
    let mut y2 = [0.; 192];
    let y2len = fast_expansion_sum_zeroelim(&y1[..y1len], &detytyt[..ytytlen], &mut y2);
    let mut adet = [0.; 384];
    let alen = fast_expansion_sum_zeroelim(&x2[..x2len], &y2[..y2len], &mut adet);
    let temp16len = fast_expansion_sum_zeroelim(&cxay, &axcy, &mut temp16);
    let xlen = scale_expansion_zeroelim(&temp16[..temp16len], bdx, &mut detx);
    let xxlen = scale_expansion_zeroelim(&detx[..xlen], bdx, &mut detxx);
    let xtlen = scale_expansion_zeroelim(&temp16[..temp16len], bdxtail, &mut detxt);
    let xxtlen = scale_expansion_zeroelim(&detxt[..xtlen], bdx, &mut detxxt);
    detxxt[..xxtlen].iter_mut().for_each(|x| *x *= 2.0);
    let xtxtlen = scale_expansion_zeroelim(&detxt[..xtlen], bdxtail, &mut detxtxt);
    let x1len = fast_expansion_sum_zeroelim(&detxx[..xxlen], &detxxt[..xxtlen], &mut x1);
    let x2len = fast_expansion_sum_zeroelim(&x1[..x1len], &detxtxt[..xtxtlen], &mut x2);
    let ylen = scale_expansion_zeroelim(&temp16[..temp16len], bdy, &mut dety);
    let yylen = scale_expansion_zeroelim(&dety[..ylen], bdy, &mut detyy);
    let ytlen = scale_expansion_zeroelim(&temp16[..temp16len], bdytail, &mut detyt);
    let yytlen = scale_expansion_zeroelim(&detyt[..ytlen], bdy, &mut detyyt);
    detyyt[..yytlen].iter_mut().for_each(|x| *x *= 2.0);
    let ytytlen = scale_expansion_zeroelim(&detyt[..ytlen], bdytail, &mut detytyt);
    let y1len = fast_expansion_sum_zeroelim(&detyy[..yylen], &detyyt[..yytlen], &mut y1);
    let y2len = fast_expansion_sum_zeroelim(&y1[..y1len], &detytyt[..ytytlen], &mut y2);
    let mut bdet = [0.; 384];
    let blen = fast_expansion_sum_zeroelim(&x2[..x2len], &y2[..y2len], &mut bdet);
    let temp16len = fast_expansion_sum_zeroelim(&axby, &bxay, &mut temp16);
    let xlen = scale_expansion_zeroelim(&temp16[..temp16len], cdx, &mut detx);
    let xxlen = scale_expansion_zeroelim(&detx[..xlen], cdx, &mut detxx);
    let xtlen = scale_expansion_zeroelim(&temp16[..temp16len], cdxtail, &mut detxt);
    let xxtlen = scale_expansion_zeroelim(&detxt[..xtlen], cdx, &mut detxxt);
    detxxt[..xxtlen].iter_mut().for_each(|x| *x *= 2.0);
    let xtxtlen = scale_expansion_zeroelim(&detxt[..xtlen], cdxtail, &mut detxtxt);
    let x1len = fast_expansion_sum_zeroelim(&detxx[..xxlen], &detxxt[..xxtlen], &mut x1);
    let x2len = fast_expansion_sum_zeroelim(&x1[..x1len], &detxtxt[..xtxtlen], &mut x2);
    let ylen = scale_expansion_zeroelim(&temp16[..temp16len], cdy, &mut dety);
    let yylen = scale_expansion_zeroelim(&dety[..ylen], cdy, &mut detyy);
    let ytlen = scale_expansion_zeroelim(&temp16[..temp16len], cdytail, &mut detyt);
    let yytlen = scale_expansion_zeroelim(&detyt[..ytlen], cdy, &mut detyyt);
    detyyt[..yytlen].iter_mut().for_each(|x| *x *= 2.0);
    let ytytlen = scale_expansion_zeroelim(&detyt[..ytlen], cdytail, &mut detytyt);
    let y1len = fast_expansion_sum_zeroelim(&detyy[..yylen], &detyyt[..yytlen], &mut y1);
    let y2len = fast_expansion_sum_zeroelim(&y1[..y1len], &detytyt[..ytytlen], &mut y2);
    let mut cdet = [0.; 384];
    let clen = fast_expansion_sum_zeroelim(&x2[..x2len], &y2[..y2len], &mut cdet);
    let mut abdet = [0.; 768];
    let ablen = fast_expansion_sum_zeroelim(&adet[..alen], &bdet[..blen], &mut abdet);
    let mut deter = [0.; 1152];
    let deterlen = fast_expansion_sum_zeroelim(&abdet[..ablen], &cdet[..clen], &mut deter);
    deter[deterlen - 1]
}
