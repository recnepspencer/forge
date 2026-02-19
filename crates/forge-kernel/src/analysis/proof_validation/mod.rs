//! DOMAIN: Proof validation test suites for geometric invariant checks.
//!
//! PV suites verify that the validation system correctly detects
//! degenerate or invalid geometric entities. Each test constructs
//! a known-bad mesh and asserts that the appropriate error is returned.
//!
//! DEPENDENCIES: `forge-topo` (validate), `forge-geom` (Plane, BSP),
//!               `crate::mesh_builder` (make_cube), `crate::geometry_store`

#[cfg(test)]
mod pv_p0_1_tests;
