//! Spatial validator integration tests.
//!
//! Exercises all 5 geometric validators from `forge-spatial::validators`
//! with both valid (baseline) and poisoned (corrupted) topology.
//! Lives in `forge-kernel` so tests have access to `make_cube` and
//! the full kernel test infrastructure.

mod area_tests;
mod edge_length_tests;
mod loop_orientation_tests;
mod shell_orientation_tests;
mod test_support;
mod volume_tests;
