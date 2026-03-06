//! Shape integrity — end-to-end tests proving primitives and Euler
//! operators produce correct, deterministic results.
//! DOMAIN: These tests prove the kernel produces correct results using
//! the harness infrastructure (verify, snapshot, determinism, chains,
//! selectors). No theatre — every assertion uses production code.

mod determinism;
mod euler_deltas;
mod normal_correctness;
mod operator_chains;
mod primitive_verification;
mod selector_queries;
mod surface_curve_completeness;
mod volume_metamorphic;
mod volume_oracle;
