//! Radial-edge invariant validators.
//!
//! DOMAIN: Radial cycle closure, edge-entity consistency, cycle
//! uniqueness, neighbor orientation, and splice integrity.
//!
//! STRUCTURE:
//!   ring_closure.rs         — Radial ring walks must close
//!   edge_consistency.rs     — All ring members share the same EdgeId
//!   cycle_uniqueness.rs     — No duplicate halfedges in a ring
//!   neighbor_consistency.rs — Manifold twins have opposite orientations
//!   broken_splices.rs       — No disjoint sub-rings sharing an EdgeId

mod broken_splices;
mod cycle_uniqueness;
mod edge_consistency;
mod neighbor_consistency;
mod ring_closure;

pub(crate) use broken_splices::validate_no_broken_radial_splices;
pub(crate) use cycle_uniqueness::validate_radial_cycle_uniqueness;
pub use edge_consistency::validate_radial_edge_consistency;
pub(crate) use neighbor_consistency::validate_radial_neighbor_consistency;
pub(crate) use ring_closure::validate_radial_rings;

pub(crate) use super::shared::vf;
