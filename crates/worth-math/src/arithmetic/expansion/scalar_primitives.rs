//! Error-free scalar transformations used by expansion arithmetic.

use super::constants::SPLITTER;

/// Absolute value without branching (bit manipulation).
#[inline]
pub fn abs(a: f64) -> f64 {
    f64::from_bits(a.to_bits() & 0x7FFF_FFFF_FFFF_FFFF)
}

/// Error-free sum: returns `[roundoff, sum]` where `a + b == sum + roundoff` exactly.
///
/// Requires `|a| >= |b|`.
#[inline]
pub fn fast_two_sum(a: f64, b: f64) -> [f64; 2] {
    let x = a + b;
    let bvirt = x - a;
    [b - bvirt, x]
}

/// Tail of `fast_two_sum` when the sum is already known.
#[inline]
pub fn fast_two_sum_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt = x - a;
    b - bvirt
}

/// Error-free sum: returns `[roundoff, sum]` where `a + b == sum + roundoff` exactly.
///
/// No precondition on relative magnitudes of `a` and `b`.
#[inline]
pub fn two_sum(a: f64, b: f64) -> [f64; 2] {
    let x = a + b;
    let bvirt = x - a;
    let avirt = x - bvirt;
    let bround = b - bvirt;
    let around = a - avirt;
    [around + bround, x]
}

/// Tail of `two_sum` when the sum is already known.
#[inline]
pub fn two_sum_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt = x - a;
    let avirt = x - bvirt;
    let bround = b - bvirt;
    let around = a - avirt;
    around + bround
}

/// Error-free difference: returns `[roundoff, difference]`.
#[inline]
pub fn two_diff(a: f64, b: f64) -> [f64; 2] {
    let x = a - b;
    let bvirt = a - x;
    let avirt = x + bvirt;
    let bround = bvirt - b;
    let around = a - avirt;
    [around + bround, x]
}

/// Tail of `two_diff` when the difference is already known.
#[inline]
pub fn two_diff_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt = a - x;
    let avirt = x + bvirt;
    let bround = bvirt - b;
    let around = a - avirt;
    around + bround
}

/// Dekker split: splits `a` into high and low halves for exact multiplication.
///
/// Returns `[lo, hi]` where `a = hi + lo` and both have ≤26 significant bits.
#[inline]
pub fn split(a: f64) -> [f64; 2] {
    let c = SPLITTER * a;
    let abig = c - a;
    let ahi = c - abig;
    let alo = a - ahi;
    [alo, ahi]
}

/// Error-free product: returns `[roundoff, product]` where `a * b == product + roundoff` exactly.
#[inline]
pub fn two_product(a: f64, b: f64) -> [f64; 2] {
    let x = a * b;
    let [alo, ahi] = split(a);
    let [blo, bhi] = split(b);
    let err1 = x - ahi * bhi;
    let err2 = err1 - alo * bhi;
    let err3 = err2 - ahi * blo;
    [alo * blo - err3, x]
}

/// Tail of `two_product` when the product is already known.
#[inline]
pub fn two_product_tail(a: f64, b: f64, x: f64) -> f64 {
    let [alo, ahi] = split(a);
    let [blo, bhi] = split(b);
    let err1 = x - ahi * bhi;
    let err2 = err1 - alo * bhi;
    let err3 = err2 - ahi * blo;
    alo * blo - err3
}

/// Error-free product where `b` has already been split.
#[inline]
pub fn two_product_presplit(a: f64, b: f64, bhi: f64, blo: f64) -> [f64; 2] {
    let x = a * b;
    let [alo, ahi] = split(a);
    let err1 = x - ahi * bhi;
    let err2 = err1 - alo * bhi;
    let err3 = err2 - ahi * blo;
    [alo * blo - err3, x]
}

/// Error-free product where both inputs have already been split.
#[inline]
pub fn two_product_2presplit(a: f64, ahi: f64, alo: f64, b: f64, bhi: f64, blo: f64) -> [f64; 2] {
    let x = a * b;
    let err1 = x - ahi * bhi;
    let err2 = err1 - alo * bhi;
    let err3 = err2 - ahi * blo;
    [alo * blo - err3, x]
}

/// Exact square: faster than `two_product(a, a)`.
#[inline]
pub fn square(a: f64) -> [f64; 2] {
    let x = a * a;
    let [alo, ahi] = split(a);
    let err1 = x - ahi * ahi;
    let err3 = err1 - (ahi + ahi) * alo;
    [alo * alo - err3, x]
}
