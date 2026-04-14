//! Geometry transforms — coordinate space conversions.
//!
//! DOMAIN: Transforms geometric data between world and local coordinate
//! spaces using exact rational arithmetic. Decoupled from GeometryStore.
//!
//! DEPENDENCIES: `worth-geom` (Plane, LocalCoordinateSpace)
//!
//! Phase 1 only: Only planes and positions are transformed. Parametric
//! surfaces, curves, and coedges require surface-specific transform logic
//! that will be added in Phase 4+.

use crate::geometry::data::store::GeometryStore;
use worth_geom::facade::LocalCoordinateSpace;

/// Transform all planes and positions to local coordinates.
///
/// **Scope (Phase 1)**: Only `planes` and `positions` are transformed.
/// Surfaces, curves, and coedges are not yet supported.
pub fn transform_geometry(store: &mut GeometryStore, space: &LocalCoordinateSpace) {
    for plane in store.planes.values_mut() {
        *plane = space.transform_plane_exact(plane);
    }
    for ep in store.positions.values_mut() {
        ep.transform_in_place(space);
    }
}

/// Transform all planes and positions from local back to world coordinates.
///
/// **Scope (Phase 1)**: See `transform_geometry()` for scope limitations.
pub fn inverse_transform_geometry(store: &mut GeometryStore, space: &LocalCoordinateSpace) {
    for plane in store.planes.values_mut() {
        *plane = space.inverse_transform_plane_exact(plane);
    }
    for ep in store.positions.values_mut() {
        ep.inverse_transform_in_place(space);
    }
}
