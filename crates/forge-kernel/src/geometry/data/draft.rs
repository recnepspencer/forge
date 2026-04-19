//! Transactional draft for the geometry store.
//!
//! DOMAIN: Wraps a `GeometryStore` in transactional overlays.
//! Commit finalizes all mutations; drop rolls back.
//! The draft is PURE DATA — all behavior lives in the logic layer.

use std::sync::Arc;

use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId, VertexId};
use worth_geom::facade::{Coedge, CurveGeom, Plane, SurfaceData};

use super::layer::PropertyPatch;
use super::position::ExactPosition;
use super::store::GeometryStore;

/// Transactional geometry draft — one `PropertyPatch` per layer.
///
/// Adding a new geometric property = one field here + one in GeometryStore.
pub struct GeometryDraft {
    pub planes: PropertyPatch<FaceId, Plane>,
    pub positions: PropertyPatch<VertexId, ExactPosition>,
    pub surfaces: PropertyPatch<FaceId, Arc<SurfaceData>>,
    pub curves: PropertyPatch<EdgeId, Arc<CurveGeom>>,
    pub coedges: PropertyPatch<HalfEdgeId, (Arc<Coedge>, bool)>,
}

impl std::fmt::Debug for GeometryDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeometryDraft")
            .field("planes_count", &self.planes.len())
            .field("positions_count", &self.positions.len())
            .field("surfaces_count", &self.surfaces.len())
            .field("curves_count", &self.curves.len())
            .field("coedges_count", &self.coedges.len())
            .finish()
    }
}

impl GeometryDraft {
    /// Create a draft from a resting geometry store.
    pub fn new(store: GeometryStore) -> Self {
        Self {
            planes: PropertyPatch::new(store.planes),
            positions: PropertyPatch::new(store.positions),
            surfaces: PropertyPatch::new(store.surfaces),
            curves: PropertyPatch::new(store.curves),
            coedges: PropertyPatch::new(store.coedges),
        }
    }

    /// Commit all mutations. Returns the updated `GeometryStore`.
    pub fn commit(self) -> GeometryStore {
        GeometryStore {
            planes: self.planes.commit(),
            positions: self.positions.commit(),
            surfaces: self.surfaces.commit(),
            curves: self.curves.commit(),
            coedges: self.coedges.commit(),
        }
    }

    /// Discard all mutations. Returns the original `GeometryStore`.
    pub fn rollback(self) -> GeometryStore {
        GeometryStore {
            planes: self.planes.rollback(),
            positions: self.positions.rollback(),
            surfaces: self.surfaces.rollback(),
            curves: self.curves.rollback(),
            coedges: self.coedges.rollback(),
        }
    }
}
