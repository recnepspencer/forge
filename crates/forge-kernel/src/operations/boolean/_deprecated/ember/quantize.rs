//! Pre-boolean mesh quantization and topology collapse.
//!
//! DOMAIN: Compute integer grid keys for all vertices, then merge
//! vertices that snap to the same grid coordinate before the split
//! phase begins. This eliminates floating-point drift at the source.
//!
//! DEPENDENCIES: QuantizedSpace, GeometryState, forge-topo (MutableDraft, KillEdgeVertex).
//!
//! INVARIANTS:
//!   - After collapse, no two distinct vertices share the same `[i64; 3]`.
//!   - Zero-length edges created by merging are cleaned up.
//!   - Vertex positions are updated to grid-snapped values for consistency.

use std::collections::BTreeMap;

use forge_core::KernelError;
use forge_topo::handles::VertexId;
use forge_topo::state::TopologyState;

use super::schema::QuantizedSpace;
use crate::geometry_state::GeometryState;

/// Quantized vertex identity map.
///
/// Maps each `VertexId` to its integer grid coordinate for dedup purposes.
pub struct QuantizedVertices {
    grid_positions: BTreeMap<u32, [i64; 3]>,
}

impl QuantizedVertices {
    /// Compute grid keys for all vertex positions (read-only).
    pub fn compute_keys(
        topo: &TopologyState,
        geom: &GeometryState,
        space: &QuantizedSpace,
    ) -> Self {
        let mut grid_positions = BTreeMap::new();

        for (vid, _) in topo.arena().iter_vertices() {
            if let Some(pos) = geom.get_vertex_position(vid) {
                let grid_pos = space.quantize(pos);
                grid_positions.insert(vid.index(), grid_pos);
            }
        }

        Self { grid_positions }
    }

    /// Look up the grid position for a vertex.
    pub fn get_grid_position(&self, vid: VertexId) -> Option<&[i64; 3]> {
        self.grid_positions.get(&vid.index())
    }

    /// Find all vertices that quantized to the same grid coordinate.
    ///
    /// Returns groups of vertex indices that share the same `[i64; 3]`.
    /// Groups of size 1 are omitted (no merge needed).
    pub fn find_coincident_groups(&self) -> Vec<Vec<u32>> {
        let mut by_grid: BTreeMap<[i64; 3], Vec<u32>> = BTreeMap::new();
        for (&vid_idx, grid_pos) in &self.grid_positions {
            by_grid.entry(*grid_pos).or_default().push(vid_idx);
        }
        by_grid
            .into_values()
            .filter(|group| group.len() > 1)
            .collect()
    }

    /// Number of quantized vertices.
    pub fn len(&self) -> usize {
        self.grid_positions.len()
    }

    /// Whether no vertices are quantized.
    pub fn is_empty(&self) -> bool {
        self.grid_positions.is_empty()
    }
}

/// Collapse vertices that quantized to the same grid coordinate.
///
/// For each coincident group, picks the lowest-index vertex as survivor
/// and redirects all half-edges from doomed vertices to point at the
/// survivor. Then cleans up zero-length edges via `KillEdgeVertex`.
///
/// This is the step that makes EMBER's grid actually affect the pipeline:
/// vertices that were 1e-14 apart become the SAME VertexId, so the
/// downstream split/stitch phases never see near-misses.
pub fn collapse_coincident_vertices(
    topo: TopologyState,
    geom: &mut GeometryState,
    quant: &QuantizedVertices,
    space: &QuantizedSpace,
) -> Result<TopologyState, KernelError> {
    let groups = quant.find_coincident_groups();
    if groups.is_empty() {
        return Ok(topo);
    }

    let mut draft = topo.into_mutation();

    for group in &groups {
        let survivor_idx = *group.iter().min().unwrap();

        for &doomed_idx in group {
            if doomed_idx == survivor_idx {
                continue;
            }

            redirect_vertex_references(&mut draft, survivor_idx, doomed_idx)?;
        }
    }

    let topo_after_redirect = draft.commit()?;

    let mut draft = topo_after_redirect.into_mutation();
    let _ = forge_topo::algorithms::simplify::cleanup_degenerate_topology(
        &mut draft,
    )?;
    let topo_clean = draft.commit()?;

    snap_vertex_positions(&topo_clean, geom, quant, space);

    Ok(topo_clean)
}

/// Redirect all half-edge origins from `doomed` vertex to `survivor` vertex.
fn redirect_vertex_references(
    draft: &mut forge_topo::state::MutableDraft,
    survivor_idx: u32,
    doomed_idx: u32,
) -> Result<(), KernelError> {
    let arena = draft.arena();
    let all_he_ids: Vec<_> = arena.iter_half_edges().map(|(id, _)| id).collect();

    let survivor_vid = find_vertex_by_index(draft.arena(), survivor_idx)?;
    let doomed_vid = find_vertex_by_index(draft.arena(), doomed_idx)?;

    let mut edges_to_redirect = Vec::new();
    for he_id in &all_he_ids {
        let he_data = draft.arena().get_half_edge(*he_id)?;
        if he_data.origin() == doomed_vid {
            edges_to_redirect.push(*he_id);
        }
    }

    let arena = draft.arena_mut();
    for he_id in edges_to_redirect {
        arena.get_half_edge_mut(he_id)?.set_origin(survivor_vid);
    }

    let survivor_outgoing = arena.get_vertex(survivor_vid)?.outgoing();
    if arena.get_half_edge(survivor_outgoing).is_err() {
        for he_id in &all_he_ids {
            if let Ok(he) = arena.get_half_edge(*he_id) {
                if he.origin() == survivor_vid {
                    arena.get_vertex_mut(survivor_vid)?.set_outgoing(*he_id);
                    break;
                }
            }
        }
    }

    draft.remove_vertex(doomed_vid)?;

    Ok(())
}

/// Find a VertexId by its index in the arena.
fn find_vertex_by_index(
    arena: &forge_topo::arena::TopologyArena,
    target_idx: u32,
) -> Result<VertexId, KernelError> {
    for (vid, _) in arena.iter_vertices() {
        if vid.index() == target_idx {
            return Ok(vid);
        }
    }
    Err(KernelError::InvalidInput {
        message: format!("Vertex with index {} not found in arena", target_idx),
        context: None,
    })
}

/// Snap all vertex f64 positions to their grid-restored values.
///
/// After collapse, the surviving vertices' positions are updated to exactly
/// match the grid coordinates. This ensures downstream plane equations
/// see clean, consistent positions.
fn snap_vertex_positions(
    topo: &TopologyState,
    geom: &mut GeometryState,
    quant: &QuantizedVertices,
    space: &QuantizedSpace,
) {
    for (vid, _) in topo.arena().iter_vertices() {
        if let Some(grid_pos) = quant.get_grid_position(vid) {
            let restored = space.restore(grid_pos);
            geom.set_vertex_position(vid, restored);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_quantization() {
        let qv = QuantizedVertices {
            grid_positions: BTreeMap::new(),
        };
        assert!(qv.is_empty());
        assert_eq!(qv.find_coincident_groups().len(), 0);
    }

    #[test]
    fn coincident_groups_detected() {
        let mut grid_positions = BTreeMap::new();
        grid_positions.insert(0, [100, 200, 300]);
        grid_positions.insert(1, [100, 200, 300]);
        grid_positions.insert(2, [400, 500, 600]);

        let qv = QuantizedVertices { grid_positions };
        let groups = qv.find_coincident_groups();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 2);
    }
}
