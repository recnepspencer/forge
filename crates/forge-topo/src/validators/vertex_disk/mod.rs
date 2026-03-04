//! Vertex-disk and umbrella invariant validators.
//!
//! DOMAIN: Vertex disk partition correctness, disk closure,
//! ordering determinism, cross-disk coedge detection, and
//! pinch-point consistency for NMT states.
//!
//! STRUCTURE:
//!   vertex_outgoing.rs         — Outgoing halfedge validity and origin check
//!   outgoing_reachability.rs   — All half-edges at a vertex reachable from outgoing

mod vertex_outgoing;
mod outgoing_reachability;

pub(crate) use vertex_outgoing::validate_vertex_outgoing;
pub(crate) use outgoing_reachability::validate_vertex_outgoing_reachability;
