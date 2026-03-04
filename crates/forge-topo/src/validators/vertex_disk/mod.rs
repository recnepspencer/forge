//! Vertex-disk and umbrella invariant validators.
//!
//! DOMAIN: Vertex disk partition correctness, disk closure,
//! ordering determinism, cross-disk coedge detection, and
//! pinch-point consistency for NMT states.
//!
//! STRUCTURE:
//!   vertex_outgoing.rs — Outgoing halfedge validity and origin check

mod vertex_outgoing;

pub(crate) use vertex_outgoing::validate_vertex_outgoing;
