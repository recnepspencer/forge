//! Face splitting along plane-plane intersections (corefinement).
//!
//! Uses a Bounding Volume Hierarchy (BVH) to find spatially overlapping
//! face pairs, then computes intersection lines and splits faces.
//!
//! Every vertex (original or intersection) is identified by exactly 3
//! sorted plane indices — its canonical 3-plane provenance. This ensures
//! cross-solid vertex identity is automatic and tolerance-free.

use std::collections::{HashMap, HashSet};

use forge_core::KernelError;
use forge_geom::Aabb;
use forge_geom::spatial::bvh::{BvhNode, query_overlapping_pairs};
use forge_geom::primitives::plane::{Plane, classify_point, classify_point_exact, signed_distance,
                                    intersect_three_planes_exact};
use forge_math::arithmetic::Rational;
use forge_math::sign::TriSign;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId, HalfEdgeId};
use forge_topo::state::{TopologyState, MutableDraft};
use forge_topo::traverse::FaceEdgeIterator;
use forge_topo::operator::apply_op;
use forge_topo::euler::split_edge::SplitEdge;
use forge_topo::euler::make_edge_face::MakeEdgeFace;

use crate::geometry_store::GeometryStore;
use crate::operations::boolean::eval::{VertexMatchKey, planes_are_parallel};
use crate::core::ModelingContext;
use forge_core::result::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};

/// A centralized table of unique planes in the operation.
/// Used to assign stable IDs to planes for provenance tracking.
pub struct PlaneTable {
    planes: Vec<Plane>,
}

impl PlaneTable {
    fn new() -> Self {
        Self { planes: Vec::new() }
    }

    /// Intern a plane, returning its index.
    /// Uses exact rational equality — no tolerances, no scale sensitivity.
    fn intern(&mut self, plane: &Plane) -> usize {
        for (i, p) in self.planes.iter().enumerate() {
            if forge_geom::primitives::plane::exact_eq(p, plane) {
                return i;
            }
        }
        
        let idx = self.planes.len();
        let (a, b, c, d) = plane.exact_coefficients();
        eprintln!("PLANE_INTERN_EXACT: idx={} n_approx=[{:.6},{:.6},{:.6}] d_approx={:.6} a={} b={} c={} d={}", 
            idx, plane.raw_normal()[0], plane.raw_normal()[1], plane.raw_normal()[2], plane.raw_offset(),
            a, b, c, d);
        self.planes.push(plane.clone());
        idx
    }
    
    fn get(&self, index: usize) -> &Plane {
        &self.planes[index]
    }
}

pub struct SplitPhaseResult {
    pub target_topology: TopologyState,
    pub target_geometry: GeometryStore,
    pub tool_topology: TopologyState,
    pub tool_geometry: GeometryStore,
    pub split_count: usize,
    pub target_provenance: HashMap<VertexId, VertexMatchKey>,
    pub tool_provenance: HashMap<VertexId, VertexMatchKey>,
}

impl SplitPhaseResult {
    pub fn split_count(&self) -> usize { self.split_count }
    pub fn into_parts(self) -> (TopologyState, GeometryStore, TopologyState, GeometryStore, HashMap<VertexId, VertexMatchKey>, HashMap<VertexId, VertexMatchKey>) {
        (
            self.target_topology, 
            self.target_geometry, 
            self.tool_topology, 
            self.tool_geometry, 
            self.target_provenance,
            self.tool_provenance
        )
    }
}

/// Maps an undirected edge (sorted vertex index pair) to the cut plane
/// index that created it. Used to resolve provenance for edges between
/// coplanar sub-faces (where face_plane == twin_plane).
type EdgeCutMap = HashMap<(u32, u32), usize>;

/// Create a canonical (sorted) edge key from two vertex IDs.
fn make_edge_key(v1: VertexId, v2: VertexId) -> (u32, u32) {
    let a = v1.index();
    let b = v2.index();
    if a <= b { (a, b) } else { (b, a) }
}

/// Deduplication map for a single solid's vertices.
struct LocalVertexDedup {
    /// VertexId → MatchKey (for all vertices in the solid)
    provenance: HashMap<VertexId, VertexMatchKey>,
    /// MatchKey → VertexId (reverse lookup for finding existing vertices)
    lookup: HashMap<VertexMatchKey, VertexId>,
}

