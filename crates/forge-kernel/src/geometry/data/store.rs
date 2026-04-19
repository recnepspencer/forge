//! Unified geometry store — the resting state of all handle → geometry bindings.
//!
//! DOMAIN: Single source of truth for all geometric properties attached
//! to topology entities. Replaces the old `GeometryState` + `BrepState` split.
//!
//! INVARIANTS:
//! - Every PropertyLayer stores source data only, never derived data.
//! - Adding a new geometric property = adding one field here + one in GeometryDraft.
//! - The store is PURE DATA — all behavior lives in the logic layer.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId, VertexId};
use worth_geom::facade::{Coedge, CurveGeom, Plane, SurfaceData};

use super::layer::PropertyLayer;
use super::position::ExactPosition;

/// Unified geometry store — all handle → geometry bindings in one struct.
///
/// Adding a new geometric property is one line: add a `PropertyLayer<K, V>` field,
/// then add the matching field to `GeometryDraft`.
///
/// **Transient properties** (runtime caches, debug annotations) that should NOT
/// be serialized must be annotated with `#[serde(skip)]`. This works because
/// `PropertyLayer` implements `Default` (returns an empty layer).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeometryStore {
    // ── Phase 1: Planar Foundation ──────────────────────────────────────
    /// Face → plane equation.
    pub planes: PropertyLayer<FaceId, Plane>,
    /// Vertex → exact rational position with f64 cache.
    pub positions: PropertyLayer<VertexId, ExactPosition>,

    // ── Phase 4: Parametric/NURBS ───────────────────────────────────────
    /// Face → parametric surface definition.
    pub surfaces: PropertyLayer<FaceId, Arc<SurfaceData>>,
    /// Edge → 3D curve geometry.
    pub curves: PropertyLayer<EdgeId, Arc<CurveGeom>>,
    /// Half-edge → UV trim curve + orientation.
    pub coedges: PropertyLayer<HalfEdgeId, (Arc<Coedge>, bool)>,
}

impl GeometryStore {
    /// Create an empty geometry store.
    pub fn new() -> Self {
        Self {
            planes: PropertyLayer::new(),
            positions: PropertyLayer::new(),
            surfaces: PropertyLayer::new(),
            curves: PropertyLayer::new(),
            coedges: PropertyLayer::new(),
        }
    }
}

impl Default for GeometryStore {
    fn default() -> Self {
        Self::new()
    }
}
