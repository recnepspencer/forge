//! Integration tests for topology operators.
//!
//! DOMAIN: Tests that apply operators to real BSP-generated solids
//! (cubes, tetrahedra) and verify structural invariants hold.
//! The harness is designed so lineage and persistent naming changes
//! only require updating the harness, not individual tests.

pub mod harness;

mod boundary_editing;
mod entity_lifecycle;
mod observability;
mod shape_integrity;
mod spatial_validators;
