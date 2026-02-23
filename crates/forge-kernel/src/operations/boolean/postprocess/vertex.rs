//! Redundant vertex removal.
//!
//! DOMAIN: Simplification pass — remove valence-2 vertices whose
//! adjacent edges are collinear, consolidating the edges via `KillEdgeVertex`.
//!
//! DEPENDENCIES: forge_topo (KillEdgeVertex), GeometryStore.

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_core::tracing::TopologyDelta;
use forge_topo::state::TopologyState;
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::operator::apply_op;
use forge_topo::euler::kill_edge_vertex::KillEdgeVertex;

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::geometry_store::GeometryStore;

use super::run_iterative_pass;
use super::compute_vertex_degree;

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

    let pre_snapshot = ArenaSnapshot::capture(draft.arena());
    match apply_op(&mut draft, KillEdgeVertex { edge: incoming_he }) {
        Ok(_) => {
            let delta = compute_topology_delta(&pre_snapshot, draft.arena());
            log_vertex_removal(vid, delta, ctx);
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
    let target_a = arena.get_half_edge(e1_data.radial_next()).ok()?.origin();
    let target_b = arena.get_half_edge(e2_data.radial_next()).ok()?.origin();
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
        Some(e1_data.radial_next())
    } else {
        None
    }
}

/// Log a vertex removal decision.
fn log_vertex_removal(vid: VertexId, delta: TopologyDelta, ctx: &mut ModelingContext) {
    let mut decision = TracedDecision::new(
        DecisionId(vid.index() as u64),
        DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
        DecisionTier::Deterministic, 1.0,
        DecisionContext::Degeneracy {
            description: format!("Removed redundant collinear vertex #{}", vid.index()),
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::Vertex, vid.index()));
    decision.set_topology_delta(delta);
    ctx.get_decision_log_mut().record(decision);
}
