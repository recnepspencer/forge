//! Face classification for Boolean operations.
//!
//! Classifies each face of a solid relative to the other solid.
//!
//! ALGORITHM: Per-face ray-cast classification
//! 1. Build BVH spatial index on the "other" solid
//! 2. For each face, compute centroid and ray-cast into "other" solid
//! 3. Interpret result: Inside / Outside / OnBoundary / OppositeBoundary
//! 4. Apply counterfactual overrides if present (P3.3)

mod coplanar;
mod eval;

pub(crate) use coplanar::find_coplanar_face_pairs;
pub use eval::classify_faces;
