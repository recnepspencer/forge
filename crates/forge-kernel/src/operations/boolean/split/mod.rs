//! Face splitting along plane-plane intersections (corefinement).
//!
//! DOMAIN: Decompose target and tool faces along their mutual intersection lines.
//!
//! ALGORITHM:
//! 1. Build a global PlaneTable — every face plane gets a stable index.
//! 2. Query overlapping face pairs via BVH.
//! 3. Propose cuts: each face to be split by the planes of the opposing solid.
//! 4. Apply cuts via a queue: split_face_by_plane runs MakeEdgeFace for each segment.
//!
//! MODULES:
//! - schema  — data shapes (PlaneTable, LocalVertexDedup, SharedVertexRegistry, CutPoint)
//! - eval    — orchestration (split_all_faces, split_solid)
//! - cut     — per-face logic (split_face_by_plane, compute_face_chord, resolve_chord_endpoints)

mod schema;
mod eval;
mod cut;

pub use schema::SplitPhaseResult;
pub use eval::split_all_faces;
