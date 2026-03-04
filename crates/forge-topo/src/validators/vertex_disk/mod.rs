//! Vertex-disk and umbrella invariant validators.
//!
//! DOMAIN: Vertex disk partition correctness, disk closure,
//! ordering determinism, cross-disk coedge detection, and
//! pinch-point consistency for NMT states.
//!
//! STRUCTURE:
//!   vertex_outgoing.rs         — Outgoing halfedge validity and origin check
//!   disk_closure.rs            — Every vertex disk forms a closed `twin -> next` cycle
//!   disk_partition.rs          — Half-edges correctly partitioned into distinct disks
//!   cross_disk_coedges.rs      — Co-edges do not span across different disks at a vertex

mod cross_disk_coedges;
mod disk_closure;
mod disk_partition;
mod disk_walker;
mod vertex_outgoing;

pub(crate) use cross_disk_coedges::validate_no_cross_disk_coedges;
pub(crate) use disk_closure::validate_disk_closure;
pub(crate) use disk_partition::validate_vertex_disk_partition;
pub(crate) use vertex_outgoing::validate_vertex_outgoing;
