//! Mesh builder test modules.
//!
//! Tests organized by domain concern rather than monolithic files.

mod structural_invariants_tests;
mod shape_counts_tests;
mod scale_and_position_tests;
mod edge_cases_tests;
mod determinism_tests;
mod geometric_fidelity_tests;

use crate::core::config::resolve::{resolve_config, ResolvedConfig};
use crate::core::config::schema::KernelConfig;

/// Build a default `ResolvedConfig` for tests.
pub(super) fn test_config() -> ResolvedConfig {
    resolve_config(&KernelConfig::default(), None, None, None).unwrap()
}
