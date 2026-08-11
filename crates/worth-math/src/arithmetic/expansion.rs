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

mod constants;
mod dynamic_expansions;
mod fixed_expansions;
mod scalar_primitives;

#[cfg(test)]
mod tests;

pub use constants::{
    CCW_ERR_BOUND_A, CCW_ERR_BOUND_B, CCW_ERR_BOUND_C, EPSILON, ICC_ERR_BOUND_A, ICC_ERR_BOUND_B,
    ICC_ERR_BOUND_C, ISP_ERR_BOUND_A, ISP_ERR_BOUND_B, ISP_ERR_BOUND_C, O3D_ERR_BOUND_A,
    O3D_ERR_BOUND_B, O3D_ERR_BOUND_C, RESULT_ERR_BOUND, SPLITTER,
};
pub use dynamic_expansions::{
    estimate, fast_expansion_sum_zeroelim, grow_expansion_zeroelim, scale_expansion_zeroelim,
};
pub use fixed_expansions::{
    two_one_diff, two_one_product, two_one_sum, two_square, two_two_diff, two_two_product,
    two_two_sum,
};
pub use scalar_primitives::{
    abs, fast_two_sum, fast_two_sum_tail, split, square, two_diff, two_diff_tail, two_product,
    two_product_2presplit, two_product_presplit, two_product_tail, two_sum, two_sum_tail,
};
