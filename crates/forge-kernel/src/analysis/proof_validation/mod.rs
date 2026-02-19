//! Proof validation suites and checkpoint system.
//!
//! DOMAIN: Invariant validation tests for the proof system milestones.
//!
//! - `checkpoint`: Schema types for the invariant checkpoint system (P0.5)
//! - `pv_p0_1_tests`: Geometric invariant tests (P0.1)
//! - `pv_p0_2_tests`: Euler characteristic hardening tests (P0.2)
//! - `pv_p0_3_tests`: Orientation canonicalization tests (P0.3)
//! - `pv_p0_4_tests`: Non-manifold edge detection tests (P0.4)
//! - `pv_p0_5_tests`: Checkpoint system tests (P0.5)

pub mod checkpoint;

#[cfg(test)]
mod pv_p0_1_tests;
#[cfg(test)]
mod pv_p0_2_tests;
#[cfg(test)]
mod pv_p0_3_tests;
#[cfg(test)]
mod pv_p0_4_tests;
#[cfg(test)]
mod pv_p0_5_tests;
