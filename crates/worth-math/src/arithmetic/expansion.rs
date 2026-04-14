//! Shewchuk expansion arithmetic — exact f64-based adaptive precision.
//!
//! DOMAIN: Non-overlapping f64 expansions for exact determinant evaluation.
//! Vendored from the MIT-licensed `geometry-predicates` crate (Egor Larionov, 2017),
//! which is itself a Rust port of Shewchuk's `predicates.c` (public domain, 1996).
//!
//! INVARIANTS:
//! - Components are non-overlapping: the true value is their exact sum.
//! - Operations preserve the non-overlapping property under IEEE 754 round-to-nearest-even.
//! - `_zeroelim` variants remove exact-zero components without breaking invariants.
//!
//! DEPENDENCIES: None (pure f64 arithmetic).
//!
//! # References
//!
//! - Shewchuk, "Adaptive Precision Floating-Point Arithmetic and Fast Robust
//!   Geometric Predicates," Discrete & Computational Geometry, 1997.
//! - `geometry-predicates` crate: <https://github.com/elrnv/geometry-predicates-rs> (MIT)

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

/// Largest power of two such that `1.0 + EPSILON == 1.0` in f64 arithmetic.
///
/// Bounds the relative roundoff error. Pre-computed for IEEE 754 doubles
/// (53-bit mantissa): `2^{-53} ≈ 1.11e-16`.
pub const EPSILON: f64 = 1.110_223_024_625_156_5e-16;

/// Splitter for exact multiplication via Dekker splitting.
///
/// `2^{ceil(53/2)} + 1 = 2^{27} + 1 = 134_217_729`.
pub const SPLITTER: f64 = 134_217_729.0;

/// Error bound for the `fast` path of various predicates.
pub const RESULT_ERR_BOUND: f64 = (3.0 + 8.0 * EPSILON) * EPSILON;

/// orient2d (ccw) error bounds — stages A, B, C.
pub const CCW_ERR_BOUND_A: f64 = (3.0 + 16.0 * EPSILON) * EPSILON;
pub const CCW_ERR_BOUND_B: f64 = (2.0 + 12.0 * EPSILON) * EPSILON;
pub const CCW_ERR_BOUND_C: f64 = (9.0 + 64.0 * EPSILON) * EPSILON * EPSILON;

/// orient3d error bounds — stages A, B, C.
pub const O3D_ERR_BOUND_A: f64 = (7.0 + 56.0 * EPSILON) * EPSILON;
pub const O3D_ERR_BOUND_B: f64 = (3.0 + 28.0 * EPSILON) * EPSILON;
pub const O3D_ERR_BOUND_C: f64 = (26.0 + 288.0 * EPSILON) * EPSILON * EPSILON;

/// incircle error bounds — stages A, B, C.
pub const ICC_ERR_BOUND_A: f64 = (10.0 + 96.0 * EPSILON) * EPSILON;
pub const ICC_ERR_BOUND_B: f64 = (4.0 + 48.0 * EPSILON) * EPSILON;
pub const ICC_ERR_BOUND_C: f64 = (44.0 + 576.0 * EPSILON) * EPSILON * EPSILON;

/// insphere error bounds — stages A, B, C.
pub const ISP_ERR_BOUND_A: f64 = (16.0 + 224.0 * EPSILON) * EPSILON;
pub const ISP_ERR_BOUND_B: f64 = (5.0 + 72.0 * EPSILON) * EPSILON;
pub const ISP_ERR_BOUND_C: f64 = (71.0 + 1408.0 * EPSILON) * EPSILON * EPSILON;

