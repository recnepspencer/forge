//! Redundant vertex removal.
//!
//! DOMAIN: Simplification pass — remove valence-2 vertices whose
//! adjacent edges are collinear, consolidating the edges via `KillEdgeVertex`.
//!
//! DEPENDENCIES: forge_topo (KillEdgeVertex), GeometryState.

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_core::tracing::TopologyDelta;
use forge_topo::state::TopologyState;
use forge_topo::handles::VertexId;
use forge_topo::validate::{validate_topology, ValidationLevel};

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta, KernelState};
use crate::geometry_state::GeometryState;

use super::run_iterative_pass;
/// Remove redundant vertices (valence-2, collinear edges).
pub fn remove_redundant_vertices(
    state: KernelState,
    ctx: &mut ModelingContext,
) -> Result<(KernelState, usize), KernelError> {
    let config = crate::core::ToleranceConfig::default();
    run_iterative_pass(state, |current| run_vertex_cleanup_pass(current, &config, ctx))
}

/// Find and remove one redundant collinear vertex.
fn run_vertex_cleanup_pass(
    state: KernelState,
    config: &crate::core::ToleranceConfig,
    ctx: &mut ModelingContext,
) -> Result<(KernelState, usize), KernelError> {
    let (topo, geom) = state.into_parts();
    let candidate = find_collinear_vertex_candidate(topo.arena(), &geom, config);
    let Some(vid) = candidate else {
        return Ok((KernelState::new(topo, geom), 0));
    };
    let original = topo.clone();
    let mut draft = topo.into_mutation();

    let pre_snapshot = ArenaSnapshot::capture(draft.arena());
    match forge_topo::algorithms::simplify::consolidate_one_collinear_vertex(
        &mut draft,
        |v| geom.get_vertex_position(v).copied(),
        config.get_min_edge_length(),
        config.get_collinearity_dot_tolerance(),
    )? {
        Some((_, incoming_he)) => {
            if let Err(err) = validate_topology(draft.arena(), ValidationLevel::Full) {
                eprintln!(
                    "[postprocess/vertex] skip invalid redundant-vertex removal v#{} via he#{}: {}",
                    vid.index(),
                    incoming_he.index(),
                    err
                );
                return Ok((KernelState::new(original, geom), 0));
            }
            let delta = compute_topology_delta(&pre_snapshot, draft.arena());
            log_vertex_removal(vid, delta, ctx);
            Ok((KernelState::new(draft.commit()?, geom), 1))
        }
        None => Ok((KernelState::new(draft.commit()?, geom), 0)),
    }
}

/// Find the first valence-2 vertex whose adjacent edges are collinear.
fn find_collinear_vertex_candidate(
    arena: &forge_topo::arena::TopologyArena,
    geom: &GeometryState,
    config: &crate::core::ToleranceConfig,
) -> Option<VertexId> {
    forge_topo::algorithms::simplify::find_collinear_vertex_candidate(
        arena,
        |v| geom.get_vertex_position(v).copied(),
        config.get_min_edge_length(),
        config.get_collinearity_dot_tolerance(),
    )
    .ok()
    .flatten()
    .map(|(vid, _)| vid)
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
