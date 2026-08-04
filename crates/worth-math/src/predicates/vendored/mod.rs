//! Vendored Shewchuk predicates from the `geometry-predicates` crate.
//!
//! Source: <https://github.com/elrnv/geometry-predicates-rs>
//! License: MIT (Copyright (c) 2017 Egor Larionov)
//! Version: 0.3.0
//!
//! This module tree is a module-partitioned copy of `predicates.rs` from the crate.
//! It provides exact adaptive arithmetic for geometric predicates:
//! - `orient2d`: 2D orientation (3 points)
//! - `orient3d`: 3D orientation (4 points)
//! - `incircle`: 2D circumcircle test (4 points)
//! - `insphere`: 3D circumsphere test (5 points)
//!
//! We vendor this rather than taking a crate dependency so we can:
//! 1. Keep the original as a dev-dependency oracle for testing
//! 2. Add tracing hooks in our wrapper layer
//! 3. Avoid version-lock issues in the kernel
//!
//! Active arithmetic statements remain unchanged; WORTH wrapper-facing error bounds are retained at the vendored facade.

#![allow(
    clippy::excessive_precision,
    clippy::many_single_char_names,
    clippy::needless_return,
    clippy::too_many_arguments,
    clippy::cognitive_complexity,
    clippy::manual_range_contains,
    clippy::let_and_return,
    dead_code,
    unused_variables,
    unused_mut,
    unused_assignments
)]

mod expansion;
mod incircle;
mod insphere;
mod orient2d;
mod orient3d;
mod parameters;

// These exports mirror the original private vendored surface for parity/oracle access.
#[allow(unused_imports)]
pub(in crate::predicates) use expansion::{
    eight_four_sum, eight_one_sum, eight_two_sum, expansion_sum, expansion_sum_zeroelim1,
    expansion_sum_zeroelim2, fast_expansion_sum_zeroelim, fast_two_diff, fast_two_diff_tail,
    fast_two_sum, fast_two_sum_tail, four_four_sum, four_one_product, four_one_sum, four_two_sum,
    grow_expansion, grow_expansion_zeroelim, scale_expansion_zeroelim, split, square, square_tail,
    two_diff, two_diff_tail, two_one_diff, two_one_product, two_one_sum, two_product,
    two_product_2presplit, two_product_presplit, two_product_tail, two_square, two_sum,
    two_sum_tail, two_two_diff, two_two_product, two_two_sum,
};
// These exports mirror the original private vendored surface for parity/oracle access.
#[allow(unused_imports)]
pub(in crate::predicates) use incircle::{
    incircle, incircle_exact, incircle_fast, incircle_slow, incircleadapt,
};
// These exports mirror the original private vendored surface for parity/oracle access.
#[allow(unused_imports)]
pub(in crate::predicates) use insphere::{
    insphere, insphere_exact, insphere_fast, insphere_slow, insphereadapt,
};
// These exports mirror the original private vendored surface for parity/oracle access.
#[allow(unused_imports)]
pub(in crate::predicates) use orient2d::{
    orient2d, orient2d_exact, orient2d_fast, orient2d_slow, orient2dadapt,
};
// These exports mirror the original private vendored surface for parity/oracle access.
#[allow(unused_imports)]
pub(in crate::predicates) use orient3d::{
    orient3d, orient3d_exact, orient3d_fast, orient3d_slow, orient3dadapt,
};
pub(in crate::predicates) use parameters::{abs, CCW_ERRBOUND_A, O3D_ERRBOUND_A};
