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
//! - schema  — data shapes (PlaneTable, SplitConfig, LocalVertexDedup, SharedVertexRegistry, CutPoint)
//! - gate    — cut gate logic (compute_face_chord)
//! - signs   — exact vertex-vs-plane sign classification
//! - eval    — orchestration (split_all_faces, split_solid)
//! - cut     — per-face application (split_face_by_plane, resolve_cut_point)

mod schema;
pub(crate) mod gate;
pub(crate) mod signs;
mod eval;
mod cut;
mod reconcile;

pub use schema::{SplitPhaseResult, SplitConfig};
pub use eval::split_all_faces;
