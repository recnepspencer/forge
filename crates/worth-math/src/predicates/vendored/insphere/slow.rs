//! Vendored insphere predicate slow.

use super::super::expansion::{
    fast_expansion_sum_zeroelim, scale_expansion_zeroelim, two_diff, two_two_product,
};

#[inline]
pub(in crate::predicates) fn insphere_slow(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
    pe: [f64; 3],
) -> f64 {
    let mut ab = [0.; 16];
    let mut bc = [0.; 16];
    let mut cd = [0.; 16];
    let mut da = [0.; 16];
    let mut ac = [0.; 16];
    let mut bd = [0.; 16];
    let mut temp32a = [0.; 32];
    let mut temp32b = [0.; 32];
    let mut temp64a = [0.; 64];
    let mut temp64b = [0.; 64];
    let mut temp64c = [0.; 64];
    let mut temp128 = [0.; 128];
    let mut temp192 = [0.; 192];
    let mut detx = [0.; 384];
    let mut detxx = [0.; 768];
    let mut detxt = [0.; 384];
    let mut detxxt = [0.; 768];
    let mut detxtxt = [0.; 768];
    let mut x1 = [0.; 1536];
    let mut x2 = [0.; 2304];
    let mut dety = [0.; 384];
    let mut detyy = [0.; 768];
    let mut detyt = [0.; 384];
    let mut detyyt = [0.; 768];
    let mut detytyt = [0.; 768];
    let mut y1 = [0.; 1536];
    let mut y2 = [0.; 2304];
    let mut detz = [0.; 384];
    let mut detzz = [0.; 768];
    let mut detzt = [0.; 384];
    let mut detzzt = [0.; 768];
    let mut detztzt = [0.; 768];
    let mut z1 = [0.; 1536];
    let mut z2 = [0.; 2304];
    let mut detxy = [0.; 4608];
    let mut adet = [0.; 6912];
    let mut bdet = [0.; 6912];
    let mut cdet = [0.; 6912];
    let mut ddet = [0.; 6912];
    let mut abdet = [0.; 13824];
    let mut cddet = [0.; 13824];
    let mut deter = [0.; 27648];
    let [aextail, aex] = two_diff(pa[0], pe[0]);
    let [aeytail, aey] = two_diff(pa[1], pe[1]);
    let [aeztail, aez] = two_diff(pa[2], pe[2]);
    let [bextail, bex] = two_diff(pb[0], pe[0]);
    let [beytail, bey] = two_diff(pb[1], pe[1]);
    let [beztail, bez] = two_diff(pb[2], pe[2]);
    let [cextail, cex] = two_diff(pc[0], pe[0]);
    let [ceytail, cey] = two_diff(pc[1], pe[1]);
    let [ceztail, cez] = two_diff(pc[2], pe[2]);
    let [dextail, dex] = two_diff(pd[0], pe[0]);
    let [deytail, dey] = two_diff(pd[1], pe[1]);
    let [deztail, dez] = two_diff(pd[2], pe[2]);
    let axby = two_two_product(aex, aextail, bey, beytail);
    let negate = -aey;
    let negatetail = -aeytail;
    let bxay = two_two_product(bex, bextail, negate, negatetail);
    let ablen = fast_expansion_sum_zeroelim(&axby, &bxay, &mut ab);
    let bxcy = two_two_product(bex, bextail, cey, ceytail);
    let negate = -bey;
    let negatetail = -beytail;
    let cxby = two_two_product(cex, cextail, negate, negatetail);
    let bclen = fast_expansion_sum_zeroelim(&bxcy, &cxby, &mut bc);
    let cxdy = two_two_product(cex, cextail, dey, deytail);
    let negate = -cey;
    let negatetail = -ceytail;
    let dxcy = two_two_product(dex, dextail, negate, negatetail);
    let cdlen = fast_expansion_sum_zeroelim(&cxdy, &dxcy, &mut cd);
    let dxay = two_two_product(dex, dextail, aey, aeytail);
    let negate = -dey;
    let negatetail = -deytail;
    let axdy = two_two_product(aex, aextail, negate, negatetail);
    let dalen = fast_expansion_sum_zeroelim(&dxay, &axdy, &mut da);
    let axcy = two_two_product(aex, aextail, cey, ceytail);
    let negate = -aey;
    let negatetail = -aeytail;
    let cxay = two_two_product(cex, cextail, negate, negatetail);
    let aclen = fast_expansion_sum_zeroelim(&axcy, &cxay, &mut ac);
    let bxdy = two_two_product(bex, bextail, dey, deytail);
    let negate = -bey;
    let negatetail = -beytail;
    let dxby = two_two_product(dex, dextail, negate, negatetail);
    let bdlen = fast_expansion_sum_zeroelim(&bxdy, &dxby, &mut bd);
    let temp32alen = scale_expansion_zeroelim(&cd[..cdlen], -bez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&cd[..cdlen], -beztail, &mut temp32b);
    let temp64alen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64a);
    let temp32alen = scale_expansion_zeroelim(&bd[..bdlen], cez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&bd[..bdlen], ceztail, &mut temp32b);
    let temp64blen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64b);
    let temp32alen = scale_expansion_zeroelim(&bc[..bclen], -dez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&bc[..bclen], -deztail, &mut temp32b);
    let temp64clen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64c);
    let temp128len =
        fast_expansion_sum_zeroelim(&temp64a[..temp64alen], &temp64b[..temp64blen], &mut temp128);
    let temp192len =
        fast_expansion_sum_zeroelim(&temp64c[..temp64clen], &temp128[..temp128len], &mut temp192);
    let xlen = scale_expansion_zeroelim(&temp192[..temp192len], aex, &mut detx);
    let xxlen = scale_expansion_zeroelim(&detx[..xlen], aex, &mut detxx);
    let xtlen = scale_expansion_zeroelim(&temp192[..temp192len], aextail, &mut detxt);
    let xxtlen = scale_expansion_zeroelim(&detxt[..xtlen], aex, &mut detxxt);
    detxxt[..xxtlen].iter_mut().for_each(|x| *x *= 2.0);
    let xtxtlen = scale_expansion_zeroelim(&detxt[..xtlen], aextail, &mut detxtxt);
    let x1len = fast_expansion_sum_zeroelim(&detxx[..xxlen], &detxxt[..xxtlen], &mut x1);
    let x2len = fast_expansion_sum_zeroelim(&x1[..x1len], &detxtxt[..xtxtlen], &mut x2);
    let ylen = scale_expansion_zeroelim(&temp192[..temp192len], aey, &mut dety);
    let yylen = scale_expansion_zeroelim(&dety[..ylen], aey, &mut detyy);
    let ytlen = scale_expansion_zeroelim(&temp192[..temp192len], aeytail, &mut detyt);
    let yytlen = scale_expansion_zeroelim(&detyt[..ytlen], aey, &mut detyyt);
    detyyt[..yytlen].iter_mut().for_each(|x| *x *= 2.0);
    let ytytlen = scale_expansion_zeroelim(&detyt[..ytlen], aeytail, &mut detytyt);
    let y1len = fast_expansion_sum_zeroelim(&detyy[..yylen], &detyyt[..yytlen], &mut y1);
    let y2len = fast_expansion_sum_zeroelim(&y1[..y1len], &detytyt[..ytytlen], &mut y2);
    let zlen = scale_expansion_zeroelim(&temp192[..temp192len], aez, &mut detz);
    let zzlen = scale_expansion_zeroelim(&detz[..zlen], aez, &mut detzz);
    let ztlen = scale_expansion_zeroelim(&temp192[..temp192len], aeztail, &mut detzt);
    let zztlen = scale_expansion_zeroelim(&detzt[..ztlen], aez, &mut detzzt);
    detzzt[..zztlen].iter_mut().for_each(|x| *x *= 2.0);
    let ztztlen = scale_expansion_zeroelim(&detzt[..ztlen], aeztail, &mut detztzt);
    let z1len = fast_expansion_sum_zeroelim(&detzz[..zzlen], &detzzt[..zztlen], &mut z1);
    let z2len = fast_expansion_sum_zeroelim(&z1[..z1len], &detztzt[..ztztlen], &mut z2);
    let xylen = fast_expansion_sum_zeroelim(&x2[..x2len], &y2[..y2len], &mut detxy);
    let alen = fast_expansion_sum_zeroelim(&z2[..z2len], &detxy[..xylen], &mut adet);
    let temp32alen = scale_expansion_zeroelim(&da[..dalen], cez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&da[..dalen], ceztail, &mut temp32b);
    let temp64alen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64a);
    let temp32alen = scale_expansion_zeroelim(&ac[..aclen], dez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&ac[..aclen], deztail, &mut temp32b);
    let temp64blen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64b);
    let temp32alen = scale_expansion_zeroelim(&cd[..cdlen], aez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&cd[..cdlen], aeztail, &mut temp32b);
    let temp64clen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64c);
    let temp128len =
        fast_expansion_sum_zeroelim(&temp64a[..temp64alen], &temp64b[..temp64blen], &mut temp128);
    let temp192len =
        fast_expansion_sum_zeroelim(&temp64c[..temp64clen], &temp128[..temp128len], &mut temp192);
    let xlen = scale_expansion_zeroelim(&temp192[..temp192len], bex, &mut detx);
    let xxlen = scale_expansion_zeroelim(&detx[..xlen], bex, &mut detxx);
    let xtlen = scale_expansion_zeroelim(&temp192[..temp192len], bextail, &mut detxt);
    let xxtlen = scale_expansion_zeroelim(&detxt[..xtlen], bex, &mut detxxt);
    detxxt[..xxtlen].iter_mut().for_each(|x| *x *= 2.0);
    let xtxtlen = scale_expansion_zeroelim(&detxt[..xtlen], bextail, &mut detxtxt);
    let x1len = fast_expansion_sum_zeroelim(&detxx[..xxlen], &detxxt[..xxtlen], &mut x1);
    let x2len = fast_expansion_sum_zeroelim(&x1[..x1len], &detxtxt[..xtxtlen], &mut x2);
    let ylen = scale_expansion_zeroelim(&temp192[..temp192len], bey, &mut dety);
    let yylen = scale_expansion_zeroelim(&dety[..ylen], bey, &mut detyy);
    let ytlen = scale_expansion_zeroelim(&temp192[..temp192len], beytail, &mut detyt);
    let yytlen = scale_expansion_zeroelim(&detyt[..ytlen], bey, &mut detyyt);
    detyyt[..yytlen].iter_mut().for_each(|x| *x *= 2.0);
    let ytytlen = scale_expansion_zeroelim(&detyt[..ytlen], beytail, &mut detytyt);
    let y1len = fast_expansion_sum_zeroelim(&detyy[..yylen], &detyyt[..yytlen], &mut y1);
    let y2len = fast_expansion_sum_zeroelim(&y1[..y1len], &detytyt[..ytytlen], &mut y2);
    let zlen = scale_expansion_zeroelim(&temp192[..temp192len], bez, &mut detz);
    let zzlen = scale_expansion_zeroelim(&detz[..zlen], bez, &mut detzz);
    let ztlen = scale_expansion_zeroelim(&temp192[..temp192len], beztail, &mut detzt);
    let zztlen = scale_expansion_zeroelim(&detzt[..ztlen], bez, &mut detzzt);
    detzzt[..zztlen].iter_mut().for_each(|x| *x *= 2.0);
    let ztztlen = scale_expansion_zeroelim(&detzt[..ztlen], beztail, &mut detztzt);
    let z1len = fast_expansion_sum_zeroelim(&detzz[..zzlen], &detzzt[..zztlen], &mut z1);
    let z2len = fast_expansion_sum_zeroelim(&z1[..z1len], &detztzt[..ztztlen], &mut z2);
    let xylen = fast_expansion_sum_zeroelim(&x2[..x2len], &y2[..y2len], &mut detxy);
    let blen = fast_expansion_sum_zeroelim(&z2[..z2len], &detxy[..xylen], &mut bdet);
    let temp32alen = scale_expansion_zeroelim(&ab[..ablen], -dez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&ab[..ablen], -deztail, &mut temp32b);
    let temp64alen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64a);
    let temp32alen = scale_expansion_zeroelim(&bd[..bdlen], -aez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&bd[..bdlen], -aeztail, &mut temp32b);
    let temp64blen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64b);
    let temp32alen = scale_expansion_zeroelim(&da[..dalen], -bez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&da[..dalen], -beztail, &mut temp32b);
    let temp64clen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64c);
    let temp128len =
        fast_expansion_sum_zeroelim(&temp64a[..temp64alen], &temp64b[..temp64blen], &mut temp128);
    let temp192len =
        fast_expansion_sum_zeroelim(&temp64c[..temp64clen], &temp128[..temp128len], &mut temp192);
    let xlen = scale_expansion_zeroelim(&temp192[..temp192len], cex, &mut detx);
    let xxlen = scale_expansion_zeroelim(&detx[..xlen], cex, &mut detxx);
    let xtlen = scale_expansion_zeroelim(&temp192[..temp192len], cextail, &mut detxt);
    let xxtlen = scale_expansion_zeroelim(&detxt[..xtlen], cex, &mut detxxt);
    detxxt[..xxtlen].iter_mut().for_each(|x| *x *= 2.0);
    let xtxtlen = scale_expansion_zeroelim(&detxt[..xtlen], cextail, &mut detxtxt);
    let x1len = fast_expansion_sum_zeroelim(&detxx[..xxlen], &detxxt[..xxtlen], &mut x1);
    let x2len = fast_expansion_sum_zeroelim(&x1[..x1len], &detxtxt[..xtxtlen], &mut x2);
    let ylen = scale_expansion_zeroelim(&temp192[..temp192len], cey, &mut dety);
    let yylen = scale_expansion_zeroelim(&dety[..ylen], cey, &mut detyy);
    let ytlen = scale_expansion_zeroelim(&temp192[..temp192len], ceytail, &mut detyt);
    let yytlen = scale_expansion_zeroelim(&detyt[..ytlen], cey, &mut detyyt);
    detyyt[..yytlen].iter_mut().for_each(|x| *x *= 2.0);
    let ytytlen = scale_expansion_zeroelim(&detyt[..ytlen], ceytail, &mut detytyt);
    let y1len = fast_expansion_sum_zeroelim(&detyy[..yylen], &detyyt[..yytlen], &mut y1);
    let y2len = fast_expansion_sum_zeroelim(&y1[..y1len], &detytyt[..ytytlen], &mut y2);
    let zlen = scale_expansion_zeroelim(&temp192[..temp192len], cez, &mut detz);
    let zzlen = scale_expansion_zeroelim(&detz[..zlen], cez, &mut detzz);
    let ztlen = scale_expansion_zeroelim(&temp192[..temp192len], ceztail, &mut detzt);
    let zztlen = scale_expansion_zeroelim(&detzt[..ztlen], cez, &mut detzzt);
    detzzt[..zztlen].iter_mut().for_each(|x| *x *= 2.0);
    let ztztlen = scale_expansion_zeroelim(&detzt[..ztlen], ceztail, &mut detztzt);
    let z1len = fast_expansion_sum_zeroelim(&detzz[..zzlen], &detzzt[..zztlen], &mut z1);
    let z2len = fast_expansion_sum_zeroelim(&z1[..z1len], &detztzt[..ztztlen], &mut z2);
    let xylen = fast_expansion_sum_zeroelim(&x2[..x2len], &y2[..y2len], &mut detxy);
    let clen = fast_expansion_sum_zeroelim(&z2[..z2len], &detxy[..xylen], &mut cdet);
    let temp32alen = scale_expansion_zeroelim(&bc[..bclen], aez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&bc[..bclen], aeztail, &mut temp32b);
    let temp64alen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64a);
    let temp32alen = scale_expansion_zeroelim(&ac[..aclen], -bez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&ac[..aclen], -beztail, &mut temp32b);
    let temp64blen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64b);
    let temp32alen = scale_expansion_zeroelim(&ab[..ablen], cez, &mut temp32a);
    let temp32blen = scale_expansion_zeroelim(&ab[..ablen], ceztail, &mut temp32b);
    let temp64clen =
        fast_expansion_sum_zeroelim(&temp32a[..temp32alen], &temp32b[..temp32blen], &mut temp64c);
    let temp128len =
        fast_expansion_sum_zeroelim(&temp64a[..temp64alen], &temp64b[..temp64blen], &mut temp128);
    let temp192len =
        fast_expansion_sum_zeroelim(&temp64c[..temp64clen], &temp128[..temp128len], &mut temp192);
    let xlen = scale_expansion_zeroelim(&temp192[..temp192len], dex, &mut detx);
    let xxlen = scale_expansion_zeroelim(&detx[..xlen], dex, &mut detxx);
    let xtlen = scale_expansion_zeroelim(&temp192[..temp192len], dextail, &mut detxt);
    let xxtlen = scale_expansion_zeroelim(&detxt[..xtlen], dex, &mut detxxt);
    detxxt[..xxtlen].iter_mut().for_each(|x| *x *= 2.0);
    let xtxtlen = scale_expansion_zeroelim(&detxt[..xtlen], dextail, &mut detxtxt);
    let x1len = fast_expansion_sum_zeroelim(&detxx[..xxlen], &detxxt[..xxtlen], &mut x1);
    let x2len = fast_expansion_sum_zeroelim(&x1[..x1len], &detxtxt[..xtxtlen], &mut x2);
    let ylen = scale_expansion_zeroelim(&temp192[..temp192len], dey, &mut dety);
    let yylen = scale_expansion_zeroelim(&dety[..ylen], dey, &mut detyy);
    let ytlen = scale_expansion_zeroelim(&temp192[..temp192len], deytail, &mut detyt);
    let yytlen = scale_expansion_zeroelim(&detyt[..ytlen], dey, &mut detyyt);
    detyyt[..yytlen].iter_mut().for_each(|x| *x *= 2.0);
    let ytytlen = scale_expansion_zeroelim(&detyt[..ytlen], deytail, &mut detytyt);
    let y1len = fast_expansion_sum_zeroelim(&detyy[..yylen], &detyyt[..yytlen], &mut y1);
    let y2len = fast_expansion_sum_zeroelim(&y1[..y1len], &detytyt[..ytytlen], &mut y2);
    let zlen = scale_expansion_zeroelim(&temp192[..temp192len], dez, &mut detz);
    let zzlen = scale_expansion_zeroelim(&detz[..zlen], dez, &mut detzz);
    let ztlen = scale_expansion_zeroelim(&temp192[..temp192len], deztail, &mut detzt);
    let zztlen = scale_expansion_zeroelim(&detzt[..ztlen], dez, &mut detzzt);
    detzzt[..zztlen].iter_mut().for_each(|x| *x *= 2.0);
    let ztztlen = scale_expansion_zeroelim(&detzt[..ztlen], deztail, &mut detztzt);
    let z1len = fast_expansion_sum_zeroelim(&detzz[..zzlen], &detzzt[..zztlen], &mut z1);
    let z2len = fast_expansion_sum_zeroelim(&z1[..z1len], &detztzt[..ztztlen], &mut z2);
    let xylen = fast_expansion_sum_zeroelim(&x2[..x2len], &y2[..y2len], &mut detxy);
    let dlen = fast_expansion_sum_zeroelim(&z2[..z2len], &detxy[..xylen], &mut ddet);
    let ablen = fast_expansion_sum_zeroelim(&adet[..alen], &bdet[..blen], &mut abdet);
    let cdlen = fast_expansion_sum_zeroelim(&cdet[..clen], &ddet[..dlen], &mut cddet);
    let deterlen = fast_expansion_sum_zeroelim(&abdet[..ablen], &cddet[..cdlen], &mut deter);
    deter[deterlen - 1]
}
