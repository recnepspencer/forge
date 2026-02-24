//! Topology traversal utilities.
//!
//! DOMAIN: Read-only traversal of face loops and vertex rings.
//!
//! INVARIANTS:
//! - Traversal uses explicit twin, next, prev pointers on HalfEdgeData
//! - Cycle detection guards against infinite loops (max iterations)
//! - Iterators yield `Result<HalfEdgeId, KernelError>` to handle topology corruption lazily
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs)

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, VertexId};

// =========================================================================
// Face Edge Iterator (Zero Allocation)
// =========================================================================

/// Iterator over halfedges in a face loop.
///
/// Yields `HalfEdgeId`s following the `next` pointer chain.
/// Returns `Err` if the loop exceeds `MAX_ITER` (corrupted topology) or if
/// a handle is stale.
pub struct FaceEdgeIterator<'a> {
    arena: &'a TopologyArena,
    start: HalfEdgeId,
    current: Option<HalfEdgeId>,
    steps: usize,
    finished: bool,
}

impl<'a> FaceEdgeIterator<'a> {
    const MAX_ITER: usize = 100_000;

    /// Create a new iterator around a face.
    pub fn new(arena: &'a TopologyArena, face: FaceId) -> Result<Self, KernelError> {
        let face_data = arena.get_face(face)?;
        let loop_data = arena.get_loop(face_data.outer_loop())?;
        let start = loop_data.half_edge();
        
        Ok(Self {
            arena,
            start,
            current: Some(start),
            steps: 0,
            finished: false,
        })
    }
}

impl<'a> Iterator for FaceEdgeIterator<'a> {
    type Item = Result<HalfEdgeId, KernelError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let curr_id = match self.current {
            Some(id) => id,
            None => {
                self.finished = true;
                return None;
            }
        };

        // Cycle guard
        self.steps += 1;
        if self.steps >= Self::MAX_ITER {
            self.finished = true;
            return Some(Err(KernelError::InternalError {
                message: format!("Face loop exceeded {} iterations — likely corrupted", Self::MAX_ITER),
                context: None,
            }));
        }

        // Fetch data for next step
        match self.arena.get_half_edge(curr_id) {
            Ok(he_data) => {
                let next_id = he_data.next();
                if next_id == self.start {
                    self.current = None; // Finishes after this yield
                } else {
                    self.current = Some(next_id);
                }
                Some(Ok(curr_id))
            }
            Err(e) => {
                self.finished = true;
                Some(Err(e))
            }
        }
    }
}

// =========================================================================
// Vertex Ring Iterator (Zero Allocation)
// =========================================================================

/// Iterator over outgoing halfedges around a vertex.
///
/// Scans the arena and returns all halfedges originating at this vertex.
pub struct VertexRingIterator<'a> {
    arena: &'a TopologyArena,
    vertex: VertexId,
    outgoing_halfedges: std::vec::IntoIter<HalfEdgeId>,
}

impl<'a> VertexRingIterator<'a> {
    /// Create a new iterator around a vertex.
    /// 
    /// Scans the arena for all halfedges originating at this vertex.
    /// This is necessary because non-manifold (radial) edges can break 
    /// the continuous `twin -> next` cycle into disjoint orbits.
    pub fn new(arena: &'a TopologyArena, vertex: VertexId) -> Result<Self, KernelError> {
        let mut edges = Vec::new();
        for (id, data) in arena.iter_half_edges() {
            if data.origin() == vertex {
                edges.push(id);
            }
        }
        
        // Ensure deterministic order
        edges.sort_by_key(|h| h.index());

        Ok(Self {
            arena,
            vertex,
            outgoing_halfedges: edges.into_iter(),
        })
    }
}

impl<'a> Iterator for VertexRingIterator<'a> {
    type Item = Result<HalfEdgeId, KernelError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.outgoing_halfedges.next().map(Ok)
    }
}

// =========================================================================
// Vertex Neighborhood Orbits
// =========================================================================

