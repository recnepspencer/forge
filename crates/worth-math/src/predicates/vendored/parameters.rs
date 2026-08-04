//! Vendored arithmetic parameters and reference initialization.

/// Returns the absolute value of the given number.
///
/// This function exists since [`std::f64::abs`](std::f64::abs) is not available in core.
/// See [#50145](https://github.com/rust-lang/rust/issues/50145)
///
/// This implementation is identical to [`std::f64::abs`](std::f64::abs) on x86 but not on ARM at the time of this writing.
#[inline]
pub(in crate::predicates) fn abs(a: f64) -> f64 {
    f64::from_bits(a.to_bits() & 0x7FFF_FFFF_FFFF_FFFF)
}

#[derive(Debug)]
pub(in crate::predicates::vendored) struct PredicateParams {
    // Used to split floats in half.
    pub(in crate::predicates::vendored) splitter: f64, // = 2^ceiling(p / 2) + 1.
    /* A set of coefficients used to calculate maximum roundoff errors.          */
    pub(in crate::predicates::vendored) resulterrbound: f64,
    pub(in crate::predicates::vendored) ccwerrbound_a: f64,
    pub(in crate::predicates::vendored) ccwerrbound_b: f64,
    pub(in crate::predicates::vendored) ccwerrbound_c: f64,
    pub(in crate::predicates::vendored) o3derrbound_a: f64,
    pub(in crate::predicates::vendored) o3derrbound_b: f64,
    pub(in crate::predicates::vendored) o3derrbound_c: f64,
    pub(in crate::predicates::vendored) iccerrbound_a: f64,
    pub(in crate::predicates::vendored) iccerrbound_b: f64,
    pub(in crate::predicates::vendored) iccerrbound_c: f64,
    pub(in crate::predicates::vendored) isperrbound_a: f64,
    pub(in crate::predicates::vendored) isperrbound_b: f64,
    pub(in crate::predicates::vendored) isperrbound_c: f64,
}

// EPSILON and PARAMS.slitter were pregenerated using exactinit on a machine with IEEE 754 floats.
// See `exactinit` function below for details.

/// The largest power of two such that 1.0 + epsilon = 1.0 in floating-point
/// arithmetic.
///
/// This number bounds the relative roundoff error. It is used for
/// floating-point error analysis.
const EPSILON: f64 = 0.000_000_000_000_000_111_022_302_462_515_65;

///  Constants used in exact arithmetic.
///
///  See exactinit() for the function used to generate these values.
pub(in crate::predicates::vendored) const PARAMS: PredicateParams = PredicateParams {
    //  Used to split floating-point numbers into two half-length significands
    //  for exact multiplication.
    splitter: 134_217_729f64,
    resulterrbound: (3.0 + 8.0 * EPSILON) * EPSILON,
    ccwerrbound_a: (3.0 + 16.0 * EPSILON) * EPSILON,
    ccwerrbound_b: (2.0 + 12.0 * EPSILON) * EPSILON,
    ccwerrbound_c: (9.0 + 64.0 * EPSILON) * EPSILON * EPSILON,
    o3derrbound_a: (7.0f64 + 56.0f64 * EPSILON) * EPSILON,
    o3derrbound_b: (3.0f64 + 28.0f64 * EPSILON) * EPSILON,
    o3derrbound_c: (26.0f64 + 288.0f64 * EPSILON) * EPSILON * EPSILON,
    iccerrbound_a: (10.0 + 96.0 * EPSILON) * EPSILON,
    iccerrbound_b: (4.0 + 48.0 * EPSILON) * EPSILON,
    iccerrbound_c: (44.0 + 576.0 * EPSILON) * EPSILON * EPSILON,
    isperrbound_a: (16.0f64 + 224.0f64 * EPSILON) * EPSILON,
    isperrbound_b: (5.0f64 + 72.0f64 * EPSILON) * EPSILON,
    isperrbound_c: (71.0f64 + 1408.0f64 * EPSILON) * EPSILON * EPSILON,
};

/// orient3d Stage A error bound (public for cascade-splitting wrappers).
pub(in crate::predicates) const O3D_ERRBOUND_A: f64 = (7.0f64 + 56.0f64 * EPSILON) * EPSILON;

/// orient2d (ccw) Stage A error bound (public for cascade-splitting wrappers).
pub(in crate::predicates) const CCW_ERRBOUND_A: f64 = (3.0f64 + 16.0f64 * EPSILON) * EPSILON;

/* ****************************************************************************/
/*                                                                           */
/*  exactinit()   Initialize the variables used for exact arithmetic.        */
/*                                                                           */
/*  `epsilon' is the largest power of two such that 1.0 + epsilon = 1.0 in   */
/*  floating-point arithmetic.  `epsilon' bounds the relative roundoff       */
/*  error.  It is used for floating-point error analysis.                    */
/*                                                                           */
/*  `splitter' is used to split floating-point numbers into two half-        */
/*  length significands for exact multiplication.                            */
/*                                                                           */
/*  I imagine that a highly optimizing compiler might be too smart for its   */
/*  own good, and somehow cause this routine to fail, if it pretends that    */
/*  floating-point arithmetic is too much like real arithmetic.              */
/*                                                                           */
/*  Don't change this routine unless you fully understand it.                */
/*                                                                           */
/* ****************************************************************************/
#[allow(dead_code)] // This function is for reference only.
fn exactinit() -> PredicateParams {
    let mut check = 1.0_f64;
    let mut lastcheck;
    let mut every_other = 1_i32;
    let mut epsilon = 1.0f64;
    let mut splitter = 1.0f64;
    loop {
        /* Repeatedly divide `epsilon' by two until it is too small to add to    */
        /*   one without causing roundoff.  (Also check if the sum is equal to   */
        /*   the previous sum, for machines that round up instead of using exact */
        /*   rounding.  Not that this library will work on such machines anyway. */
        lastcheck = check;
        epsilon *= 0.5;
        if every_other != 0 {
            splitter *= 2.0f64
        }
        every_other = (every_other == 0) as i32;
        check = 1.0f64 + epsilon;
        if !(check != 1.0f64 && check != lastcheck) {
            break;
        }
    }
    splitter += 1.0f64;
    PredicateParams {
        splitter,
        /* Error bounds for orientation and incircle tests. */
        resulterrbound: (3.0f64 + 8.0f64 * epsilon) * epsilon,
        ccwerrbound_a: (3.0f64 + 16.0f64 * epsilon) * epsilon,
        ccwerrbound_b: (2.0f64 + 12.0f64 * epsilon) * epsilon,
        ccwerrbound_c: (9.0f64 + 64.0f64 * epsilon) * epsilon * epsilon,
        o3derrbound_a: (7.0f64 + 56.0f64 * epsilon) * epsilon,
        o3derrbound_b: (3.0f64 + 28.0f64 * epsilon) * epsilon,
        o3derrbound_c: (26.0f64 + 288.0f64 * epsilon) * epsilon * epsilon,
        iccerrbound_a: (10.0f64 + 96.0f64 * epsilon) * epsilon,
        iccerrbound_b: (4.0f64 + 48.0f64 * epsilon) * epsilon,
        iccerrbound_c: (44.0f64 + 576.0f64 * epsilon) * epsilon * epsilon,
        isperrbound_a: (16.0f64 + 224.0f64 * epsilon) * epsilon,
        isperrbound_b: (5.0f64 + 72.0f64 * epsilon) * epsilon,
        isperrbound_c: (71.0f64 + 1408.0f64 * epsilon) * epsilon * epsilon,
    }
}
