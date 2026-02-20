//! Post-processing of boolean results.
//!
//! DOMAIN: Simplification passes — merge coplanar faces and remove
//! redundant collinear vertices — to restore canonical representation.
//!
//! DEPENDENCIES: forge_topo (JoinFaces, KillEdgeVertex), GeometryStore.
//! INVARIANTS: Each pass removes at most one entity, then re-enters
//! with fresh topology. This avoids invalidating arena handles mid-pass.

use std::collections::BTreeSet;

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_topo::state::TopologyState;
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::operator::apply_op;
use forge_topo::euler::join_faces::JoinFaces;
use forge_topo::euler::kill_edge_vertex::KillEdgeVertex;

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;

// ── Coplanar face merging ────────────────────────────────────────────────────

/// Merge adjacent coplanar faces to simplify the mesh.
///
/// Iteratively removes edges separating faces on the exact same plane
/// via `JoinFaces`. Critical for canonical results: `(A ∪ B) ∪ C == A ∪ (B ∪ C)`.
pub fn merge_coplanar_faces(
    topo: TopologyState,
    geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    run_iterative_pass(topo, |current| run_merge_pass(current, geom, ctx))
}

/// Find and merge one pair of coplanar faces.
fn run_merge_pass(
    topo: TopologyState,
    geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let mut draft = topo.into_mutation();

    let candidate = find_coplanar_merge_candidate(draft.arena(), geom);
    let Some((he_id, face_a, face_b)) = candidate else {
        return Ok((draft.commit()?, 0));
    };

    let shared_count = count_shared_edges(draft.arena(), face_a, face_b);
    if shared_count > 1 {
        return Ok((draft.commit()?, 0));
    }

    match apply_op(&mut draft, JoinFaces { edge: he_id }) {
        Ok(_) => {
            log_merge(he_id, face_a, face_b, ctx);
            Ok((draft.commit()?, 1))
        }
        Err(_) => Ok((draft.commit()?, 0)),
    }
}

/// Find the first edge separating two coplanar faces.
fn find_coplanar_merge_candidate(
    arena: &forge_topo::arena::TopologyArena,
    geom: &GeometryStore,
) -> Option<(HalfEdgeId, forge_topo::handles::FaceId, forge_topo::handles::FaceId)> {
    arena.iter_half_edges()
        .filter(|(he_id, he)| he.twin() >= *he_id)
        .find_map(|(he_id, he)| {
            let twin = arena.get_half_edge(he.twin()).ok()?;
            let face_a = he.face();
            let face_b = twin.face();
            if face_a == face_b { return None; }

            let plane_a = geom.get_face_plane(face_a)?;
            let plane_b = geom.get_face_plane(face_b)?;
            if forge_geom::primitives::plane::exact_eq(plane_a, plane_b) {
                Some((he_id, face_a, face_b))
            } else {
                None
            }
        })
}

/// Count edges shared between two faces.
fn count_shared_edges(
    arena: &forge_topo::arena::TopologyArena,
    face_a: forge_topo::handles::FaceId,
    face_b: forge_topo::handles::FaceId,
) -> u32 {
    arena.iter_half_edges()
        .filter(|(_, he)| {
            he.face() == face_a
                && arena.get_half_edge(he.twin())
                    .map(|tw| tw.face() == face_b)
                    .unwrap_or(false)
        })
        .count() as u32
}

/// Log a coplanar face merge decision.
fn log_merge(
    he_id: HalfEdgeId,
    face_a: forge_topo::handles::FaceId,
    face_b: forge_topo::handles::FaceId,
    ctx: &mut ModelingContext,
) {
    let mut decision = TracedDecision::new(
        DecisionId(he_id.index() as u64),
        DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
        DecisionTier::Deterministic, 1.0,
        DecisionContext::Degeneracy {
            description: format!("Merged coplanar faces #{} and #{}", face_a.index(), face_b.index()),
        },
    );
    decision.set_entity_scope(EntityRef::new("HalfEdge", he_id.index()));
    ctx.get_decision_log_mut().record(decision);
}

// ── Redundant vertex removal ─────────────────────────────────────────────────

/// Remove redundant vertices (valence-2, collinear edges).
pub fn remove_redundant_vertices(
    topo: TopologyState,
    geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let config = crate::core::ToleranceConfig::default();
    run_iterative_pass(topo, |current| run_vertex_cleanup_pass(current, geom, &config, ctx))
}

