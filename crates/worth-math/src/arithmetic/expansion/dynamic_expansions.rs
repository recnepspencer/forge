//! Variable-width expansion arithmetic with zero elimination.

use super::scalar_primitives::{fast_two_sum, split, two_product_presplit, two_sum};

/// Add a scalar to an expansion, eliminating zero components.
///
/// Sets `h = e + b`. Maintains the non-overlapping property.
#[inline]
pub fn grow_expansion_zeroelim(e: &[f64], b: f64, h: &mut [f64]) -> usize {
    let mut hindex = 0;
    let mut q = b;
    for &enow in e {
        let [hh, q_new] = two_sum(q, enow);
        q = q_new;
        if hh != 0.0 {
            h[hindex] = hh;
            hindex += 1;
        }
    }
    if q != 0.0 || hindex == 0 {
        h[hindex] = q;
        hindex += 1;
    }
    hindex
}

/// Sum two expansions, eliminating zero components.
///
/// Sets `h = e + f`. Maintains the strongly non-overlapping property
/// under IEEE 754 round-to-nearest-even.
///
/// The output buffer `h` must have capacity `e.len() + f.len()`.
#[inline]
pub fn fast_expansion_sum_zeroelim(e: &[f64], f: &[f64], h: &mut [f64]) -> usize {
    let mut cursor = ExpansionMergeCursor::initialize(e, f, h);
    merge_overlapping_terms(&mut cursor);
    merge_remainder_terms(&mut cursor);
    cursor.finish()
}

struct ExpansionMergeCursor<'a> {
    e: &'a [f64],
    f: &'a [f64],
    h: &'a mut [f64],
    eindex: usize,
    findex: usize,
    hindex: usize,
    q: f64,
}

impl<'a> ExpansionMergeCursor<'a> {
    fn initialize(e: &'a [f64], f: &'a [f64], h: &'a mut [f64]) -> Self {
        let q;
        let mut findex = 0;
        let mut eindex = 0;
        let enow = e[0];
        let fnow = f[0];
        if (fnow > enow) == (fnow > -enow) {
            q = enow;
            eindex += 1;
        } else {
            q = fnow;
            findex += 1;
        }
        Self {
            e,
            f,
            h,
            eindex,
            findex,
            hindex: 0,
            q,
        }
    }

    fn append_nonzero(&mut self, hh: f64) {
        if hh != 0.0 {
            self.h[self.hindex] = hh;
            self.hindex += 1;
        }
    }

    fn finish(self) -> usize {
        let mut cursor = self;
        if cursor.q != 0.0 || cursor.hindex == 0 {
            cursor.h[cursor.hindex] = cursor.q;
            cursor.hindex += 1;
        }
        cursor.hindex
    }
}

fn merge_overlapping_terms(cursor: &mut ExpansionMergeCursor<'_>) {
    if cursor.eindex < cursor.e.len() && cursor.findex < cursor.f.len() {
        let enow = cursor.e[cursor.eindex];
        let fnow = cursor.f[cursor.findex];
        let [hh, q_new] = if (fnow > enow) == (fnow > -enow) {
            cursor.eindex += 1;
            fast_two_sum(enow, cursor.q)
        } else {
            cursor.findex += 1;
            fast_two_sum(fnow, cursor.q)
        };
        cursor.q = q_new;
        cursor.append_nonzero(hh);
        while cursor.eindex < cursor.e.len() && cursor.findex < cursor.f.len() {
            let enow = cursor.e[cursor.eindex];
            let fnow = cursor.f[cursor.findex];
            let [hh, q_new] = if (fnow > enow) == (fnow > -enow) {
                cursor.eindex += 1;
                two_sum(cursor.q, enow)
            } else {
                cursor.findex += 1;
                two_sum(cursor.q, fnow)
            };
            cursor.q = q_new;
            cursor.append_nonzero(hh);
        }
    }
}

fn merge_remainder_terms(cursor: &mut ExpansionMergeCursor<'_>) {
    while cursor.eindex < cursor.e.len() {
        let [hh, q_new] = two_sum(cursor.q, cursor.e[cursor.eindex]);
        cursor.eindex += 1;
        cursor.q = q_new;
        cursor.append_nonzero(hh);
    }
    while cursor.findex < cursor.f.len() {
        let [hh, q_new] = two_sum(cursor.q, cursor.f[cursor.findex]);
        cursor.findex += 1;
        cursor.q = q_new;
        cursor.append_nonzero(hh);
    }
}

/// Multiply an expansion by a scalar, eliminating zero components.
///
/// Sets `h = e * b`. Maintains the non-overlapping property.
///
/// The output buffer `h` must have capacity `2 * e.len()`.
pub fn scale_expansion_zeroelim(e: &[f64], b: f64, h: &mut [f64]) -> usize {
    let [blo, bhi] = split(b);
    let [hh, mut q] = two_product_presplit(e[0], b, bhi, blo);

    let mut hindex = 0;
    if hh != 0.0 {
        h[hindex] = hh;
        hindex += 1;
    }
    for &enow in e.iter().skip(1) {
        let [product0, product1] = two_product_presplit(enow, b, bhi, blo);
        let [hh, sum] = two_sum(q, product0);
        if hh != 0.0 {
            h[hindex] = hh;
            hindex += 1;
        }
        let [hh, q_new] = fast_two_sum(product1, sum);
        q = q_new;
        if hh != 0.0 {
            h[hindex] = hh;
            hindex += 1;
        }
    }
    if q != 0.0 || hindex == 0 {
        h[hindex] = q;
        hindex += 1;
    }
    hindex
}

/// Approximate value of an expansion (sum of all components).
///
/// The last component has the highest magnitude and dominates the sum,
/// but summing all components gives a better approximation.
#[inline]
pub fn estimate(e: &[f64]) -> f64 {
    e.iter().sum()
}
