//! Vendored expansion fast sum.

use super::primitives::{fast_two_sum, two_sum};

///  Sums two expansions, eliminating zero components from the output expansion.
///
///  Sets `h = e + f`.  See [the
///  paper](http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf) for details.
///
///  If round-to-even is used (as with IEEE 754), maintains the strongly
///  nonoverlapping property.  (That is, if `e` is strongly nonoverlapping, `h`
///  will be also.)  Does NOT maintain the nonoverlapping or nonadjacent
///  properties.
#[inline]
pub(in crate::predicates) fn fast_expansion_sum_zeroelim(
    e: &[f64],
    f: &[f64],
    h: &mut [f64],
) -> usize {
    let mut q;
    let mut findex = 0;
    let mut eindex = findex;

    let enow = e[0];
    let fnow = f[0];
    if (fnow > enow) == (fnow > -enow) {
        q = enow;
        eindex += 1;
    } else {
        q = fnow;
        findex += 1;
    }

    let mut hindex = 0;
    if eindex < e.len() && findex < f.len() {
        let enow = e[eindex];
        let fnow = f[findex];
        let [hh, q_new] = if (fnow > enow) == (fnow > -enow) {
            eindex += 1;
            fast_two_sum(enow, q)
        } else {
            findex += 1;
            fast_two_sum(fnow, q)
        };
        q = q_new;
        if hh != 0.0f64 {
            h[hindex] = hh;
            hindex += 1;
        }
        while eindex < e.len() && findex < f.len() {
            let enow = e[eindex];
            let fnow = f[findex];
            let [hh, q_new] = if (fnow > enow) == (fnow > -enow) {
                eindex += 1;
                two_sum(q, enow)
            } else {
                findex += 1;
                two_sum(q, fnow)
            };
            q = q_new;
            if hh != 0.0f64 {
                h[hindex] = hh;
                hindex += 1;
            }
        }
    }
    while eindex < e.len() {
        let [hh, q_new] = two_sum(q, e[eindex]);
        eindex += 1;
        q = q_new;
        if hh != 0.0f64 {
            h[hindex] = hh;
            hindex += 1;
        }
    }
    while findex < f.len() {
        let [hh, q_new] = two_sum(q, f[findex]);
        findex += 1;
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
