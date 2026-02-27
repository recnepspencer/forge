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
//! - `centroid.rs` — Algorithms for computing centroids of geometric entities
//! - `coincidence.rs` — BVH-accelerated face coincidence prepass
//! - `copy.rs` — Arena-to-arena geometry/topology copy helpers
//! - `equivalence.rs` — Vertex equivalence for cross-solid dedup
//! - `fragment.rs` — Face fragment utilities
//! - `intersection_registry.rs` — Canonical position store for multi-solid intersection points
//! - `normal_alignment.rs` — Face normal direction alignment queries
//! - `point_dedup.rs` — Spatial-tolerance point deduplication
//! - `rebuild_face.rs` — Face rebuild helpers for topology repair
//! - `spatial.rs` — Spatial BVH/AABB helpers
//! - `stitch.rs` — Seam stitching helpers
//! - `vertex_identity.rs` — Exact rational vertex match key for cross-solid dedup
//! - `vertex_lookup.rs` — Raw slot-index → typed GeometryState position bridge

pub use centroid::compute_face_centroid;
pub use coincidence::build_face_coincidence_prepass;
pub use edge_key::make_edge_key;
pub use intersection_registry::IntersectionRegistry;
pub use normal_alignment::faces_have_aligned_normals;
pub use point_dedup::dedup_points_by_tolerance;
pub use spatial::{build_face_bvh_pairs, compute_face_aabbs};
pub use vertex_identity::build_vertex_provenance;
pub use vertex_lookup::lookup_vertex_position_by_slot;

pub mod centroid;
pub mod coincidence;
pub mod copy;
pub mod edge_key;
pub mod equivalence;
pub mod fragment;
pub mod intersection_registry;
pub mod normal_alignment;
pub mod point_dedup;
pub mod rebuild_face;
pub mod spatial;
pub mod stitch;
pub mod vertex_identity;
pub mod vertex_lookup;
