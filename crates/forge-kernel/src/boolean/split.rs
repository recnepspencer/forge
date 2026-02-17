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
use forge_core::result::DecisionLog;
use forge_geom::aabb::Aabb;
use forge_geom::bvh::{BvhNode, query_overlapping_pairs};
use forge_geom::plane::{Plane, classify_point, signed_distance};
use forge_math::sign::TriSign;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId, HalfEdgeId};
use forge_topo::state::{TopologyState, MutableDraft};
use forge_topo::traverse::face_edges;
use forge_topo::operator::apply_op;
use forge_topo::euler::split_edge::SplitEdge;
use forge_topo::euler::make_edge_face::MakeEdgeFace;

use crate::geometry_store::GeometryStore;
use crate::boolean::eval::{VertexMatchKey, planes_are_parallel};

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
    pub decision_log: DecisionLog,
}

impl SplitPhaseResult {
    pub fn split_count(&self) -> usize { self.split_count }
    pub fn take_decision_log(&mut self) -> DecisionLog { std::mem::take(&mut self.decision_log) }
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

pub fn split_all_faces(
    target_topo: TopologyState,
    target_geom: GeometryStore,
    tool_topo: TopologyState,
    tool_geom: GeometryStore,
) -> Result<SplitPhaseResult, KernelError> {
    
    let log = DecisionLog::new();
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
    let target_bvh = BvhNode::build(target_aabbs_indexed);
    let tool_bvh = BvhNode::build(tool_aabbs_indexed);
    
    let mut potential_pairs = Vec::new();
    if let (Some(root_a), Some(root_b)) = (target_bvh, tool_bvh) {
        potential_pairs = query_overlapping_pairs(&root_a, &root_b);
    }
    
    // 4. Collect cuts per face
    let mut target_cuts: HashMap<FaceId, Vec<usize>> = HashMap::new();
    let mut tool_cuts: HashMap<FaceId, Vec<usize>> = HashMap::new();
    
    for (idx_a_raw, idx_b_raw) in potential_pairs {
        let face_a = target_aabbs_full[idx_a_raw].0;
        let face_b = tool_aabbs_full[idx_b_raw].0;
        
        if let Some(&plane_idx_b) = tool_face_planes.get(&face_b) {
            if let Some(plane_idx_a) = target_face_planes.get(&face_a) {
                let plane_a = plane_table.get(*plane_idx_a);
                let plane_b = plane_table.get(plane_idx_b);
                if !planes_are_parallel(plane_a, plane_b) {
                    target_cuts.entry(face_a).or_default().push(plane_idx_b);
                }
            }
        }
        
        if let Some(&plane_idx_a) = target_face_planes.get(&face_a) {
             if let Some(plane_idx_b) = tool_face_planes.get(&face_b) {
                 let plane_a = plane_table.get(plane_idx_a);
                 let plane_b = plane_table.get(*plane_idx_b);
                 if !planes_are_parallel(plane_b, plane_a) {
                    tool_cuts.entry(face_b).or_default().push(plane_idx_a);
                 }
             }
        }
    }
    
    for cuts in target_cuts.values_mut() { cuts.sort_unstable(); cuts.dedup(); }
    for cuts in tool_cuts.values_mut() { cuts.sort_unstable(); cuts.dedup(); }

    // 5. Perform Splits
    let (target_res_topo, target_res_geom, target_splits, target_dedup) = split_solid(
        target_topo, target_geom, target_cuts, &target_face_planes, &mut plane_table, &config
    )?;
    
    let (tool_res_topo, tool_res_geom, tool_splits, tool_dedup) = split_solid(
        tool_topo, tool_geom, tool_cuts, &tool_face_planes, &mut plane_table, &config
    )?;

    Ok(SplitPhaseResult {
        target_topology: target_res_topo,
        target_geometry: target_res_geom,
        tool_topology: tool_res_topo,
        tool_geometry: tool_res_geom,
        split_count: target_splits + tool_splits,
        target_provenance: target_dedup.provenance,
        tool_provenance: tool_dedup.provenance,
        decision_log: log,
    })
}

fn split_solid(
    topo: TopologyState, 
    mut geom: GeometryStore, 
    cuts_map: HashMap<FaceId, Vec<usize>>,
    initial_face_planes: &HashMap<FaceId, usize>,
    plane_table: &mut PlaneTable,
    config: &crate::core::ToleranceConfig,
) -> Result<(TopologyState, GeometryStore, usize, LocalVertexDedup), KernelError> {
    
    let mut draft = topo.begin_mutation();
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
            &current_face_planes
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
fn assign_original_vertex_provenance(
    arena: &TopologyArena,
    face_plane_map: &HashMap<FaceId, usize>,
    dedup: &mut LocalVertexDedup,
) -> Result<(), KernelError> {
    for (vid, _) in arena.iter_vertices() {
        let adjacent_planes = collect_vertex_plane_indices(arena, vid, face_plane_map)?;
        
        if adjacent_planes.len() >= 3 {
            let prov = VertexMatchKey::from_planes(
                adjacent_planes[0],
                adjacent_planes[1],
                adjacent_planes[2],
            );
            dedup.insert(vid, prov);
        }
        // If < 3 adjacent planes, vertex is degenerate — skip provenance.
        // It won't participate in cross-solid matching.
    }
    Ok(())
}

/// Collect the unique, sorted face-plane indices adjacent to a vertex.
///
/// Walks the fan of halfedges around `vertex` via the twin→next orbit.
fn collect_vertex_plane_indices(
    arena: &TopologyArena,
    vertex: VertexId,
    face_plane_map: &HashMap<FaceId, usize>,
) -> Result<Vec<usize>, KernelError> {
    let mut plane_set = HashSet::new();
    let start_he = arena.get_vertex(vertex)?.outgoing;
    let mut current = start_he;
    let max_iter = 100;
    
    for _ in 0..max_iter {
        let he_data = arena.get_half_edge(current)?;
        if let Some(&plane_idx) = face_plane_map.get(&he_data.face) {
            plane_set.insert(plane_idx);
        }
        
        let twin = he_data.twin;
        let twin_data = arena.get_half_edge(twin)?;
        current = twin_data.next;
        
        if current == start_he {
            break;
        }
    }
    
    let mut result: Vec<usize> = plane_set.into_iter().collect();
    result.sort_unstable();
    Ok(result)
}

fn compute_face_aabbs(arena: &TopologyArena, geom: &GeometryStore) -> Result<Vec<(FaceId, Aabb)>, KernelError> {
    let mut list = Vec::new();
    for (fid, _) in arena.iter_faces() {
         let edges = face_edges(arena, fid)?;
         let mut points = Vec::new();
         for he in edges {
              let v = arena.get_half_edge(he)?.origin;
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
) -> Result<Option<FaceId>, KernelError> {
    
    let both_sides = has_vertices_on_both_sides(draft.arena(), geometry, face, cut_plane)?;
    eprintln!("  split_face_by_plane: face={}, cut_plane_idx={}, both_sides={}", face, cut_plane_idx, both_sides);
    if !both_sides {
        return Ok(None);
    }

    let cut_points = find_cut_points_provenance(
        draft.arena(), geometry, face, cut_plane, cut_plane_idx, 
        dedup, face_plane_map, edge_cut_map
    )?;

    eprintln!("    cut_points.len()={}", cut_points.len());
    for (i, cp) in cut_points.iter().enumerate() {
        match cp {
            CutPoint::Existing(v) => eprintln!("    cp[{}] = Existing({})", i, v),
            CutPoint::NewOnEdge { half_edge, position, .. } =>
                eprintln!("    cp[{}] = NewOnEdge(he={}, pos={:?})", i, half_edge, position),
        }
    }

    if cut_points.len() < 2 { return Ok(None); }

    let v_a = resolve_cut_point(&cut_points[0], draft, geometry, dedup)?;
    let v_b = resolve_cut_point(&cut_points[1], draft, geometry, dedup)?;

    if v_a == v_b { return Ok(None); }

    // Check if an edge already exists between v_a and v_b (prevents double-split).
    // This happens when coplanar tool faces (e.g., adjacent faces on the same flat
    // side of a cube) both try to cut the same target edge.
    let mut edge_already_exists = false;
    let edges = face_edges(draft.arena(), face)?;
    for he in edges {
        let origin = draft.arena().get_half_edge(he)?.origin;
        let next_he = draft.arena().get_half_edge(he)?.next;
        let dest = draft.arena().get_half_edge(next_he)?.origin;
        
        if (origin == v_a && dest == v_b) || (origin == v_b && dest == v_a) {
            edge_already_exists = true;
            break;
        }
    }

    if edge_already_exists {
        eprintln!("    -> ABORT: Edge between {} and {} already exists. Skipping split.", v_a, v_b);
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
    eprintln!("    -> split succeeded, new_face={}", new_face);
    
    Ok(Some(new_face))
}

fn has_vertices_on_both_sides(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    cut_plane: &Plane,
) -> Result<bool, KernelError> {
    let edges = face_edges(arena, face)?;
    let mut has_pos = false;
    let mut has_neg = false;
    for he in edges {
        let v = arena.get_half_edge(he)?.origin;
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
) -> Result<Vec<CutPoint>, KernelError> {
    
    let edges = face_edges(arena, face)?;
    let mut points = Vec::new();
    
    for he in edges {
        let he_data = arena.get_half_edge(he)?;
        let origin = he_data.origin;
        let next_data = arena.get_half_edge(he_data.next)?;
        let dest = next_data.origin;
        
        let p_o = geometry.get_vertex_position(origin).unwrap();
        let p_d = geometry.get_vertex_position(dest).unwrap();
        
        let s_o = classify_point(cut_plane, p_o).unwrap().sign();
        let s_d = classify_point(cut_plane, p_d).unwrap().sign();
        
        if s_o == TriSign::Zero {
            points.push(CutPoint::Existing(origin));
        } else if (s_o == TriSign::Pos && s_d == TriSign::Neg) || (s_o == TriSign::Neg && s_d == TriSign::Pos) {
            let twin = he_data.twin;
            let twin_face = arena.get_half_edge(twin)?.face;
            
            let p_face_idx = *face_plane_map.get(&face).unwrap();
            let p_twin_idx = *face_plane_map.get(&twin_face).unwrap_or(&p_face_idx);
            
            let provenance = if p_face_idx != p_twin_idx {
                VertexMatchKey::from_planes(p_face_idx, p_twin_idx, cut_plane_idx)
            } else {
                // Degenerate edge: face and twin share the same plane.
                // This edge was created by a previous cut — look up
                // the prior cut plane from the EdgeCutMap.
                let edge_key = make_edge_key(origin, dest);
                let prior_cut_idx = edge_cut_map.get(&edge_key).copied()
                    .unwrap_or(p_face_idx);
                VertexMatchKey::from_planes(p_face_idx, prior_cut_idx, cut_plane_idx)
            };
            
            let dist_o = signed_distance(cut_plane, p_o);
            let dist_d = signed_distance(cut_plane, p_d);
            let t = dist_o / (dist_o - dist_d);
            let pos = [
                p_o[0] + t*(p_d[0]-p_o[0]),
                p_o[1] + t*(p_d[1]-p_o[1]),
                p_o[2] + t*(p_d[2]-p_o[2]),
            ];
            
            if let Some(vid) = dedup.find_by_provenance(&provenance) {
                points.push(CutPoint::Existing(vid));
            } else {
                points.push(CutPoint::NewOnEdge { half_edge: he, provenance, position: pos });
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
