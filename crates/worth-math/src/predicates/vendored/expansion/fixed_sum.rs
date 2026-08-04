//! Vendored expansion fixed sum.

use super::primitives::{two_diff, two_sum};

#[inline]
pub(in crate::predicates) fn two_one_sum(a1: f64, a0: f64, b: f64) -> [f64; 3] {
    let [x0, _i] = two_sum(a0, b);
    let [x1, x2] = two_sum(a1, _i);
    [x0, x1, x2]
}

#[inline]
pub(in crate::predicates) fn two_one_diff(a1: f64, a0: f64, b: f64) -> [f64; 3] {
    let [x0, _i] = two_diff(a0, b);
    let [x1, x2] = two_sum(a1, _i);
    [x2, x1, x0]
}

#[inline]
pub(in crate::predicates) fn two_two_sum(a1: f64, a0: f64, b1: f64, b0: f64) -> [f64; 4] {
    let [x0, _0, _j] = two_one_sum(a1, a0, b0);
    let [x1, x2, x3] = two_one_sum(_j, _0, b1);
    [x0, x1, x2, x3]
}

#[inline]
pub(in crate::predicates) fn two_two_diff(a1: f64, a0: f64, b1: f64, b0: f64) -> [f64; 4] {
    let [_j, _0, x0] = two_one_diff(a1, a0, b0);
    let [x3, x2, x1] = two_one_diff(_j, _0, b1);
    [x0, x1, x2, x3]
}

#[inline]
pub(in crate::predicates) fn four_one_sum(a3: f64, a2: f64, a1: f64, a0: f64, b: f64) -> [f64; 5] {
    let [x0, x1, _j] = two_one_sum(a1, a0, b);
    let [x2, x3, x4] = two_one_sum(a3, a2, _j);
    [x0, x1, x2, x3, x4]
}

#[inline]
pub(in crate::predicates) fn four_two_sum(
    a3: f64,
    a2: f64,
    a1: f64,
    a0: f64,
    b1: f64,
    b0: f64,
) -> [f64; 6] {
    let [x0, _0, _1, _2, _k] = four_one_sum(a3, a2, a1, a0, b0);
    let [x1, x2, x3, x4, x5] = four_one_sum(_k, _2, _1, _0, b1);
    [x0, x1, x2, x3, x4, x5]
}

#[inline]
pub(in crate::predicates) fn four_four_sum(
    a3: f64,
    a2: f64,
    a1: f64,
    a0: f64,
    b4: f64,
    b3: f64,
    b1: f64,
    b0: f64,
) -> [f64; 8] {
    let [x0, x1, _0, _1, _2, _l] = four_two_sum(a3, a2, a1, a0, b1, b0);
    let [x2, x3, x4, x5, x6, x7] = four_two_sum(_l, _2, _1, _0, b4, b3);
    [x7, x6, x5, x4, x3, x2, x1, x0]
}

#[inline]
pub(in crate::predicates) fn eight_one_sum(
    a7: f64,
    a6: f64,
    a5: f64,
    a4: f64,
    a3: f64,
    a2: f64,
    a1: f64,
    a0: f64,
    b: f64,
) -> [f64; 9] {
    let [x0, x1, x2, x3, _j] = four_one_sum(a3, a2, a1, a0, b);
    let [x4, x5, x6, x7, x8] = four_one_sum(a7, a6, a5, a4, _j);
    [x0, x1, x2, x3, x4, x5, x6, x7, x8]
}

#[inline]
pub(in crate::predicates) fn eight_two_sum(
    a7: f64,
    a6: f64,
    a5: f64,
    a4: f64,
    a3: f64,
    a2: f64,
    a1: f64,
    a0: f64,
    b1: f64,
    b0: f64,
) -> [f64; 10] {
    let [x0, _0, _1, _2, _3, _4, _5, _6, _k] = eight_one_sum(a7, a6, a5, a4, a3, a2, a1, a0, b0);
    let [x1, x2, x3, x4, x5, x6, x7, x8, x9] = eight_one_sum(_k, _6, _5, _4, _3, _2, _1, _0, b1);
    [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9]
}

#[inline]
pub(in crate::predicates) fn eight_four_sum(
    a7: f64,
    a6: f64,
    a5: f64,
    a4: f64,
    a3: f64,
    a2: f64,
    a1: f64,
    a0: f64,
    b4: f64,
    b3: f64,
    b1: f64,
    b0: f64,
) -> [f64; 12] {
    let [x0, x1, _0, _1, _2, _3, _4, _5, _6, _l] =
        eight_two_sum(a7, a6, a5, a4, a3, a2, a1, a0, b1, b0);
    let [x2, x3, x4, x5, x6, x7, x8, x9, x10, x11] =
        eight_two_sum(_l, _6, _5, _4, _3, _2, _1, _0, b4, b3);
    [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11]
}
