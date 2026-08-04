//! Vendored orient3d predicate adaptive.

use super::super::expansion::{
    fast_expansion_sum_zeroelim, scale_expansion_zeroelim, two_diff_tail, two_one_product,
    two_product, two_two_diff,
};

use super::super::parameters::{abs, PARAMS};

#[inline]
pub(in crate::predicates) fn orient3dadapt(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
    permanent: f64,
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

    let [bdxcdy0, bdxcdy1] = two_product(bdx, cdy);
    let [cdxbdy0, cdxbdy1] = two_product(cdx, bdy);
    let bc = two_two_diff(bdxcdy1, bdxcdy0, cdxbdy1, cdxbdy0);
    let mut adet = [0.; 8];
    let alen = scale_expansion_zeroelim(&bc, adz, &mut adet);

    let [cdxady0, cdxady1] = two_product(cdx, ady);
    let [adxcdy0, adxcdy1] = two_product(adx, cdy);
    let ca = two_two_diff(cdxady1, cdxady0, adxcdy1, adxcdy0);
    let mut bdet = [0.; 8];
    let blen = scale_expansion_zeroelim(&ca, bdz, &mut bdet);

    let [adxbdy0, adxbdy1] = two_product(adx, bdy);
    let [bdxady0, bdxady1] = two_product(bdx, ady);
    let ab = two_two_diff(adxbdy1, adxbdy0, bdxady1, bdxady0);
    let mut cdet = [0.; 8];
    let clen = scale_expansion_zeroelim(&ab, cdz, &mut cdet);

    let mut abdet = [0.; 16];
    let ablen = fast_expansion_sum_zeroelim(&adet[..alen], &bdet[..blen], &mut abdet);
    let mut fin1 = [0.; 192];
    let mut finlength = fast_expansion_sum_zeroelim(&abdet[..ablen], &cdet[..clen], &mut fin1);

    let mut det: f64 = fin1[..finlength].iter().sum();
    let errbound = PARAMS.o3derrbound_b * permanent;
    if det >= errbound || -det >= errbound {
        return det;
    }

    let adxtail = two_diff_tail(pa[0], pd[0], adx);
    let bdxtail = two_diff_tail(pb[0], pd[0], bdx);
    let cdxtail = two_diff_tail(pc[0], pd[0], cdx);
    let adytail = two_diff_tail(pa[1], pd[1], ady);
    let bdytail = two_diff_tail(pb[1], pd[1], bdy);
    let cdytail = two_diff_tail(pc[1], pd[1], cdy);
    let adztail = two_diff_tail(pa[2], pd[2], adz);
    let bdztail = two_diff_tail(pb[2], pd[2], bdz);
    let cdztail = two_diff_tail(pc[2], pd[2], cdz);
    if adxtail == 0.0
        && bdxtail == 0.0
        && cdxtail == 0.0
        && adytail == 0.0
        && bdytail == 0.0
        && cdytail == 0.0
        && adztail == 0.0
        && bdztail == 0.0
        && cdztail == 0.0
    {
        return det;
    }
    let errbound = PARAMS.o3derrbound_c * permanent + PARAMS.resulterrbound * abs(det);
    det += adz * (bdx * cdytail + cdy * bdxtail - (bdy * cdxtail + cdx * bdytail))
        + adztail * (bdx * cdy - bdy * cdx)
        + (bdz * (cdx * adytail + ady * cdxtail - (cdy * adxtail + adx * cdytail))
            + bdztail * (cdx * ady - cdy * adx))
        + (cdz * (adx * bdytail + bdy * adxtail - (ady * bdxtail + bdx * adytail))
            + cdztail * (adx * bdy - ady * bdx));

    if det >= errbound || -det >= errbound {
        return det;
    }

    let at_blen;
    let at_clen;
    let at_b;
    let at_c;
    if adxtail == 0.0 {
        if adytail == 0.0 {
            at_b = [0.; 4];
            at_blen = 1;
            at_c = [0.; 4];
            at_clen = 1;
        } else {
            let negate = -adytail;
            let [at_b0, at_blarge] = two_product(negate, bdx);
            at_b = [at_b0, at_blarge, 0., 0.];
            at_blen = 2;
            let [at_c0, at_clarge] = two_product(adytail, cdx);
            at_c = [at_c0, at_clarge, 0., 0.];
            at_clen = 2;
        }
    } else if adytail == 0.0 {
        let [at_b0, at_blarge] = two_product(adxtail, bdy);
        at_b = [at_b0, at_blarge, 0., 0.];
        at_blen = 2;
        let negate = -adxtail;
        let [at_c0, at_clarge] = two_product(negate, cdy);
        at_c = [at_c0, at_clarge, 0., 0.];
        at_clen = 2;
    } else {
        let [adxt_bdy0, adxt_bdy1] = two_product(adxtail, bdy);
        let [adyt_bdx0, adyt_bdx1] = two_product(adytail, bdx);
        at_b = two_two_diff(adxt_bdy1, adxt_bdy0, adyt_bdx1, adyt_bdx0);
        at_blen = 4;
        let [adyt_cdx0, adyt_cdx1] = two_product(adytail, cdx);
        let [adxt_cdy0, adxt_cdy1] = two_product(adxtail, cdy);
        at_c = two_two_diff(adyt_cdx1, adyt_cdx0, adxt_cdy1, adxt_cdy0);
        at_clen = 4;
    }
    let bt_clen;
    let bt_alen;
    let bt_c;
    let bt_a;
    if bdxtail == 0.0 {
        if bdytail == 0.0 {
            bt_c = [0.0; 4];
            bt_clen = 1;
            bt_a = [0.0; 4];
            bt_alen = 1;
        } else {
            let negate = -bdytail;
            let [bt_c0, bt_clarge] = two_product(negate, cdx);
            bt_c = [bt_c0, bt_clarge, 0., 0.];
            bt_clen = 2;
            let [bt_a0, bt_alarge] = two_product(bdytail, adx);
            bt_a = [bt_a0, bt_alarge, 0., 0.];
            bt_alen = 2;
        }
    } else if bdytail == 0.0 {
        let [bt_c0, bt_clarge] = two_product(bdxtail, cdy);
        bt_c = [bt_c0, bt_clarge, 0., 0.];
        bt_clen = 2;
        let negate = -bdxtail;
        let [bt_a0, bt_alarge] = two_product(negate, ady);
        bt_a = [bt_a0, bt_alarge, 0., 0.];
        bt_alen = 2
    } else {
        let [bdxt_cdy0, bdxt_cdy1] = two_product(bdxtail, cdy);
        let [bdyt_cdx0, bdyt_cdx1] = two_product(bdytail, cdx);
        bt_c = two_two_diff(bdxt_cdy1, bdxt_cdy0, bdyt_cdx1, bdyt_cdx0);
        bt_clen = 4;
        let [bdyt_adx0, bdyt_adx1] = two_product(bdytail, adx);
        let [bdxt_ady0, bdxt_ady1] = two_product(bdxtail, ady);
        bt_a = two_two_diff(bdyt_adx1, bdyt_adx0, bdxt_ady1, bdxt_ady0);
        bt_alen = 4;
    }
    let ct_alen;
    let ct_blen;
    let ct_a;
    let ct_b;
    if cdxtail == 0.0 {
        if cdytail == 0.0 {
            ct_a = [0.; 4];
            ct_alen = 1;
            ct_b = [0.; 4];
            ct_blen = 1;
        } else {
            let negate = -cdytail;
            let [ct_a0, ct_alarge] = two_product(negate, adx);
            ct_a = [ct_a0, ct_alarge, 0., 0.];
            ct_alen = 2;
            let [ct_b0, ct_blarge] = two_product(cdytail, bdx);
            ct_b = [ct_b0, ct_blarge, 0., 0.];
            ct_blen = 2;
        }
    } else if cdytail == 0.0 {
        let [ct_a0, ct_alarge] = two_product(cdxtail, ady);
        ct_a = [ct_a0, ct_alarge, 0., 0.];
        ct_alen = 2;
        let negate = -cdxtail;
        let [ct_b0, ct_blarge] = two_product(negate, bdy);
        ct_b = [ct_b0, ct_blarge, 0., 0.];
        ct_blen = 2;
    } else {
        let [cdxt_ady0, cdxt_ady1] = two_product(cdxtail, ady);
        let [cdyt_adx0, cdyt_adx1] = two_product(cdytail, adx);
        ct_a = two_two_diff(cdxt_ady1, cdxt_ady0, cdyt_adx1, cdyt_adx0);
        ct_alen = 4;
        let [cdyt_bdx0, cdyt_bdx1] = two_product(cdytail, bdx);
        let [cdxt_bdy0, cdxt_bdy1] = two_product(cdxtail, bdy);
        ct_b = two_two_diff(cdyt_bdx1, cdyt_bdx0, cdxt_bdy1, cdxt_bdy0);
        ct_blen = 4;
    }

    let mut fin2 = [0.; 192];

    let mut w = [0.; 16];

    let mut bct = [0.; 8];
    let bctlen = fast_expansion_sum_zeroelim(&bt_c[..bt_clen], &ct_b[..ct_blen], &mut bct);
    let wlength = scale_expansion_zeroelim(&bct[..bctlen], adz, &mut w);
    finlength = fast_expansion_sum_zeroelim(&fin1[..finlength], &w[..wlength], &mut fin2);

    let mut cat = [0.; 8];
    let catlen = fast_expansion_sum_zeroelim(&ct_a[..ct_alen], &at_c[..at_clen], &mut cat);
    let wlength = scale_expansion_zeroelim(&cat[..catlen], bdz, &mut w);
    finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &w[..wlength], &mut fin1);

    let mut abt = [0.; 8];
    let abtlen = fast_expansion_sum_zeroelim(&at_b[..at_blen], &bt_a[..bt_alen], &mut abt);
    let wlength = scale_expansion_zeroelim(&abt[..abtlen], cdz, &mut w);
    finlength = fast_expansion_sum_zeroelim(&fin1[..finlength], &w[..wlength], &mut fin2);

    let mut v = [0.; 12];

    // TODO: replace these swaps with destructuring assignment when it is stable;
    // https://github.com/rust-lang/rfcs/pull/2909
    let (mut fin1, fin2) = if adztail != 0.0 {
        let vlength = scale_expansion_zeroelim(&bc, adztail, &mut v);
        finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &v[..vlength], &mut fin1);
        (fin2, fin1)
    } else {
        (fin1, fin2)
    };
    let (mut fin1, fin2) = if bdztail != 0.0 {
        let vlength = scale_expansion_zeroelim(&ca, bdztail, &mut v);
        finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &v[..vlength], &mut fin1);
        (fin2, fin1)
    } else {
        (fin1, fin2)
    };
    let (mut fin1, fin2) = if cdztail != 0.0 {
        let vlength = scale_expansion_zeroelim(&ab, cdztail, &mut v);
        finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &v[..vlength], &mut fin1);
        (fin2, fin1)
    } else {
        (fin1, fin2)
    };
    let (mut fin1, fin2) = if adxtail != 0.0 {
        let (mut fin1, fin2) = if bdytail != 0.0 {
            let [adxt_bdyt0, adxt_bdyt1] = two_product(adxtail, bdytail);
            let u = two_one_product(adxt_bdyt1, adxt_bdyt0, cdz);
            finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
            let (mut fin1, fin2) = (fin2, fin1);
            if cdztail != 0.0 {
                let u = two_one_product(adxt_bdyt1, adxt_bdyt0, cdztail);
                finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
                (fin2, fin1)
            } else {
                (fin1, fin2)
            }
        } else {
            (fin1, fin2)
        };
        if cdytail != 0.0 {
            let negate = -adxtail;
            let [adxt_cdyt0, adxt_cdyt1] = two_product(negate, cdytail);
            let u = two_one_product(adxt_cdyt1, adxt_cdyt0, bdz);
            finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
            let (mut fin1, fin2) = (fin2, fin1);
            if bdztail != 0.0 {
                let u = two_one_product(adxt_cdyt1, adxt_cdyt0, bdztail);
                finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
                (fin2, fin1)
            } else {
                (fin1, fin2)
            }
        } else {
            (fin1, fin2)
        }
    } else {
        (fin1, fin2)
    };
    let (mut fin1, fin2) = if bdxtail != 0.0 {
        let (mut fin1, fin2) = if cdytail != 0.0 {
            let [bdxt_cdyt0, bdxt_cdyt1] = two_product(bdxtail, cdytail);
            let u = two_one_product(bdxt_cdyt1, bdxt_cdyt0, adz);
            finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
            let (mut fin1, fin2) = (fin2, fin1);
            if adztail != 0.0 {
                let u = two_one_product(bdxt_cdyt1, bdxt_cdyt0, adztail);
                finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
                (fin2, fin1)
            } else {
                (fin1, fin2)
            }
        } else {
            (fin1, fin2)
        };
        if adytail != 0.0 {
            let negate = -bdxtail;
            let [bdxt_adyt0, bdxt_adyt1] = two_product(negate, adytail);
            let u = two_one_product(bdxt_adyt1, bdxt_adyt0, cdz);
            finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
            let (mut fin1, fin2) = (fin2, fin1);
            if cdztail != 0.0 {
                let u = two_one_product(bdxt_adyt1, bdxt_adyt0, cdztail);
                finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
                (fin2, fin1)
            } else {
                (fin1, fin2)
            }
        } else {
            (fin1, fin2)
        }
    } else {
        (fin1, fin2)
    };
    let (mut fin1, fin2) = if cdxtail != 0.0 {
        let (mut fin1, fin2) = if adytail != 0.0 {
            let [cdxt_adyt0, cdxt_adyt1] = two_product(cdxtail, adytail);
            let u = two_one_product(cdxt_adyt1, cdxt_adyt0, bdz);
            finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
            let (mut fin1, fin2) = (fin2, fin1);
            if bdztail != 0.0 {
                let u = two_one_product(cdxt_adyt1, cdxt_adyt0, bdztail);
                finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
                (fin2, fin1)
            } else {
                (fin1, fin2)
            }
        } else {
            (fin1, fin2)
        };
        if bdytail != 0.0 {
            let negate = -cdxtail;
            let [cdxt_bdyt0, cdxt_bdyt1] = two_product(negate, bdytail);
            let u = two_one_product(cdxt_bdyt1, cdxt_bdyt0, adz);
            finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
            let (mut fin1, fin2) = (fin2, fin1);
            if adztail != 0.0 {
                let u = two_one_product(cdxt_bdyt1, cdxt_bdyt0, adztail);
                finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &u, &mut fin1);
                (fin2, fin1)
            } else {
                (fin1, fin2)
            }
        } else {
            (fin1, fin2)
        }
    } else {
        (fin1, fin2)
    };
    let (mut fin1, fin2) = if adztail != 0.0 {
        let wlength = scale_expansion_zeroelim(&bct[..bctlen], adztail, &mut w);
        finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &w[..wlength], &mut fin1);
        (fin2, fin1)
    } else {
        (fin1, fin2)
    };
    let (mut fin1, fin2) = if bdztail != 0.0 {
        let wlength = scale_expansion_zeroelim(&cat[..catlen], bdztail, &mut w);
        finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &w[..wlength], &mut fin1);
        (fin2, fin1)
    } else {
        (fin1, fin2)
    };
    let fin2 = if cdztail != 0.0 {
        let wlength = scale_expansion_zeroelim(&abt[..abtlen], cdztail, &mut w);
        finlength = fast_expansion_sum_zeroelim(&fin2[..finlength], &w[..wlength], &mut fin1);
        fin1
    } else {
        fin2
    };
    fin2[finlength - 1]
}
