//! Proof validation test suites and test support.
//!
//! DOMAIN: PV milestone tests and shared test infrastructure.
//! All PV test modules are currently disabled (commented out) pending
//! re-integration with the updated kernel architecture.
//!
//! DEPENDENCIES: `forge-topo`, `forge-core`, kernel operations

pub(crate) mod test_support;

#[cfg(test)]
mod pv_p0_1_tests;
// #[cfg(test)]
// mod pv_p0_2_tests;
// #[cfg(test)]
// mod pv_p0_3_tests;
// #[cfg(test)]
// mod pv_p0_4_tests;
// #[cfg(test)]
// mod pv_p0_5_tests;
// #[cfg(test)]
// mod pv_p0_5b_tests;
// #[cfg(test)]
// mod pv_p2_1_tests;
// #[cfg(test)]
// mod pv_p2_2_tests;
// #[cfg(test)]
// mod pv_p2_3_tests;
// #[cfg(test)]
// mod pv_p2_4_tests;
// #[cfg(test)]
// mod pv_p2_5_mb_n_tests;
// #[cfg(test)]
// mod pv_p3_1_tests;
// #[cfg(test)]
// mod pv_p3_2_tests;
// #[cfg(test)]
// mod pv_p3_3_tests;
// #[cfg(test)]
// mod pv_p3_4_tests;
// #[cfg(test)]
// mod pv_p3_5_tests;
// #[cfg(test)]
// mod pv_p3_6_tests;
#[cfg(test)]
mod spec_checkpoint_tests;
#[cfg(test)]
mod spec_projection_tests;
