//! Global Polygon Extraction (Maximal Region Extraction).
//!
//! DOMAIN: Discover coplanar face groups, then delegate the merge to a
//! topology-preserving compound Euler algorithm in `forge-topo`.
//!
//! ALGORITHM:
//!   1. Graph discovery: BFS to find connected coplanar face clusters
//!   2. Topology merge: iteratively JoinFaces across internal region edges
//!
//! DEPENDENCIES: forge_topo (arena, handles), GeometryStore, forge_geom (exact_eq)

use forge_topo::bitset::EntityBitset;

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext};
use forge_core::tracing::TopologyDelta;
use forge_topo::arena::TopologyArena;
use forge_topo::state::TopologyState;

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::geometry_store::GeometryStore;

/// Extract and merge coplanar regions using the topology compound-merge path.
///
/// Discovers all coplanar face groups, then merges each group by iteratively
/// applying `JoinFaces` across internal region edges. Falls back to None
/// (caller should use legacy path) if any group merge fails.
pub fn extract_coplanar_regions(
    topo: TopologyState,
    geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let groups = discover_coplanar_groups(topo.arena(), geom)?;
    let mergeable: Vec<_> = groups.into_iter().filter(|g| g.count() >= 2).collect();

    if mergeable.is_empty() {
        return Ok((topo, 0));
    }

    let mut draft = topo.into_mutation();
    let pre_snapshot = ArenaSnapshot::capture(draft.arena());
    let mut total_merged = 0usize;

    for group in &mergeable {
        let _surviving = forge_topo::algorithms::region_extraction::merge_face_group_by_join_faces(
            &mut draft,
            group,
        )?;
        total_merged += (group.count() - 1) as usize;
    }

    let delta = compute_topology_delta(&pre_snapshot, draft.arena());
    log_extraction(total_merged, mergeable.len(), delta, ctx);

    Ok((draft.commit()?, total_merged))
}

/// Discover connected coplanar face groups via BFS.
///
/// Two faces are in the same group if:
/// 1. They share an edge (via twin pointers)
/// 2. Their plane equations are exactly equal (via `exact_eq`)
fn discover_coplanar_groups(
    arena: &TopologyArena,
    geom: &GeometryStore,
) -> Result<Vec<EntityBitset>, KernelError> {
    let groups = forge_topo::algorithms::components::collect_connected_face_components(
        arena,
        |face_id| Ok(geom.get_face_plane(face_id)),
        |seed_plane, _current, neighbor| {
            let Some(neighbor_plane) = geom.get_face_plane(neighbor) else {
                return Ok(false);
            };
            Ok(forge_geom::primitives::plane::exact_eq(*seed_plane, neighbor_plane))
        },
    )?;

    Ok(groups.into_iter().filter(|group| group.count() >= 2).collect())
}

/// Log the extraction decision.
fn log_extraction(
    merged_count: usize,
    group_count: usize,
    delta: TopologyDelta,
    ctx: &mut ModelingContext,
) {
    let mut decision = TracedDecision::new(
        DecisionId(0),
        DecisionKind::PolicyApplied {
            policy: forge_core::PolicyKind::CoincidentGeometry,
            default_used: true,
        },
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!(
                "Polygon extraction: merged {} faces across {} coplanar groups",
                merged_count, group_count
            ),
        },
    );
    decision.set_topology_delta(delta);
    ctx.get_decision_log_mut().record(decision);
}
