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
//! - `pv_p3_4_tests`: Counterfactual replay tests (PV-39, PV-40, PV-40.5)
//! - `pv_p3_5_tests`: MetaBoss replay torture suite (MB-R1 through MB-R7)
//! - `pv_p3_6_tests`: Zero-split + FeatureTree proof metadata integrity (PV-37b/c/d, MB-R1b, MB-R8, MB-R9)

pub mod checkpoint;
pub mod diagnose_pipeline;
pub mod proof_invariants;
#[cfg(test)]
pub(crate) mod test_support;

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
#[cfg(test)]
mod pv_p3_4_tests;
#[cfg(test)]
mod pv_p3_5_tests;
#[cfg(test)]
mod pv_p3_6_tests;
#[cfg(test)]
mod pv_p2_3_tests;
#[cfg(test)]
mod pv_p2_4_tests;
#[cfg(test)]
mod pv_p2_1_tests;
#[cfg(test)]
mod pv_p2_2_tests;
#[cfg(test)]
mod pv_p2_5_mb_n_tests;
