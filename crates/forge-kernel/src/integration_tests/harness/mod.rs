//! Integration test harness for topology operator tests.
//!
//! DOMAIN: Provides shape builders (real BSP-generated solids) and
//! structural assertion helpers. Designed so lineage and persistent
//! naming can be wired in without modifying any existing tests.

pub mod assertions;
pub mod shapes;