/// Find and remove one redundant collinear vertex.
fn run_vertex_cleanup_pass(
    topo: TopologyState,
    geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let mut draft = topo.into_mutation();

    let candidate = find_collinear_vertex_candidate(draft.arena(), geom, config);
    let Some((vid, incoming_he)) = candidate else {
        return Ok((draft.commit()?, 0));
    };

    match apply_op(&mut draft, KillEdgeVertex { edge: incoming_he }) {
        Ok(_) => {
            log_vertex_removal(vid, ctx);
            Ok((draft.commit()?, 1))
        }
        Err(_) => Ok((draft.commit()?, 0)),
    }
}

/// Find the first valence-2 vertex whose adjacent edges are collinear.
fn find_collinear_vertex_candidate(
    arena: &forge_topo::arena::TopologyArena,
    geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
) -> Option<(VertexId, HalfEdgeId)> {
    let mut candidates: Vec<(VertexId, HalfEdgeId)> = Vec::new();

    for (vid, v) in arena.iter_vertices() {
        let he_first = v.outgoing();
        let (degree, edges) = compute_vertex_degree(arena, he_first)?;
        if degree == 2 {
            let incoming = check_collinearity(arena, geom, vid, &edges, config);
            if let Some(he) = incoming {
                candidates.push((vid, he));
            }
        }
    }

    candidates.sort_by_key(|k| k.0);
    candidates.into_iter().next()
}

/// Check if a valence-2 vertex is collinear with its neighbors.
///
/// Returns the incoming half-edge for KillEdgeVertex if collinear.
fn check_collinearity(
    arena: &forge_topo::arena::TopologyArena,
    geom: &GeometryStore,
    vid: VertexId,
    edges: &[HalfEdgeId],
    config: &crate::core::ToleranceConfig,
) -> Option<HalfEdgeId> {
    let e1_data = arena.get_half_edge(edges[0]).ok()?;
    let e2_data = arena.get_half_edge(edges[1]).ok()?;

    let p_v = geom.get_vertex_position(vid)?;
    let target_a = arena.get_half_edge(e1_data.twin()).ok()?.origin();
    let target_b = arena.get_half_edge(e2_data.twin()).ok()?.origin();
    let p_a = geom.get_vertex_position(target_a)?;
    let p_b = geom.get_vertex_position(target_b)?;

    let v_va = forge_math::linalg::sub(*p_a, *p_v);
    let v_vb = forge_math::linalg::sub(*p_b, *p_v);

    let len_a = forge_math::linalg::norm(v_va);
    let len_b = forge_math::linalg::norm(v_vb);

    if len_a < config.get_min_edge_length() || len_b < config.get_min_edge_length() {
        return None;
    }

    let dot = forge_math::linalg::dot(v_va, v_vb) / (len_a * len_b);
    if (dot + 1.0).abs() < config.get_collinearity_dot_tolerance() {
        Some(e1_data.twin())
    } else {
        None
    }
}

/// Log a vertex removal decision.
fn log_vertex_removal(vid: VertexId, ctx: &mut ModelingContext) {
    let mut decision = TracedDecision::new(
        DecisionId(vid.index() as u64),
        DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
        DecisionTier::Deterministic, 1.0,
        DecisionContext::Degeneracy {
            description: format!("Removed redundant collinear vertex #{}", vid.index()),
        },
    );
    decision.set_entity_scope(EntityRef::new("Vertex", vid.index()));
    ctx.get_decision_log_mut().record(decision);
}

// ── Shared helpers ───────────────────────────────────────────────────────────

/// Run an iterative pass until no more changes occur.
fn run_iterative_pass(
    mut topo: TopologyState,
    mut pass_fn: impl FnMut(TopologyState) -> Result<(TopologyState, usize), KernelError>,
) -> Result<(TopologyState, usize), KernelError> {
    let mut total = 0;
    let mut changed = 1;
    while changed > 0 {
        let (new_topo, count) = pass_fn(topo)?;
        topo = new_topo;
        changed = count;
        total += count;
    }
    Ok((topo, total))
}

/// Walk the half-edge ring around a vertex to compute degree.
fn compute_vertex_degree(
    arena: &forge_topo::arena::TopologyArena,
    he_first: HalfEdgeId,
) -> Option<(usize, Vec<HalfEdgeId>)> {
    let mut count = 0;
    let mut curr = he_first;
    let mut edges = Vec::new();

    loop {
        if count > 100 { return None; }
        count += 1;
        edges.push(curr);

        let curr_data = arena.get_half_edge(curr).ok()?;
        let twin_data = arena.get_half_edge(curr_data.twin()).ok()?;
        let next_outgoing = twin_data.next();
        if next_outgoing == he_first {
            return Some((count, edges));
        }
        curr = next_outgoing;
    }
}
