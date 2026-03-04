//! Half-edge and loop wiring invariant validators.
//!
//! DOMAIN: Structural invariants for the half-edge data structure —
//! next/prev symmetry, loop closure, vertex continuity, cardinality,
//! duplicate detection, face membership, and edge endpoint matching.
//!
//! STRUCTURE:
//!   prev_consistency.rs   — he.prev.next == he
//!   vertex_continuity.rs  — Edge endpoint count ≤ 2
//!   loop_closure.rs       — Loop walk closure and face ownership
//!   loop_cardinality.rs   — Minimum 2 halfedges per loop
//!   duplicate_coedges.rs  — No duplicate halfedges in a loop
//!   face_membership.rs    — All face halfedges reachable from loops
//!   edge_endpoints.rs     — Twin origin matches destination vertex

mod prev_consistency;
mod vertex_continuity;
mod loop_closure;
mod loop_cardinality;
mod duplicate_coedges;
mod face_membership;
mod edge_endpoints;

use forge_core::KernelError;

pub(crate) use prev_consistency::validate_prev_consistency;
pub use vertex_continuity::validate_vertex_continuity;
pub(crate) use loop_closure::validate_loops;
pub(crate) use loop_cardinality::validate_loop_minimum_cardinality;
pub(crate) use duplicate_coedges::validate_no_duplicate_coedges_in_loop;
pub(crate) use face_membership::validate_face_loop_membership_complete;
pub(crate) use edge_endpoints::validate_edge_endpoints_match_loop_vertices;

pub(crate) use super::shared::vf;
