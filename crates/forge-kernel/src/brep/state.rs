use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::geometry_state::schema::pack_handle;
use forge_core::KernelError;
use crate::geom::{Coedge, CurveGeom, SurfaceData};
use forge_topo::handles::{CoedgeRef, CurveRef, EdgeId, FaceId, HalfEdgeId, SurfaceRef};

/// A generational slot in the B-Rep arenas.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct BrepSlot<T> {
    data: Option<T>,
    generation: u32,
}

impl<T> BrepSlot<T> {
    fn vacant(gen: u32) -> Self {
        Self {
            data: None,
            generation: gen,
        }
    }

    fn occupied(data: T, gen: u32) -> Self {
        Self {
            data: Some(data),
            generation: gen,
        }
    }
}

/// B-Rep data storage mapping topology handles to parametric curves and surfaces.
///
/// This store holds the advanced parametric representation data: NURBS, Sweeps,
/// analytic curves and surfaces, and their bindings to the pure topology graph.
/// Separating this from `GeometryState` keeps the planar foundation clean and fast.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrepState {
    /// Parametric surface definitions (indexed by `SurfaceRef`).
    surfaces: Vec<BrepSlot<SurfaceData>>,
    /// 3D edge curve geometries (indexed by `CurveRef`).
    curves: Vec<BrepSlot<CurveGeom>>,
    /// UV trim curves / coedges (indexed by `CoedgeRef`).
    coedges: Vec<BrepSlot<Coedge>>,

    /// Map from face handle → `SurfaceRef` for faces with attached surfaces.
    pub(crate) face_surfaces: HashMap<u64, SurfaceRef>,
    /// Map from halfedge handle → `(CoedgeRef, direction)` for curved halfedges.
    pub(crate) halfedge_coedges: HashMap<u64, (CoedgeRef, bool)>,
    /// Map from edge handle → `CurveRef` for edges with attached curves.
    pub(crate) edge_curves: HashMap<u64, CurveRef>,
}

impl BrepState {
    /// Create an empty B-Rep store.
    pub fn new() -> Self {
        Self {
            surfaces: Vec::new(),
            curves: Vec::new(),
            coedges: Vec::new(),
            face_surfaces: HashMap::new(),
            halfedge_coedges: HashMap::new(),
            edge_curves: HashMap::new(),
        }
    }

    // ── Surface CRUD ────────────────────────────────────────────

    /// Insert a new surface, returning its `SurfaceRef` handle.
    pub fn insert_surface(&mut self, data: SurfaceData) -> SurfaceRef {
        let index = self.surfaces.len() as u32;
        self.surfaces.push(BrepSlot::occupied(data, 0));
        SurfaceRef::from_raw_parts(index, 0)
    }

