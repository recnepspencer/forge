//! Fixed-width expansion arithmetic.

use super::scalar_primitives::{
    fast_two_sum, split, square, two_diff, two_product, two_product_2presplit,
    two_product_presplit, two_sum,
};

/// Sum a 2-component expansion with a scalar: `[a1, a0] + b`.
#[inline]
pub fn two_one_sum(a1: f64, a0: f64, b: f64) -> [f64; 3] {
    let [x0, _i] = two_sum(a0, b);
    let [x1, x2] = two_sum(a1, _i);
    [x0, x1, x2]
}

/// Difference of a 2-component expansion and a scalar: `[a1, a0] - b`.
#[inline]
pub fn two_one_diff(a1: f64, a0: f64, b: f64) -> [f64; 3] {
    let [x0, _i] = two_diff(a0, b);
    let [x1, x2] = two_sum(a1, _i);
    [x2, x1, x0]
}

/// Sum two 2-component expansions: `[a1, a0] + [b1, b0]`.
#[inline]
pub fn two_two_sum(a1: f64, a0: f64, b1: f64, b0: f64) -> [f64; 4] {
    let [x0, _0, _j] = two_one_sum(a1, a0, b0);
    let [x1, x2, x3] = two_one_sum(_j, _0, b1);
    [x0, x1, x2, x3]
}

/// Difference of two 2-component expansions: `[a1, a0] - [b1, b0]`.
#[inline]
pub fn two_two_diff(a1: f64, a0: f64, b1: f64, b0: f64) -> [f64; 4] {
    let [_j, _0, x0] = two_one_diff(a1, a0, b0);
    let [x3, x2, x1] = two_one_diff(_j, _0, b1);
    [x0, x1, x2, x3]
}

/// Product of a 2-component expansion and a scalar: `[a1, a0] * b`.
#[inline]
pub fn two_one_product(a1: f64, a0: f64, b: f64) -> [f64; 4] {
    let [blo, bhi] = split(b);
    let [x0, _i] = two_product_presplit(a0, b, bhi, blo);
    let [_0, _j] = two_product_presplit(a1, b, bhi, blo);
    let [x1, _k] = two_sum(_i, _0);
    let [x2, x3] = fast_two_sum(_j, _k);
    [x0, x1, x2, x3]
}

/// Product of two 2-component expansions: `[a1, a0] * [b1, b0]`.
#[inline]
pub fn two_two_product(a1: f64, a0: f64, b1: f64, b0: f64) -> [f64; 8] {
    let [a0lo, a0hi] = split(a0);
    let [blo, bhi] = split(b0);
    let [x0, _i] = two_product_2presplit(a0, a0hi, a0lo, b0, bhi, blo);
    let [a1lo, a1hi] = split(a1);
    let [_0, _j] = two_product_2presplit(a1, a1hi, a1lo, b0, bhi, blo);
    let [_1, _k] = two_sum(_i, _0);
    let [_2, _l] = fast_two_sum(_j, _k);
    let [blo, bhi] = split(b1);
    let [_0, _i] = two_product_2presplit(a0, a0hi, a0lo, b1, bhi, blo);
    let [x1, _k] = two_sum(_1, _0);
    let [_1, _j] = two_sum(_2, _k);
    let [_2, _m] = two_sum(_l, _j);
    let [_0, _j] = two_product_2presplit(a1, a1hi, a1lo, b1, bhi, blo);
    let [_0, _n] = two_sum(_i, _0);
    let [x2, _i] = two_sum(_1, _0);
    let [_1, _k] = two_sum(_2, _i);
    let [_2, _l] = two_sum(_m, _k);
    let [_0, _k] = two_sum(_j, _n);
    let [x3, _j] = two_sum(_1, _0);
    let [_1, _i] = two_sum(_2, _j);
    let [_2, _m] = two_sum(_l, _i);
    let [x4, _i] = two_sum(_1, _k);
    let [x5, _k] = two_sum(_2, _i);
    let [x6, x7] = two_sum(_m, _k);
    [x0, x1, x2, x3, x4, x5, x6, x7]
}

/// Square a 2-component expansion. Result guaranteed ≤6 components.
#[inline]
pub fn two_square(a1: f64, a0: f64) -> [f64; 6] {
    let [x0, _j] = square(a0);
    let _0 = a0 + a0;
    let [_1, _k] = two_product(a1, _0);
    let [x1, _2, _l] = two_one_sum(_k, _1, _j);
    let [_1, _j] = square(a1);
    let [x2, x3, x4, x5] = two_two_sum(_j, _1, _l, _2);
    [x0, x1, x2, x3, x4, x5]
}