/// Decompose a vertex's outgoing half-edges into connected neighborhood orbits.
///
/// Each orbit is a maximal set of outgoing half-edges whose incident faces
/// form a connected component around this vertex. The algorithm uses
/// face-based BFS: two outgoing half-edges belong to the same orbit if
/// their faces are connected through edges incident to this vertex.
///
/// This correctly handles:
/// - **Interior manifold vertices**: one orbit (standard `twin → next` cycle)
/// - **Boundary vertices**: one orbit (self-radial boundary edges don't
///   break face connectivity — faces are still reachable via `next`/`prev`)
/// - **Non-manifold pinch vertices**: multiple orbits (disjoint bodies
///   sharing a geometric point have no face connectivity through the vertex)
///
/// Academic basis: Weiler (1988) Radial Edge Structure §3.2;
/// Kettner (1997) halfedge surface design §4; Lienhardt (1994) G-map orbits.
///
/// Returns orbits sorted by minimum half-edge index for deterministic
/// ordering (Doctrine D1). Half-edges within each orbit are in index order.
pub fn vertex_neighborhood_orbits(
    arena: &TopologyArena,
    vertex: VertexId,
) -> Result<Vec<Vec<HalfEdgeId>>, KernelError> {
    let all_outgoing: Vec<HalfEdgeId> = VertexRingIterator::new(arena, vertex)?
        .collect::<Result<Vec<_>, _>>()?;

    if all_outgoing.is_empty() {
        return Ok(Vec::new());
    }

    let mut outgoing_set = crate::topology::bitset::EntityBitset::for_half_edges(arena);
    for &h in &all_outgoing {
        outgoing_set.insert(h.index()).ok();
    }

    let mut he_to_orbit: std::collections::BTreeMap<u32, usize> = std::collections::BTreeMap::new();
    let mut orbits: Vec<Vec<HalfEdgeId>> = Vec::new();

    for &seed_he in &all_outgoing {
        if he_to_orbit.contains_key(&seed_he.index()) {
            continue;
        }

        let orbit_idx = orbits.len();
        let mut orbit = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(seed_he);
        he_to_orbit.insert(seed_he.index(), orbit_idx);

        while let Some(he_id) = queue.pop_front() {
            orbit.push(he_id);

            let he_data = arena.get_half_edge(he_id)?;

            let radial = he_data.radial_next();
            if radial != he_id {
                let radial_data = arena.get_half_edge(radial)?;
                let candidate = radial_data.next();
                let candidate_data = arena.get_half_edge(candidate)?;
                if candidate_data.origin() == vertex
                    && outgoing_set.contains(candidate.index()).unwrap_or(false)
                    && !he_to_orbit.contains_key(&candidate.index())
                {
                    he_to_orbit.insert(candidate.index(), orbit_idx);
                    queue.push_back(candidate);
                }
            }

            let prev_data = arena.get_half_edge(he_data.prev())?;
            let prev_radial = prev_data.radial_next();
            if prev_radial != he_data.prev() {
                if outgoing_set.contains(prev_radial.index()).unwrap_or(false)
                    && !he_to_orbit.contains_key(&prev_radial.index())
                {
                    let prev_radial_data = arena.get_half_edge(prev_radial)?;
                    if prev_radial_data.origin() == vertex {
                        he_to_orbit.insert(prev_radial.index(), orbit_idx);
                        queue.push_back(prev_radial);
                    }
                }
            }
        }

        orbit.sort_by_key(|h| h.index());
        orbits.push(orbit);
    }

    orbits.sort_by_key(|orbit| orbit.iter().map(|h| h.index()).min().unwrap_or(u32::MAX));

    Ok(orbits)
}

// =========================================================================
// Radial Edge Iterator (Zero Allocation)
// =========================================================================

/// Iterator over halfedges in a radial ring around a geometric edge.
///
/// Follows `radial_next` to circle the edge.
pub struct RadialEdgeIterator<'a> {
    arena: &'a TopologyArena,
    start: HalfEdgeId,
    current: HalfEdgeId,
    first: bool,
    iter_count: usize,
}

impl<'a> RadialEdgeIterator<'a> {
    const MAX_ITER: usize = 100_000;

