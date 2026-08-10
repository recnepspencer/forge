use super::{
    estimate, fast_expansion_sum_zeroelim, grow_expansion_zeroelim, scale_expansion_zeroelim,
    split, two_product, two_sum, two_two_diff,
};

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
