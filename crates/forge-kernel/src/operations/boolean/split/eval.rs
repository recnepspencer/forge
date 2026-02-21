//! Orchestration for the face-splitting phase.
//!
//! DOMAIN: BVH query, cut-proposal, and per-face split queuing for both solids.
//! DEPENDENCIES: schema, cut, GeometryStore, forge_geom BVH, forge_topo.
//! INVARIANTS: split_solid processes each face-cut pair atomically via MutableDraft.

use std::collections::{BTreeMap, BTreeSet};

use forge_core::{
    KernelError, TracedDecision, DecisionId, DecisionKind, DecisionTier,
    DecisionContext, EntityRef,
};
use forge_geom::Aabb;
use forge_geom::spatial::bvh::{BvhNode, query_overlapping_pairs};
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;
use forge_topo::state::TopologyState;
use forge_topo::traverse::FaceEdgeIterator;

use crate::geometry_store::GeometryStore;
use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::operations::boolean::eval::{VertexMatchKey, planes_are_parallel};

use super::schema::{
    EdgeCutMap, LocalVertexDedup, PlaneTable, SharedVertexRegistry, SplitPhaseResult, SplitConfig,
};
use super::cut::split_face_by_plane;

/// Run the full split phase: find overlapping face pairs via BVH, propose cuts,
/// and split both solids.
pub fn split_all_faces(
    target_topo: TopologyState,
    target_geom: GeometryStore,
    tool_topo: TopologyState,
    tool_geom: GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<SplitPhaseResult, KernelError> {
    let config = crate::core::ToleranceConfig::default();

    let (mut plane_table, target_face_planes, tool_face_planes) =
        build_plane_tables(&target_topo, &target_geom, &tool_topo, &tool_geom);

    let bvh_pairs = build_bvh_overlap_pairs(
        target_topo.arena(), &target_geom,
        tool_topo.arena(), &tool_geom,
        &config,
    )?;

    let (target_cuts, tool_cuts) = propose_cuts(
        &bvh_pairs,
        &target_face_planes, &tool_face_planes,
        &plane_table,
        target_topo.arena(), tool_topo.arena(),
    );

    let mut shared_registry = SharedVertexRegistry::new();

    let (target_res_topo, target_res_geom, target_splits, target_dedup) = split_solid(
        target_topo, target_geom, target_cuts, &target_face_planes,
        &mut plane_table, &config, &mut shared_registry, ctx,
    )?;

    let (tool_res_topo, tool_res_geom, tool_splits, tool_dedup) = split_solid(
        tool_topo, tool_geom, tool_cuts, &tool_face_planes,
        &mut plane_table, &config, &mut shared_registry, ctx,
    )?;

    Ok(SplitPhaseResult {
        target_topology: target_res_topo,
        target_geometry: target_res_geom,
        tool_topology: tool_res_topo,
        tool_geometry: tool_res_geom,
        split_count: target_splits + tool_splits,
        target_provenance: target_dedup.provenance,
        tool_provenance: tool_dedup.provenance,
    })
}

// ── Plane table construction ─────────────────────────────────────────────────

/// Build the shared PlaneTable and per-solid face→plane-index maps.
fn build_plane_tables(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
) -> (PlaneTable, BTreeMap<FaceId, usize>, BTreeMap<FaceId, usize>) {
    let mut plane_table = PlaneTable::new();
    let mut target_face_planes = BTreeMap::new();
    let mut tool_face_planes = BTreeMap::new();

    for (fid, _) in target_topo.arena().iter_faces() {
        if let Some(p) = target_geom.get_face_plane(fid) {
            target_face_planes.insert(fid, plane_table.intern(p));
        }
    }
    for (fid, _) in tool_topo.arena().iter_faces() {
        if let Some(p) = tool_geom.get_face_plane(fid) {
            tool_face_planes.insert(fid, plane_table.intern(p));
        }
    }

    (plane_table, target_face_planes, tool_face_planes)
}

// ── BVH overlap detection ────────────────────────────────────────────────────

/// Build BVH trees for both solids and query overlapping face pairs.
///
/// Returns pairs as `(target_face_index, tool_face_index)` into the AABB lists.
fn build_bvh_overlap_pairs(
    target_arena: &TopologyArena,
    target_geom: &GeometryStore,
    tool_arena: &TopologyArena,
    tool_geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
) -> Result<Vec<(FaceId, FaceId)>, KernelError> {
    let target_aabbs = compute_face_aabbs(target_arena, target_geom, config)?;
    let tool_aabbs = compute_face_aabbs(tool_arena, tool_geom, config)?;

    let target_indexed: Vec<(usize, Aabb)> = target_aabbs.iter()
        .enumerate().map(|(i, (_, aabb))| (i, aabb.clone())).collect();
    let tool_indexed: Vec<(usize, Aabb)> = tool_aabbs.iter()
        .enumerate().map(|(i, (_, aabb))| (i, aabb.clone())).collect();

    let root_a = BvhNode::build(target_indexed).ok_or_else(|| KernelError::InternalError {
        message: "Failed to build target BVH".into(), context: None,
    })?;
    let root_b = BvhNode::build(tool_indexed).ok_or_else(|| KernelError::InternalError {
        message: "Failed to build tool BVH".into(), context: None,
    })?;

    let mut raw_pairs = query_overlapping_pairs(&root_a, &root_b);
    raw_pairs.sort_unstable_by_key(|(a, b)| (*a, *b));

    let resolved: Vec<(FaceId, FaceId)> = raw_pairs.iter()
        .map(|(ia, ib)| (target_aabbs[*ia].0, tool_aabbs[*ib].0))
        .collect();

    Ok(resolved)
}

// ── Cut proposal ─────────────────────────────────────────────────────────────

/// Transform BVH overlap pairs into per-face cut proposals.
///
/// For non-parallel pairs: each face is cut by the opposing face's plane.
/// For coplanar pairs: boundary planes of the opposing face are propagated.
fn propose_cuts(
    bvh_pairs: &[(FaceId, FaceId)],
    target_face_planes: &BTreeMap<FaceId, usize>,
    tool_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &PlaneTable,
    target_arena: &TopologyArena,
    tool_arena: &TopologyArena,
) -> (BTreeMap<FaceId, Vec<usize>>, BTreeMap<FaceId, Vec<usize>>) {
    let mut target_cuts: BTreeMap<FaceId, Vec<usize>> = BTreeMap::new();
    let mut tool_cuts: BTreeMap<FaceId, Vec<usize>> = BTreeMap::new();

    for &(face_a, face_b) in bvh_pairs {
        let plane_idx_a = target_face_planes.get(&face_a).copied();
        let plane_idx_b = tool_face_planes.get(&face_b).copied();

        if let (Some(pa), Some(pb)) = (plane_idx_a, plane_idx_b) {
            let plane_a = plane_table.get(pa);
            let plane_b = plane_table.get(pb);

            if !planes_are_parallel(plane_a, plane_b) {
                target_cuts.entry(face_a).or_default().push(pb);
                tool_cuts.entry(face_b).or_default().push(pa);
            } else if forge_geom::primitives::plane::exact_eq(plane_a, plane_b) {
                propagate_boundary_planes(
                    tool_arena, face_b, pb, tool_face_planes, plane_table, plane_a,
                    &mut target_cuts, face_a,
                );
                propagate_boundary_planes(
                    target_arena, face_a, pa, target_face_planes, plane_table, plane_b,
                    &mut tool_cuts, face_b,
                );
            }
        }
    }

    dedup_cut_lists(&mut target_cuts);
    dedup_cut_lists(&mut tool_cuts);

    (target_cuts, tool_cuts)
}

/// Propagate boundary planes from a source face to a destination face's cut list.
///
/// When two faces are coplanar, we can't cut one by the other's plane.
/// Instead, cut the destination by each non-parallel adjacent plane of the source.
fn propagate_boundary_planes(
    source_arena: &TopologyArena,
    source_face: FaceId,
    source_plane_idx: usize,
    source_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &PlaneTable,
    dest_plane: &forge_geom::Plane,
    dest_cuts: &mut BTreeMap<FaceId, Vec<usize>>,
    dest_face: FaceId,
) {
    let edges = match FaceEdgeIterator::new(source_arena, source_face) {
        Ok(iter) => iter,
        Err(_) => return,
    };

    for he_res in edges {
        let he = match he_res {
            Ok(h) => h,
            Err(_) => return,
        };
        let he_data = match source_arena.get_half_edge(he) {
            Ok(d) => d,
            Err(_) => return,
        };
        let twin_data = match source_arena.get_half_edge(he_data.twin()) {
            Ok(d) => d,
            Err(_) => return,
        };
        if let Some(&adj_plane_idx) = source_face_planes.get(&twin_data.face()) {
            if adj_plane_idx != source_plane_idx {
                let adj_plane = plane_table.get(adj_plane_idx);
                if !planes_are_parallel(dest_plane, adj_plane) {
                    dest_cuts.entry(dest_face).or_default().push(adj_plane_idx);
                }
            }
        }
    }
}

/// Sort and deduplicate each face's cut list.
fn dedup_cut_lists(cuts: &mut BTreeMap<FaceId, Vec<usize>>) {
    for list in cuts.values_mut() {
        list.sort_unstable();
        list.dedup();
    }
}

// ── Per-solid splitting ──────────────────────────────────────────────────────

/// Apply all proposed cuts to a single solid via a queue.
fn split_solid(
    topo: TopologyState,
    mut geom: GeometryStore,
    cuts_map: BTreeMap<FaceId, Vec<usize>>,
    initial_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &mut PlaneTable,
    config: &crate::core::ToleranceConfig,
    shared_registry: &mut SharedVertexRegistry,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, GeometryStore, usize, LocalVertexDedup), KernelError> {
    let mut draft = topo.into_mutation();
    let mut splits = 0;
    let mut dedup = LocalVertexDedup::new();
    let mut edge_cut_map: EdgeCutMap = BTreeMap::new();

    assign_original_vertex_provenance(draft.arena(), &mut dedup, &geom)?;

    let mut queue: Vec<(FaceId, Vec<usize>)> = cuts_map.into_iter().collect();
    let mut current_face_planes = initial_face_planes.clone();

    while let Some((fid, cuts)) = queue.pop() {
        if !cuts.is_empty() {
            let cut_idx = cuts[0];
            let remaining_cuts = cuts[1..].to_vec();

            let cut_plane = plane_table.get(cut_idx);
            let face_plane_idx = *current_face_planes.get(&fid)
                .ok_or(KernelError::InternalError { message: "Missing plane for face".into(), context: None })?;
            let face_plane = plane_table.get(face_plane_idx);

            let split_cfg = SplitConfig {
                plane_table,
                face_plane_map: &current_face_planes,
                tolerance: config,
            };

            let pre_snapshot = ArenaSnapshot::capture(draft.arena());

            let new_faces = split_face_by_plane(
                &mut draft, &mut geom, &mut dedup, &mut edge_cut_map,
                fid, face_plane, cut_plane, cut_idx,
                &split_cfg, shared_registry, ctx,
            )?;

            if !new_faces.is_empty() {
                let delta = compute_topology_delta(&pre_snapshot, draft.arena());
                let mut decision = TracedDecision::new(
                    DecisionId(fid.index() as u64),
                    DecisionKind::Exact,
                    DecisionTier::Deterministic,
                    1.0,
                    DecisionContext::Degeneracy {
                        description: format!(
                            "Split face {} by plane {} → {} new faces",
                            fid, cut_idx, new_faces.len()
                        ),
                    },
                );
                decision.set_entity_scope(EntityRef::new("Face", fid.index()));
                decision.set_topology_delta(delta);
                ctx.get_decision_log_mut().record(decision);
            }

            if !new_faces.is_empty() {
                splits += 1;
                for &nf in &new_faces {
                    current_face_planes.insert(nf, face_plane_idx);
                }
                let mut cuts_with_current = vec![cut_idx];
                cuts_with_current.extend_from_slice(&remaining_cuts);
                for nf in new_faces {
                    queue.push((nf, cuts_with_current.clone()));
                }
            } else if !remaining_cuts.is_empty() {
                    queue.push((fid, remaining_cuts));
            }
        }
    }

    Ok((draft.commit()?, geom, splits, dedup))
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Assign position-based provenance to every original vertex.
fn assign_original_vertex_provenance(
    arena: &TopologyArena,
    dedup: &mut LocalVertexDedup,
    geom: &GeometryStore,
) -> Result<(), KernelError> {
    for (vid, _) in arena.iter_vertices() {
        if let Some(exact) = geom.get_vertex_position_exact(vid) {
            let key = VertexMatchKey::from_exact_position(
                exact[0].clone(), exact[1].clone(), exact[2].clone(),
            );
            dedup.insert(vid, key);
        }
    }
    Ok(())
}

/// Compute AABBs for all faces in a solid.
pub fn compute_face_aabbs(
    arena: &TopologyArena,
    geom: &GeometryStore,
    config: &crate::core::ToleranceConfig,
) -> Result<Vec<(FaceId, Aabb)>, KernelError> {
    let inflation = config.get_aabb_inflation();
    let mut list = Vec::new();
    for (fid, _) in arena.iter_faces() {
        let edges: Vec<_> = FaceEdgeIterator::new(arena, fid)?
            .collect::<Result<Vec<_>, _>>()?;
        let mut points = Vec::new();
        for he in edges {
            let v = arena.get_half_edge(he)?.origin();
            if let Some(p) = geom.get_vertex_position(v) {
                points.push(*p);
            }
        }
        if let Some(mut aabb) = Aabb::from_points(&points) {
            aabb.expand(inflation);
            list.push((fid, aabb));
        }
    }
    Ok(list)
}
