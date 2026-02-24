//! Global Polygon Extraction (Maximal Region Extraction).
//!
//! DOMAIN: Replace iterative JoinFaces for coplanar merging with an O(N)
//! "nuke-and-pave" approach. Discovers coplanar face groups, walks their
//! boundary perimeter, deletes the fragments, and rebuilds a single clean face.
//!
//! ALGORITHM:
//!   1. Graph discovery: BFS to find connected coplanar face clusters
//!   2. Boundary walk: collect perimeter vertices via twin-hopping
//!   3. Nuke and pave: delete group, rebuild single face from perimeter
//!
//! DEPENDENCIES: forge_topo (arena, handles), GeometryStore, forge_geom (exact_eq)

use forge_topo::bitset::EntityBitset;

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext};
use forge_core::tracing::TopologyDelta;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId, EdgeId};
use forge_topo::state::{TopologyState, MutableDraft};

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::geometry_store::GeometryStore;

/// Extract and merge coplanar regions using the global polygon approach.
///
/// Discovers all coplanar face groups, walks their perimeter boundaries,
/// then rebuilds each group as a single clean face. Falls back to None
/// (caller should use legacy path) if any group extraction fails.
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
        let perimeter = forge_topo::algorithms::region_extraction::walk_face_group_boundary_perimeter(
            draft.arena(),
            group,
        )?;
        if perimeter.len() < 3 {
            return Err(KernelError::InternalError {
                message: format!(
                    "Coplanar group of {} faces produced only {} perimeter vertices",
                    group.count(), perimeter.len()
                ),
                context: None,
            });
        }

        let sample_idx = group.iter_ones().next().ok_or_else(|| KernelError::InternalError {
            message: "Empty coplanar group".to_string(),
            context: None,
        })?;
        let sample_face = FaceId::from_raw_parts(sample_idx, 0);
        let lineage = draft.arena().get_face(sample_face)?.lineage().cloned();

        rebuild_face_from_perimeter(&mut draft, group, &perimeter, lineage.as_ref(), geom)?;
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

/// Delete all entities in the group and rebuild a single face from perimeter vertices.
// DEFECT(D3): rebuild_face_from_perimeter uses raw insert_radial_pair instead of Euler ops.
fn rebuild_face_from_perimeter(
    draft: &mut MutableDraft,
    group: &EntityBitset,
    perimeter: &[VertexId],
    lineage: Option<&forge_topo::lineage::Lineage>,
    _geom: &GeometryStore,
) -> Result<FaceId, KernelError> {
    let edges_to_delete = forge_topo::algorithms::region_extraction::collect_face_group_edges(
        draft.arena(),
        group,
    )?;
    let internal_vertices = forge_topo::algorithms::region_extraction::find_face_group_internal_vertices(
        draft.arena(),
        group,
        perimeter,
    )?;

    let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
    let placeholder_loop = forge_topo::handles::LoopId::from_raw_parts(u32::MAX, 0);
    
    let sample_idx = group.iter_ones().next().unwrap();
    let sample_face = FaceId::from_raw_parts(sample_idx, 0);
    let shell = draft.arena().get_face(sample_face)?.shell();

    let new_face = draft.insert_face(
        forge_topo::arena::FaceData::with_lineage(placeholder_loop, shell, lineage.cloned()),
    );
    let new_loop = draft.insert_loop(
        forge_topo::arena::LoopData::new(placeholder_he, new_face),
    );
    draft.arena_mut().get_face_mut(new_face)?.set_outer_loop(new_loop);

    let n = perimeter.len();
    let mut new_half_edges: Vec<HalfEdgeId> = Vec::with_capacity(n);

    for i in 0..n {
        let origin = perimeter[i];
        let (he, twin_he) = draft.insert_radial_pair(
            forge_topo::arena::HalfEdgeData::new(
                placeholder_he, placeholder_he, placeholder_he, new_face, origin, EdgeId::from_raw_parts(u32::MAX, 0),
            ),
            forge_topo::arena::HalfEdgeData::new(
                placeholder_he, placeholder_he, placeholder_he, new_face, perimeter[(i + 1) % n], EdgeId::from_raw_parts(u32::MAX, 0),
            ),
        );
        let edge = draft.insert_edge(forge_topo::arena::EdgeData::new(he));
        draft.arena_mut().get_half_edge_mut(he)?.set_edge(edge);
        draft.arena_mut().get_half_edge_mut(twin_he)?.set_edge(edge);
        
        new_half_edges.push(he);
    }

    for i in 0..n {
        let next_idx = (i + 1) % n;
        let prev_idx = if i == 0 { n - 1 } else { i - 1 };

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(new_half_edges[i])?.set_next(new_half_edges[next_idx]);
        arena.get_half_edge_mut(new_half_edges[i])?.set_prev(new_half_edges[prev_idx]);
    }

    draft.arena_mut().get_loop_mut(new_loop)?.set_half_edge(new_half_edges[0]);

    for &vid in perimeter {
        let matching_he = new_half_edges.iter().find(|&&he_id| {
            draft.arena().get_half_edge(he_id)
                .map(|he| he.origin() == vid)
                .unwrap_or(false)
        });
        if let Some(&he_id) = matching_he {
            draft.arena_mut().get_vertex_mut(vid).ok().map(|v| v.set_outgoing(he_id));
        }
    }

    for &(he_a, he_b) in &edges_to_delete {
        let _ = draft.remove_half_edge(he_a);
        let _ = draft.remove_half_edge(he_b);
    }

    for face_idx in group.iter_ones() {
        let face_id = FaceId::from_raw_parts(face_idx, 0);
        let face_data = draft.arena().get_face(face_id)?;
        let loop_id = face_data.outer_loop();
        let _ = draft.remove_loop(loop_id);
        let _ = draft.remove_face(face_id);
    }

    for &vid in &internal_vertices {
        let _ = draft.remove_vertex(vid);
    }

    Ok(new_face)
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