    /// Create a new iterator around an edge's radial ring.
    pub fn new(arena: &'a TopologyArena, start: HalfEdgeId) -> Result<Self, KernelError> {
        arena.get_half_edge(start)?; // validate existence
        Ok(Self {
            arena,
            start,
            current: start,
            first: true,
            iter_count: 0,
        })
    }
}

impl<'a> Iterator for RadialEdgeIterator<'a> {
    type Item = Result<HalfEdgeId, KernelError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_count > Self::MAX_ITER {
            return Some(Err(KernelError::InternalError {
                message: format!("Radial loop traversal exceeded {} iterations (corrupted topology)", Self::MAX_ITER),
                context: None,
            }));
        }

        if !self.first && self.current == self.start {
            return None; // Loop completed
        }

        let he_data = match self.arena.get_half_edge(self.current) {
            Ok(h) => h,
            Err(e) => return Some(Err(e)),
        };

        let result = self.current;
        self.current = he_data.radial_next();
        self.first = false;
        self.iter_count += 1;

        Some(Ok(result))
    }
}

// =========================================================================
// Helpers
// =========================================================================

/// Count the number of edges in a face loop (allocates nothing).
pub fn face_edge_count(arena: &TopologyArena, face: FaceId) -> Result<usize, KernelError> {
    let mut count = 0;
    for res in FaceEdgeIterator::new(arena, face)? {
        res?;
        count += 1;
    }
    Ok(count)
}

/// Check if an edge is a boundary edge (self-radial, valence 1).
pub fn is_boundary_edge(arena: &TopologyArena, he: HalfEdgeId) -> Result<bool, KernelError> {
    let he_data = arena.get_half_edge(he)?;
    Ok(he_data.radial_next() == he)
}

/// Count the number of faces sharing a geometric edge (radial valence).
pub fn radial_valence(arena: &TopologyArena, he: HalfEdgeId) -> Result<usize, KernelError> {
    let mut count = 0;
    for res in RadialEdgeIterator::new(arena, he)? {
        res?;
        count += 1;
    }
    Ok(count)
}

/// Get all faces adjacent to a geometric edge from its radial ring.
pub fn edge_faces(arena: &TopologyArena, he: HalfEdgeId) -> Result<Vec<FaceId>, KernelError> {
    let mut faces = Vec::new();
    for res in RadialEdgeIterator::new(arena, he)? {
        let h = res?;
        let he_data = arena.get_half_edge(h)?;
        faces.push(he_data.face());
    }
    Ok(faces)
}

/// Walk a G1-continuous (tangent-continuous) edge chain starting at `start_edge`.
///
/// Fillet operations act on edge *chains* — sequences of edges that share vertices
/// and whose adjacent faces are tangent-continuous (G1) at those vertices.
/// This function walks the chain by following `twin → next` at each shared vertex
/// (the standard manifold-edge successor) and extends the chain while the dihedral
/// angle between the adjacent face normals is below `angle_threshold` (in radians).
///
/// Non-bridge edges are followed; bridge halfedges (from `BridgeEdge`) are detected
/// and skipped so that synthetic boundaries do not splice into organic fillet chains.
///
/// # Arguments
/// - `arena`: topology arena
/// - `start_edge`: the first halfedge of the chain
/// - `position_fn`: maps vertex index → 3D world position (needed for normals)
/// - `angle_threshold`: maximum dihedral angle (radians) to consider G1-continuous
///
/// Returns the ordered list of halfedge IDs forming the G1 chain.
pub fn find_g1_chain(
    arena: &TopologyArena,
    start_edge: HalfEdgeId,
    position_fn: &dyn Fn(crate::handles::VertexId) -> Option<[f64; 3]>,
    angle_threshold: f64,
    degeneracy_tol: f64,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let cos_threshold = angle_threshold.cos();
    let max_iter = arena.half_edge_count().max(1);
    let mut chain = vec![start_edge];
    let mut current = start_edge;

    for _ in 0..max_iter {
        let he_data = arena.get_half_edge(current)?;

        // Advance: twin of current → next of that twin → candidate next edge.
        let twin_id = he_data.radial_next();
        let twin_data = arena.get_half_edge(twin_id)?;
        let candidate = twin_data.next();

        // Stop if we've looped back to the start.
        if candidate == start_edge { break; }

        // Skip bridge halfedges — they are synthetic and should not be filleted.
        let candidate_data = arena.get_half_edge(candidate)?;
        if candidate_data.is_bridge() { break; }

        // Compute dihedral angle between the face of `current` and the face of `candidate`.
        let face_a = he_data.face();
        let face_b = candidate_data.face();

        if face_a == face_b {
            // Same face — the chain has folded back; stop.
            break;
        }

        let normal_a = face_normal_from_loop(arena, face_a, position_fn, degeneracy_tol)?;
        let normal_b = face_normal_from_loop(arena, face_b, position_fn, degeneracy_tol)?;

        if let (Some(na), Some(nb)) = (normal_a, normal_b) {
            let dot = na[0]*nb[0] + na[1]*nb[1] + na[2]*nb[2];
            if dot < cos_threshold {
                // Dihedral angle exceeds threshold — chain ends here.
                break;
            }
        } else {
            // Degenerate face — cannot compute normal, stop cascading.
            break;
        }

        chain.push(candidate);
        current = candidate;
    }

    Ok(chain)
}

