//! Vendored expansion fixed product.

use super::fixed_sum::{two_one_sum, two_two_sum};

use super::primitives::{
    fast_two_sum, split, square, two_product, two_product_2presplit, two_product_presplit, two_sum,
};

#[inline]
pub(in crate::predicates) fn two_one_product(a1: f64, a0: f64, b: f64) -> [f64; 4] {
    let [blo, bhi] = split(b);
    let [x0, _i] = two_product_presplit(a0, b, bhi, blo);
    let [_0, _j] = two_product_presplit(a1, b, bhi, blo);
    let [x1, _k] = two_sum(_i, _0);
    let [x2, x3] = fast_two_sum(_j, _k);
    [x0, x1, x2, x3]
}

#[inline]
pub(in crate::predicates) fn four_one_product(
    a3: f64,
    a2: f64,
    a1: f64,
    a0: f64,
    b: f64,
) -> [f64; 8] {
    let [blo, bhi] = split(b);
    let [x0, _i] = two_product_presplit(a0, b, bhi, blo);
    let [_0, _j] = two_product_presplit(a1, b, bhi, blo);
    let [x1, _k] = two_sum(_i, _0);
    let [x2, _i] = fast_two_sum(_j, _k);
    let [_0, _j] = two_product_presplit(a2, b, bhi, blo);
    let [x3, _k] = two_sum(_i, _0);
    let [x4, _i] = fast_two_sum(_j, _k);
    let [_0, _j] = two_product_presplit(a3, b, bhi, blo);
    let [x5, _k] = two_sum(_i, _0);
    let [x6, x7] = fast_two_sum(_j, _k);
    [x0, x1, x2, x3, x4, x5, x6, x7]
}

#[inline]
pub(in crate::predicates) fn two_two_product(a1: f64, a0: f64, b1: f64, b0: f64) -> [f64; 8] {
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

#[inline]
pub(in crate::predicates) fn two_square(a1: f64, a0: f64) -> [f64; 6] {
    let [x0, _j] = square(a0);
    let _0: f64 = a0 + a0;
    let [_1, _k] = two_product(a1, _0);
    let [x1, _2, _l] = two_one_sum(_k, _1, _j);
    let [_1, _j] = square(a1);
    let [x2, x3, x4, x5] = two_two_sum(_j, _1, _l, _2);
    [x0, x1, x2, x3, x4, x5]
}
