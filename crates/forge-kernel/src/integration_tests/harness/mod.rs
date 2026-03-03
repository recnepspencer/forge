//! Integration test harness for topology operator tests.
//!
//! DOMAIN: Provides shape factories (real BSP-generated solids),
//! structural assertion helpers, fluent verification chains, entity
//! selectors, and failure diagnostics. Designed so lineage, persistent
//! naming, and new operations can be wired in without modifying
//! individual tests.
//!
//! ## Modules
//!
//! - `assertions` — Topology, geometry, and decision correctness checks
//! - `builders`   — Config fixtures, seeders (future)
//! - `shapes`     — Shape factories (`unit_cube()`, `tetrahedron()`, etc.)
//! - `verify`     — Fluent `verify(&env).euler(2).faces(6).pass()` chain
//! - `selectors`  — Entity query DSL (`select(&env).faces().where_normal_near(...)`)
//! - `dump`       — OBJ export for visual debugging on failure

pub mod assertions;
pub mod builders;
pub mod dump;
pub mod selectors;
pub mod shapes;
pub mod verify;
