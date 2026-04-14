//! Merge eligibility for boundary certification.
//!
//! DOMAIN: Certify face-group boundaries are geometrically valid for merge.
//! Bridges `forge-topo` + `GeometryState` → `worth-geom::boundary_cert`.
//!
//! - `boundary_adapter`: Extract boundary candidate from topology + geometry
//! - `eval`: Kernel-side certification wrapper with OperationResult
//!
//! DEPENDENCIES: `worth-geom::boundary_cert`, `forge-topo`, `GeometryState`.
//! INVARIANTS: Policy lives here, not in worth-geom.

pub mod boundary_adapter;
pub mod eval;
pub mod nmt_eval;
pub mod schema;

#[cfg(test)]
mod tests;
