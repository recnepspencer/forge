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
//! - schema       — data shapes
//! - gate         — cut gate logic (compute_face_chord)
//! - signs        — exact vertex-vs-plane sign classification
//! - eval         — entry point (split_all_faces)
//! - plane_table  — PlaneTable construction + vertex provenance
//! - cut_proposal — BVH overlap + cut proposals + supplement pass
//! - solid_split  — per-solid split loop
//! - hint_norm    — ExpectedCutHint normalization
//! - cut          — per-face application (split_face_by_plane)
//! - apply        — MakeEdgeFace execution and pair selection
//! - expected     — proof-system hint matching
//! - walk         — edge sign-walk and cut-point provenance
//! - log          — decision logging

pub(super) mod apply;
pub(super) mod cut;
pub(super) mod cut_proposal;
pub(super) mod eval;
pub(crate) mod gate;
pub(super) mod expected;
pub(super) mod hint_norm;
pub(super) mod log;
pub(super) mod plane_table;
mod reconcile;
pub(super) mod schema;
pub(crate) mod signs;
pub(super) mod solid_split;
pub(super) mod walk;

pub use eval::split_all_faces;
pub use schema::{SplitConfig, SplitPhaseResult};