impl LocalVertexDedup {
    fn new() -> Self {
        Self {
            provenance: HashMap::new(),
            lookup: HashMap::new(),
        }
    }
    
    fn insert(&mut self, vid: VertexId, prov: VertexMatchKey) {
        self.provenance.insert(vid, prov.clone());
        self.lookup.insert(prov, vid);
    }
    
    fn find_by_provenance(&self, prov: &VertexMatchKey) -> Option<VertexId> {
        self.lookup.get(prov).copied()
    }
}

/// Shared registry of canonical intersection positions.
///
/// Each 3-plane intersection is computed once and stored here.
/// Both `split_solid` calls reference the same registry, so the
/// same geometric point always gets the same position — zero
/// floating-point divergence between solids.
pub struct SharedVertexRegistry {
    positions: HashMap<VertexMatchKey, [f64; 3]>,
}

impl SharedVertexRegistry {
    fn new() -> Self {
        Self { positions: HashMap::new() }
    }

    /// Register a position for a 3-plane key. If the key already
    /// exists, returns the previously stored (canonical) position.
    /// If new, stores and returns the provided position.
    fn canonical_position(&mut self, key: &VertexMatchKey, computed: [f64; 3]) -> [f64; 3] {
        *self.positions.entry(key.clone()).or_insert(computed)
    }
}

