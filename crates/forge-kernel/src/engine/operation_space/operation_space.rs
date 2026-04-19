//! Local coordinate transform lifecycle for multi-solid operations.
//!
//! DOMAIN: Numerical conditioning infrastructure. Manages the scale
//! analysis and coordinate transforms that P2.4 mandates for operations
//! at extreme scales.
//!
//! DEPENDENCIES: `worth-geom` (LocalCoordinateSpace, ScaleAnalysis),
//! `forge-topo` (TopologyState), GeometryStore, GeometryView.
//!
//! INVARIANTS: `transform_geometry` and `restore_geometry` are inverse
//! operations — calling both is a no-op within floating-point ULP.

use forge_topo::transactions::TopologyState;
use worth_geom::facade::{LocalCoordinateSpace, Plane, ScaleAnalysis};

use crate::engine::output::solid_envelope::SolidEnvelope;
use crate::geometry::facade::{inverse_transform_geometry, transform_geometry};
use crate::geometry::facade::{GeometryStore, GeometryView};

/// Local coordinate transform lifecycle for multi-solid operations.
///
/// Holds the `LocalCoordinateSpace` and tracks whether a transform is active.
/// Zero cost when the scale is safe (identity transform, `active = false`).
#[derive(Debug, Clone)]
pub struct OperationSpace {
    local_space: LocalCoordinateSpace,
    active: bool,
    condition_number: f64,
}

impl OperationSpace {
    /// Identity (no-op) operation space. Used as default when no
    /// scale analysis has been performed.
    pub fn identity() -> Self {
        Self {
            local_space: LocalCoordinateSpace::identity(),
            active: false,
            condition_number: 1.0,
        }
    }

    /// Analyze two solids and compute a local coordinate space if needed.
    pub fn analyze_binary(
        target_topo: &TopologyState,
        target_geom: &impl GeometryView,
        tool_topo: &TopologyState,
        tool_geom: &impl GeometryView,
        feature_tolerance: f64,
    ) -> Self {
        let mut points = collect_vertex_positions(target_topo, target_geom);
        points.extend(collect_vertex_positions(tool_topo, tool_geom));
        Self::from_points(&points, feature_tolerance)
    }

    /// Analyze all input solids and compute a shared local coordinate space.
    ///
    /// Used by the pipeline for `ConditioningMode::BinaryAnalysis` and
    /// `ConditioningMode::UnaryAnalysis`. Collects vertex positions from
    /// all input envelopes and computes a single `OperationSpace`.
    pub fn analyze_envelopes<'a>(
        inputs: impl Iterator<Item = &'a SolidEnvelope>,
        feature_tolerance: f64,
    ) -> Self {
        let mut points = Vec::new();
        for env in inputs {
            points.extend(collect_vertex_positions(env.topology(), env.geometry()));
        }
        Self::from_points(&points, feature_tolerance)
    }

    /// Analyze a single solid (for unary operations like fillet/extrude).
    pub fn analyze_unary(
        topo: &TopologyState,
        geom: &impl GeometryView,
        feature_tolerance: f64,
    ) -> Self {
        let points = collect_vertex_positions(topo, geom);
        Self::from_points(&points, feature_tolerance)
    }

    // =====================================================================
    // Pure coordinate lens (geometry stays immutable in world space)
    // =====================================================================

    /// Transform a point to local coordinates (pure function).
    ///
    /// Steps call this to read world-space geometry in the local frame.
    /// The source geometry is never mutated.
    pub fn to_local(&self, point: [f64; 3]) -> [f64; 3] {
        self.local_space.to_local(point)
    }

    /// Transform a point from local to world coordinates (pure function).
    pub fn to_world(&self, local: [f64; 3]) -> [f64; 3] {
        self.local_space.from_local(local)
    }

    /// Transform a plane to local coordinates using exact arithmetic (pure function).
    pub fn plane_to_local(&self, plane: &Plane) -> Plane {
        self.local_space.transform_plane_exact(plane)
    }

    /// Transform a plane from local to world coordinates using exact arithmetic (pure function).
    pub fn plane_to_world(&self, local_plane: &Plane) -> Plane {
        self.local_space.inverse_transform_plane_exact(local_plane)
    }

    // =====================================================================
    // Mutable geometry transform (used by boolean merge internally)
    // =====================================================================

    /// Transform a geometry store to local coordinates (call before pipeline).
    pub fn transform_store(&self, geom: &mut GeometryStore) {
        if self.active {
            transform_geometry(geom, &self.local_space);
        }
    }

    /// Transform a geometry store from local back to world (call after pipeline).
    pub fn restore_store(&self, geom: &mut GeometryStore) {
        if self.active {
            inverse_transform_geometry(geom, &self.local_space);
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
    pub fn from_points(points: &[[f64; 3]], feature_tolerance: f64) -> Self {
        let analysis = ScaleAnalysis::compute(points, feature_tolerance);
        let active = analysis.get_needs_local_transform();
        let condition_number = analysis.get_condition_number();

        let local_space = if active {
            LocalCoordinateSpace::from_points(points)
        } else {
            LocalCoordinateSpace::identity()
        };

        Self {
            local_space,
            active,
            condition_number,
        }
    }
}

/// Collect all f64 vertex positions from a topology+geometry pair.
fn collect_vertex_positions(topo: &TopologyState, geom: &impl GeometryView) -> Vec<[f64; 3]> {
    let mut points = Vec::new();
    for (v_id, _) in topo.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(v_id) {
            points.push(*pos);
        }
    }
    points
}
