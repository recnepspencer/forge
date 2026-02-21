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
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::state::{TopologyState, MutableDraft};
use forge_topo::traverse::FaceEdgeIterator;

use crate::geometry_store::GeometryStore;
use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::operations::boolean::eval::{VertexMatchKey, planes_are_parallel};

use super::schema::{
    EdgeCutMap, LocalVertexDedup, PlaneTable, SharedVertexRegistry, SplitPhaseResult, SplitConfig,
};
use super::cut::split_face_by_plane;
use super::gate::compute_face_chord;
use super::reconcile::reconcile_boundary_vertices;

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

    let (mut target_cuts, mut tool_cuts) = propose_cuts(
        &bvh_pairs,
        &target_face_planes, &tool_face_planes,
        &plane_table,
        target_topo.arena(), tool_topo.arena(),
    );

    let supplemented = supplement_cuts_exhaustive(
        target_topo.arena(), &target_geom, &target_face_planes,
        tool_topo.arena(), &tool_geom, &tool_face_planes,
        &plane_table, &config,
        &mut target_cuts, &mut tool_cuts,
    )?;

    eprintln!("[split] BVH pairs: {}, supplemented: {}, target faces with cuts: {}, tool faces with cuts: {}",
        bvh_pairs.len(), supplemented, target_cuts.len(), tool_cuts.len());

    let mut shared_registry = SharedVertexRegistry::new();

    let (mut target_draft, mut target_geom_out, target_splits, mut target_dedup, target_original_vids) = split_solid(
        target_topo, target_geom, target_cuts, &target_face_planes,
        &mut plane_table, &config, &mut shared_registry, ctx,
    )?;

    let (mut tool_draft, mut tool_geom_out, tool_splits, mut tool_dedup, tool_original_vids) = split_solid(
        tool_topo, tool_geom, tool_cuts, &tool_face_planes,
        &mut plane_table, &config, &mut shared_registry, ctx,
    )?;

    let weld_tol = config.get_residual();
    let weld_tol_sq = weld_tol * weld_tol;
    let _reconciled = reconcile_boundary_vertices(
        &mut target_draft, &mut target_geom_out, &mut target_dedup,
        &mut tool_draft, &mut tool_geom_out, &mut tool_dedup,
        &shared_registry, weld_tol_sq,
        &target_original_vids, &tool_original_vids,
    )?;

    let target_res_topo = target_draft.commit()?;
    let tool_res_topo = tool_draft.commit()?;

    Ok(SplitPhaseResult {
        target_topology: target_res_topo,
        target_geometry: target_geom_out,
        tool_topology: tool_res_topo,
        tool_geometry: tool_geom_out,
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

/// Exhaustive chord-gated supplemental cut pass.
///
/// The BVH overlap only connects faces whose AABBs intersect.
/// But a face may need to be cut by a plane from a non-overlapping face
/// if that plane's intersection line passes through the face polygon.
/// This pass checks every face against every opposing plane using
/// `compute_face_chord` as the gate, adding missing cuts.
///
/// Returns the number of supplemental cuts added.
fn supplement_cuts_exhaustive(
    target_arena: &TopologyArena,
    target_geom: &GeometryStore,
    target_face_planes: &BTreeMap<FaceId, usize>,
    tool_arena: &TopologyArena,
    tool_geom: &GeometryStore,
    tool_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &PlaneTable,
    config: &crate::core::ToleranceConfig,
    target_cuts: &mut BTreeMap<FaceId, Vec<usize>>,
    tool_cuts: &mut BTreeMap<FaceId, Vec<usize>>,
) -> Result<usize, KernelError> {
    let mut added = 0;

    let tool_plane_indices: Vec<usize> = tool_face_planes.values().copied().collect();
    let target_plane_indices: Vec<usize> = target_face_planes.values().copied().collect();

    added += supplement_one_direction(
        target_arena, target_geom, target_face_planes,
        &tool_plane_indices, plane_table, config, target_cuts,
    )?;

    added += supplement_one_direction(
        tool_arena, tool_geom, tool_face_planes,
        &target_plane_indices, plane_table, config, tool_cuts,
    )?;

    dedup_cut_lists(target_cuts);
    dedup_cut_lists(tool_cuts);

    Ok(added)
}

/// Check faces that already have BVH-based cuts against all opposing planes.
///
/// Only supplements faces that are ALREADY being cut (have at least one
/// BVH-proposed cut). This prevents false positives for concentric/contained
/// geometry where planes cross face polygons without actual boundary
/// intersection.
fn supplement_one_direction(
    face_arena: &TopologyArena,
    face_geom: &GeometryStore,
    face_planes: &BTreeMap<FaceId, usize>,
    opposing_planes: &[usize],
    plane_table: &PlaneTable,
    config: &crate::core::ToleranceConfig,
    cuts: &mut BTreeMap<FaceId, Vec<usize>>,
) -> Result<usize, KernelError> {
    let mut new_cuts: Vec<(FaceId, usize)> = Vec::new();

    let faces_with_cuts: Vec<FaceId> = cuts.keys().copied().collect();

    for face_id in &faces_with_cuts {
        let face_plane_idx = match face_planes.get(face_id) {
            Some(&idx) => idx,
            None => continue,
        };
        let face_plane = plane_table.get(face_plane_idx);
        let existing = cuts.get(face_id);

        for &cut_plane_idx in opposing_planes {
            if cut_plane_idx == face_plane_idx {
                continue;
            }

            let already_proposed = existing
                .map(|list| list.contains(&cut_plane_idx))
                .unwrap_or(false);
            if already_proposed {
                continue;
            }

            let cut_plane = plane_table.get(cut_plane_idx);
            if planes_are_parallel(face_plane, cut_plane) {
                continue;
            }

            let chord = compute_face_chord(
                face_arena, face_geom, *face_id, face_plane, cut_plane, config,
            )?;

            if chord.is_some() {
                new_cuts.push((*face_id, cut_plane_idx));
            }
        }
    }

    let added = new_cuts.len();
    for (face_id, cut_plane_idx) in new_cuts {
        cuts.entry(face_id).or_default().push(cut_plane_idx);
    }

    Ok(added)
}

// ── Per-solid splitting ──────────────────────────────────────────────────────

/// Apply all proposed cuts to a single solid via a queue.
///
/// Uses a two-pass strategy: the main pass processes all face-cut pairs.
/// Face-cut pairs that fail to produce a split (e.g. vertex-touch grazing
/// where only 1 cut point is found) are collected for a retry round.
/// The retry gives these faces a second chance after neighboring SplitEdge
/// operations have propagated new vertices onto shared edges.
fn split_solid(
    topo: TopologyState,
    mut geom: GeometryStore,
    cuts_map: BTreeMap<FaceId, Vec<usize>>,
    initial_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &mut PlaneTable,
    config: &crate::core::ToleranceConfig,
    shared_registry: &mut SharedVertexRegistry,
    ctx: &mut ModelingContext,
) -> Result<(MutableDraft, GeometryStore, usize, LocalVertexDedup, std::collections::BTreeSet<VertexId>), KernelError> {
    let mut draft = topo.into_mutation();
    let mut splits = 0;
    let mut dedup = LocalVertexDedup::new();
    let mut edge_cut_map: EdgeCutMap = BTreeMap::new();

    assign_original_vertex_provenance(draft.arena(), &mut dedup, &geom, initial_face_planes, plane_table)?;

    let original_vids: std::collections::BTreeSet<VertexId> =
        draft.arena().iter_vertices().map(|(vid, _)| vid).collect();

    let mut queue: Vec<(FaceId, Vec<usize>)> = cuts_map.into_iter().collect();
    let mut current_face_planes = initial_face_planes.clone();
    let mut deferred: Vec<(FaceId, usize)> = Vec::new();

    while let Some((fid, cuts)) = queue.pop() {
        if !cuts.is_empty() {
            let cut_idx = cuts[0];
            let remaining_cuts = cuts[1..].to_vec();

            let result = try_split_face(
                &mut draft, &mut geom, &mut dedup, &mut edge_cut_map,
                fid, cut_idx, &current_face_planes,
                plane_table, config, shared_registry, ctx,
            )?;

            match result {
                SplitAttempt::Split(new_faces, face_plane_idx) => {
                    splits += 1;
                    for &nf in &new_faces {
                        current_face_planes.insert(nf, face_plane_idx);
                    }
                    let mut cuts_with_current = vec![cut_idx];
                    cuts_with_current.extend_from_slice(&remaining_cuts);
                    for nf in new_faces {
                        queue.push((nf, cuts_with_current.clone()));
                    }
                }
                SplitAttempt::NoSplit => {
                    if !remaining_cuts.is_empty() {
                        queue.push((fid, remaining_cuts));
                    } else {
                        deferred.push((fid, cut_idx));
                    }
                }
            }
        }
    }

    for (fid, cut_idx) in deferred {
        let face_exists = current_face_planes.contains_key(&fid);
        if !face_exists {
            let _ = fid;
        } else {
            let result = try_split_face(
                &mut draft, &mut geom, &mut dedup, &mut edge_cut_map,
                fid, cut_idx, &current_face_planes,
                plane_table, config, shared_registry, ctx,
            )?;

            if let SplitAttempt::Split(new_faces, face_plane_idx) = result {
                splits += 1;
                for &nf in &new_faces {
                    current_face_planes.insert(nf, face_plane_idx);
                }
            }
        }
    }

    Ok((draft, geom, splits, dedup, original_vids))
}

/// Result of a single face split attempt.
enum SplitAttempt {
    Split(Vec<FaceId>, usize),
    NoSplit,
}

/// Attempt to split a single face by a single cut plane.
fn try_split_face(
    draft: &mut MutableDraft,
    geom: &mut GeometryStore,
    dedup: &mut LocalVertexDedup,
    edge_cut_map: &mut EdgeCutMap,
    fid: FaceId,
    cut_idx: usize,
    current_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &mut PlaneTable,
    config: &crate::core::ToleranceConfig,
    shared_registry: &mut SharedVertexRegistry,
    ctx: &mut ModelingContext,
) -> Result<SplitAttempt, KernelError> {
    let face_plane_idx = *current_face_planes.get(&fid)
        .ok_or(KernelError::InternalError { message: "Missing plane for face".into(), context: None })?;
    let cut_plane = plane_table.get(cut_idx);
    let face_plane = plane_table.get(face_plane_idx);

    let split_cfg = SplitConfig {
        plane_table,
        face_plane_map: current_face_planes,
        tolerance: config,
    };

    let pre_snapshot = ArenaSnapshot::capture(draft.arena());

    let new_faces = split_face_by_plane(
        draft, geom, dedup, edge_cut_map,
        fid, face_plane, cut_plane, cut_idx,
        &split_cfg, shared_registry, ctx,
    )?;

    if new_faces.is_empty() {
        return Ok(SplitAttempt::NoSplit);
    }

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

    Ok(SplitAttempt::Split(new_faces, face_plane_idx))
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Assign implicit provenance to every original vertex.
///
/// Derives the VertexMatchKey from the exact intersection of the 3
/// incident face planes (via `intersect_three_planes_exact`), NOT from
/// stored vertex coordinates. This guarantees that the same physical
/// point — whether computed as a cut vertex on one solid or as an
/// original corner on the other — produces bitwise-identical rational
/// coordinates and therefore the same VertexMatchKey.
///
/// Falls back to stored exact coordinates for vertices with fewer
/// than 3 distinct incident face planes (non-manifold or boundary).
fn assign_original_vertex_provenance(
    arena: &TopologyArena,
    dedup: &mut LocalVertexDedup,
    geom: &GeometryStore,
    face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &PlaneTable,
) -> Result<(), KernelError> {
    let mut implicit_count = 0;
    let mut fallback_count = 0;
    for (vid, vdata) in arena.iter_vertices() {
        let incident = collect_incident_plane_indices(arena, vid, vdata.outgoing(), face_planes);

        let key = if incident.len() >= 3 {
            implicit_count += 1;
            compute_implicit_key(&incident, plane_table, geom, vid)
        } else {
            fallback_count += 1;
            compute_explicit_key(geom, vid)
        };

        if let Some(k) = key {
            dedup.insert(vid, k);
        }
    }
    eprintln!("[provenance] {} vertices implicit, {} fallback", implicit_count, fallback_count);
    Ok(())
}

/// Walk the vertex umbrella to collect distinct incident face plane indices.
fn collect_incident_plane_indices(
    arena: &TopologyArena,
    _vid: VertexId,
    start_he: forge_topo::handles::HalfEdgeId,
    face_planes: &BTreeMap<FaceId, usize>,
) -> Vec<usize> {
    let mut plane_indices = Vec::new();
    let mut he = start_he;
    let max_iters = 32;

    for _ in 0..max_iters {
        if let Ok(he_data) = arena.get_half_edge(he) {
            let face = he_data.face();
            if let Some(&pi) = face_planes.get(&face) {
                if !plane_indices.contains(&pi) {
                    plane_indices.push(pi);
                }
            }

            let twin = he_data.twin();
            if let Ok(twin_data) = arena.get_half_edge(twin) {
                let next = twin_data.next();
                if next == start_he {
                    break;
                }
                he = next;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    plane_indices
}

/// Compute a VertexMatchKey from the intersection of 3 incident planes.
fn compute_implicit_key(
    incident: &[usize],
    plane_table: &PlaneTable,
    geom: &GeometryStore,
    vid: VertexId,
) -> Option<VertexMatchKey> {
    let p0 = plane_table.get(incident[0]);
    let p1 = plane_table.get(incident[1]);
    let p2 = plane_table.get(incident[2]);

    match forge_geom::primitives::plane::intersect_three_planes_exact(p0, p1, p2) {
        Ok(exact_pos) => Some(VertexMatchKey::from_exact_position(
            exact_pos[0].clone(), exact_pos[1].clone(), exact_pos[2].clone(),
        )),
        Err(_) => compute_explicit_key(geom, vid),
    }
}

/// Fallback: compute VertexMatchKey from stored exact coordinates.
fn compute_explicit_key(geom: &GeometryStore, vid: VertexId) -> Option<VertexMatchKey> {
    geom.get_vertex_position_exact(vid).map(|exact| {
        VertexMatchKey::from_exact_position(
            exact[0].clone(), exact[1].clone(), exact[2].clone(),
        )
    })
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
