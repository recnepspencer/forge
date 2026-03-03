//! Seeder modules — named, composable test scenarios.
//!
//! DOMAIN: Seeders tell stories. Each one produces a specific
//! configuration of solids for a test scenario. Seeders compose:
//! Phase 2 seeders call Phase 1 seeders internally.

pub mod primitives;
pub mod regressions;
