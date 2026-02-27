//! Shared kernel-level operations.
//!
//! DOMAIN: Cross-operation algorithms that are too high-level for
//! `forge-geom` or `forge-topo` but shared across multiple features
//! (boolean, fillet, shell, extrude, etc.).
//!
//! DEPENDENCIES: forge-geom (spatial), forge-topo (arena, queries),
//!   forge-kernel::geometry_state
//!
//! SUBDIRECTORIES:
//! - `vertex/` — Centroids, position lookup, identity matching, dedup
//! - `spatial/` — BVH, face coincidence, normal alignment
//! - `assembly/` — Arena copy, stitching, face rebuild, fragments
//!
//! FLAT FILES:
//! - `edge_key.rs` — Canonical edge key for deterministic ordering
//! - `intersection_registry.rs` — Position store for multi-solid intersections

pub mod assembly;
pub mod edge_key;
pub mod intersection_registry;
pub mod spatial;
pub mod vertex;
