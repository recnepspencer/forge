//! Face classification for Boolean operations.
//!
//! Classifies each face of a solid relative to the other solid.
//!
//! ALGORITHM: Flood-fill classification
//! 1. Build face adjacency graph via twin edges
//! 2. Decompose into connected patches (faces reachable via shared edges)
//! 3. Ray-cast classify ONE seed face per patch
//! 4. Propagate classification to all faces in the patch

mod eval;
mod coplanar;

pub use eval::classify_faces;
pub(crate) use coplanar::find_coplanar_face_pairs;