pub fn split_all_faces(
    target_topo: TopologyState,
    target_geom: GeometryStore,
    tool_topo: TopologyState,
    tool_geom: GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<SplitPhaseResult, KernelError> {
    
    let config = crate::core::ToleranceConfig::default();
    
    // 1. Build Global Plane Table
    let mut plane_table = PlaneTable::new();
    
    let mut target_face_planes = HashMap::new();
    let mut tool_face_planes = HashMap::new();
    
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
    
    // 2. Compute AABBs
    let target_aabbs_full = compute_face_aabbs(target_topo.arena(), &target_geom)?;
    let tool_aabbs_full = compute_face_aabbs(tool_topo.arena(), &tool_geom)?;
    
    let target_aabbs_indexed: Vec<(usize, Aabb)> = target_aabbs_full.iter().enumerate().map(|(i, (_, aabb))| (i, aabb.clone())).collect();
    let tool_aabbs_indexed: Vec<(usize, Aabb)> = tool_aabbs_full.iter().enumerate().map(|(i, (_, aabb))| (i, aabb.clone())).collect();
    
    // 3. Build BVH & Query
    let root_a = BvhNode::build(target_aabbs_indexed).ok_or_else(|| KernelError::InternalError { message: "Failed to build target BVH".into(), context: None })?;
    let root_b = BvhNode::build(tool_aabbs_indexed).ok_or_else(|| KernelError::InternalError { message: "Failed to build tool BVH".into(), context: None })?;
    
    // 3. Find potentially intersecting pairs
    let mut potential_pairs = query_overlapping_pairs(&root_a, &root_b);
    
    // Sort for determinism
    potential_pairs.sort_unstable_by_key(|(a, b)| {
        (*a, *b)
    });
    
    // 4. Collect cuts per face
    let mut target_cuts: HashMap<FaceId, Vec<usize>> = HashMap::new();
    let mut tool_cuts: HashMap<FaceId, Vec<usize>> = HashMap::new();
    
    for (idx_a_raw, idx_b_raw) in potential_pairs {
        let face_a = target_aabbs_full[idx_a_raw].0;
        let face_b = tool_aabbs_full[idx_b_raw].0;
        
        if let Some(&plane_idx_b) = tool_face_planes.get(&face_b) {
            if let Some(plane_idx_a) = target_face_planes.get(&face_a) {
                if *plane_idx_a != plane_idx_b {
                    let plane_a = plane_table.get(*plane_idx_a);
                    let plane_b = plane_table.get(plane_idx_b);
                    if !planes_are_parallel(plane_a, plane_b) {
                        eprintln!("DEBUG: PROPOSE Target#{} cut by Tool Plane (idx={})", face_a, plane_idx_b);
                        target_cuts.entry(face_a).or_default().push(plane_idx_b);
                    } else {
                        eprintln!("DEBUG: SKIP Target#{} vs Tool Plane (idx={}) - PARALLEL", face_a, plane_idx_b);
                    }
                }
            }
        }
        
        if let Some(&plane_idx_a) = target_face_planes.get(&face_a) {
             if let Some(plane_idx_b) = tool_face_planes.get(&face_b) {
                 if plane_idx_a != *plane_idx_b {
                     let plane_a = plane_table.get(plane_idx_a);
                     let plane_b = plane_table.get(*plane_idx_b);
                     if !planes_are_parallel(plane_b, plane_a) {
                        eprintln!("DEBUG: PROPOSE Tool#{} cut by Target Plane (idx={})", face_b, plane_idx_a);
                        tool_cuts.entry(face_b).or_default().push(plane_idx_a);
                     } else {
                        eprintln!("DEBUG: SKIP Tool#{} vs Target Plane (idx={}) - PARALLEL", face_b, plane_idx_a);
                     }
                 }
             }
        }
    }
    
    // (Removed 4b and 4c: heuristic coplanar adjacency expansion was causing over-splitting
    //  by propagating ALL planes from the other solid to local side-walls. Exact AABBs
    //  should be sufficient now.)

    for cuts in target_cuts.values_mut() { cuts.sort_unstable(); cuts.dedup(); }
    for cuts in tool_cuts.values_mut() { cuts.sort_unstable(); cuts.dedup(); }

    // 5. Perform Splits (shared registry ensures each intersection is computed once)
    let mut shared_registry = SharedVertexRegistry::new();

    eprintln!("=== TARGET SPLIT PHASE (cuts: {} faces) ===", target_cuts.len());
    for (fid, cuts) in &target_cuts {
        eprintln!("  Target face={} cuts={:?}", fid, cuts);
    }

    let (target_res_topo, target_res_geom, target_splits, target_dedup) = split_solid(
        target_topo, target_geom, target_cuts, &target_face_planes, &mut plane_table, &config, &mut shared_registry, ctx
    )?;
    
    let (tool_res_topo, tool_res_geom, tool_splits, tool_dedup) = split_solid(
        tool_topo, tool_geom, tool_cuts, &tool_face_planes, &mut plane_table, &config, &mut shared_registry, ctx
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

fn split_solid(
    topo: TopologyState, 
    mut geom: GeometryStore, 
    cuts_map: HashMap<FaceId, Vec<usize>>,
    initial_face_planes: &HashMap<FaceId, usize>,
    plane_table: &mut PlaneTable,
    config: &crate::core::ToleranceConfig,
    shared_registry: &mut SharedVertexRegistry,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, GeometryStore, usize, LocalVertexDedup), KernelError> {
    
    let mut draft = topo.into_mutation();

    let mut splits = 0;
    let mut dedup = LocalVertexDedup::new();
    let mut edge_cut_map: EdgeCutMap = HashMap::new();
    
    // Assign 3-plane provenance to EVERY original vertex.
    // Each vertex is at the intersection of the face planes of its adjacent faces.
    assign_original_vertex_provenance(draft.arena(), initial_face_planes, &mut dedup, &geom)?;

    let mut queue: Vec<(FaceId, Vec<usize>)> = Vec::new();
    for (fid, cuts) in cuts_map {
        queue.push((fid, cuts));
    }
    
    let mut current_face_planes = initial_face_planes.clone();

    while let Some((fid, cuts)) = queue.pop() {
        if cuts.is_empty() { continue; }
        
        let cut_idx = cuts[0];
        let remaining_cuts = cuts[1..].to_vec();
        
        let cut_plane = plane_table.get(cut_idx);
        let face_plane_idx = *current_face_planes.get(&fid).ok_or(KernelError::InternalError { message: "Missing plane for face".into(), context: None })?;
        let face_plane = plane_table.get(face_plane_idx);
        
        let new_face_opt = split_face_by_plane(
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
            ctx
        )?;
        
        if let Some(new_face) = new_face_opt {
            splits += 1;
            current_face_planes.insert(new_face, face_plane_idx);
            
            if !remaining_cuts.is_empty() {
                queue.push((fid, remaining_cuts.clone()));
                queue.push((new_face, remaining_cuts));
            }
        } else {
            if !remaining_cuts.is_empty() {
                queue.push((fid, remaining_cuts));
            }
        }
    }
    
    Ok((draft.commit()?, geom, splits, dedup))
}

/// Assign position-based provenance to every original vertex.
///
/// Each original vertex gets a `VertexMatchKey` derived from its exact
/// rational position in the geometry store. This is order-independent,
/// handles high-valence vertices correctly, and matches across solids
/// since the same physical point always has the same rational coordinates.
///
/// Vertices without a stored exact position are skipped (they will fall
/// through to the spatial-fallback in the copy phase).
fn assign_original_vertex_provenance(
    arena: &TopologyArena,
    _face_plane_map: &HashMap<FaceId, usize>,
    dedup: &mut LocalVertexDedup,
    geom: &crate::geometry_store::GeometryStore,
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

fn compute_face_aabbs(arena: &TopologyArena, geom: &GeometryStore) -> Result<Vec<(FaceId, Aabb)>, KernelError> {
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
         if let Some(aabb) = Aabb::from_points(&points) {
              list.push((fid, aabb));
         }
    }
    Ok(list)
}

/// Expand cuts to coplanar target faces AND their adjacent faces.
///
/// When a target face is coplanar with a tool face, the BVH overlap
/// loop skips it (same plane index → no cut proposed). But the coplanar
/// face may extend beyond the tool's footprint and must be subdivided
/// by the tool's non-coplanar boundary planes. Adjacent faces (side
/// walls, etc.) also need these cuts so that dropping the coplanar
/// region doesn't leave orphaned edges.
fn expand_coplanar_adjacency(
    arena: &TopologyArena,
    target_face_planes: &HashMap<FaceId, usize>,
    tool_face_planes: &HashMap<FaceId, usize>,
    _existing_target_cuts: &HashMap<FaceId, Vec<usize>>,
    plane_table: &PlaneTable,
) -> Vec<(FaceId, Vec<usize>)> {
    let tool_plane_set: HashSet<usize> = tool_face_planes.values().copied().collect();

    let mut coplanar_targets: HashSet<u32> = HashSet::new();
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
        if !coplanar_targets.contains(&target_fid.index()) {
            continue;
        }

        let mut applicable_cuts = Vec::new();
        for &cut_idx in &tool_plane_set {
            if cut_idx == target_plane_idx {
                continue;
            }
            let target_plane = plane_table.get(target_plane_idx);
            let cut_plane = plane_table.get(cut_idx);
            if !planes_are_parallel(target_plane, cut_plane) {
                applicable_cuts.push(cut_idx);
            }
        }

        if !applicable_cuts.is_empty() {
            extra_cuts.push((*target_fid, applicable_cuts));
        }
    }

    for (he_id, he_data) in arena.iter_half_edges() {
        let face_a = he_data.face();
        if !coplanar_targets.contains(&face_a.index()) {
            continue;
        }

        let twin_id = he_data.twin();
        if twin_id == he_id {
            continue;
        }
        let twin_data = match arena.get_half_edge(twin_id) {
            Ok(d) => d,
            Err(_) => continue,
        };
        let adjacent_face = twin_data.face();

        if adjacent_face == face_a || coplanar_targets.contains(&adjacent_face.index()) {
            continue;
        }

        let adj_plane_idx = match target_face_planes.get(&adjacent_face) {
            Some(&idx) => idx,
            None => continue,
        };

        let mut applicable_cuts = Vec::new();
        for &cut_idx in &tool_plane_set {
            if cut_idx == adj_plane_idx {
                continue;
            }
            let adj_plane = plane_table.get(adj_plane_idx);
            let cut_plane = plane_table.get(cut_idx);
            if !planes_are_parallel(adj_plane, cut_plane) {
                applicable_cuts.push(cut_idx);
            }
        }

        if !applicable_cuts.is_empty() {
            extra_cuts.push((adjacent_face, applicable_cuts));
        }
    }

    extra_cuts
}

fn split_face_by_plane(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    dedup: &mut LocalVertexDedup,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    _face_plane_idx: usize,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    _config: &crate::core::ToleranceConfig,
    _plane_table: &PlaneTable,
    face_plane_map: &HashMap<FaceId, usize>,
    shared_registry: &mut SharedVertexRegistry,
    ctx: &mut ModelingContext,
) -> Result<Option<FaceId>, KernelError> {
    
    eprintln!("DEBUG: split_face_by_plane face={} plane_n={:?}", 
        face, cut_plane.raw_normal());

    let both_sides = has_vertices_on_both_sides(draft.arena(), geometry, face, cut_plane)?;
    if !both_sides {
        eprintln!("DEBUG: face={} rejected - not on both sides", face);
        return Ok(None);
    }

    let cut_points = find_cut_points_provenance(
        draft.arena(), geometry, face, cut_plane, cut_plane_idx, 
        dedup, face_plane_map, edge_cut_map, shared_registry, _plane_table
    )?;


    if cut_points.len() < 2 { 
        eprintln!("DEBUG: face={} rejected - cut_points < 2 ({})", face, cut_points.len());
        return Ok(None); 
    }

    // Resolve ALL cut points to vertex IDs first.
    let mut resolved: Vec<VertexId> = Vec::new();
    for cp in &cut_points {
        let vid = resolve_cut_point(cp, draft, geometry, dedup)?;
        resolved.push(vid);
    }

    // Remove duplicate VertexIds (can occur when a vertex sits exactly on the cut plane).
    resolved.dedup_by_key(|v| v.index());
    if resolved.len() < 2 {
        return Ok(None);
    }

    // Sort cut points by their 1D projection along the cut line direction.
    // The cut plane's normal is `n`. The cut line direction inside the face plane
    // lies perpendicular to both `n` and the face normal. We project each vertex
    // position onto an arbitrary reference direction perpendicular to `n` to get
    // a stable 1D ordering. This correctly handles concave faces with 5+ cut points:
    // for a concave polygon, the sorted sequence alternates inside/outside the face,
    // and pairing (0,1), (2,3), ... selects only the inside segments.
    let cut_normal = cut_plane.raw_normal();
    let ref_direction = {
        let n = cut_normal;
        // Project any world axis perpendicular to n to get a reference direction.
        let candidates = [[1.0_f64, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let mut best = candidates[0];
        let mut min_dot = f64::MAX;
        for c in &candidates {
            let dot = (c[0]*n[0] + c[1]*n[1] + c[2]*n[2]).abs();
            if dot < min_dot {
                min_dot = dot;
                best = *c;
            }
        }
        // Gram-Schmidt: subtract projection onto n.
        let dot_n = best[0]*n[0] + best[1]*n[1] + best[2]*n[2];
        let proj = [best[0] - dot_n*n[0], best[1] - dot_n*n[1], best[2] - dot_n*n[2]];
        let len = (proj[0]*proj[0] + proj[1]*proj[1] + proj[2]*proj[2]).sqrt();
        if len > 1e-12 { [proj[0]/len, proj[1]/len, proj[2]/len] } else { best }
    };

    resolved.sort_by(|a, b| {
        let pa = geometry.get_vertex_position(*a).map(|p| {
            p[0]*ref_direction[0] + p[1]*ref_direction[1] + p[2]*ref_direction[2]
        }).unwrap_or(0.0);
        let pb = geometry.get_vertex_position(*b).map(|p| {
            p[0]*ref_direction[0] + p[1]*ref_direction[1] + p[2]*ref_direction[2]
        }).unwrap_or(0.0);
        pa.partial_cmp(&pb).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Build the set of adjacent vertex pairs on this face's boundary.
    let face_edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut adjacent_pairs: HashSet<(u32, u32)> = HashSet::new();
    for he in &face_edges {
        let origin = draft.arena().get_half_edge(*he)?.origin();
        let next_he = draft.arena().get_half_edge(*he)?.next();
        let dest = draft.arena().get_half_edge(next_he)?.origin();
        let key = if origin.index() <= dest.index() {
            (origin.index(), dest.index())
        } else {
            (dest.index(), origin.index())
        };
        adjacent_pairs.insert(key);
    }

    // Apply MakeEdgeFace for each sequential (even, odd) pair.
    //
    // For a convex face: sorted has exactly 2 points, pair (0,1) is the only cut.
    // For a concave face: sorted has 4+ points; pairs (0,1), (2,3), ... each represent
    // a "crossing" through the interior of the face at the cut plane.
    //
    // We return the LAST new_face created, which is what the queue system tracks
    // to propagate remaining cuts. The earlier new_faces also need remaining cuts,
    // but they'll be re-queued via `find_extra_face_cuts` in `split_all_faces`.
    let mut last_new_face: Option<FaceId> = None;
    let mut i = 0;
    while i + 1 < resolved.len() {
        let v_a = resolved[i];
        let v_b = resolved[i + 1];
        i += 2;

        if v_a == v_b { continue; }
        let key = if v_a.index() <= v_b.index() {
            (v_a.index(), v_b.index())
        } else {
            (v_b.index(), v_a.index())
        };
        if adjacent_pairs.contains(&key) { continue; }

        let pos_a = geometry.get_vertex_position(v_a).map(|p| format!("[{:.4},{:.4},{:.4}]", p[0], p[1], p[2])).unwrap_or_else(|| "??".into());
        let pos_b = geometry.get_vertex_position(v_b).map(|p| format!("[{:.4},{:.4},{:.4}]", p[0], p[1], p[2])).unwrap_or_else(|| "??".into());
        eprintln!("DEBUG: face={} cut pair v_a={} {} v_b={} {} (from {} cut points)", face, v_a, pos_a, v_b, pos_b, resolved.len());

        let op = MakeEdgeFace { vertex_a: v_a, vertex_b: v_b, face };
        let res = match apply_op(draft, op) {
            Ok(r) => r,
            Err(_) => continue,
        };

        let edge_key = make_edge_key(v_a, v_b);
        edge_cut_map.insert(edge_key, cut_plane_idx);

        let new_face = res.get_value().new_face;
        geometry.set_face_plane(new_face, face_plane.clone());

        let mut decision = TracedDecision::new(
            DecisionId(face.index() as u64),
            DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!("Split face #{} by plane #{} -> new face #{}",
                    face.index(), cut_plane_idx, new_face.index())
            },
        );
        decision.set_entity_scope(EntityRef::new("Face", face.index()));
        ctx.get_decision_log_mut().record(decision);

        // Re-read face_edges after topology mutation to keep adjacent_pairs current.
        if i + 1 < resolved.len() {
            let new_edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), face)
                .ok()
                .map(|iter| iter.collect::<Result<Vec<_>, _>>().unwrap_or_default())
                .unwrap_or_default();
            for he in &new_edges {
                if let Ok(he_data) = draft.arena().get_half_edge(*he) {
                    let origin = he_data.origin();
                    if let Ok(next_data) = draft.arena().get_half_edge(he_data.next()) {
                        let dest = next_data.origin();
                        let pk = if origin.index() <= dest.index() {
                            (origin.index(), dest.index())
                        } else {
                            (dest.index(), origin.index())
                        };
                        adjacent_pairs.insert(pk);
                    }
                }
            }
        }

        last_new_face = Some(new_face);
    }

    Ok(last_new_face)
}


fn has_vertices_on_both_sides(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    cut_plane: &Plane,
) -> Result<bool, KernelError> {
    let edges: Vec<_> = FaceEdgeIterator::new(arena, face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut has_pos = false;
    let mut has_neg = false;
    for he in edges {
        let v = arena.get_half_edge(he)?.origin();
        let sign = if let Some(exact_pos) = geometry.get_vertex_position_exact(v) {
            classify_point_exact(cut_plane, exact_pos)
        } else if let Some(pos) = geometry.get_vertex_position(v) {
            if !pos[0].is_finite() || !pos[1].is_finite() || !pos[2].is_finite() {
                TriSign::Zero
            } else {
                match classify_point(cut_plane, pos) {
                    Ok(cert) => cert.sign(),
                    Err(_) => TriSign::Zero,
                }
            }
        } else {
            TriSign::Zero
        };
        match sign {
            TriSign::Pos => has_pos = true,
            TriSign::Neg => has_neg = true,
            _ => {}
        }
        if has_pos && has_neg { return Ok(true); }
    }
    Ok(false)
}

#[derive(Debug)]
enum CutPoint {
    Existing(VertexId),
    NewOnEdge {
        half_edge: HalfEdgeId,
        provenance: VertexMatchKey,
        position: [f64; 3],
        exact_position: Option<[Rational; 3]>,
    }
}

fn find_cut_points_provenance(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    dedup: &LocalVertexDedup,
    face_plane_map: &HashMap<FaceId, usize>,
    edge_cut_map: &EdgeCutMap,
    shared_registry: &mut SharedVertexRegistry,
    plane_table: &PlaneTable,
) -> Result<Vec<CutPoint>, KernelError> {
    
    let edges: Vec<_> = FaceEdgeIterator::new(arena, face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut points = Vec::new();
    
    for he in edges {
        let he_data = arena.get_half_edge(he)?;
        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next())?;
        let dest = next_data.origin();
        
        let p_o = match geometry.get_vertex_position(origin) {
            Some(p) => p,
            None => continue,
        };
        let p_d = match geometry.get_vertex_position(dest) {
            Some(p) => p,
            None => continue,
        };
        
        let s_o = if let Some(exact_o) = geometry.get_vertex_position_exact(origin) {
            classify_point_exact(cut_plane, exact_o)
        } else if !p_o[0].is_finite() || !p_o[1].is_finite() || !p_o[2].is_finite() {
            TriSign::Zero
        } else {
            match classify_point(cut_plane, p_o) {
                Ok(cert) => cert.sign(),
                Err(_) => TriSign::Zero,
            }
        };
        let s_d = if let Some(exact_d) = geometry.get_vertex_position_exact(dest) {
            classify_point_exact(cut_plane, exact_d)
        } else if !p_d[0].is_finite() || !p_d[1].is_finite() || !p_d[2].is_finite() {
            TriSign::Zero
        } else {
            match classify_point(cut_plane, p_d) {
                Ok(cert) => cert.sign(),
                Err(_) => TriSign::Zero,
            }
        };
        
        if s_o == TriSign::Zero {
            points.push(CutPoint::Existing(origin));
        } else if (s_o == TriSign::Pos && s_d == TriSign::Neg) || (s_o == TriSign::Neg && s_d == TriSign::Pos) {
            let twin = he_data.twin();
            let twin_face = arena.get_half_edge(twin)?.face();
            
            let p_face_idx = *face_plane_map.get(&face).unwrap();
            let p_twin_idx = *face_plane_map.get(&twin_face).unwrap_or(&p_face_idx);
            
            // Compute the exact intersection position.
            //
            // Primary path: use exact 3-plane intersection when the face and
            // adjacent face lie on different planes (the normal case).
            //
            // Fallback: use exact rational edge parameterization when both
            // faces share the same plane (coplanar case, e.g. y=0.5 cutting
            // across z=5 sub-faces after x-plane splits). Using p_o here
            // would give the wrong position and create vertices at edge corners
            // instead of the actual cut intersection.
            let (exact_pos, computed_pos): (Option<[Rational; 3]>, [f64; 3]) = {
                if p_face_idx != p_twin_idx {
                    let p0 = plane_table.get(p_face_idx);
                    let p1 = plane_table.get(p_twin_idx);
                    let p2 = plane_table.get(cut_plane_idx);
                    match intersect_three_planes_exact(p0, p1, p2) {
                        Ok(ep) => {
                            let fx = ep[0].to_f64_approx();
                            let fy = ep[1].to_f64_approx();
                            let fz = ep[2].to_f64_approx();
                            let f64_pos = if fx.is_finite() && fy.is_finite() && fz.is_finite() {
                                [fx, fy, fz]
                            } else {
                                let dist_o = signed_distance(cut_plane, p_o);
                                let dist_d = signed_distance(cut_plane, p_d);
                                let denom = dist_o - dist_d;
                                if denom.abs() < 1e-30 {
                                    [0.5*(p_o[0]+p_d[0]), 0.5*(p_o[1]+p_d[1]), 0.5*(p_o[2]+p_d[2])]
                                } else {
                                    let t = dist_o / denom;
                                    [p_o[0]+t*(p_d[0]-p_o[0]), p_o[1]+t*(p_d[1]-p_o[1]), p_o[2]+t*(p_d[2]-p_o[2])]
                                }
                            };
                            (Some(ep), f64_pos)
                        }
                        Err(_) => {
                            let dist_o = signed_distance(cut_plane, p_o);
                            let dist_d = signed_distance(cut_plane, p_d);
                            let denom = dist_o - dist_d;
                            let f64_pos = if denom.abs() < 1e-30 {
                                [0.5*(p_o[0]+p_d[0]), 0.5*(p_o[1]+p_d[1]), 0.5*(p_o[2]+p_d[2])]
                            } else {
                                let t = dist_o / denom;
                                [p_o[0]+t*(p_d[0]-p_o[0]), p_o[1]+t*(p_d[1]-p_o[1]), p_o[2]+t*(p_d[2]-p_o[2])]
                            };
                            (None, f64_pos)
                        }
                    }
                } else {
                    // Coplanar case: both face and adjacent face lie on the same plane.
                    // Use exact rational edge parameterization: P = origin + t*(dest - origin)
                    // where t satisfies cut_plane(P) = 0.
                    let dist_o = signed_distance(cut_plane, p_o);
                    let dist_d = signed_distance(cut_plane, p_d);
                    let denom = dist_o - dist_d;
                    let f64_pos = if denom.abs() < 1e-30 {
                        [0.5*(p_o[0]+p_d[0]), 0.5*(p_o[1]+p_d[1]), 0.5*(p_o[2]+p_d[2])]
                    } else {
                        let t = dist_o / denom;
                        [p_o[0]+t*(p_d[0]-p_o[0]), p_o[1]+t*(p_d[1]-p_o[1]), p_o[2]+t*(p_d[2]-p_o[2])]
                    };
                    // Build exact rational position from the f64 result.
                    // Every finite f64 has an exact rational form so this is lossless.
                    let exact_from_f64 = [
                        Rational::try_from_f64(f64_pos[0]).ok(),
                        Rational::try_from_f64(f64_pos[1]).ok(),
                        Rational::try_from_f64(f64_pos[2]).ok(),
                    ];
                    let ep = match (exact_from_f64[0].clone(), exact_from_f64[1].clone(), exact_from_f64[2].clone()) {
                        (Some(x), Some(y), Some(z)) => Some([x, y, z]),
                        _ => None,
                    };
                    (ep, f64_pos)
                }
            };

            let provenance = match &exact_pos {
                Some(ep) => VertexMatchKey::from_exact_position(ep[0].clone(), ep[1].clone(), ep[2].clone()),
                None => {
                    let rx = Rational::try_from_f64(computed_pos[0]).unwrap_or_else(|_| Rational::zero());
                    let ry = Rational::try_from_f64(computed_pos[1]).unwrap_or_else(|_| Rational::zero());
                    let rz = Rational::try_from_f64(computed_pos[2]).unwrap_or_else(|_| Rational::zero());
                    VertexMatchKey::from_exact_position(rx, ry, rz)
                }
            };

            let canonical_pos = shared_registry.canonical_position(&provenance, computed_pos);

            
            if let Some(vid) = dedup.find_by_provenance(&provenance) {
                points.push(CutPoint::Existing(vid));
            } else {
                points.push(CutPoint::NewOnEdge { half_edge: he, provenance, position: canonical_pos, exact_position: exact_pos });
            }
        }
    }
    Ok(points)
}

fn resolve_cut_point(
    cp: &CutPoint,
    draft: &mut MutableDraft,
    geom: &mut GeometryStore,
    dedup: &mut LocalVertexDedup,
) -> Result<VertexId, KernelError> {
    match cp {
        CutPoint::Existing(v) => Ok(*v),
        CutPoint::NewOnEdge { half_edge, provenance, position, exact_position } => {
            let res = apply_op(draft, SplitEdge { edge: *half_edge, parameter: 0.5 })?;
            let v = res.get_value().new_vertex;
            if let Some(exact) = exact_position {
                geom.set_vertex_position_exact(v, exact.clone());
            } else {
                geom.set_vertex_position(v, *position);
            }
            dedup.insert(v, provenance.clone());
            Ok(v)
        }
    }
}
