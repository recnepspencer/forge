//! Integration test harness for topology operator tests.
//!
//! DOMAIN: Provides shape factories (real BSP-generated solids),
//! fluent verification chains, entity selectors, determinism
//! verification, chain testing, and failure diagnostics.
//!
//! ## Modules
//!
//! - `builders`         — Shape factories, config fixtures, scenes, seeders
//! - `verify`           — Fluent `verify(&env).faces(6).volume_approx(8.0, 1e-6).pass()` chain
//! - `snapshot`         — Entity count snapshots for delta assertions
//! - `selectors`        — Entity query DSL (`select(&env).faces().where_normal_near(...)`)
//! - `dump`             — OBJ export for visual debugging on failure
//! - `determinism`      — Production topology hashing and replay-based determinism assertions
//! - `chains`           — Fluent operation chains with per-step production validation
//! - `tolerance_sweep`  — Multi-tolerance test runner

pub mod builders;
pub mod chains;
pub mod determinism;
pub mod dump;
pub mod selectors;
pub mod snapshot;
pub mod tolerance_sweep;
pub mod verify;

// Re-export builders::shapes at harness level for backward compatibility.
// Tests can use `harness::shapes::unit_cube()` or `harness::builders::shapes::unit_cube()`.
pub use builders::shapes;
