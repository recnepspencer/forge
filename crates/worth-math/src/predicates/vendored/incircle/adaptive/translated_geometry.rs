//! Translated coordinates, base cross expansions, and lift facts.

use super::super::super::expansion::{
    fast_expansion_sum_zeroelim, scale_expansion_zeroelim, two_diff_tail, two_product, two_two_diff,
};

pub(in crate::predicates::vendored::incircle::adaptive) struct AdaptiveInput {
    pub(in crate::predicates::vendored::incircle::adaptive) pa: [f64; 2],
    pub(in crate::predicates::vendored::incircle::adaptive) pb: [f64; 2],
    pub(in crate::predicates::vendored::incircle::adaptive) pc: [f64; 2],
    pub(in crate::predicates::vendored::incircle::adaptive) pd: [f64; 2],
    pub(in crate::predicates::vendored::incircle::adaptive) permanent: f64,
}

impl AdaptiveInput {
    pub(in crate::predicates::vendored::incircle::adaptive) fn new(
        pa: [f64; 2],
        pb: [f64; 2],
        pc: [f64; 2],
        pd: [f64; 2],
        permanent: f64,
    ) -> Self {
        Self {
            pa,
            pb,
            pc,
            pd,
            permanent,
        }
    }
}

pub(in crate::predicates::vendored::incircle::adaptive) struct TranslatedGeometry {
    pub(in crate::predicates::vendored::incircle::adaptive) adx: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) bdx: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) cdx: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) ady: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) bdy: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) cdy: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) bc: [f64; 4],
    pub(in crate::predicates::vendored::incircle::adaptive) ca: [f64; 4],
    pub(in crate::predicates::vendored::incircle::adaptive) ab: [f64; 4],
}

impl TranslatedGeometry {
    pub(in crate::predicates::vendored::incircle::adaptive) fn from_input(
        input: &AdaptiveInput,
    ) -> (Self, LiftedBaseDeterminants) {
        let adx = input.pa[0] - input.pd[0];
        let bdx = input.pb[0] - input.pd[0];
        let cdx = input.pc[0] - input.pd[0];
        let ady = input.pa[1] - input.pd[1];
        let bdy = input.pb[1] - input.pd[1];
        let cdy = input.pc[1] - input.pd[1];

        let [bdxcdy0, bdxcdy1] = two_product(bdx, cdy);
        let [cdxbdy0, cdxbdy1] = two_product(cdx, bdy);
        let bc = two_two_diff(bdxcdy1, bdxcdy0, cdxbdy1, cdxbdy0);

        let mut axbc = [0.; 8];
        let mut axxbc = [0.; 16];
        let mut aybc = [0.; 8];
        let mut ayybc = [0.; 16];
        let mut adet = [0.; 32];
        let axbclen = scale_expansion_zeroelim(&bc, adx, &mut axbc);
        let axxbclen = scale_expansion_zeroelim(&axbc[..axbclen], adx, &mut axxbc);
        let aybclen = scale_expansion_zeroelim(&bc, ady, &mut aybc);
        let ayybclen = scale_expansion_zeroelim(&aybc[..aybclen], ady, &mut ayybc);
        let alen = fast_expansion_sum_zeroelim(&axxbc[..axxbclen], &ayybc[..ayybclen], &mut adet);

        let [cdxady0, cdxady1] = two_product(cdx, ady);
        let [adxcdy0, adxcdy1] = two_product(adx, cdy);
        let ca = two_two_diff(cdxady1, cdxady0, adxcdy1, adxcdy0);

        let mut bxca = [0.; 8];
        let mut bxxca = [0.; 16];
        let mut byca = [0.; 8];
        let mut byyca = [0.; 16];
        let mut bdet = [0.; 32];
        let bxcalen = scale_expansion_zeroelim(&ca, bdx, &mut bxca);
        let bxxcalen = scale_expansion_zeroelim(&bxca[..bxcalen], bdx, &mut bxxca);
        let bycalen = scale_expansion_zeroelim(&ca, bdy, &mut byca);
        let byycalen = scale_expansion_zeroelim(&byca[..bycalen], bdy, &mut byyca);
        let blen = fast_expansion_sum_zeroelim(&bxxca[..bxxcalen], &byyca[..byycalen], &mut bdet);

        let [adxbdy0, adxbdy1] = two_product(adx, bdy);
        let [bdxady0, bdxady1] = two_product(bdx, ady);
        let ab = two_two_diff(adxbdy1, adxbdy0, bdxady1, bdxady0);

        let mut cxab = [0.; 8];
        let mut cxxab = [0.; 16];
        let mut cyab = [0.; 8];
        let mut cyyab = [0.; 16];
        let mut cdet = [0.; 32];
        let cxablen = scale_expansion_zeroelim(&ab, cdx, &mut cxab);
        let cxxablen = scale_expansion_zeroelim(&cxab[..cxablen], cdx, &mut cxxab);
        let cyablen = scale_expansion_zeroelim(&ab, cdy, &mut cyab);
        let cyyablen = scale_expansion_zeroelim(&cyab[..cyablen], cdy, &mut cyyab);
        let clen = fast_expansion_sum_zeroelim(&cxxab[..cxxablen], &cyyab[..cyyablen], &mut cdet);

        let geometry = Self {
            adx,
            bdx,
            cdx,
            ady,
            bdy,
            cdy,
            bc,
            ca,
            ab,
        };
        let lifted = LiftedBaseDeterminants {
            adet,
            alen,
            bdet,
            blen,
            cdet,
            clen,
        };
        (geometry, lifted)
    }

