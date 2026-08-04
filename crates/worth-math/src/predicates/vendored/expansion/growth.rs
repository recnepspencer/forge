//! Vendored expansion growth.

use super::primitives::two_sum;

///  Adds a scalar to an expansion.
///
///  Sets `h = e + b`.  See [the paper](http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf) for details.
///
///  Maintains the nonoverlapping property.  If round-to-even is used (as
///  with IEEE 754), maintains the strongly nonoverlapping and nonadjacent
///  properties as well.  (That is, if `e` has one of these properties, so
///  will `h`.)
#[inline]
pub(in crate::predicates) fn grow_expansion(e: &[f64], b: f64, h: &mut [f64]) -> usize {
    let mut q = b;
    let mut eindex = 0;
    while eindex < e.len() {
        let [hnew, q_new] = two_sum(q, e[eindex]);
        q = q_new;
        h[eindex] = hnew;
        eindex += 1;
    }
    h[eindex] = q;
    eindex + 1
}

///  Adds a scalar to an expansion, eliminating zero components from the output
///  expansion.
///
///  Sets `h = e + b`. See [the paper](http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf) for details.
///
///  Maintains the nonoverlapping property.  If round-to-even is used (as
///  with IEEE 754), maintains the strongly nonoverlapping and nonadjacent
///  properties as well.  (That is, if `e` has one of these properties, so
///  will `h`.)
#[inline]
pub(in crate::predicates) fn grow_expansion_zeroelim(e: &[f64], b: f64, h: &mut [f64]) -> usize {
    let mut hindex = 0;
    let mut q = b;
    let mut eindex = 0;
    while eindex < e.len() {
        let [hh, q_new] = two_sum(q, e[eindex]);
        q = q_new;
        if hh != 0.0f64 {
            let fresh0 = hindex;
            hindex = hindex + 1;
            h[fresh0] = hh;
        }
        eindex += 1
    }
    if q != 0.0f64 || hindex == 0 {
        let fresh1 = hindex;
        hindex = hindex + 1;
        h[fresh1] = q;
    }
    hindex
}
