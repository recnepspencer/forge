//! Shared test fixtures for Boolean operation tests.
//!
//! Shape construction delegates to `crate::mesh_builder` which is the
//! single source of truth for planes → BSP → halfedge mesh.
//!
//! Traces are persisted automatically by `OperationResult::into_value()`
//! when `FORGE_TRACE_DIR` is set. No manual wiring needed.

use std::str::FromStr;

use forge_geom::Plane;
use forge_topo::state::TopologyState;

use crate::geometry_store::GeometryStore;
use crate::mesh_builder;
use super::eval::compute_face_centroid;
use super::schema::{BooleanInput, BooleanOp, BooleanResult};
use super::assemble::execute_boolean_direct;
use super::execute_boolean;

/// Boolean pipeline selection for tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestPipeline {
    /// Production router (`execute_boolean`) with adaptive EMBER/legacy selection.
    Adaptive,
    /// Legacy/standard planar pipeline (`execute_boolean_direct`).
    Legacy,
    /// Forced EMBER planar engine (`execute_boolean_ember`).
    Ember,
}

impl FromStr for TestPipeline {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "adaptive" | "auto" | "router" => Ok(Self::Adaptive),
            "legacy" | "standard" => Ok(Self::Legacy),
            "ember" => Ok(Self::Ember),
            _ => Err(()),
        }
    }
}

/// Read the test pipeline from `FORGE_TEST_PIPELINE`.
///
/// Accepted values: `adaptive`, `legacy`, `ember`.
/// Defaults to `Legacy` to preserve existing test helper behavior.
pub fn selected_test_pipeline() -> TestPipeline {
    std::env::var("FORGE_TEST_PIPELINE")
        .ok()
        .and_then(|value| TestPipeline::from_str(&value).ok())
        .unwrap_or(TestPipeline::Legacy)
}

/// Build a cube mesh centered at `center` with the given `half_size`.
pub fn build_cube(
    center: [f64; 3],
    half_size: f64,
) -> (TopologyState, GeometryStore) {
    mesh_builder::make_cube(center, half_size * 2.0).unwrap().into_parts()
}

/// Compute face centroid for test assertions (wraps the shared eval function).
pub fn face_centroid(
    arena: &forge_topo::arena::TopologyArena,
    geom: &GeometryStore,
    face: forge_topo::handles::FaceId,
) -> [f64; 3] {
    compute_face_centroid(arena, geom, face).unwrap()
}

/// Execute a boolean and return the result, panicking with context on failure.
///
/// Logs a compact `DecisionLog` to stderr by default.
/// Set `FORGE_LOG=full` to include Euler operator decisions.
/// Traces auto-persist via `OperationResult::into_value()` when
/// `FORGE_TRACE_DIR` is set.
pub fn run_boolean(
    center_a: [f64; 3],
    half_a: f64,
    center_b: [f64; 3],
    half_b: f64,
    op: BooleanOp,
) -> BooleanResult {
    run_boolean_with_pipeline(
        center_a,
        half_a,
        center_b,
        half_b,
        op,
        selected_test_pipeline(),
    )
}

/// Execute a boolean via the selected test pipeline.
pub fn run_boolean_with_pipeline(
    center_a: [f64; 3],
    half_a: f64,
    center_b: [f64; 3],
    half_b: f64,
    op: BooleanOp,
    pipeline: TestPipeline,
) -> BooleanResult {
    let (topo_a, geom_a) = build_cube(center_a, half_a);
    let (topo_b, geom_b) = build_cube(center_b, half_b);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, op);
    let envelope = execute_boolean_logged_with_pipeline(input, pipeline);
    envelope.into_result().unwrap_or_else(|e| {
        panic!("Boolean {:?} failed: {:?}", op, e);
    })
}

/// Attempt a boolean, returning the Result for tests that expect errors.
///
/// Traces and error logging are handled automatically by `into_result()`.
pub fn try_boolean(
    center_a: [f64; 3],
    half_a: f64,
    center_b: [f64; 3],
    half_b: f64,
    op: BooleanOp,
) -> Result<BooleanResult, forge_core::KernelError> {
    try_boolean_with_pipeline(
        center_a,
        half_a,
        center_b,
        half_b,
        op,
        selected_test_pipeline(),
    )
}

/// Attempt a boolean via the selected test pipeline.
pub fn try_boolean_with_pipeline(
    center_a: [f64; 3],
    half_a: f64,
    center_b: [f64; 3],
    half_b: f64,
    op: BooleanOp,
    pipeline: TestPipeline,
) -> Result<BooleanResult, forge_core::KernelError> {
    let (topo_a, geom_a) = build_cube(center_a, half_a);
    let (topo_b, geom_b) = build_cube(center_b, half_b);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, op);
    execute_boolean_logged_with_pipeline(input, pipeline).into_result()
}

