//! First-order coordinate-tail contribution construction.

use super::super::super::expansion::{fast_expansion_sum_zeroelim, scale_expansion_zeroelim};
use super::final_expansion::FinalExpansion;
use super::second_order::ExactTailContext;
use super::tail_expansion_scratch::TailExpansionScratch;

pub(in crate::predicates::vendored::incircle::adaptive) struct FirstOrderCrossExpansions {
    pub(in crate::predicates::vendored::incircle::adaptive) axtbc: [f64; 8],
    pub(in crate::predicates::vendored::incircle::adaptive) axtbclen: usize,
    pub(in crate::predicates::vendored::incircle::adaptive) aytbc: [f64; 8],
    pub(in crate::predicates::vendored::incircle::adaptive) aytbclen: usize,
    pub(in crate::predicates::vendored::incircle::adaptive) bxtca: [f64; 8],
    pub(in crate::predicates::vendored::incircle::adaptive) bxtcalen: usize,
    pub(in crate::predicates::vendored::incircle::adaptive) bytca: [f64; 8],
    pub(in crate::predicates::vendored::incircle::adaptive) bytcalen: usize,
    pub(in crate::predicates::vendored::incircle::adaptive) cxtab: [f64; 8],
    pub(in crate::predicates::vendored::incircle::adaptive) cxtablen: usize,
    pub(in crate::predicates::vendored::incircle::adaptive) cytab: [f64; 8],
    pub(in crate::predicates::vendored::incircle::adaptive) cytablen: usize,
}

