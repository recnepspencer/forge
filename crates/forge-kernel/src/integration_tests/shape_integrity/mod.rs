//! Shape integrity — end-to-end tests proving primitives and Euler
//! operators produce correct, deterministic results.
//! DOMAIN: These tests prove the kernel produces correct results using
//! the harness infrastructure (verify, snapshot, determinism, chains,
//! selectors). No theatre — every assertion uses production code.

mod primitive_verification;
mod determinism;
mod euler_deltas;
mod selector_queries;
mod operator_chains;
