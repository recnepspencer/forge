//! Vendored expansion sum.

use super::primitives::two_sum;

///  Sums two expansions.
///
///  Sets `h = e + f`. See [the paper](http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf) for details.
///
///  Maintains the nonoverlapping property.  If round-to-even is used (as
///  with IEEE 754), maintains the nonadjacent property as well.  (That is,
///  if `e` has one of these properties, so will `h`.)  Does NOT maintain the
///  strongly nonoverlapping property.
#[inline]
pub(in crate::predicates) fn expansion_sum(e: &[f64], f: &[f64], h: &mut [f64]) -> usize {
    let mut q = f[0];
    let mut hindex = 0;
    while hindex < e.len() {
        let [hh, qnew] = two_sum(q, e[hindex]);
        h[hindex] = hh;
        q = qnew;
        hindex += 1
    }
    h[hindex] = q;
    let mut hlast = hindex;
    let mut findex = 1;
    while findex < f.len() {
        q = f[findex];
        hindex = findex;
        while hindex <= hlast {
            let [hh, qnew] = two_sum(q, h[hindex]);
            h[hindex] = hh;
            q = qnew;
            hindex += 1
        }
        hlast += 1;
        h[hlast] = q;
        findex += 1
    }
    hlast + 1
}

///  Sums two expansions, eliminating zero components from the output expansion.
///
///  Sets `h = e + f`. See [the
///  paper](http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf) for details.
///
///  Maintains the nonoverlapping property.  If round-to-even is used (as
///  with IEEE 754), maintains the nonadjacent property as well.  (That is,
///  if `e` has one of these properties, so will `h`.)  Does NOT maintain the
///  strongly nonoverlapping property.
#[inline]
pub(in crate::predicates) fn expansion_sum_zeroelim1(e: &[f64], f: &[f64], h: &mut [f64]) -> usize {
    let mut q = f[0];
    let mut hindex = 0;
    while hindex < e.len() {
        let [hh, qnew] = two_sum(q, e[hindex]);
        h[hindex] = hh;
        q = qnew;
        hindex += 1
    }
    h[hindex] = q;
    let mut hlast = hindex;
    let mut findex = 1;
    while findex < f.len() {
        q = f[findex];
        hindex = findex;
        while hindex <= hlast {
            let [hh, qnew] = two_sum(q, h[hindex]);
            h[hindex] = hh;
            q = qnew;
            hindex += 1
        }
        hlast += 1;
        h[hlast] = q;
        findex += 1
    }
    let mut hindex: isize = -1;
    let mut index = 0;
    while index <= hlast {
        let hnow = h[index];
        if hnow != 0.0 {
            hindex += 1;
            h[hindex as usize] = hnow
        }
        index += 1
    }
    if hindex == -1 {
        1
    } else {
        hindex as usize + 1
    }
}

///  Sums two expansions, eliminating zero components from the output expansion.
///
///  Sets `h = e + f`. See [the
///  paper](http://www.cs.berkeley.edu/~jrs/papers/robustr.pdf) for details.
///
///  Maintains the nonoverlapping property.  If round-to-even is used (as
///  with IEEE 754), maintains the nonadjacent property as well.  (That is,
///  if `e` has one of these properties, so will `h`.)  Does NOT maintain the
///  strongly nonoverlapping property.
#[inline]
pub(in crate::predicates) fn expansion_sum_zeroelim2(e: &[f64], f: &[f64], h: &mut [f64]) -> usize {
    let mut hindex = 0;
    let mut q = f[0];
    let mut eindex = 0;
    while eindex < e.len() {
        let [hh, qnew] = two_sum(q, e[eindex]);
        q = qnew;
        if hh != 0.0 {
            h[hindex] = hh;
            hindex += 1;
        }
        eindex += 1
    }
    h[hindex] = q;
    let mut hlast = hindex;
    let mut findex = 1;
    while findex < f.len() {
        hindex = 0;
        q = f[findex];
        eindex = 0;
        while eindex <= hlast {
            let [hh, qnew] = two_sum(q, h[eindex]);
            q = qnew;
            if hh != 0.0 {
                h[hindex] = hh;
                hindex += 1;
            }
            eindex += 1
        }
        h[hindex] = q;
        hlast = hindex;
        findex += 1
    }
    hlast + 1
}
