//! B-vertex second-order exact-tail contributions.

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
    if context.tails.bdxtail != 0.0 || context.tails.bdytail != 0.0 {
        let mut catlen = 1;
        let mut cattlen = 1;
        let mut cat: [f64; 8] = [0.; 8];
        let mut catt: [f64; 4] = [0.; 4];
        if context.tails.cdxtail != 0.0
            || context.tails.cdytail != 0.0
            || context.tails.adxtail != 0.0
            || context.tails.adytail != 0.0
        {
            let [ti0, ti1] = two_product(context.tails.cdxtail, context.geometry.ady);
            let [tj0, tj1] = two_product(context.geometry.cdx, context.tails.adytail);
            let u = two_two_sum(ti1, ti0, tj1, tj0);
            let negate = -context.geometry.cdy;
            let [ti0, ti1] = two_product(context.tails.adxtail, negate);
            let negate = -context.tails.cdytail;
            let [tj0, tj1] = two_product(context.geometry.adx, negate);
            let v = two_two_sum(ti1, ti0, tj1, tj0);
            catlen = fast_expansion_sum_zeroelim(&u, &v, &mut cat);
            let [ti0, ti1] = two_product(context.tails.cdxtail, context.tails.adytail);
            let [tj0, tj1] = two_product(context.tails.adxtail, context.tails.cdytail);
            catt = two_two_diff(ti1, ti0, tj1, tj0);
            cattlen = 4;
        }
        if context.tails.bdxtail != 0.0 {
            let temp16alen = scale_expansion_zeroelim(
                &first.bxtca[..first.bxtcalen],
                context.tails.bdxtail,
                &mut scratch.temp16a,
            );
            let bxtcatlen = scale_expansion_zeroelim(
                &cat[..catlen],
                context.tails.bdxtail,
                &mut scratch.temp16b,
            );
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..bxtcatlen],
                2.0 * context.geometry.bdx,
                &mut scratch.temp32a,
            );
            let temp48len = fast_expansion_sum_zeroelim(
                &scratch.temp16a[..temp16alen],
                &scratch.temp32a[..temp32alen],
                &mut scratch.temp48,
            );
            final_expansion.append_expansion(&scratch.temp48[..temp48len]);
            if context.tails.cdytail != 0.0 {
                let temp8len = scale_expansion_zeroelim(
                    &context.aa,
                    context.tails.bdxtail,
                    &mut scratch.temp8,
                );
                let temp16alen = scale_expansion_zeroelim(
                    &scratch.temp8[..temp8len],
                    context.tails.cdytail,
                    &mut scratch.temp16a,
                );
                final_expansion.append_expansion(&scratch.temp16a[..temp16alen]);
            }
            if context.tails.adytail != 0.0 {
                let temp8len = scale_expansion_zeroelim(
                    &context.cc,
                    -context.tails.bdxtail,
                    &mut scratch.temp8,
                );
                let temp16alen = scale_expansion_zeroelim(
                    &scratch.temp8[..temp8len],
                    context.tails.adytail,
                    &mut scratch.temp16a,
                );
                final_expansion.append_expansion(&scratch.temp16a[..temp16alen]);
            }
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..bxtcatlen],
                context.tails.bdxtail,
                &mut scratch.temp32a,
            );
            let bxtcattlen = scale_expansion_zeroelim(
                &catt[..cattlen],
                context.tails.bdxtail,
                &mut scratch.temp16c,
            );
            let temp16alen = scale_expansion_zeroelim(
                &scratch.temp16c[..bxtcattlen],
                2.0 * context.geometry.bdx,
                &mut scratch.temp16a,
            );
            let temp16blen = scale_expansion_zeroelim(
                &scratch.temp16c[..bxtcattlen],
                context.tails.bdxtail,
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
        if context.tails.bdytail != 0.0 {
            let temp16alen = scale_expansion_zeroelim(
                &first.bytca[..first.bytcalen],
                context.tails.bdytail,
                &mut scratch.temp16a,
            );
            let bytcatlen = scale_expansion_zeroelim(
                &cat[..catlen],
                context.tails.bdytail,
                &mut scratch.temp16b,
            );
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..bytcatlen],
                2.0 * context.geometry.bdy,
                &mut scratch.temp32a,
            );
            let temp48len = fast_expansion_sum_zeroelim(
                &scratch.temp16a[..temp16alen],
                &scratch.temp32a[..temp32alen],
                &mut scratch.temp48,
            );
            final_expansion.append_expansion(&scratch.temp48[..temp48len]);
            let temp32alen = scale_expansion_zeroelim(
                &scratch.temp16b[..bytcatlen],
                context.tails.bdytail,
                &mut scratch.temp32a,
            );
            let bytcattlen = scale_expansion_zeroelim(
                &catt[..cattlen],
                context.tails.bdytail,
                &mut scratch.temp16c,
            );
            let temp16alen = scale_expansion_zeroelim(
                &scratch.temp16c[..bytcattlen],
                2.0 * context.geometry.bdy,
                &mut scratch.temp16a,
            );
            let temp16blen = scale_expansion_zeroelim(
                &scratch.temp16c[..bytcattlen],
                context.tails.bdytail,
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
