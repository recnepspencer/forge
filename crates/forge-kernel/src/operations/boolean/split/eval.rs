//! Orchestration for the face-splitting phase.
//!
//! DOMAIN: BVH query, cut-proposal, and per-face split queuing for both solids.
//! DEPENDENCIES: schema, cut, GeometryStore, forge_geom BVH, forge_topo.
//! INVARIANTS: split_solid processes each face-cut pair atomically via MutableDraft.

use std::collections::{BTreeMap, BTreeSet};

use forge_core::KernelError;
use forge_geom::Aabb;
use forge_geom::spatial::bvh::{BvhNode, query_overlapping_pairs};
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;
use forge_topo::state::TopologyState;
use forge_topo::traverse::FaceEdgeIterator;

use crate::geometry_store::GeometryStore;
use crate::core::ModelingContext;
use crate::operations::boolean::eval::{VertexMatchKey, planes_are_parallel};

use super::schema::{
    EdgeCutMap, LocalVertexDedup, PlaneTable, SharedVertexRegistry, SplitPhaseResult,
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

    let mut plane_table = PlaneTable::new();
    let mut target_face_planes = BTreeMap::new();
    let mut tool_face_planes = BTreeMap::new();

    for (fid, _) in target_topo.arena().iter_faces() {
        if let Some(p) = target_geom.get_face_plane(fid) {
            let idx = plane_table.intern(p);
            target_face_planes.insert(fid, idx);
        }
    }
    for (fid, _) in tool_topo.arena().iter_faces() {
        if let Some(p) = tool_geom.get_face_plane(fid) {
            let idx = plane_table.intern(p);
            tool_face_planes.insert(fid, idx);
        }
    }

    let target_aabbs_full = compute_face_aabbs(target_topo.arena(), &target_geom, &config)?;
    let tool_aabbs_full = compute_face_aabbs(tool_topo.arena(), &tool_geom, &config)?;

    let target_aabbs_indexed: Vec<(usize, Aabb)> = target_aabbs_full.iter()
        .enumerate().map(|(i, (_, aabb))| (i, aabb.clone())).collect();
    let tool_aabbs_indexed: Vec<(usize, Aabb)> = tool_aabbs_full.iter()
        .enumerate().map(|(i, (_, aabb))| (i, aabb.clone())).collect();

    let root_a = BvhNode::build(target_aabbs_indexed).ok_or_else(|| KernelError::InternalError {
        message: "Failed to build target BVH".into(), context: None,
    })?;
    let root_b = BvhNode::build(tool_aabbs_indexed).ok_or_else(|| KernelError::InternalError {
        message: "Failed to build tool BVH".into(), context: None,
    })?;

    let mut potential_pairs = query_overlapping_pairs(&root_a, &root_b);
    potential_pairs.sort_unstable_by_key(|(a, b)| (*a, *b));

    let mut target_cuts: BTreeMap<FaceId, Vec<usize>> = BTreeMap::new();
    let mut tool_cuts: BTreeMap<FaceId, Vec<usize>> = BTreeMap::new();

    for (idx_a_raw, idx_b_raw) in potential_pairs {
        let face_a = target_aabbs_full[idx_a_raw].0;
        let face_b = tool_aabbs_full[idx_b_raw].0;

        let plane_idx_a = target_face_planes.get(&face_a).copied();
        let plane_idx_b = tool_face_planes.get(&face_b).copied();

        if let (Some(pa), Some(pb)) = (plane_idx_a, plane_idx_b) {
            let plane_a = plane_table.get(pa);
            let plane_b = plane_table.get(pb);

            if !planes_are_parallel(plane_a, plane_b) {
                // Normal cross-cutting: each face is cut by the other's plane.
                eprintln!("DEBUG: PROPOSE Target#{} cut by Tool Plane (idx={})", face_a, pb);
                target_cuts.entry(face_a).or_default().push(pb);
                eprintln!("DEBUG: PROPOSE Tool#{} cut by Target Plane (idx={})", face_b, pa);
                tool_cuts.entry(face_b).or_default().push(pa);
            } else if forge_geom::primitives::plane::exact_eq(plane_a, plane_b) {
                // Coplanar overlap: the faces share a plane so they cannot cut each other.
                // Instead, propagate each face's BOUNDARY planes to the other face's cut list.
                // This ensures the tool's cap gets cut by the target's notch wall planes
                // and vice versa, even when BVH walls don't produce direct proposals.

                if let Ok(edges_b) = FaceEdgeIterator::new(tool_topo.arena(), face_b) {
                    for he_res in edges_b {
                        if let Ok(he_b) = he_res {
                            if let Ok(he_data_b) = tool_topo.arena().get_half_edge(he_b) {
                                if let Ok(twin_data_b) = tool_topo.arena().get_half_edge(he_data_b.twin()) {
                                    if let Some(&adj_plane_idx) = tool_face_planes.get(&twin_data_b.face()) {
                                        if adj_plane_idx != pb {
                                            let adj_plane = plane_table.get(adj_plane_idx);
                                            if !planes_are_parallel(plane_a, adj_plane) {
                                                target_cuts.entry(face_a).or_default().push(adj_plane_idx);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if let Ok(edges_a) = FaceEdgeIterator::new(target_topo.arena(), face_a) {
                    for he_res in edges_a {
                        if let Ok(he_a) = he_res {
                            if let Ok(he_data_a) = target_topo.arena().get_half_edge(he_a) {
                                if let Ok(twin_data_a) = target_topo.arena().get_half_edge(he_data_a.twin()) {
                                    if let Some(&adj_plane_idx) = target_face_planes.get(&twin_data_a.face()) {
                                        if adj_plane_idx != pa {
                                            let adj_plane = plane_table.get(adj_plane_idx);
                                            if !planes_are_parallel(plane_b, adj_plane) {
                                                tool_cuts.entry(face_b).or_default().push(adj_plane_idx);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for cuts in target_cuts.values_mut() { cuts.sort_unstable(); cuts.dedup(); }
    for cuts in tool_cuts.values_mut() { cuts.sort_unstable(); cuts.dedup(); }

    let mut shared_registry = SharedVertexRegistry::new();

    eprintln!("=== TARGET SPLIT PHASE (cuts: {} faces) ===", target_cuts.len());
    for (fid, cuts) in &target_cuts {
        eprintln!("  Target face={} cuts={:?}", fid, cuts);
    }

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

/// Apply all proposed cuts to a single solid.
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

    assign_original_vertex_provenance(draft.arena(), initial_face_planes, &mut dedup, &geom)?;

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

            let new_faces = split_face_by_plane(
                &mut draft,
                &mut geom,
                &mut dedup,
                &mut edge_cut_map,
                fid,
                face_plane,
                face_plane_idx,
                cut_plane,
                cut_idx,
                config,
                plane_table,
                &current_face_planes,
                shared_registry,
                ctx,
            )?;

            if !new_faces.is_empty() {
                splits += 1;
                for &nf in &new_faces {
                    current_face_planes.insert(nf, face_plane_idx);
                }
                let mut cuts_including_current = vec![cut_idx];
                cuts_including_current.extend_from_slice(&remaining_cuts);
                for nf in new_faces {
                    queue.push((nf, cuts_including_current.clone()));
                }
            } else if !remaining_cuts.is_empty() {
                queue.push((fid, remaining_cuts));
            }
        }
    }

    Ok((draft.commit()?, geom, splits, dedup))
}

/// Assign position-based provenance to every original vertex.
///
/// Each vertex gets a `VertexMatchKey` derived from its exact rational position.
/// Vertices without stored exact positions are skipped (spatial fallback in copy phase).
fn assign_original_vertex_provenance(
    arena: &TopologyArena,
    _face_plane_map: &BTreeMap<FaceId, usize>,
    dedup: &mut LocalVertexDedup,
    geom: &GeometryStore,
) -> Result<(), KernelError> {
    for (vid, _) in arena.iter_vertices() {
        if let Some(exact) = geom.get_vertex_position_exact(vid) {
            let key = VertexMatchKey::from_exact_position(
                exact[0].clone(),
                exact[1].clone(),
                exact[2].clone(),
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

/// Expand cuts to coplanar target faces AND their adjacent faces.
///
/// When a target face is coplanar with a tool face, the BVH overlap loop skips
/// it (same plane index → no cut proposed). But the coplanar face may extend
/// beyond the tool footprint and must be subdivided by the tool's non-coplanar
/// boundary planes. Adjacent side walls also need these cuts so that dropping
/// the coplanar region does not leave orphaned edges.
pub fn expand_coplanar_adjacency(
    arena: &TopologyArena,
    target_face_planes: &BTreeMap<FaceId, usize>,
    tool_face_planes: &BTreeMap<FaceId, usize>,
    _existing_target_cuts: &BTreeMap<FaceId, Vec<usize>>,
    plane_table: &PlaneTable,
) -> Vec<(FaceId, Vec<usize>)> {
    let tool_plane_set: BTreeSet<usize> = tool_face_planes.values().copied().collect();

    let mut coplanar_targets: BTreeSet<u32> = BTreeSet::new();
    for (target_fid, &target_plane_idx) in target_face_planes {
        if tool_plane_set.contains(&target_plane_idx) {
            coplanar_targets.insert(target_fid.index());
        }
    }

    if coplanar_targets.is_empty() {
        return Vec::new();
    }

    let mut extra_cuts: Vec<(FaceId, Vec<usize>)> = Vec::new();

    for (target_fid, &target_plane_idx) in target_face_planes {
        if coplanar_targets.contains(&target_fid.index()) {
            let applicable_cuts: Vec<usize> = tool_plane_set.iter()
                .filter(|&&cut_idx| {
                    cut_idx != target_plane_idx
                        && !planes_are_parallel(
                            plane_table.get(target_plane_idx),
                            plane_table.get(cut_idx),
                        )
                })
                .copied()
                .collect();
            if !applicable_cuts.is_empty() {
                extra_cuts.push((*target_fid, applicable_cuts));
            }
        }
    }

    for (he_id, he_data) in arena.iter_half_edges() {
        let face_a = he_data.face();
        if coplanar_targets.contains(&face_a.index()) {
            let twin_id = he_data.twin();
            if twin_id != he_id {
                if let Ok(twin_data) = arena.get_half_edge(twin_id) {
                    let adjacent_face = twin_data.face();
                    if adjacent_face != face_a && !coplanar_targets.contains(&adjacent_face.index()) {
                        if let Some(&adj_plane_idx) = target_face_planes.get(&adjacent_face) {
                            let applicable_cuts: Vec<usize> = tool_plane_set.iter()
                                .filter(|&&cut_idx| {
                                    cut_idx != adj_plane_idx
                                        && !planes_are_parallel(
                                            plane_table.get(adj_plane_idx),
                                            plane_table.get(cut_idx),
                                        )
                                })
                                .copied()
                                .collect();
                            if !applicable_cuts.is_empty() {
                                extra_cuts.push((adjacent_face, applicable_cuts));
                            }
                        }
                    }
                }
            }
        }
    }

    extra_cuts
}
