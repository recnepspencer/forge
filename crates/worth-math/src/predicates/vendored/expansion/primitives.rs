//! Vendored expansion primitives.

use super::super::parameters::PARAMS;

#[inline]
pub(in crate::predicates) fn fast_two_sum_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt: f64 = x - a;
    b - bvirt
}

#[inline]
pub(in crate::predicates) fn fast_two_sum(a: f64, b: f64) -> [f64; 2] {
    let x: f64 = a + b;
    [fast_two_sum_tail(a, b, x), x]
}

#[inline]
pub(in crate::predicates) fn fast_two_diff_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt: f64 = a - x;
    return bvirt - b;
}

#[inline]
pub(in crate::predicates) fn fast_two_diff(a: f64, b: f64) -> [f64; 2] {
    let x: f64 = a - b;
    [fast_two_diff_tail(a, b, x), x]
}

#[inline]
pub(in crate::predicates) fn two_sum_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt: f64 = x - a;
    let avirt: f64 = x - bvirt;
    let bround: f64 = b - bvirt;
    let around: f64 = a - avirt;
    around + bround
}

#[inline]
pub(in crate::predicates) fn two_sum(a: f64, b: f64) -> [f64; 2] {
    let x: f64 = a + b;
    [two_sum_tail(a, b, x), x]
}

#[inline]
pub(in crate::predicates) fn two_diff_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt: f64 = a - x;
    let avirt: f64 = x + bvirt;
    let bround: f64 = bvirt - b;
    let around: f64 = a - avirt;
    around + bround
}

#[inline]
pub(in crate::predicates) fn two_diff(a: f64, b: f64) -> [f64; 2] {
    let x: f64 = a - b;
    [two_diff_tail(a, b, x), x]
}

#[inline]
pub(in crate::predicates) fn split(a: f64) -> [f64; 2] {
    let c: f64 = PARAMS.splitter * a;
    let abig: f64 = c - a;
    let ahi = c - abig;
    let alo = a - ahi;
    [alo, ahi]
}

#[inline]
pub(in crate::predicates) fn two_product_tail(a: f64, b: f64, x: f64) -> f64 {
    let [alo, ahi] = split(a);
    let [blo, bhi] = split(b);
    let err1: f64 = x - ahi * bhi;
    let err2: f64 = err1 - alo * bhi;
    let err3: f64 = err2 - ahi * blo;
    alo * blo - err3
}

#[inline]
pub(in crate::predicates) fn two_product(a: f64, b: f64) -> [f64; 2] {
    let x = a * b;
    [two_product_tail(a, b, x), x]
}

/// Same as [`two_product`] where one of the inputs has
/// already been split.
///
/// Avoids redundant splitting.
#[inline]
pub(in crate::predicates) fn two_product_presplit(a: f64, b: f64, bhi: f64, blo: f64) -> [f64; 2] {
    let x = a * b;
    let [alo, ahi] = split(a);
    let err1: f64 = x - ahi * bhi;
    let err2: f64 = err1 - alo * bhi;
    let err3: f64 = err2 - ahi * blo;
    [alo * blo - err3, x]
}

/// Same as [`two_product`] where both of the inputs have
/// already been split.
///
/// Avoids redundant splitting.
#[inline]
pub(in crate::predicates) fn two_product_2presplit(
    a: f64,
    ahi: f64,
    alo: f64,
    b: f64,
    bhi: f64,
    blo: f64,
) -> [f64; 2] {
    let x = a * b;
    let err1: f64 = x - ahi * bhi;
    let err2: f64 = err1 - alo * bhi;
    let err3: f64 = err2 - ahi * blo;
    [alo * blo - err3, x]
}

#[inline]
pub(in crate::predicates) fn square_tail(a: f64, x: f64) -> f64 {
    let [alo, ahi] = split(a);
    let err1: f64 = x - ahi * ahi;
    let err3: f64 = err1 - (ahi + ahi) * alo;
    alo * alo - err3
}

/// Squaring can be done more quickly than [`two_product`].
#[inline]
pub(in crate::predicates) fn square(a: f64) -> [f64; 2] {
    let x = a * a;
    [square_tail(a, x), x]
}
