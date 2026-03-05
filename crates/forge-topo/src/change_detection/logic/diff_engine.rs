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

use std::collections::BTreeSet;

use crate::b_rep::TopologyArena;

/// A single entity-level change between two arena snapshots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityDelta {
    /// Entity was added (did not exist before, exists after).
    Added { index: usize },
    /// Entity was removed (existed before, does not exist after).
    Removed { index: usize },
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
/// Captures entity-level changes across all seven entity kinds.
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
    /// Changes to loop entities.
    pub loops: Vec<EntityDelta>,
    /// Changes to edge entities.
    pub edges: Vec<EntityDelta>,
    /// Changes to shell entities.
    pub shells: Vec<EntityDelta>,
    /// Changes to solid entities.
    pub solids: Vec<EntityDelta>,
    /// Epoch of the "before" state.
    pub epoch_before: u64,
    /// Epoch of the "after" state.
    pub epoch_after: u64,
}

impl TopologyDiff {
    /// Total number of entity deltas across all kinds.
    pub fn total_changes(&self) -> usize {
        self.faces.len()
            + self.half_edges.len()
            + self.vertices.len()
            + self.loops.len()
            + self.edges.len()
            + self.shells.len()
            + self.solids.len()
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
            + self.loops.iter().filter(|d| predicate(d)).count()
            + self.edges.iter().filter(|d| predicate(d)).count()
            + self.shells.iter().filter(|d| predicate(d)).count()
            + self.solids.iter().filter(|d| predicate(d)).count()
    }
}

/// Compare two arenas using active-index union iteration.
///
/// Iterates only over slot indices that are occupied in at least one
/// arena, making this O(active_entities) instead of O(capacity).
/// The accessors `before_info` and `after_info` return `Option<(generation, version)>`.
fn diff_slots(
    before_active: impl Iterator<Item = usize>,
    after_active: impl Iterator<Item = usize>,
    before_info: impl Fn(usize) -> Option<(u32, u32)>,
    after_info: impl Fn(usize) -> Option<(u32, u32)>,
) -> Vec<EntityDelta> {
    let all_indices: BTreeSet<usize> = before_active.chain(after_active).collect();
    let mut deltas = Vec::new();

    for index in all_indices {
        let before = before_info(index);
        let after = after_info(index);

        match (before, after) {
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
/// Uses active-index union iteration — only visits slot indices that
/// are occupied in at least one arena. O(active) not O(capacity).
pub fn compute_diff(
    before: &TopologyArena,
    after: &TopologyArena,
    epoch_before: u64,
    epoch_after: u64,
) -> TopologyDiff {
    let faces = diff_slots(
        before.active_face_indices(),
        after.active_face_indices(),
        |i| {
            before
                .face_generation(i)
                .map(|g| (g, before.face_version(i).unwrap_or(0)))
        },
        |i| {
            after
                .face_generation(i)
                .map(|g| (g, after.face_version(i).unwrap_or(0)))
        },
    );

    let half_edges = diff_slots(
        before.active_half_edge_indices(),
        after.active_half_edge_indices(),
        |i| {
            before
                .half_edge_generation(i)
                .map(|g| (g, before.half_edge_version(i).unwrap_or(0)))
        },
        |i| {
            after
                .half_edge_generation(i)
                .map(|g| (g, after.half_edge_version(i).unwrap_or(0)))
        },
    );

    let vertices = diff_slots(
        before.active_vertex_indices(),
        after.active_vertex_indices(),
        |i| {
            before
                .vertex_generation(i)
                .map(|g| (g, before.vertex_version(i).unwrap_or(0)))
        },
        |i| {
            after
                .vertex_generation(i)
                .map(|g| (g, after.vertex_version(i).unwrap_or(0)))
        },
    );

    let loops = diff_slots(
        before.active_loop_indices(),
        after.active_loop_indices(),
        |i| {
            before
                .loop_generation(i)
                .map(|g| (g, before.loop_version(i).unwrap_or(0)))
        },
        |i| {
            after
                .loop_generation(i)
                .map(|g| (g, after.loop_version(i).unwrap_or(0)))
        },
    );

    let edges = diff_slots(
        before.active_edge_indices(),
        after.active_edge_indices(),
        |i| {
            before
                .edge_generation(i)
                .map(|g| (g, before.edge_version(i).unwrap_or(0)))
        },
        |i| {
            after
                .edge_generation(i)
                .map(|g| (g, after.edge_version(i).unwrap_or(0)))
        },
    );

    let shells = diff_slots(
        before.active_shell_indices(),
        after.active_shell_indices(),
        |i| {
            before
                .shell_generation(i)
                .map(|g| (g, before.shell_version(i).unwrap_or(0)))
        },
        |i| {
            after
                .shell_generation(i)
                .map(|g| (g, after.shell_version(i).unwrap_or(0)))
        },
    );

    let solids = diff_slots(
        before.active_body_indices(),
        after.active_body_indices(),
        |i| {
            before
                .body_generation(i)
                .map(|g| (g, before.body_version(i).unwrap_or(0)))
        },
        |i| {
            after
                .body_generation(i)
                .map(|g| (g, after.body_version(i).unwrap_or(0)))
        },
    );

    TopologyDiff {
        faces,
        half_edges,
        vertices,
        loops,
        edges,
        shells,
        solids,
        epoch_before,
        epoch_after,
    }
}

#[cfg(test)]
mod tests {
    use crate::b_rep::ShellKind;
    use super::*;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::transactions::TopologyState;

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

        let mut draft = state.into_mutation();
        let _mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();
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
        let mut draft = state.into_mutation();
        let mvf = draft.execute(MakeVertexFace { shell_kind: ShellKind::Sheet }).unwrap().into_value();
        let state_1 = draft.commit().unwrap();

        let before_arena = state_1.arena().clone();

        // Use clone().into_mutation() because we need state_1's arena for the diff below?
        // Actually, we cloned the BEFORE arena above. But we need `state_1` to describe the epoch_before?
        // Wait, `compute_diff` takes arenas.
        // We cloned `before_arena` from strict clone.
        // state_1 is consumed by into_mutation.
        let mut draft2 = state_1.into_mutation();
        let _se = draft2.execute(
            SplitEdge {
                edge: mvf.half_edge,
            },
        )
        .unwrap()
        .into_value();
        let state_2 = draft2.commit().unwrap();

        let diff = compute_diff(&before_arena, state_2.arena(), 1, 2);

        assert!(diff.total_added() > 0, "split_edge should add entities");
        assert_eq!(diff.epoch_before, 1);
        assert_eq!(diff.epoch_after, 2);
    }
}
