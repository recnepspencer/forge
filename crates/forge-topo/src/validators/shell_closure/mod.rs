//! Shell and body closure/orientation validators.
//!
//! DOMAIN: Watertightness for solid shells, manifold edge enforcement,
//! orientation consistency, face adjacency, boundary integrity, and
//! laminar edge enforcement for sheet shells.
//!
//! STRUCTURE:
//!   shell_data.rs              — Shared helper for face traversal data
//!   shell_consistency.rs       — Solid shells must be watertight
//!   manifold_edges.rs          — Radial valence ≤ 2 (Doctrine D8)
//!   orientation_consistency.rs — Twin pairs traverse opposite directions
//!   face_adjacency.rs          — Adjacent faces share the same shell
//!   broken_boundary.rs         — Face boundary walks are correct
//!   laminar_edges.rs           — Sheet shells have no NMT edges

mod broken_boundary;
mod face_adjacency;
mod laminar_edges;
mod manifold_edges;
mod orientation_consistency;
mod shell_consistency;
pub(crate) mod shell_data;

pub(crate) use broken_boundary::validate_no_broken_face_boundary;
pub(crate) use face_adjacency::validate_face_adjacency_consistency;
pub(crate) use laminar_edges::validate_boundary_edges_laminar_only;
pub use manifold_edges::validate_manifold_edges;
#[cfg(test)]
pub(crate) use orientation_consistency::validate_orientation_consistency;
#[cfg(test)]
pub(crate) use shell_consistency::validate_shell_consistency;
#[cfg(test)]
pub(crate) use shell_data::collect_shell_data_for_face;

pub(crate) use super::shared::vf;
