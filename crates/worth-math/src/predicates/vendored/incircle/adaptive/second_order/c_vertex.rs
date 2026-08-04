//! C-vertex second-order exact-tail contributions.

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
    if context.tails.cdxtail != 0.0 || context.tails.cdytail != 0.0 {
        let mut abtlen = 1;
        let mut abttlen = 1;
        let mut abt: [f64; 8] = [0.; 8];
        let mut abtt: [f64; 4] = [0.; 4];
        if context.tails.adxtail != 0.0
            || context.tails.adytail != 0.0
            || context.tails.bdxtail != 0.0
            || context.tails.bdytail != 0.0
        {
            let [ti0, ti1] = two_product(context.tails.adxtail, context.geometry.bdy);
            let [tj0, tj1] = two_product(context.geometry.adx, context.tails.bdytail);
            let u = two_two_sum(ti1, ti0, tj1, tj0);
            let negate = -context.geometry.ady;
            let [ti0, ti1] = two_product(context.tails.bdxtail, negate);
            let negate = -context.tails.adytail;
            let [tj0, tj1] = two_product(context.geometry.bdx, negate);
            let v = two_two_sum(ti1, ti0, tj1, tj0);
            abtlen = fast_expansion_sum_zeroelim(&u, &v, &mut abt);
            let [ti0, ti1] = two_product(context.tails.adxtail, context.tails.bdytail);
            let [tj0, tj1] = two_product(context.tails.bdxtail, context.tails.adytail);
            abtt = two_two_diff(ti1, ti0, tj1, tj0);
            abttlen = 4;
        }
        if context.tails.cdxtail != 0.0 {
            let temp16alen = scale_expansion_zeroelim(
                &first.cxtab[..first.cxtablen],
                context.tails.cdxtail,
                &mut scratch.temp16a,
            );
            let cxtabtlen = scale_expansion_zeroelim(
                &abt[..abtlen],
                context.tails.cdxtail,
                &mut scratch.temp16b,
            );
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..cxtabtlen],
                2.0 * context.geometry.cdx,
                &mut scratch.temp32a,
            );
            let temp48len = fast_expansion_sum_zeroelim(
                &scratch.temp16a[..temp16alen],
                &scratch.temp32a[..temp32alen],
                &mut scratch.temp48,
            );
            final_expansion.append_expansion(&scratch.temp48[..temp48len]);
            if context.tails.adytail != 0.0 {
                let temp8len = scale_expansion_zeroelim(
                    &context.bb,
                    context.tails.cdxtail,
                    &mut scratch.temp8,
                );
                let temp16alen = scale_expansion_zeroelim(
                    &scratch.temp8[..temp8len],
                    context.tails.adytail,
                    &mut scratch.temp16a,
                );
                final_expansion.append_expansion(&scratch.temp16a[..temp16alen]);
            }
            if context.tails.bdytail != 0.0 {
                let temp8len = scale_expansion_zeroelim(
                    &context.aa,
                    -context.tails.cdxtail,
                    &mut scratch.temp8,
                );
                let temp16alen = scale_expansion_zeroelim(
                    &scratch.temp8[..temp8len],
                    context.tails.bdytail,
                    &mut scratch.temp16a,
                );
                final_expansion.append_expansion(&scratch.temp16a[..temp16alen]);
            }
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..cxtabtlen],
                context.tails.cdxtail,
                &mut scratch.temp32a,
            );
            let cxtabttlen = scale_expansion_zeroelim(
                &abtt[..abttlen],
                context.tails.cdxtail,
                &mut scratch.temp16c,
            );
            let temp16alen = scale_expansion_zeroelim(
                &scratch.temp16c[..cxtabttlen],
                2.0 * context.geometry.cdx,
                &mut scratch.temp16a,
            );
            let temp16blen = scale_expansion_zeroelim(
                &scratch.temp16c[..cxtabttlen],
                context.tails.cdxtail,
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
        if context.tails.cdytail != 0.0 {
            let temp16alen = scale_expansion_zeroelim(
                &first.cytab[..first.cytablen],
                context.tails.cdytail,
                &mut scratch.temp16a,
            );
            let cytabtlen = scale_expansion_zeroelim(
                &abt[..abtlen],
                context.tails.cdytail,
                &mut scratch.temp16b,
            );
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..cytabtlen],
                2.0 * context.geometry.cdy,
                &mut scratch.temp32a,
            );
            let temp48len = fast_expansion_sum_zeroelim(
                &scratch.temp16a[..temp16alen],
                &scratch.temp32a[..temp32alen],
                &mut scratch.temp48,
            );
            final_expansion.append_expansion(&scratch.temp48[..temp48len]);
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..cytabtlen],
                context.tails.cdytail,
                &mut scratch.temp32a,
            );
            let cytabttlen = scale_expansion_zeroelim(
                &abtt[..abttlen],
                context.tails.cdytail,
                &mut scratch.temp16c,
            );
            let temp16alen = scale_expansion_zeroelim(
                &scratch.temp16c[..cytabttlen],
                2.0 * context.geometry.cdy,
                &mut scratch.temp16a,
            );
            let temp16blen = scale_expansion_zeroelim(
                &scratch.temp16c[..cytabttlen],
                context.tails.cdytail,
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
