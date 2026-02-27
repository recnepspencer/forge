//! Shared kernel-level operations.
//!
//! DOMAIN: Cross-operation algorithms that are too high-level for
//! `forge-geom` or `forge-topo` but shared across multiple features
//! (boolean, fillet, shell, extrude, etc.).
//!
//! DEPENDENCIES: forge-geom (spatial), forge-topo (arena, queries),
//!   forge-kernel::geometry_state
//!
//! FILES:
//! - `vertex_identity.rs` — Exact rational vertex match key for cross-solid dedup
//! - `coincidence.rs` — BVH-accelerated face coincidence prepass
//! - `centroid.rs` — Algorithms for computing centroids of geometric entities

pub use centroid::compute_face_centroid;
pub use coincidence::build_face_coincidence_prepass;

pub mod centroid;
pub mod coincidence;
pub mod copy;
pub mod fragment;
pub mod equivalence;
pub mod rebuild_face;
pub mod stitch;
pub mod vertex_identity;