    pub(in crate::predicates::vendored::incircle::adaptive) fn coordinate_tails(
        &self,
        input: &AdaptiveInput,
    ) -> CoordinateTails {
        let adxtail = two_diff_tail(input.pa[0], input.pd[0], self.adx);
        let adytail = two_diff_tail(input.pa[1], input.pd[1], self.ady);
        let bdxtail = two_diff_tail(input.pb[0], input.pd[0], self.bdx);
        let bdytail = two_diff_tail(input.pb[1], input.pd[1], self.bdy);
        let cdxtail = two_diff_tail(input.pc[0], input.pd[0], self.cdx);
        let cdytail = two_diff_tail(input.pc[1], input.pd[1], self.cdy);
        CoordinateTails {
            adxtail,
            adytail,
            bdxtail,
            bdytail,
            cdxtail,
            cdytail,
        }
    }
}

pub(in crate::predicates::vendored::incircle::adaptive) struct CoordinateTails {
    pub(in crate::predicates::vendored::incircle::adaptive) adxtail: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) adytail: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) bdxtail: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) bdytail: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) cdxtail: f64,
    pub(in crate::predicates::vendored::incircle::adaptive) cdytail: f64,
}

impl CoordinateTails {
    pub(in crate::predicates::vendored::incircle::adaptive) fn all_zero(&self) -> bool {
        self.adxtail == 0.0
            && self.bdxtail == 0.0
            && self.cdxtail == 0.0
            && self.adytail == 0.0
            && self.bdytail == 0.0
            && self.cdytail == 0.0
    }
}

pub(in crate::predicates::vendored::incircle::adaptive) struct LiftedBaseDeterminants {
    pub(in crate::predicates::vendored::incircle::adaptive) adet: [f64; 32],
    pub(in crate::predicates::vendored::incircle::adaptive) alen: usize,
    pub(in crate::predicates::vendored::incircle::adaptive) bdet: [f64; 32],
    pub(in crate::predicates::vendored::incircle::adaptive) blen: usize,
    pub(in crate::predicates::vendored::incircle::adaptive) cdet: [f64; 32],
    pub(in crate::predicates::vendored::incircle::adaptive) clen: usize,
}
