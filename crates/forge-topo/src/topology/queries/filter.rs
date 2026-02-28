//! Shared topological filtering queries.
//!
//! DOMAIN: Utility functions for filtering collections of topology handles.

use std::collections::BTreeSet;
use crate::b_rep::TopologyArena;
use crate::handles::FaceId;

/// Return all active faces in the arena excluding the provided set of index integers.
pub fn exclude_faces(
    arena: &TopologyArena,
    excluded_indices: &BTreeSet<u32>,
) -> Vec<FaceId> {
    arena
        .iter_faces()
        .map(|(fid, _)| fid)
        .filter(|fid: &FaceId| !excluded_indices.contains(&fid.index()))
        .collect()
}
