//! Proof validation suites and checkpoint system.
//!
//! DOMAIN: Invariant validation tests for the proof system milestones.
//!
//! - `checkpoint`: Schema types for the invariant checkpoint system (P0.5)
//! - `diagnose_pipeline`: Mid-pipeline diagnostics for boolean debugging (P0.5b)
//! - `pv_p0_1_tests`: Geometric invariant tests (P0.1)
//! - `pv_p0_2_tests`: Euler characteristic hardening tests (P0.2)
//! - `pv_p0_3_tests`: Orientation canonicalization tests (P0.3)
//! - `pv_p0_4_tests`: Non-manifold edge detection tests (P0.4)
//! - `pv_p0_5_tests`: Checkpoint system tests (P0.5)
//! - `pv_p0_5b_tests`: Mid-pipeline diagnostic tests (P0.5b)
//! - `pv_p3_1_tests`: Checkpoint diffing acceptance tests (PV-33, PV-33b, PV-34)
//! - `pv_p3_2_tests`: Region extractor + delta-debug tests (PV-35, PV-35b, PV-36)
//! - `pv_p3_3_tests`: Causal chain reconstruction tests (PV-37, PV-38, PV-54, PV-54.5)

pub mod checkpoint;
pub mod diagnose_pipeline;

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
#[cfg(test)]
mod pv_p0_5b_tests;
#[cfg(test)]
mod pv_p3_1_tests;
#[cfg(test)]
mod pv_p3_2_tests;
#[cfg(test)]
mod pv_p3_3_tests;