pub(super) fn apply(
    context: &ExactTailContext,
    final_expansion: &mut FinalExpansion,
    scratch: &mut TailExpansionScratch,
) -> FirstOrderCrossExpansions {
    let mut axtbb = [0.; 8];
    let mut axtcc = [0.; 8];
    let mut aytbb = [0.; 8];
    let mut aytcc = [0.; 8];
    let mut bxtaa = [0.; 8];
    let mut bxtcc = [0.; 8];
    let mut bytaa = [0.; 8];
    let mut bytcc = [0.; 8];
    let mut cxtaa = [0.; 8];
    let mut cxtbb = [0.; 8];
    let mut cytaa = [0.; 8];
    let mut cytbb = [0.; 8];

    let mut axtbclen = 8;
    let mut axtbc = [0.; 8];
    if context.tails.adxtail != 0.0 {
        axtbclen =
            scale_expansion_zeroelim(&context.geometry.bc, context.tails.adxtail, &mut axtbc);
        let temp16alen = scale_expansion_zeroelim(
            &axtbc[..axtbclen],
            2.0 * context.geometry.adx,
            &mut scratch.temp16a,
        );
        let axtcclen = scale_expansion_zeroelim(&context.cc, context.tails.adxtail, &mut axtcc);
        let temp16blen = scale_expansion_zeroelim(
            &axtcc[..axtcclen],
            context.geometry.bdy,
            &mut scratch.temp16b,
        );
        let axtbblen = scale_expansion_zeroelim(&context.bb, context.tails.adxtail, &mut axtbb);
        let temp16clen = scale_expansion_zeroelim(
            &axtbb[..axtbblen],
            -context.geometry.cdy,
            &mut scratch.temp16c,
        );
        let temp32alen = fast_expansion_sum_zeroelim(
            &scratch.temp16a[..temp16alen],
            &scratch.temp16b[..temp16blen],
            &mut scratch.temp32a,
        );
        let temp48len = fast_expansion_sum_zeroelim(
            &scratch.temp16c[..temp16clen],
            &scratch.temp32a[..temp32alen],
            &mut scratch.temp48,
        );
        final_expansion.append_expansion(&scratch.temp48[..temp48len]);
    }
    let mut aytbclen = 8;
    let mut aytbc = [0.; 8];
    if context.tails.adytail != 0.0 {
        aytbclen =
            scale_expansion_zeroelim(&context.geometry.bc, context.tails.adytail, &mut aytbc);
        let temp16alen = scale_expansion_zeroelim(
            &aytbc[..aytbclen],
            2.0 * context.geometry.ady,
            &mut scratch.temp16a,
        );
        let aytbblen = scale_expansion_zeroelim(&context.bb, context.tails.adytail, &mut aytbb);
        let temp16blen = scale_expansion_zeroelim(
            &aytbb[..aytbblen],
            context.geometry.cdx,
            &mut scratch.temp16b,
        );
        let aytcclen = scale_expansion_zeroelim(&context.cc, context.tails.adytail, &mut aytcc);
        let temp16clen = scale_expansion_zeroelim(
            &aytcc[..aytcclen],
            -context.geometry.bdx,
            &mut scratch.temp16c,
        );
        let temp32alen = fast_expansion_sum_zeroelim(
            &scratch.temp16a[..temp16alen],
            &scratch.temp16b[..temp16blen],
            &mut scratch.temp32a,
        );
        let temp48len = fast_expansion_sum_zeroelim(
            &scratch.temp16c[..temp16clen],
            &scratch.temp32a[..temp32alen],
            &mut scratch.temp48,
        );
        final_expansion.append_expansion(&scratch.temp48[..temp48len]);
    }
    let mut bxtcalen = 8;
    let mut bxtca = [0.; 8];
    if context.tails.bdxtail != 0.0 {
        bxtcalen =
            scale_expansion_zeroelim(&context.geometry.ca, context.tails.bdxtail, &mut bxtca);
        let temp16alen = scale_expansion_zeroelim(
            &bxtca[..bxtcalen],
            2.0 * context.geometry.bdx,
            &mut scratch.temp16a,
        );
        let bxtaalen = scale_expansion_zeroelim(&context.aa, context.tails.bdxtail, &mut bxtaa);
        let temp16blen = scale_expansion_zeroelim(
            &bxtaa[..bxtaalen],
            context.geometry.cdy,
            &mut scratch.temp16b,
        );
        let bxtcclen = scale_expansion_zeroelim(&context.cc, context.tails.bdxtail, &mut bxtcc);
        let temp16clen = scale_expansion_zeroelim(
            &bxtcc[..bxtcclen],
            -context.geometry.ady,
            &mut scratch.temp16c,
        );
        let temp32alen = fast_expansion_sum_zeroelim(
            &scratch.temp16a[..temp16alen],
            &scratch.temp16b[..temp16blen],
            &mut scratch.temp32a,
        );
        let temp48len = fast_expansion_sum_zeroelim(
            &scratch.temp16c[..temp16clen],
            &scratch.temp32a[..temp32alen],
            &mut scratch.temp48,
        );
        final_expansion.append_expansion(&scratch.temp48[..temp48len]);
    }
    let mut bytcalen = 8;
    let mut bytca = [0.; 8];
    if context.tails.bdytail != 0.0 {
        bytcalen =
            scale_expansion_zeroelim(&context.geometry.ca, context.tails.bdytail, &mut bytca);
        let temp16alen = scale_expansion_zeroelim(
            &bytca[..bytcalen],
            2.0 * context.geometry.bdy,
            &mut scratch.temp16a,
        );
        let bytcclen = scale_expansion_zeroelim(&context.cc, context.tails.bdytail, &mut bytcc);
        let temp16blen = scale_expansion_zeroelim(
            &bytcc[..bytcclen],
            context.geometry.adx,
            &mut scratch.temp16b,
        );
        let bytaalen = scale_expansion_zeroelim(&context.aa, context.tails.bdytail, &mut bytaa);
        let temp16clen = scale_expansion_zeroelim(
            &bytaa[..bytaalen],
            -context.geometry.cdx,
            &mut scratch.temp16c,
        );
        let temp32alen = fast_expansion_sum_zeroelim(
            &scratch.temp16a[..temp16alen],
            &scratch.temp16b[..temp16blen],
            &mut scratch.temp32a,
        );
        let temp48len = fast_expansion_sum_zeroelim(
            &scratch.temp16c[..temp16clen],
            &scratch.temp32a[..temp32alen],
            &mut scratch.temp48,
        );
        final_expansion.append_expansion(&scratch.temp48[..temp48len]);
    }
    let cxtablen = 8;
    let mut cxtab = [0.; 8];
    if context.tails.cdxtail != 0.0 {
        let cxtablen =
            scale_expansion_zeroelim(&context.geometry.ab, context.tails.cdxtail, &mut cxtab);
        let temp16alen = scale_expansion_zeroelim(
            &cxtab[..cxtablen],
            2.0 * context.geometry.cdx,
            &mut scratch.temp16a,
        );
        let cxtbblen = scale_expansion_zeroelim(&context.bb, context.tails.cdxtail, &mut cxtbb);
        let temp16blen = scale_expansion_zeroelim(
            &cxtbb[..cxtbblen],
            context.geometry.ady,
            &mut scratch.temp16b,
        );
        let cxtaalen = scale_expansion_zeroelim(&context.aa, context.tails.cdxtail, &mut cxtaa);
        let temp16clen = scale_expansion_zeroelim(
            &cxtaa[..cxtaalen],
            -context.geometry.bdy,
            &mut scratch.temp16c,
        );
        let temp32alen = fast_expansion_sum_zeroelim(
            &scratch.temp16a[..temp16alen],
            &scratch.temp16b[..temp16blen],
            &mut scratch.temp32a,
        );
        let temp48len = fast_expansion_sum_zeroelim(
            &scratch.temp16c[..temp16clen],
            &scratch.temp32a[..temp32alen],
            &mut scratch.temp48,
        );
        final_expansion.append_expansion(&scratch.temp48[..temp48len]);
    }
    let mut cytablen = 8;
    let mut cytab = [0.; 8];
    if context.tails.cdytail != 0.0 {
        cytablen =
            scale_expansion_zeroelim(&context.geometry.ab, context.tails.cdytail, &mut cytab);
        let temp16alen = scale_expansion_zeroelim(
            &cytab[..cytablen],
            2.0 * context.geometry.cdy,
            &mut scratch.temp16a,
        );
        let cytaalen = scale_expansion_zeroelim(&context.aa, context.tails.cdytail, &mut cytaa);
        let temp16blen = scale_expansion_zeroelim(
            &cytaa[..cytaalen],
            context.geometry.bdx,
            &mut scratch.temp16b,
        );
        let cytbblen = scale_expansion_zeroelim(&context.bb, context.tails.cdytail, &mut cytbb);
        let temp16clen = scale_expansion_zeroelim(
            &cytbb[..cytbblen],
            -context.geometry.adx,
            &mut scratch.temp16c,
        );
        let temp32alen = fast_expansion_sum_zeroelim(
            &scratch.temp16a[..temp16alen],
            &scratch.temp16b[..temp16blen],
            &mut scratch.temp32a,
        );
        let temp48len = fast_expansion_sum_zeroelim(
            &scratch.temp16c[..temp16clen],
            &scratch.temp32a[..temp32alen],
            &mut scratch.temp48,
        );
        final_expansion.append_expansion(&scratch.temp48[..temp48len]);
    }
    FirstOrderCrossExpansions {
        axtbc,
        axtbclen,
        aytbc,
        aytbclen,
        bxtca,
        bxtcalen,
        bytca,
        bytcalen,
        cxtab,
        cxtablen,
        cytab,
        cytablen,
    }
}
