use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

use forge_core::ToleranceProvider;
use forge_math::{MathError, GeometrySource, PlaneCoefficients};

use crate::geometry_state::{GeometryState, ExactPosition};
use crate::geometry_state::schema::pack_handle;
use forge_topo::handles::{FaceId, VertexId};
use forge_geom::Plane;

/// Transactional mutation handle for geometry.
///
/// Wraps an underlying `GeometryState` snapshot and tracks pending diffs
/// for faces and vertices. Used inside `KernelDraft` to enforce that 
/// opportunistic geometry mutations in failing topological phases do not
/// corrupt the canonical geometry store.
#[derive(Debug)]
pub struct GeometryPatch {
    /// The immutable baseline state we are patching on top of.
    pub(crate) base: GeometryState,
    
    // -- Face Data --
    face_plane_inserts: HashMap<u64, Plane>,
    face_plane_removes: HashSet<u64>,
    
    // -- Vertex Data --
    vertex_position_inserts: HashMap<u64, ExactPosition>,
    vertex_position_removes: HashSet<u64>,
    
    vertex_tolerance_inserts: HashMap<u64, f64>,
    vertex_tolerance_removes: HashSet<u64>,

    // Note: Future NURBS layers (surfaces, curves, coedges) will 
    // add their own diff maps here tracking index generations.
}

impl GeometryPatch {
    /// Create a new patch over an existing geometry state snapshot.
    pub fn new(base: GeometryState) -> Self {
        Self {
            base,
            face_plane_inserts: HashMap::new(),
            face_plane_removes: HashSet::new(),
            vertex_position_inserts: HashMap::new(),
            vertex_position_removes: HashSet::new(),
            vertex_tolerance_inserts: HashMap::new(),
            vertex_tolerance_removes: HashSet::new(),
        }
    }

    // ─── Base Access ────────────────────────────────────────────

    /// Access the underlying immutable geometry state.
    pub fn base(&self) -> &GeometryState {
        &self.base
    }

    // ─── Face Plane Layer ───────────────────────────────────────
    
    pub fn get_face_plane(&self, face: FaceId) -> Option<&Plane> {
        let key = pack_handle(face.index(), face.generation());
        if self.face_plane_removes.contains(&key) {
            return None;
        }
        if let Some(p) = self.face_plane_inserts.get(&key) {
            return Some(p);
        }
        self.base.get_face_plane(face)
    }

    pub fn set_face_plane(&mut self, face: FaceId, plane: Plane) {
        let key = pack_handle(face.index(), face.generation());
        self.face_plane_removes.remove(&key);
        self.face_plane_inserts.insert(key, plane);
    }

    pub fn remove_face_plane(&mut self, face: FaceId) {
        let key = pack_handle(face.index(), face.generation());
        self.face_plane_inserts.remove(&key);
        self.face_plane_removes.insert(key);
    }

    // ─── Vertex Position Layer ──────────────────────────────────
    
    pub fn get_vertex_position(&self, vertex: VertexId) -> Option<&[f64; 3]> {
        let key = pack_handle(vertex.index(), vertex.generation());
        if self.vertex_position_removes.contains(&key) {
            return None;
        }
        if let Some(pos) = self.vertex_position_inserts.get(&key) {
            return Some(pos.approx());
        }
        self.base.get_vertex_position(vertex)
    }

    pub fn set_vertex_position(&mut self, vertex: VertexId, pos: ExactPosition) {
        let key = pack_handle(vertex.index(), vertex.generation());
        self.vertex_position_removes.remove(&key);
        self.vertex_position_inserts.insert(key, pos);
    }

    pub fn remove_vertex_position(&mut self, vertex: VertexId) {
        let key = pack_handle(vertex.index(), vertex.generation());
        self.vertex_position_inserts.remove(&key);
        self.vertex_position_removes.insert(key);
    }

    // ─── Vertex Tolerance Layer ─────────────────────────────────

    pub fn get_vertex_tolerance(&self, vertex: VertexId) -> Option<f64> {
        let key = pack_handle(vertex.index(), vertex.generation());
        if self.vertex_tolerance_removes.contains(&key) {
            return None;
        }
        if let Some(tol) = self.vertex_tolerance_inserts.get(&key) {
            return Some(*tol);
        }
        Some(self.base.vertex_tolerance(vertex.index(), vertex.generation()))
    }