    /// Retrieve a surface by its handle.
    pub fn get_surface(&self, r: SurfaceRef) -> Result<&SurfaceData, KernelError> {
        let slot =
            self.surfaces
                .get(r.index() as usize)
                .ok_or_else(|| KernelError::InternalError {
                    message: format!("SurfaceRef {} out of range", r),
                    context: None,
                })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale SurfaceRef {} (arena gen {})", r, slot.generation),
                context: None,
            });
        }
        slot.data
            .as_ref()
            .ok_or_else(|| KernelError::InternalError {
                message: format!("SurfaceRef {} points to a vacant slot", r),
                context: None,
            })
    }

    /// Remove a surface, returning its data.
    pub fn remove_surface(&mut self, r: SurfaceRef) -> Result<SurfaceData, KernelError> {
        let slot = self.surfaces.get_mut(r.index() as usize).ok_or_else(|| {
            KernelError::InternalError {
                message: format!("SurfaceRef {} out of range", r),
                context: None,
            }
        })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale SurfaceRef {}", r),
                context: None,
            });
        }
        let data = slot.data.take().ok_or_else(|| KernelError::InternalError {
            message: format!("SurfaceRef {} already vacant", r),
            context: None,
        })?;
        slot.generation += 1;
        Ok(data)
    }

    // ── Curve CRUD ──────────────────────────────────────────────

    /// Insert a new curve geometry, returning its `CurveRef` handle.
    pub fn insert_curve(&mut self, data: CurveGeom) -> CurveRef {
        let index = self.curves.len() as u32;
        self.curves.push(BrepSlot::occupied(data, 0));
        CurveRef::from_raw_parts(index, 0)
    }

    /// Retrieve a curve geometry by its handle.
    pub fn get_curve(&self, r: CurveRef) -> Result<&CurveGeom, KernelError> {
        let slot =
            self.curves
                .get(r.index() as usize)
                .ok_or_else(|| KernelError::InternalError {
                    message: format!("CurveRef {} out of range", r),
                    context: None,
                })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale CurveRef {}", r),
                context: None,
            });
        }
        slot.data
            .as_ref()
            .ok_or_else(|| KernelError::InternalError {
                message: format!("CurveRef {} points to a vacant slot", r),
                context: None,
            })
    }

    /// Remove a curve geometry, returning its data.
    pub fn remove_curve(&mut self, r: CurveRef) -> Result<CurveGeom, KernelError> {
        let slot =
            self.curves
                .get_mut(r.index() as usize)
                .ok_or_else(|| KernelError::InternalError {
                    message: format!("CurveRef {} out of range", r),
                    context: None,
                })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale CurveRef {}", r),
                context: None,
            });
        }
        let data = slot.data.take().ok_or_else(|| KernelError::InternalError {
            message: format!("CurveRef {} already vacant", r),
            context: None,
        })?;
        slot.generation += 1;
        Ok(data)
    }

    // ── Coedge CRUD ─────────────────────────────────────────────

    /// Insert a new coedge (UV trim curve), returning its `CoedgeRef` handle.
    pub fn insert_coedge(&mut self, data: Coedge) -> CoedgeRef {
        let index = self.coedges.len() as u32;
        self.coedges.push(BrepSlot::occupied(data, 0));
        CoedgeRef::from_raw_parts(index, 0)
    }

    /// Retrieve a coedge by its handle.
    pub fn get_coedge(&self, r: CoedgeRef) -> Result<&Coedge, KernelError> {
        let slot =
            self.coedges
                .get(r.index() as usize)
                .ok_or_else(|| KernelError::InternalError {
                    message: format!("CoedgeRef {} out of range", r),
                    context: None,
                })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale CoedgeRef {}", r),
                context: None,
            });
        }
        slot.data
            .as_ref()
            .ok_or_else(|| KernelError::InternalError {
                message: format!("CoedgeRef {} points to a vacant slot", r),
                context: None,
            })
    }

    // ── Attachment APIs ──────────────────────────────────────────

    /// Attach a surface to a face.
    pub fn attach_surface_to_face(&mut self, face: FaceId, surface: SurfaceRef) {
        let key = pack_handle(face.index(), face.generation());
        self.face_surfaces.insert(key, surface);
    }

    /// Attach a coedge + direction to a halfedge.
    pub fn attach_coedge_to_halfedge(
        &mut self,
        he: HalfEdgeId,
        coedge: CoedgeRef,
        direction: bool,
    ) {
        let key = pack_handle(he.index(), he.generation());
        self.halfedge_coedges.insert(key, (coedge, direction));
    }

    /// Attach a curve to an edge.
    pub fn attach_curve_to_edge(&mut self, edge: EdgeId, curve: CurveRef) {
        let key = pack_handle(edge.index(), edge.generation());
        self.edge_curves.insert(key, curve);
    }

    /// Retrieve the surface attached to a face, if any.
    pub fn get_face_surface(&self, face: FaceId) -> Option<SurfaceRef> {
        let key = pack_handle(face.index(), face.generation());
        self.face_surfaces.get(&key).copied()
    }

    /// Retrieve the coedge + direction attached to a halfedge, if any.
    pub fn get_halfedge_coedge(&self, he: HalfEdgeId) -> Option<(CoedgeRef, bool)> {
        let key = pack_handle(he.index(), he.generation());
        self.halfedge_coedges.get(&key).copied()
    }

    /// Retrieve the curve attached to an edge, if any.
    pub fn get_edge_curve(&self, edge: EdgeId) -> Option<CurveRef> {
        let key = pack_handle(edge.index(), edge.generation());
        self.edge_curves.get(&key).copied()
    }

    /// Number of active surfaces.
    pub fn surface_count(&self) -> usize {
        self.surfaces.iter().filter(|s| s.data.is_some()).count()
    }

    /// Number of active curves.
    pub fn curve_count(&self) -> usize {
        self.curves.iter().filter(|s| s.data.is_some()).count()
    }

    /// Number of active coedges.
    pub fn coedge_count(&self) -> usize {
        self.coedges.iter().filter(|s| s.data.is_some()).count()
    }

    /// Internal: commit a raw handle mapping directly.
    pub(crate) fn _set_face_surface_raw(&mut self, packed_key: u64, surface: SurfaceRef) {
        self.face_surfaces.insert(packed_key, surface);
    }

    /// Internal: remove a raw handle mapping directly.
    pub(crate) fn _remove_face_surface_raw(&mut self, packed_key: u64) {
        self.face_surfaces.remove(&packed_key);
    }

    /// Internal: commit a raw handle mapping directly.
    pub(crate) fn _set_halfedge_coedge_raw(
        &mut self,
        packed_key: u64,
        coedge: CoedgeRef,
        direction: bool,
    ) {
        self.halfedge_coedges
            .insert(packed_key, (coedge, direction));
    }

    /// Internal: remove a raw handle mapping directly.
    pub(crate) fn _remove_halfedge_coedge_raw(&mut self, packed_key: u64) {
        self.halfedge_coedges.remove(&packed_key);
    }

    /// Internal: commit a raw handle mapping directly.
    pub(crate) fn _set_edge_curve_raw(&mut self, packed_key: u64, curve: CurveRef) {
        self.edge_curves.insert(packed_key, curve);
    }

    /// Internal: remove a raw handle mapping directly.
    pub(crate) fn _remove_edge_curve_raw(&mut self, packed_key: u64) {
        self.edge_curves.remove(&packed_key);
    }
}

impl Default for BrepState {
    fn default() -> Self {
        Self::new()
    }
}
