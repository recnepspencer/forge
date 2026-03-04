//! Collinear degree-2 vertex consolidation.
//!
//! DOMAIN: Detect and optionally collapse valence-2 vertices whose adjacent
//! edges are collinear.

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
use forge_topo::transactions::MutableDraft;

/// Find the first valence-2 vertex whose adjacent edges are collinear.
pub fn find_collinear_vertex_candidate<F>(
    arena: &TopologyArena,
    mut position_fn: F,
    min_edge_length: f64,
    collinearity_dot_tolerance: f64,
) -> Result<Option<(VertexId, HalfEdgeId)>, KernelError>
where
    F: FnMut(VertexId) -> Option<[f64; 3]>,
{
    let mut candidates: Vec<(VertexId, HalfEdgeId)> = Vec::new();

    for (vid, v) in arena.iter_vertices() {
        let he_first = v.outgoing();
        let Some((degree, edges)) = compute_vertex_degree(arena, he_first) else {
            continue;
        };
        if degree != 2 {
            continue;
        }

        let incoming = check_collinearity(
            arena,
            &mut position_fn,
            vid,
            &edges,
            min_edge_length,
            collinearity_dot_tolerance,
        );
        if let Some(he) = incoming {
            candidates.push((vid, he));
        }
    }

    candidates.sort_by_key(|pair| pair.0);
    Ok(candidates.into_iter().next())
}

/// Attempt to consolidate one collinear degree-2 vertex.
pub fn consolidate_one_collinear_vertex<F>(
    draft: &mut MutableDraft,
    position_fn: F,
    min_edge_length: f64,
    collinearity_dot_tolerance: f64,
) -> Result<Option<(VertexId, HalfEdgeId)>, KernelError>
where
    F: FnMut(VertexId) -> Option<[f64; 3]>,
{
    let candidate = find_collinear_vertex_candidate(
        draft.arena(),
        position_fn,
        min_edge_length,
        collinearity_dot_tolerance,
    )?;
    let Some((vid, incoming_he)) = candidate else {
        return Ok(None);
    };

    if draft.execute(KillEdgeVertex { edge: incoming_he }).is_err() {
        return Ok(None);
    }
    Ok(Some((vid, incoming_he)))
}

fn compute_vertex_degree(
    arena: &TopologyArena,
    he_first: HalfEdgeId,
) -> Option<(usize, Vec<HalfEdgeId>)> {
    let mut count = 0usize;
    let mut curr = he_first;
    let mut edges = Vec::new();

    loop {
        if count > 100 { return None; }
        count += 1;
        edges.push(curr);

        let curr_data = arena.get_half_edge(curr).ok()?;
        let twin_data = arena.get_half_edge(curr_data.radial_next()).ok()?;
        let next_outgoing = twin_data.next();
        if next_outgoing == he_first {
            return Some((count, edges));
        }
        curr = next_outgoing;
    }
}

fn check_collinearity<F>(
    arena: &TopologyArena,
    position_fn: &mut F,
    vid: VertexId,
    edges: &[HalfEdgeId],
    min_edge_length: f64,
    collinearity_dot_tolerance: f64,
) -> Option<HalfEdgeId>
where
    F: FnMut(VertexId) -> Option<[f64; 3]>,
{
    let e1_data = arena.get_half_edge(edges[0]).ok()?;
    let e2_data = arena.get_half_edge(edges[1]).ok()?;

    let p_v = position_fn(vid)?;
    let target_a = arena.get_half_edge(e1_data.next()).ok()?.origin();
    let target_b = arena.get_half_edge(e2_data.next()).ok()?.origin();
    let p_a = position_fn(target_a)?;
    let p_b = position_fn(target_b)?;

    let v_va = forge_math::linalg::sub(p_a, p_v);
    let v_vb = forge_math::linalg::sub(p_b, p_v);
    let len_a = forge_math::linalg::norm(v_va);
    let len_b = forge_math::linalg::norm(v_vb);
    if len_a < min_edge_length || len_b < min_edge_length {
        return None;
    }

    let dot = forge_math::linalg::dot(v_va, v_vb) / (len_a * len_b);
    if (dot + 1.0).abs() < collinearity_dot_tolerance {
        Some(e1_data.radial_next())
    } else {
        None
    }
}