    pub fn set_vertex_tolerance(&mut self, vertex: VertexId, tolerance: f64) {
        let key = pack_handle(vertex.index(), vertex.generation());
        self.vertex_tolerance_removes.remove(&key);
        self.vertex_tolerance_inserts.insert(key, tolerance);
    }

    // ─── Lifecycle ──────────────────────────────────────────────
    
    /// Commits all pending mutations to the base `GeometryState`.
    ///
    /// Consumes the patch and applies every diff in-place because we own the base.
    pub fn commit(mut self) -> GeometryState {
        
        for (f_idx, plane) in self.face_plane_inserts {
            self.base._set_face_plane_raw(f_idx, plane);
        }
        for f_idx in self.face_plane_removes {
            self.base._remove_face_plane_raw(f_idx);
        }

        for (v_idx, pos) in self.vertex_position_inserts {
            self.base._set_vertex_position_raw(v_idx, pos);
        }
        for v_idx in self.vertex_position_removes {
            self.base._remove_vertex_position_raw(v_idx);
        }

        for (v_packed, tol) in self.vertex_tolerance_inserts {
            self.base._set_vertex_tolerance_raw(v_packed, tol);
        }
        for v_packed in self.vertex_tolerance_removes {
            self.base._remove_vertex_tolerance_raw(v_packed);
        }

        self.base
    }

    /// Drops any pending mutations and returns the original `GeometryState`.
    pub fn rollback(self) -> GeometryState {
        self.base
    }
}

// ─── Trait Implementations ──────────────────────────────────────

impl ToleranceProvider for GeometryPatch {
    fn vertex_tolerance(&self, vertex_index: u32, vertex_generation: u32) -> f64 {
        let key = pack_handle(vertex_index, vertex_generation);
        if self.vertex_tolerance_removes.contains(&key) {
            return self.base.global_default();
        }
        if let Some(tol) = self.vertex_tolerance_inserts.get(&key) {
            return *tol;
        }
        self.base.vertex_tolerance(vertex_index, vertex_generation)
    }

    fn edge_tolerance(&self, edge_index: u32, edge_generation: u32) -> f64 {
        self.base.edge_tolerance(edge_index, edge_generation)
    }

    fn global_default(&self) -> f64 {
        self.base.global_default()
    }
}

impl GeometrySource for GeometryPatch {
    /// Find a plane by face index scanning packed keys.
    ///
    /// The `GeometrySource` trait passes only the bare face index because
    /// lower layers do not know about generations. This means the lookup is
    /// inherently generation-agnostic: we scan all packed keys and match on
    /// the low-32-bit index. To guarantee determinism we treat multiple live
    /// entries whose index part matches as an ambiguity error — that state
    /// should never occur in a well-formed `GeometryPatch`.
    fn get_plane(&self, index: usize) -> Result<PlaneCoefficients, MathError> {
        let mut found: Option<PlaneCoefficients> = None;

        for (&key, plane) in &self.face_plane_inserts {
            let stored_index = (key & 0xFFFF_FFFF) as usize;
            if stored_index == index {
                let n = plane.normal();
                let coeff = PlaneCoefficients::try_new(n[0], n[1], n[2], plane.offset())?;
                if found.is_some() {
                    return Err(MathError::InvalidInput(format!(
                        "Ambiguous plane lookup: multiple generations live for face index {}",
                        index
                    )));
                }
                found = Some(coeff);
            }
        }
        if let Some(coeff) = found {
            return Ok(coeff);
        }

        for (&key, plane) in &self.base.face_planes {
            if self.face_plane_removes.contains(&key) {
                continue;
            }
            let stored_index = (key & 0xFFFF_FFFF) as usize;
            if stored_index == index {
                let n = plane.normal();
                return PlaneCoefficients::try_new(n[0], n[1], n[2], plane.offset());
            }
        }
        Err(MathError::InvalidInput(
            format!("No plane found for face index {}", index),
        ))
    }
}
