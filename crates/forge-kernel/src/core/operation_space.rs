//! Local coordinate transform lifecycle for multi-solid operations.
//!
//! DOMAIN: Manages the scale analysis and coordinate transforms that P2.4
//! mandates for operations at extreme scales.
//!
//! DEPENDENCIES: `forge-geom` (LocalCoordinateSpace, ScaleAnalysis),
//! `forge-topo` (TopologyState), GeometryStore.
//!
//! INVARIANTS: `transform_geometry` and `restore_geometry` are inverse
//! operations — calling both is a no-op within floating-point ULP.

use forge_geom::spatial::local_space::{LocalCoordinateSpace, ScaleAnalysis};
use forge_topo::state::TopologyState;

use crate::geometry_store::GeometryStore;

/// Local coordinate transform lifecycle for multi-solid operations.
///
/// Holds the `LocalCoordinateSpace` and tracks whether a transform is active.
/// Zero cost when the scale is safe (identity transform, `active = false`).
pub struct OperationSpace {
    local_space: LocalCoordinateSpace,
    active: bool,
    condition_number: f64,
}

impl OperationSpace {
    /// Analyze two solids and compute a local coordinate space if needed.
    pub fn analyze_binary(
        target_topo: &TopologyState,
        target_geom: &GeometryStore,
        tool_topo: &TopologyState,
        tool_geom: &GeometryStore,
        feature_tolerance: f64,
    ) -> Self {
        let mut points = collect_vertex_positions(target_topo, target_geom);
        points.extend(collect_vertex_positions(tool_topo, tool_geom));
        Self::from_points(&points, feature_tolerance)
    }

    /// Analyze a single solid (for unary operations like fillet/extrude).
    pub fn analyze_unary(
        topo: &TopologyState,
        geom: &GeometryStore,
        feature_tolerance: f64,
    ) -> Self {
        let points = collect_vertex_positions(topo, geom);
        Self::from_points(&points, feature_tolerance)
    }

    /// Transform a geometry store to local coordinates (call before pipeline).
    pub fn transform_geometry(&self, geom: &mut GeometryStore) {
        if self.active {
            geom.transform(&self.local_space);
        }
    }

    /// Transform a geometry store from local back to world (call after pipeline).
    pub fn restore_geometry(&self, geom: &mut GeometryStore) {
        if self.active {
            geom.inverse_transform(&self.local_space);
        }
    }

    /// Whether a local transform is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// The condition number of the input geometry.
    pub fn get_condition_number(&self) -> f64 {
        self.condition_number
    }

    /// The underlying local coordinate space.
    pub fn get_local_space(&self) -> &LocalCoordinateSpace {
        &self.local_space
    }

    /// Compute the operation space from a point cloud.
    fn from_points(points: &[[f64; 3]], feature_tolerance: f64) -> Self {
        let analysis = ScaleAnalysis::compute(points, feature_tolerance);
        let active = analysis.get_needs_local_transform();
        let condition_number = analysis.get_condition_number();

        let local_space = if active {
            LocalCoordinateSpace::from_points(points)
        } else {
            LocalCoordinateSpace::identity()
        };

        Self { local_space, active, condition_number }
    }
}

/// Collect all f64 vertex positions from a topology+geometry pair.
fn collect_vertex_positions(topo: &TopologyState, geom: &GeometryStore) -> Vec<[f64; 3]> {
    let mut points = Vec::new();
    for (v_id, _) in topo.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(v_id) {
            points.push(*pos);
        }
    }
    points
}
