//! Global Polygon Extraction (Maximal Region Extraction).
//!
//! DOMAIN: Discover coplanar face groups, then delegate the merge to a
//! topology-preserving compound Euler algorithm in `forge-topo`.
//!
//! ALGORITHM:
//!   1. Graph discovery: BFS to find connected coplanar face clusters
//!   2. Topology merge: iteratively JoinFaces across internal region edges
//!
//! DEPENDENCIES: forge_topo (arena, handles), GeometryState, forge_geom (exact_eq)

use forge_topo::bitset::EntityBitset;

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext};
use forge_core::tracing::TopologyDelta;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;
use forge_topo::state::TopologyState;

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta, KernelState, KernelDraft};
use crate::geometry_state::GeometryView;

/// Extract and merge coplanar regions using the topology compound-merge path.
///
/// Discovers all coplanar face groups, then merges each group by iteratively
/// applying `JoinFaces` across internal region edges. Falls back to None
/// (caller should use legacy path) if any group merge fails.
pub fn extract_coplanar_regions(
    draft: &mut KernelDraft,
    ctx: &mut ModelingContext,
) -> Result<usize, KernelError> {
    let groups = discover_coplanar_groups(draft.arena(), draft.geometry())?;
    let mergeable: Vec<_> = groups.into_iter().filter(|g| g.count() >= 2).collect();

    if mergeable.is_empty() {
        return Ok(0);
    }

    let pre_snapshot = ArenaSnapshot::capture(draft.arena());
    let mut total_merged = 0usize;
    let mut killed_faces: Vec<FaceId> = Vec::new();

    for group in &mergeable {
        let cert_result = super::merge_eligibility::eval::certify_merge_boundary(
            draft.arena(),
            group,
            draft.geometry(),
        );

        let should_merge = match cert_result {
            Ok(mut op_result) => {
                let cert_log = op_result.take_decision_log();
                ctx.get_decision_log_mut().merge(cert_log);
                let cert = op_result.into_value();
                !matches!(cert, forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate::Rejected { .. })
            }
            Err(_) => false,
        };

        if !should_merge {
            continue;
        }

        let group_faces: Vec<FaceId> = {
            let arena = draft.arena();
            (0..group.capacity())
                .filter(|&idx| group.contains(idx).unwrap_or(false))
                .filter_map(|idx| {
                    arena.iter_faces()
                        .find(|(fid, _)| fid.index() == idx)
                        .map(|(fid, _)| fid)
                })
                .collect()
        };

        let (mut_draft, _mut_geom) = draft.as_parts_mut();
        let surviving = forge_topo::algorithms::region_extraction::merge_face_group_by_join_faces(
            mut_draft,
            group,
        )?;

        for face_id in &group_faces {
            if *face_id != surviving {
                killed_faces.push(*face_id);
            }
        }

        total_merged += (group.count() - 1) as usize;
    }

    let delta = compute_topology_delta(&pre_snapshot, draft.arena());
    log_extraction(total_merged, mergeable.len(), delta, ctx);

    let (_, mut_geom) = draft.as_parts_mut();
    for face_id in &killed_faces {
        mut_geom.remove_face_plane(*face_id);
    }
    Ok(total_merged)
}

/// Discover connected coplanar face groups via BFS.
///
/// Two faces are in the same group if:
/// 1. They share an edge (via twin pointers)
/// 2. Their plane equations are exactly equal (via `exact_eq`)
fn discover_coplanar_groups(
    arena: &TopologyArena,
    geom: &dyn GeometryView,
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
        DecisionId(merged_count as u64),
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
