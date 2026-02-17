//! Topology diff infrastructure (Milestone 1B.4).
//!
//! DOMAIN: Comparing two topology arena snapshots to produce structured deltas.
//!
//! INVARIANTS:
//! - `compute_diff` walks both arenas slot-by-slot
//! - Empty-to-empty diff produces zero deltas
//! - Delta counts are consistent (added + removed = symmetric difference)
//!
//! DEPENDENCIES: `arena` (TopologyArena), `handles` (typed IDs)

use crate::arena::TopologyArena;

/// A single entity-level change between two arena snapshots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityDelta {
    /// Entity was added (did not exist before, exists after).
    Added {
        index: usize,
    },
    /// Entity was removed (existed before, does not exist after).
    Removed {
        index: usize,
    },
    /// Entity was modified (exists in both, generation changed OR version changed).
    Modified {
        index: usize,
        old_generation: u32,
        new_generation: u32,
        old_version: u32,
        new_version: u32,
    },
}

/// Structured diff between two topology arena snapshots.
///
/// Captures entity-level changes across three entity kinds.
/// Produced by `compute_diff` to support undo/redo visualization,
/// change tracking, and AI-agent introspection.
#[derive(Debug, Clone)]
pub struct TopologyDiff {
    /// Changes to face entities.
    pub faces: Vec<EntityDelta>,
    /// Changes to halfedge entities.
    pub half_edges: Vec<EntityDelta>,
    /// Changes to vertex entities.
    pub vertices: Vec<EntityDelta>,
    /// Epoch of the "before" state.
    pub epoch_before: u64,
    /// Epoch of the "after" state.
    pub epoch_after: u64,
}

impl TopologyDiff {
    /// Total number of entity deltas across all kinds.
    pub fn total_changes(&self) -> usize {
        self.faces.len() + self.half_edges.len() + self.vertices.len()
    }

    /// Whether the diff is empty (no changes).
    pub fn is_empty(&self) -> bool {
        self.total_changes() == 0
    }

    /// Count of added entities across all kinds.
    pub fn total_added(&self) -> usize {
        self.count_delta_kind(|d| matches!(d, EntityDelta::Added { .. }))
    }

    /// Count of removed entities across all kinds.
    pub fn total_removed(&self) -> usize {
        self.count_delta_kind(|d| matches!(d, EntityDelta::Removed { .. }))
    }

    /// Count of modified entities across all kinds.
    pub fn total_modified(&self) -> usize {
        self.count_delta_kind(|d| matches!(d, EntityDelta::Modified { .. }))
    }

    /// Count deltas matching a predicate across all entity kinds.
    fn count_delta_kind(&self, predicate: impl Fn(&EntityDelta) -> bool) -> usize {
        self.faces.iter().filter(|d| predicate(d)).count()
            + self.half_edges.iter().filter(|d| predicate(d)).count()
            + self.vertices.iter().filter(|d| predicate(d)).count()
    }
}

/// Compare two arenas slot-by-slot to produce entity deltas for one entity kind.
/// 
/// The accessors `before_occupied` and `after_occupied` return `Option<(generation, version)>`.
fn diff_slots(
    before_count: usize,
    after_count: usize,
    before_occupied: impl Fn(usize) -> Option<(u32, u32)>,
    after_occupied: impl Fn(usize) -> Option<(u32, u32)>,
) -> Vec<EntityDelta> {
    let max_slots = before_count.max(after_count);
    let mut deltas = Vec::new();

    for index in 0..max_slots {
        let before_info = before_occupied(index);
        let after_info = after_occupied(index);

        match (before_info, after_info) {
            (None, Some(_)) => {
                deltas.push(EntityDelta::Added { index });
            }
            (Some(_), None) => {
                deltas.push(EntityDelta::Removed { index });
            }
            (Some((old_gen, old_ver)), Some((new_gen, new_ver))) => {
                if old_gen != new_gen || old_ver != new_ver {
                     deltas.push(EntityDelta::Modified {
                        index,
                        old_generation: old_gen,
                        new_generation: new_gen,
                        old_version: old_ver,
                        new_version: new_ver,
                    });
                }
            }
            _ => {}
        }
    }

    deltas
}

/// Compute the diff between two topology arenas.
///
/// Walks all entity slots (faces, halfedges, vertices, loops) and produces
/// structured deltas. The `epoch_before` and `epoch_after` values are
/// provided by the caller (from `TopologyState`).
pub fn compute_diff(
    before: &TopologyArena,
    after: &TopologyArena,
    epoch_before: u64,
    epoch_after: u64,
) -> TopologyDiff {
    let faces = diff_slots(
        before.face_slot_count(),
        after.face_slot_count(),
        |i| before.face_generation(i).map(|g| (g, before.face_version(i).unwrap_or(0))),
        |i| after.face_generation(i).map(|g| (g, after.face_version(i).unwrap_or(0))),
    );

    let half_edges = diff_slots(
        before.half_edge_slot_count(),
        after.half_edge_slot_count(),
        |i| before.half_edge_generation(i).map(|g| (g, before.half_edge_version(i).unwrap_or(0))),
        |i| after.half_edge_generation(i).map(|g| (g, after.half_edge_version(i).unwrap_or(0))),
    );

    let vertices = diff_slots(
        before.vertex_slot_count(),
        after.vertex_slot_count(),
        |i| before.vertex_generation(i).map(|g| (g, before.vertex_version(i).unwrap_or(0))),
        |i| after.vertex_generation(i).map(|g| (g, after.vertex_version(i).unwrap_or(0))),
    );

    TopologyDiff {
        faces,
        half_edges,
        vertices,
        epoch_before,
        epoch_after,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::TopologyState;
    use crate::operator::apply_op;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;

    #[test]
    fn empty_to_empty_diff_is_empty() {
        let arena = TopologyArena::new();
        let diff = compute_diff(&arena, &arena, 0, 0);
        assert!(diff.is_empty());
        assert_eq!(diff.total_changes(), 0);
    }

    #[test]
    fn diff_detects_added_entities() {
        let state = TopologyState::empty();
        let before_arena = state.arena().clone();

        let mut draft = state.begin_mutation();
        let _mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let after_state = draft.commit().unwrap();

        let diff = compute_diff(&before_arena, after_state.arena(), 0, 1);

        assert!(!diff.is_empty());
        assert!(diff.total_added() > 0);
        assert_eq!(diff.total_removed(), 0);
        assert_eq!(diff.epoch_before, 0);
        assert_eq!(diff.epoch_after, 1);
    }

    #[test]
    fn diff_detects_growth_after_split() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let state_1 = draft.commit().unwrap();

        let before_arena = state_1.arena().clone();

        let mut draft2 = state_1.begin_mutation();
        let _se = apply_op(&mut draft2, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let state_2 = draft2.commit().unwrap();

        let diff = compute_diff(&before_arena, state_2.arena(), 1, 2);

        assert!(diff.total_added() > 0, "split_edge should add entities");
        assert_eq!(diff.epoch_before, 1);
        assert_eq!(diff.epoch_after, 2);
    }
}