// ═══════════════════════════════════════════════════════════════════════════
// ERROR-FREE TRANSFORMATIONS (Knuth / Dekker / Shewchuk)
// ═══════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════
// FIXED-LENGTH EXPANSION ARITHMETIC
// ═══════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════
// VARIABLE-LENGTH EXPANSION OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

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
    let mut q;
    let mut findex = 0;
    let mut eindex = 0;

    let mut enow = e[0];
    let mut fnow = f[0];
    if (fnow > enow) == (fnow > -enow) {
        q = enow;
        eindex += 1;
    } else {
        q = fnow;
        findex += 1;
    }

    let mut hindex = 0;
    if eindex < e.len() && findex < f.len() {
        enow = e[eindex];
        fnow = f[findex];
        let [hh, q_new] = if (fnow > enow) == (fnow > -enow) {
            eindex += 1;
            fast_two_sum(enow, q)
        } else {
            findex += 1;
            fast_two_sum(fnow, q)
        };
        q = q_new;
        if hh != 0.0 {
            h[hindex] = hh;
            hindex += 1;
        }
        while eindex < e.len() && findex < f.len() {
            enow = e[eindex];
            fnow = f[findex];
            let [hh, q_new] = if (fnow > enow) == (fnow > -enow) {
                eindex += 1;
                two_sum(q, enow)
            } else {
                findex += 1;
                two_sum(q, fnow)
            };
            q = q_new;
            if hh != 0.0 {
                h[hindex] = hh;
                hindex += 1;
            }
        }
    }
    while eindex < e.len() {
        let [hh, q_new] = two_sum(q, e[eindex]);
        eindex += 1;
        q = q_new;
        if hh != 0.0 {
            h[hindex] = hh;
            hindex += 1;
        }
    }
    while findex < f.len() {
        let [hh, q_new] = two_sum(q, f[findex]);
        findex += 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    /// T5: Verify `two_sum` is error-free (exact round-trip).
    #[test]
    fn two_sum_exact_round_trip() {
        let a = 1.0;
        let b = 1e-20;
        let [lo, hi] = two_sum(a, b);
        let reconstructed = hi + lo;
        assert_eq!(reconstructed, a + b);
    }

    /// T5: Verify `two_product` is error-free.
    #[test]
    fn two_product_exact_round_trip() {
        let a = 1.0 + 1e-10;
        let b = 1.0 - 1e-10;
        let [lo, hi] = two_product(a, b);
        let exact = a as f64 * b as f64;
        assert_eq!(hi, exact);
        assert!((hi + lo - (a * b)).abs() < 1e-30 || lo == 0.0);
    }

    /// T5: Verify `split` produces exact halves.
    #[test]
    fn split_reconstructs_original() {
        let values = [1.0, 1e-15, 1e15, std::f64::consts::PI, 134217729.0];
        for a in values {
            let [lo, hi] = split(a);
            assert_eq!(hi + lo, a, "split({a}) failed reconstruction");
        }
    }

    /// T5: Verify `fast_expansion_sum_zeroelim` produces correct sum.
    #[test]
    fn fast_expansion_sum_basic() {
        let e = [1.0, 2.0];
        let f = [3.0, 4.0];
        let mut h = [0.0; 4];
        let hlen = fast_expansion_sum_zeroelim(&e, &f, &mut h);
        let sum: f64 = h[..hlen].iter().sum();
        assert_eq!(sum, 10.0);
    }

    /// T5: Verify `scale_expansion_zeroelim` produces correct scaled value.
    #[test]
    fn scale_expansion_basic() {
        let e = [3.0, 7.0];
        let mut h = [0.0; 4];
        let hlen = scale_expansion_zeroelim(&e, 2.0, &mut h);
        let sum: f64 = h[..hlen].iter().sum();
        assert_eq!(sum, 20.0);
    }

    /// T5: Verify `grow_expansion_zeroelim` adds a scalar correctly.
    #[test]
    fn grow_expansion_basic() {
        let e = [1.0, 2.0, 3.0];
        let mut h = [0.0; 4];
        let hlen = grow_expansion_zeroelim(&e, 4.0, &mut h);
        let sum: f64 = h[..hlen].iter().sum();
        assert_eq!(sum, 10.0);
    }

    /// T5: Non-overlapping property after `two_two_diff`.
    #[test]
    fn two_two_diff_non_overlapping() {
        let [x0, x1, x2, x3] = two_two_diff(3.0, 1e-16, 2.0, 1e-16);
        let total = x0 + x1 + x2 + x3;
        assert!((total - 1.0).abs() < 1e-30, "Expected ~1.0, got {total}");
    }

    /// T5: `estimate` returns a reasonable approximation.
    #[test]
    fn estimate_approximation() {
        let expansion = [1e-20, 1e-10, 1.0, 100.0];
        let est = estimate(&expansion);
        assert!((est - 101.0).abs() < 0.01);
    }

    /// T5: Zero-elimination in `fast_expansion_sum_zeroelim`.
    #[test]
    fn zeroelim_removes_zeros() {
        let e = [0.0, 1.0];
        let f = [0.0, 2.0];
        let mut h = [0.0; 4];
        let hlen = fast_expansion_sum_zeroelim(&e, &f, &mut h);
        for i in 0..hlen {
            if hlen > 1 {
                assert!(h[i] != 0.0 || i == hlen - 1, "Non-final zero at index {i}");
            }
        }
    }
}