/// Execute a boolean from a pre-built `BooleanInput`, returning the full envelope.
///
/// Uses the standard pipeline (no EMBER coplanar resolution).
/// For tests that need EMBER coplanar resolution, use `execute_boolean_ember`.
pub fn execute_boolean_logged(
    input: BooleanInput,
) -> forge_core::OperationResult<Result<BooleanResult, forge_core::KernelError>> {
    execute_boolean_logged_with_pipeline(input, selected_test_pipeline())
}

/// Execute a boolean using a specific test pipeline.
pub fn execute_boolean_logged_with_pipeline(
    input: BooleanInput,
    pipeline: TestPipeline,
) -> forge_core::OperationResult<Result<BooleanResult, forge_core::KernelError>> {
    match pipeline {
        TestPipeline::Adaptive => execute_boolean(input),
        TestPipeline::Legacy => execute_boolean_direct(input),
        TestPipeline::Ember => execute_boolean_ember(input),
    }
}

/// Execute a boolean with the EMBER engine (coplanar resolution enabled).
///
/// Uses `planar_engine()` which includes `EmberCoplanarResolver` for
/// exact coplanar face detection. Use this in MB1/coplanar tests.
pub fn execute_boolean_ember(
    input: BooleanInput,
) -> forge_core::OperationResult<Result<BooleanResult, forge_core::KernelError>> {
    use super::assemble::execute_boolean_with_engine;
    use super::engines::planar::planar_engine;
    execute_boolean_with_engine(input, planar_engine())
}

/// Euler characteristic audit: returns (V, E, F, χ).
///
/// χ = V − E + F. For a single closed manifold shell, χ = 2.
/// Panics with a descriptive message if the arena has no faces.
pub fn euler_audit(arena: &forge_topo::arena::TopologyArena) -> (usize, usize, usize, isize) {
    let v = arena.vertex_count();
    let e = arena.half_edge_count() / 2;
    let f = arena.face_count();
    let chi = v as isize - e as isize + f as isize;
    (v, e, f, chi)
}

/// Build a tetrahedron mesh from 4 planes.
pub fn build_tetrahedron(
    center: [f64; 3],
    scale: f64,
) -> (TopologyState, GeometryStore) {
    mesh_builder::make_tetrahedron(center, scale).unwrap().into_parts()
}

/// Build a convex solid from arbitrary planes.
pub fn build_convex_solid(
    planes: Vec<Plane>,
) -> (TopologyState, GeometryStore) {
    mesh_builder::make_convex_solid(planes).unwrap().into_parts()
}

/// Build a regular dodecahedron (12 pentagonal faces) from 12 planes.
pub fn build_dodecahedron(
    center: [f64; 3],
    scale: f64,
) -> (TopologyState, GeometryStore) {
    mesh_builder::make_dodecahedron(center, scale).unwrap().into_parts()
}

/// Generate Menger sponge subtraction centers for a given level.
///
/// For a cube centered at `center` with half-size `half`, returns
/// the centers and half-sizes of all sub-cubes to subtract at all
/// levels up to `level`.
///
/// Level 1: 7 subtractions. Level 2: 147. Level 3: 2,947.
/// Level 4: ~59,000 (intentionally brutal).
pub fn menger_sponge_subtraction_centers(
    center: [f64; 3],
    half: f64,
    level: u32,
) -> Vec<([f64; 3], f64)> {
    if level == 0 {
        return vec![];
    }

    let sub_half = half / 3.0;
    let step = sub_half * 2.0;
    let mut result = Vec::new();

    let removal_offsets: &[[i32; 3]] = &[
        [0, 0, 0],
        [1, 0, 0], [-1, 0, 0],
        [0, 1, 0], [0, -1, 0],
        [0, 0, 1], [0, 0, -1],
    ];

    for off in removal_offsets {
        let c = [
            center[0] + off[0] as f64 * step,
            center[1] + off[1] as f64 * step,
            center[2] + off[2] as f64 * step,
        ];
        result.push((c, sub_half));
    }

    if level > 1 {
        let keep_offsets: Vec<[i32; 3]> = {
            let mut v = Vec::new();
            for x in -1..=1 {
                for y in -1..=1 {
                    for z in -1..=1 {
                        let zeros = (x == 0) as u8 + (y == 0) as u8 + (z == 0) as u8;
                        if zeros < 2 {
                            v.push([x, y, z]);
                        }
                    }
                }
            }
            v
        };

        for off in &keep_offsets {
            let sub_center = [
                center[0] + off[0] as f64 * step,
                center[1] + off[1] as f64 * step,
                center[2] + off[2] as f64 * step,
            ];
            result.extend(menger_sponge_subtraction_centers(
                sub_center, sub_half, level - 1,
            ));
        }
    }

    result
}
