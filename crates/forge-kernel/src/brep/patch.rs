use crate::brep::state::BrepState;
use crate::geometry_state::schema::pack_handle;
use forge_topo::handles::{CoedgeRef, CurveRef, EdgeId, FaceId, HalfEdgeId, SurfaceRef};
use std::collections::{HashMap, HashSet};

/// A transactional overlay for `BrepState`.
///
/// Changes to parametric boundaries (NURBS, analytics, etc) are accumulated
/// here and only committed to the underlying `BrepState` once the topological
/// operation succeeds.
pub struct BrepPatch {
    pub(crate) base: BrepState,

    // Phase 4 Curved Entity Data
    face_surface_inserts: HashMap<u64, SurfaceRef>,
    face_surface_removes: HashSet<u64>,

    halfedge_coedge_inserts: HashMap<u64, (CoedgeRef, bool)>,
    halfedge_coedge_removes: HashSet<u64>,

    edge_curve_inserts: HashMap<u64, CurveRef>,
    edge_curve_removes: HashSet<u64>,
}

impl BrepPatch {
    /// Create a new transaction overlaying the given base state.
    pub fn new(base: BrepState) -> Self {
        Self {
            base,
            face_surface_inserts: HashMap::new(),
            face_surface_removes: HashSet::new(),
            halfedge_coedge_inserts: HashMap::new(),
            halfedge_coedge_removes: HashSet::new(),
            edge_curve_inserts: HashMap::new(),
            edge_curve_removes: HashSet::new(),
        }
    }

    // ── Phase 4 Curved Entity Layer ────────────────────────────

    pub fn get_face_surface(&self, face: FaceId) -> Option<SurfaceRef> {
        let key = pack_handle(face.index(), face.generation());
        if self.face_surface_removes.contains(&key) {
            return None;
        }
        if let Some(s) = self.face_surface_inserts.get(&key) {
            return Some(*s);
        }
        self.base.get_face_surface(face)
    }

    pub fn attach_surface_to_face(&mut self, face: FaceId, surface: SurfaceRef) {
        let key = pack_handle(face.index(), face.generation());
        self.face_surface_removes.remove(&key);
        self.face_surface_inserts.insert(key, surface);
    }

    pub fn remove_face_surface(&mut self, face: FaceId) {
        let key = pack_handle(face.index(), face.generation());
        self.face_surface_inserts.remove(&key);
        self.face_surface_removes.insert(key);
    }

    pub fn get_halfedge_coedge(&self, he: HalfEdgeId) -> Option<(CoedgeRef, bool)> {
        let key = pack_handle(he.index(), he.generation());
        if self.halfedge_coedge_removes.contains(&key) {
            return None;
        }
        if let Some(c) = self.halfedge_coedge_inserts.get(&key) {
            return Some(*c);
        }
        self.base.get_halfedge_coedge(he)
    }

    pub fn attach_coedge_to_halfedge(
        &mut self,
        he: HalfEdgeId,
        coedge: CoedgeRef,
        direction: bool,
    ) {
        let key = pack_handle(he.index(), he.generation());
        self.halfedge_coedge_removes.remove(&key);
        self.halfedge_coedge_inserts
            .insert(key, (coedge, direction));
    }

    pub fn remove_halfedge_coedge(&mut self, he: HalfEdgeId) {
        let key = pack_handle(he.index(), he.generation());
        self.halfedge_coedge_inserts.remove(&key);
        self.halfedge_coedge_removes.insert(key);
    }

    pub fn get_edge_curve(&self, edge: EdgeId) -> Option<CurveRef> {
        let key = pack_handle(edge.index(), edge.generation());
        if self.edge_curve_removes.contains(&key) {
            return None;
        }
        if let Some(c) = self.edge_curve_inserts.get(&key) {
            return Some(*c);
        }
        self.base.get_edge_curve(edge)
    }

    pub fn attach_curve_to_edge(&mut self, edge: EdgeId, curve: CurveRef) {
        let key = pack_handle(edge.index(), edge.generation());
        self.edge_curve_removes.remove(&key);
        self.edge_curve_inserts.insert(key, curve);
    }

    pub fn remove_edge_curve(&mut self, edge: EdgeId) {
        let key = pack_handle(edge.index(), edge.generation());
        self.edge_curve_inserts.remove(&key);
        self.edge_curve_removes.insert(key);
    }

    // ── Commit ──────────────────────────────────────────────────

    /// Apply all accumulated bindings to the base `BrepState`.
    ///
    /// Consumes the patch and returns the updated `BrepState`.
    pub fn commit(mut self) -> BrepState {
        for (packed_key, s_ref) in self.face_surface_inserts {
            self.base._set_face_surface_raw(packed_key, s_ref);
        }
        for packed_key in self.face_surface_removes {
            self.base._remove_face_surface_raw(packed_key);
        }

        for (packed_key, (c_ref, dir)) in self.halfedge_coedge_inserts {
            self.base._set_halfedge_coedge_raw(packed_key, c_ref, dir);
        }
        for packed_key in self.halfedge_coedge_removes {
            self.base._remove_halfedge_coedge_raw(packed_key);
        }

        for (packed_key, c_ref) in self.edge_curve_inserts {
            self.base._set_edge_curve_raw(packed_key, c_ref);
        }
        for packed_key in self.edge_curve_removes {
            self.base._remove_edge_curve_raw(packed_key);
        }

        self.base
    }

    /// Drops any pending mutations and returns the original `BrepState`.
    pub fn rollback(self) -> BrepState {
        self.base
    }
}