/// Compute the face normal from the first three non-collinear vertices in the loop.
///
/// Returns `None` for degenerate (collinear) faces.
fn face_normal_from_loop(
    arena: &TopologyArena,
    face: FaceId,
    position_fn: &dyn Fn(crate::handles::VertexId) -> Option<[f64; 3]>,
    degeneracy_tol: f64,
) -> Result<Option<[f64; 3]>, KernelError> {
    let mut positions: Vec<[f64; 3]> = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face)? {
        let he_id = he_res?;
        let v = arena.get_half_edge(he_id)?.origin();
        if let Some(pos) = position_fn(v) {
            positions.push(pos);
            if positions.len() >= 3 { break; }
        }
    }

    if positions.len() < 3 {
        return Ok(None);
    }

    let a = positions[0];
    let b = positions[1];
    let c = positions[2];
    let ab = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
    let ac = [c[0]-a[0], c[1]-a[1], c[2]-a[2]];
    let nx = ab[1]*ac[2] - ab[2]*ac[1];
    let ny = ab[2]*ac[0] - ab[0]*ac[2];
    let nz = ab[0]*ac[1] - ab[1]*ac[0];
    let len = (nx*nx + ny*ny + nz*nz).sqrt();
    if len < degeneracy_tol {
        return Ok(None);
    }
    Ok(Some([nx/len, ny/len, nz/len]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::TopologyState;
    use crate::operator::apply_op;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::make_edge_face::MakeEdgeFace;
    use crate::euler::split_edge::SplitEdge;
    use crate::euler::kill_edge_vertex::KillEdgeVertex;

    #[test]
    fn face_edges_on_seed() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();

        let edges: Vec<HalfEdgeId> = FaceEdgeIterator::new(state.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], mvf.half_edge);
    }

    #[test]
    fn face_edges_after_split() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let _se = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).unwrap().into_value();
        let state = draft.commit().unwrap();

        let edges: Vec<HalfEdgeId> = FaceEdgeIterator::new(state.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
            
        assert_eq!(edges.len(), 2);
    }

    // ── Vertex Neighborhood Orbit Tests ───────────────────────────────────

    /// Manifold vertex shared by two triangular faces via MEF diagonal.
    ///
    /// Build quad (MVF + 3×SE), then MEF a diagonal. The shared vertex
    /// sits at the junction of both faces. Because the topology is
    /// manifold, the twin→next cycle reaches all outgoing half-edges
    /// in a single orbit.
    #[test]
    fn orbits_manifold_vertex_two_triangle_faces() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.25 }).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();
        let _se3 = apply_op(&mut draft, SplitEdge { edge: se2.he_mb, parameter: 0.75 }).unwrap().into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap()).collect();
        assert_eq!(edges.len(), 4);

        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin();

        let _mef = apply_op(&mut draft, MakeEdgeFace {
            face: mvf.face, vertex_a: v1, vertex_b: v3,
        }).unwrap().into_value();

        let orbits = vertex_neighborhood_orbits(draft.arena(), v1).unwrap();
        assert_eq!(orbits.len(), 1, "Manifold vertex must have exactly 1 orbit, got {}", orbits.len());

        let total_outgoing: Vec<_> = VertexRingIterator::new(draft.arena(), v1).unwrap()
            .map(|r| r.unwrap()).collect();
        let orbit_count: usize = orbits.iter().map(|o| o.len()).sum();
        assert_eq!(
            orbit_count, total_outgoing.len(),
            "Orbit must account for all outgoing half-edges: orbit has {}, ring has {}",
            orbit_count, total_outgoing.len()
        );
    }

    /// Boundary vertex on an open mesh — vertex shared by two faces with
    /// boundary edges.
    ///
    /// Build a quad (MVF + 3×SE), then MEF a diagonal to create two
    /// triangular faces. The diagonal's endpoints are interior but the
    /// other two vertices sit on the boundary (self-radial edges). Pick
    /// a boundary vertex and verify it has exactly 1 orbit despite some
    /// outgoing edges being boundary.
    #[test]
    fn orbits_boundary_vertex_open_mesh() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.25 }).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();
        let _se3 = apply_op(&mut draft, SplitEdge { edge: se2.he_mb, parameter: 0.75 }).unwrap().into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap()).collect();
        assert_eq!(edges.len(), 4);

        let v0 = draft.arena().get_half_edge(edges[0]).unwrap().origin();
        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin();
        let v2 = draft.arena().get_half_edge(edges[2]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin();

        let _mef = apply_op(&mut draft, MakeEdgeFace {
            face: mvf.face, vertex_a: v1, vertex_b: v3,
        }).unwrap().into_value();

        let boundary_vertex = v0;
        let orbits = vertex_neighborhood_orbits(draft.arena(), boundary_vertex).unwrap();

        let total_outgoing: Vec<_> = VertexRingIterator::new(draft.arena(), boundary_vertex).unwrap()
            .map(|r| r.unwrap()).collect();

        let has_boundary_edge = total_outgoing.iter().any(|&he| {
            let data = draft.arena().get_half_edge(he).unwrap();
            data.radial_next() == he
        });
        assert!(has_boundary_edge,
            "Test setup: boundary vertex v0 must have at least one self-radial (boundary) edge");

        let total_in_orbits: usize = orbits.iter().map(|o| o.len()).sum();
        assert_eq!(
            total_in_orbits, total_outgoing.len(),
            "All outgoing half-edges must be accounted for in orbits"
        );

        assert_eq!(
            orbits.len(), 1,
            "Boundary vertex on a connected open mesh must have exactly 1 orbit, got {}",
            orbits.len()
        );
    }

    /// Adversarial: Simulate boolean assembly — two independent bodies
    /// sharing a vertex via raw origin mutation.
    ///
    /// Build two separate MVF + 2×SE faces. Patch all half-edges of the
    /// second face to originate from a vertex of the first face, creating
    /// a non-manifold pinch point. The orbit decomposition must detect
    /// exactly 2 disjoint neighborhoods.
    #[test]
    fn orbits_adversarial_disjoint_bodies_shared_vertex() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf1 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1a = apply_op(&mut draft, SplitEdge { edge: mvf1.half_edge, parameter: 0.3 }).unwrap().into_value();
        let _se1b = apply_op(&mut draft, SplitEdge { edge: se1a.he_mb, parameter: 0.6 }).unwrap().into_value();

        let mvf2 = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se2a = apply_op(&mut draft, SplitEdge { edge: mvf2.half_edge, parameter: 0.4 }).unwrap().into_value();
        let _se2b = apply_op(&mut draft, SplitEdge { edge: se2a.he_mb, parameter: 0.7 }).unwrap().into_value();

        let shared_vertex = mvf1.vertex;
        let victim_vertex = mvf2.vertex;

        let to_patch: Vec<HalfEdgeId> = draft.arena().iter_half_edges()
            .filter(|(_, he_data)| he_data.origin() == victim_vertex)
            .map(|(he_id, _)| he_id)
            .collect();

        for he_id in to_patch {
            draft.arena_mut().get_half_edge_mut(he_id).unwrap().set_origin(shared_vertex);
        }

        let orbits = vertex_neighborhood_orbits(draft.arena(), shared_vertex).unwrap();
        assert_eq!(
            orbits.len(), 2,
            "Two disjoint bodies sharing a vertex must produce 2 orbits, got {}",
            orbits.len()
        );

        for (i, orbit) in orbits.iter().enumerate() {
            assert!(
                !orbit.is_empty(),
                "Orbit {} must not be empty", i
            );
        }

        let total_outgoing: Vec<_> = VertexRingIterator::new(draft.arena(), shared_vertex).unwrap()
            .map(|r| r.unwrap()).collect();
        let total_in_orbits: usize = orbits.iter().map(|o| o.len()).sum();
        assert_eq!(
            total_in_orbits, total_outgoing.len(),
            "Every outgoing half-edge must be assigned to exactly one orbit"
        );
    }

    /// Adversarial: High-valence fan vertex (20 edges from a pole).
    ///
    /// Build a fan of 20 edges radiating from one vertex via SE + MEF.
    /// Despite the extreme valence, the vertex is still manifold — all
    /// half-edges must land in a single orbit.
    #[test]
    fn orbits_adversarial_20_edge_fan_single_orbit() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let pole = mvf.vertex;
        let mut current_edge = mvf.half_edge;

        for _ in 0..20 {
            let se = apply_op(&mut draft, SplitEdge {
                edge: current_edge, parameter: 0.5,
            }).unwrap().into_value();

            let face_id = draft.arena().get_half_edge(current_edge).unwrap().face();
            let mef = apply_op(&mut draft, MakeEdgeFace {
                vertex_a: pole,
                vertex_b: se.new_vertex,
                face: face_id,
            }).unwrap().into_value();

            current_edge = mef.half_edge_ab;
        }

        let orbits = vertex_neighborhood_orbits(draft.arena(), pole).unwrap();
        assert_eq!(
            orbits.len(), 1,
            "High-valence manifold fan must still be 1 orbit, got {}",
            orbits.len()
        );

        let total: Vec<_> = VertexRingIterator::new(draft.arena(), pole).unwrap()
            .map(|r| r.unwrap()).collect();
        assert_eq!(
            orbits[0].len(), total.len(),
            "Single orbit must contain all {} outgoing half-edges",
            total.len()
        );
    }

    /// Adversarial: KEV collapses an edge, then orbit query on the survivor.
    ///
    /// Build quad, MEF diagonal, KEV to collapse the diagonal. The
    /// surviving vertex absorbs edges from the killed vertex. The orbit
    /// decomposition must still correctly identify the neighborhood
    /// structure after this violent topological surgery.
    #[test]
    fn orbits_adversarial_post_kev_collapse() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.25 }).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();
        let _se3 = apply_op(&mut draft, SplitEdge { edge: se2.he_mb, parameter: 0.75 }).unwrap().into_value();

        let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap()).collect();
        let v1 = draft.arena().get_half_edge(edges[1]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(edges[3]).unwrap().origin();

        let mef = apply_op(&mut draft, MakeEdgeFace {
            face: mvf.face, vertex_a: v1, vertex_b: v3,
        }).unwrap().into_value();

        let kev = apply_op(&mut draft, KillEdgeVertex {
            edge: mef.half_edge_ab,
        }).unwrap().into_value();

        let orbits = vertex_neighborhood_orbits(draft.arena(), kev.surviving_vertex).unwrap();

        let total: Vec<_> = VertexRingIterator::new(draft.arena(), kev.surviving_vertex).unwrap()
            .map(|r| r.unwrap()).collect();
        let total_in_orbits: usize = orbits.iter().map(|o| o.len()).sum();
        assert_eq!(
            total_in_orbits, total.len(),
            "Post-KEV: all {} outgoing half-edges must be in orbits, found {} in orbits",
            total.len(), total_in_orbits
        );

        assert!(
            !orbits.is_empty(),
            "Surviving vertex must have at least one orbit"
        );
    }
}
