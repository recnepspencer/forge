//! Vendored expansion scale.

use super::primitives::{fast_two_sum, split, two_product_presplit, two_sum};

///  Multiply an expansion by a scalar, eliminating zero components from the
///  output expansion.
///
///  Sets `h = be`. See either [\[1\]] or [\[2\]] for details.
///
///  Maintains the nonoverlapping property.  If round-to-even is used (as
///  with IEEE 754), maintains the strongly nonoverlapping and nonadjacent
///  properties as well.  (That is, if `e` has one of these properties, so
///  will `h`.)
///
/// [\[1\]]: http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf
/// [\[2\]]: http://www.cs.berkeley.edu/~jrs/papers/robust-predicates.pdf
pub(in crate::predicates) fn scale_expansion_zeroelim(e: &[f64], b: f64, h: &mut [f64]) -> usize {
    let [blo, bhi] = split(b);
    let [hh, mut q] = two_product_presplit(e[0], b, bhi, blo);

    let mut hindex = 0;
    if hh != 0.0f64 {
        h[hindex] = hh;
        hindex += 1;
    }
    for &enow in e.iter().skip(1) {
        let [product0, product1] = two_product_presplit(enow, b, bhi, blo);
        let [hh, sum] = two_sum(q, product0);
        if hh != 0.0f64 {
            h[hindex] = hh;
            hindex += 1;
        }
        let [hh, q_new] = fast_two_sum(product1, sum);
        q = q_new;
        if hh != 0.0f64 {
            h[hindex] = hh;
            hindex += 1;
        }
    }
    if q != 0.0f64 || hindex == 0 {
        h[hindex] = q;
        hindex += 1;
    }
    hindex
}
