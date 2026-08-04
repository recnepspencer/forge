//! A-vertex second-order exact-tail contributions.

use super::super::super::super::expansion::{
    fast_expansion_sum_zeroelim, scale_expansion_zeroelim, two_product, two_two_diff, two_two_sum,
};
use super::super::final_expansion::FinalExpansion;
use super::super::first_order::FirstOrderCrossExpansions;
use super::super::tail_expansion_scratch::TailExpansionScratch;
use super::ExactTailContext;

pub(super) fn apply(
    context: &ExactTailContext,
    first: &FirstOrderCrossExpansions,
    final_expansion: &mut FinalExpansion,
    scratch: &mut TailExpansionScratch,
) {
    if context.tails.adxtail != 0.0 || context.tails.adytail != 0.0 {
        let mut bctlen = 1;
        let mut bcttlen = 1;
        let mut bct: [f64; 8] = [0.; 8];
        let mut bctt: [f64; 4] = [0.; 4];
        if context.tails.bdxtail != 0.0
            || context.tails.bdytail != 0.0
            || context.tails.cdxtail != 0.0
            || context.tails.cdytail != 0.0
        {
            let [ti0, ti1] = two_product(context.tails.bdxtail, context.geometry.cdy);
            let [tj0, tj1] = two_product(context.geometry.bdx, context.tails.cdytail);
            let u = two_two_sum(ti1, ti0, tj1, tj0);
            let negate = -context.geometry.bdy;
            let [ti0, ti1] = two_product(context.tails.cdxtail, negate);
            let negate = -context.tails.bdytail;
            let [tj0, tj1] = two_product(context.geometry.cdx, negate);
            let v = two_two_sum(ti1, ti0, tj1, tj0);
            bctlen = fast_expansion_sum_zeroelim(&u, &v, &mut bct);
            let [ti0, ti1] = two_product(context.tails.bdxtail, context.tails.cdytail);
            let [tj0, tj1] = two_product(context.tails.cdxtail, context.tails.bdytail);
            bctt = two_two_diff(ti1, ti0, tj1, tj0);
            bcttlen = 4;
        }
        if context.tails.adxtail != 0.0 {
            let temp16alen = scale_expansion_zeroelim(
                &first.axtbc[..first.axtbclen],
                context.tails.adxtail,
                &mut scratch.temp16a,
            );
            let axtbctlen = scale_expansion_zeroelim(
                &bct[..bctlen],
                context.tails.adxtail,
                &mut scratch.temp16b,
            );
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..axtbctlen],
                2.0 * context.geometry.adx,
                &mut scratch.temp32a,
            );
            let temp48len = fast_expansion_sum_zeroelim(
                &scratch.temp16a[..temp16alen],
                &scratch.temp32a[..temp32alen],
                &mut scratch.temp48,
            );
            final_expansion.append_expansion(&scratch.temp48[..temp48len]);
            if context.tails.bdytail != 0.0 {
                let temp8len = scale_expansion_zeroelim(
                    &context.cc,
                    context.tails.adxtail,
                    &mut scratch.temp8,
                );
                let temp16alen = scale_expansion_zeroelim(
                    &scratch.temp8[..temp8len],
                    context.tails.bdytail,
                    &mut scratch.temp16a,
                );
                final_expansion.append_expansion(&scratch.temp16a[..temp16alen]);
            }
            if context.tails.cdytail != 0.0 {
                let temp8len = scale_expansion_zeroelim(
                    &context.bb,
                    -context.tails.adxtail,
                    &mut scratch.temp8,
                );
                let temp16alen = scale_expansion_zeroelim(
                    &scratch.temp8[..temp8len],
                    context.tails.cdytail,
                    &mut scratch.temp16a,
                );
                final_expansion.append_expansion(&scratch.temp16a[..temp16alen]);
            }
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..axtbctlen],
                context.tails.adxtail,
                &mut scratch.temp32a,
            );
            let axtbcttlen = scale_expansion_zeroelim(
                &bctt[..bcttlen],
                context.tails.adxtail,
                &mut scratch.temp16c,
            );
            let temp16alen = scale_expansion_zeroelim(
                &scratch.temp16c[..axtbcttlen],
                2.0 * context.geometry.adx,
                &mut scratch.temp16a,
            );
            let temp16blen = scale_expansion_zeroelim(
                &scratch.temp16c[..axtbcttlen],
                context.tails.adxtail,
                &mut scratch.temp16b,
            );
            let temp32blen = fast_expansion_sum_zeroelim(
                &scratch.temp16a[..temp16alen],
                &scratch.temp16b[..temp16blen],
                &mut scratch.temp32b,
            );
            let temp64len = fast_expansion_sum_zeroelim(
                &scratch.temp32a[..temp32alen],
                &scratch.temp32b[..temp32blen],
                &mut scratch.temp64,
            );
            final_expansion.append_expansion(&scratch.temp64[..temp64len]);
        }
        if context.tails.adytail != 0.0 {
            let temp16alen = scale_expansion_zeroelim(
                &first.aytbc[..first.aytbclen],
                context.tails.adytail,
                &mut scratch.temp16a,
            );
            let aytbctlen = scale_expansion_zeroelim(
                &bct[..bctlen],
                context.tails.adytail,
                &mut scratch.temp16b,
            );
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..aytbctlen],
                2.0 * context.geometry.ady,
                &mut scratch.temp32a,
            );
            let temp48len = fast_expansion_sum_zeroelim(
                &scratch.temp16a[..temp16alen],
                &scratch.temp32a[..temp32alen],
                &mut scratch.temp48,
            );
            final_expansion.append_expansion(&scratch.temp48[..temp48len]);
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..aytbctlen],
                context.tails.adytail,
                &mut scratch.temp32a,
            );
            let aytbcttlen = scale_expansion_zeroelim(
                &bctt[..bcttlen],
                context.tails.adytail,
                &mut scratch.temp16c,
            );
            let temp16alen = scale_expansion_zeroelim(
                &scratch.temp16c[..aytbcttlen],
                2.0 * context.geometry.ady,
                &mut scratch.temp16a,
            );
            let temp16blen = scale_expansion_zeroelim(
                &scratch.temp16c[..aytbcttlen],
                context.tails.adytail,
                &mut scratch.temp16b,
            );
            let temp32blen = fast_expansion_sum_zeroelim(
                &scratch.temp16a[..temp16alen],
                &scratch.temp16b[..temp16blen],
                &mut scratch.temp32b,
            );
            let temp64len = fast_expansion_sum_zeroelim(
                &scratch.temp32a[..temp32alen],
                &scratch.temp32b[..temp32blen],
                &mut scratch.temp64,
            );
            final_expansion.append_expansion(&scratch.temp64[..temp64len]);
        }
    }
}
