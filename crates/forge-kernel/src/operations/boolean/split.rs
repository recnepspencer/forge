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
use forge_geom::primitives::plane::{Plane, classify_point, signed_distance};
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
    /// Planes that are approximately equal get the same index.
    fn intern(&mut self, plane: &Plane) -> usize {
        for (i, p) in self.planes.iter().enumerate() {
            let n1 = p.raw_normal();
            let n2 = plane.raw_normal();
            let d1 = p.raw_offset();
            let d2 = plane.raw_offset();
            
            let dot = n1[0]*n2[0] + n1[1]*n2[1] + n1[2]*n2[2];
            let parallel = dot.abs() > 0.9999999999;
            
            if parallel {
                let sign = dot.signum();
                let dist_diff = if sign > 0.0 {
                    (d1 - d2).abs()
                } else {
                    (d1 + d2).abs()
                };
                
                if dist_diff < 1e-9 {
                    return i;
                }
            }
        }
        
        let idx = self.planes.len();
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
    
    for cuts in target_cuts.values_mut() { cuts.sort_unstable(); cuts.dedup(); }
    for cuts in tool_cuts.values_mut() { cuts.sort_unstable(); cuts.dedup(); }

    // 5. Perform Splits (shared registry ensures each intersection is computed once)
    let mut shared_registry = SharedVertexRegistry::new();

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
    assign_original_vertex_provenance(draft.arena(), initial_face_planes, &mut dedup)?;

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

/// Assign 3-plane provenance to every original vertex.
///
/// Each vertex of a planar B-Rep sits at the intersection of exactly 3
/// (or more, for non-generic geometry) face planes. We collect the unique
/// face-plane indices of all faces adjacent to a vertex. If exactly 3,
/// that's the canonical provenance. If more (e.g. 4 faces meet at a vertex
/// of a degenerate configuration), we pick the 3 smallest indices to form
/// a canonical key.
///
/// ROBUST APPROACH: Instead of walking the twin→next orbit around each
/// vertex (which breaks on re-used boolean results with stale twin pointers),
/// we iterate ALL faces and collect each face's vertices. This builds the
/// vertex→planes map directly and works regardless of twin pointer integrity.
fn assign_original_vertex_provenance(
    arena: &TopologyArena,
    face_plane_map: &HashMap<FaceId, usize>,
    dedup: &mut LocalVertexDedup,
) -> Result<(), KernelError> {
    let mut vertex_planes: HashMap<VertexId, HashSet<usize>> = HashMap::new();
    
    for (fid, _) in arena.iter_faces() {
        let plane_idx = match face_plane_map.get(&fid) {
            Some(&idx) => idx,
            None => continue,
        };
        
        let edges: Vec<_> = match FaceEdgeIterator::new(arena, fid) {
            Ok(iter) => iter.collect::<Result<Vec<_>, _>>()?,
            Err(_) => continue,
        };
        
        for he in edges {
            let v = arena.get_half_edge(he)?.origin();
            vertex_planes
                .entry(v)
                .or_default()
                .insert(plane_idx);
        }
    }
    
    for (vid, planes) in &vertex_planes {
        if planes.len() >= 3 {
            let mut sorted: Vec<usize> = planes.iter().copied().collect();
            sorted.sort_unstable();
            let prov = VertexMatchKey::from_planes(
                sorted[0],
                sorted[1],
                sorted[2],
            );
            dedup.insert(*vid, prov);
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
        dedup, face_plane_map, edge_cut_map, shared_registry
    )?;


    if cut_points.len() < 2 { 
        eprintln!("DEBUG: face={} rejected - cut_points < 2 ({})", face, cut_points.len());
        return Ok(None); 
    }

    let v_a = resolve_cut_point(&cut_points[0], draft, geometry, dedup)?;
    let v_b = resolve_cut_point(&cut_points[1], draft, geometry, dedup)?;

    eprintln!("DEBUG: face={} cut_points found v_a={} v_b={}", face, v_a, v_b);

    if v_a == v_b { 
        eprintln!("DEBUG: face={} rejected - v_a == v_b", face);
        for (i, cp) in cut_points.iter().enumerate() {
            eprintln!("  cut_point[{}]: {:?}", i, cp);
        }
        return Ok(None); 
    }

    // Check if an edge already exists between v_a and v_b (prevents double-split).
    // This happens when coplanar tool faces (e.g., adjacent faces on the same flat
    // side of a cube) both try to cut the same target edge.
    let mut edge_already_exists = false;    // Verify face is convex
    let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), face)?
        .collect::<Result<Vec<_>, _>>()?;
    for he in edges {
        let origin = draft.arena().get_half_edge(he)?.origin();
        let next_he = draft.arena().get_half_edge(he)?.next();
        let dest = draft.arena().get_half_edge(next_he)?.origin();
        
        if (origin == v_a && dest == v_b) || (origin == v_b && dest == v_a) {
            edge_already_exists = true;
            break;
        }
    }

    if edge_already_exists {
        return Ok(None);
    }

    let op = MakeEdgeFace { vertex_a: v_a, vertex_b: v_b, face };
    
    let res = match apply_op(draft, op) {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };
    
    // Record the provenance: this new edge was created by this cut plane.
    let edge_key = make_edge_key(v_a, v_b);
    edge_cut_map.insert(edge_key, cut_plane_idx);
    
    let new_face = res.get_value().new_face;
    geometry.set_face_plane(new_face, face_plane.clone());
    
    // Log the split
    let mut decision = TracedDecision::new(
        DecisionId(face.index() as u64), // Use face ID as stable base
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

    Ok(Some(new_face))
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
        let pos = geometry.get_vertex_position(v).unwrap();
        let sign = classify_point(cut_plane, pos).unwrap().sign();
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
) -> Result<Vec<CutPoint>, KernelError> {
    
    let edges: Vec<_> = FaceEdgeIterator::new(arena, face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut points = Vec::new();
    
    for he in edges {
        let he_data = arena.get_half_edge(he)?;
        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next())?;
        let dest = next_data.origin();
        
        let p_o = geometry.get_vertex_position(origin).unwrap();
        let p_d = geometry.get_vertex_position(dest).unwrap();
        
        let s_o = classify_point(cut_plane, p_o).unwrap().sign();
        let s_d = classify_point(cut_plane, p_d).unwrap().sign();
        
        if s_o == TriSign::Zero {
            points.push(CutPoint::Existing(origin));
        } else if (s_o == TriSign::Pos && s_d == TriSign::Neg) || (s_o == TriSign::Neg && s_d == TriSign::Pos) {
            let twin = he_data.twin();
            let twin_face = arena.get_half_edge(twin)?.face();
            
            let p_face_idx = *face_plane_map.get(&face).unwrap();
            let p_twin_idx = *face_plane_map.get(&twin_face).unwrap_or(&p_face_idx);
            
            let provenance = if p_face_idx != p_twin_idx {
                VertexMatchKey::from_planes(p_face_idx, p_twin_idx, cut_plane_idx)
            } else {
                // Degenerate edge: face and twin share the same plane.
                // First check EdgeCutMap (edge was created by a prior cut in this boolean).
                let edge_key = make_edge_key(origin, dest);
                if let Some(&prior_cut_idx) = edge_cut_map.get(&edge_key) {
                    VertexMatchKey::from_planes(p_face_idx, prior_cut_idx, cut_plane_idx)
                } else {
                    // Edge is original to the input solid (e.g. result of a prior boolean).
                    // Derive the edge's two supporting planes from endpoint provenance:
                    // the planes shared between origin's and dest's 3-plane keys define
                    // the line the edge lies on.
                    let origin_prov = dedup.provenance.get(&origin);
                    let dest_prov = dedup.provenance.get(&dest);
                    let second_plane = origin_prov.and_then(|op| {
                        dest_prov.and_then(|dp| {
                            let o_planes = op.planes();
                            let d_planes = dp.planes();
                            // Find the shared plane index that isn't p_face_idx
                            for &p in o_planes {
                                if p != p_face_idx && d_planes.contains(&p) {
                                    return Some(p);
                                }
                            }
                            None
                        })
                    }).unwrap_or(p_face_idx);
                    VertexMatchKey::from_planes(p_face_idx, second_plane, cut_plane_idx)
                }
            };
            
            let dist_o = signed_distance(cut_plane, p_o);
            let dist_d = signed_distance(cut_plane, p_d);
            let t = dist_o / (dist_o - dist_d);
            let computed_pos = [
                p_o[0] + t*(p_d[0]-p_o[0]),
                p_o[1] + t*(p_d[1]-p_o[1]),
                p_o[2] + t*(p_d[2]-p_o[2]),
            ];
            
            // Use the shared registry to get the canonical position.
            // If the other solid already computed this intersection,
            // we reuse its exact position — zero FP divergence.
            let canonical_pos = shared_registry.canonical_position(&provenance, computed_pos);
            
            if let Some(vid) = dedup.find_by_provenance(&provenance) {
                points.push(CutPoint::Existing(vid));
            } else {
                points.push(CutPoint::NewOnEdge { half_edge: he, provenance, position: canonical_pos });
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
        CutPoint::NewOnEdge { half_edge, provenance, position } => {
            let res = apply_op(draft, SplitEdge { edge: *half_edge, parameter: 0.5 })?;
            let v = res.get_value().new_vertex;
            geom.set_vertex_position(v, *position);
            dedup.insert(v, provenance.clone());
            Ok(v)
        }
    }
}
