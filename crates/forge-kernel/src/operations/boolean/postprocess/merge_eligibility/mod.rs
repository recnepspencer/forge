//! Merge eligibility for boundary certification.
//!
//! DOMAIN: Certify face-group boundaries are geometrically valid for merge.
//! Bridges `forge-topo` + `GeometryState` → `forge-geom::boundary_cert`.
//!
//! - `boundary_adapter`: Extract boundary candidate from topology + geometry
//! - `eval`: Kernel-side certification wrapper with OperationResult
//!
//! DEPENDENCIES: `forge-geom::boundary_cert`, `forge-topo`, `GeometryState`.
//! INVARIANTS: Policy lives here, not in forge-geom.

pub mod boundary_adapter;
pub mod eval;

#[cfg(test)]
mod tests;
