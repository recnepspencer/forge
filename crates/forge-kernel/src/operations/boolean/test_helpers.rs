#![cfg(any())]
//! Shared test fixtures for Boolean operation tests.
//!
//! DOMAIN: Shape construction, pipeline selection, and common assertions
//! for Boolean integration tests. All shape builders delegate to
//! `crate::operations::primitives` as the single source of truth.
//!
//! DEPENDENCIES: `schema` (BooleanInput, BooleanOp), `result` (BooleanResult),
//!               `router` (adaptive dispatch), `parametric` (direct dispatch)

use crate::geom_facade::Plane;
use crate::geometry_state::GeometryState;
use crate::operations::primitives;
use crate::shared_ops::vertex::centroid::compute_face_centroid;
use forge_topo::transactions::TopologyState;

use super::result::BooleanResult;
use super::schema::{BooleanInput, BooleanOp};

/// Boolean pipeline selection for tests.
///
/// Controls which execution path integration tests exercise.
/// Switch `DEFAULT_PIPELINE` or set `FORGE_TEST_PIPELINE` env var.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestPipeline {
    /// Production router (`execute_boolean_adaptive`) — routes to best pipeline.
    Adaptive,
    /// Parametric pipeline directly (`parametric::execute`) — bypasses EMBER.
    Parametric,
    // Future: Ember — when the EMBER pipeline is rebuilt
}

/// Default pipeline for all integration tests.
///
/// Change this constant to switch all tests at once.
pub const DEFAULT_PIPELINE: TestPipeline = TestPipeline::Parametric;

/// Get the active test pipeline (env override or default).
pub fn selected_test_pipeline() -> TestPipeline {
    std::env::var("FORGE_TEST_PIPELINE")
        .ok()
        .and_then(|v| match v.trim().to_ascii_lowercase().as_str() {
            "adaptive" | "auto" | "router" => Some(TestPipeline::Adaptive),
            "parametric" | "standard" => Some(TestPipeline::Parametric),
            _ => None,
        })
        .unwrap_or(DEFAULT_PIPELINE)
}

// ── Shape builders ────────────────────────────────────────────────────────

/// Build a cube mesh centered at `center` with the given `half_size`.
pub fn build_cube(center: [f64; 3], half_size: f64) -> (TopologyState, GeometryState) {
    mesh_builder::make_cube(center, half_size * 2.0)
        .unwrap()
        .into_parts()
}

/// Build a tetrahedron mesh.
pub fn build_tetrahedron(center: [f64; 3], scale: f64) -> (TopologyState, GeometryState) {
    mesh_builder::make_tetrahedron(center, scale)
        .unwrap()
        .into_parts()
}

/// Build a convex solid from arbitrary planes.
pub fn build_convex_solid(planes: Vec<Plane>) -> (TopologyState, GeometryState) {
    mesh_builder::make_convex_solid(planes)
        .unwrap()
        .into_parts()
}

/// Build a regular dodecahedron (12 pentagonal faces).
pub fn build_dodecahedron(center: [f64; 3], scale: f64) -> (TopologyState, GeometryState) {
    mesh_builder::make_dodecahedron(center, scale)
        .unwrap()
        .into_parts()
}

// ── Execution helpers ─────────────────────────────────────────────────────

/// Execute a boolean from two cubes, panicking on failure.
pub fn run_boolean(
    center_a: [f64; 3],
    half_a: f64,
    center_b: [f64; 3],
    half_b: f64,
    op: BooleanOp,
) -> BooleanResult {
    run_boolean_with_pipeline(center_a, half_a, center_b, half_b, op, selected_test_pipeline())
}

/// Execute a boolean from two cubes via a specific pipeline.
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
    let envelope = dispatch(input, pipeline);
    let result = envelope.into_result().unwrap_or_else(|e| {
        panic!("Boolean {:?} failed: {:?}", op, e);
    });
    assert_euler_formula_per_shell(result.topology().arena());
    result
}

/// Attempt a boolean, returning the Result for tests that expect errors.
pub fn try_boolean(
    center_a: [f64; 3],
    half_a: f64,
    center_b: [f64; 3],
    half_b: f64,
    op: BooleanOp,
) -> Result<BooleanResult, forge_core::KernelError> {
    let (topo_a, geom_a) = build_cube(center_a, half_a);
    let (topo_b, geom_b) = build_cube(center_b, half_b);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, op);
    let result = dispatch(input, selected_test_pipeline()).into_result();
    if let Ok(ref res) = result {
        assert_euler_formula_per_shell(res.topology().arena());
    }
    result
}

/// Execute a boolean from a pre-built `BooleanInput`, returning the full envelope.
pub fn execute_boolean_logged(
    input: BooleanInput,
) -> forge_core::OperationResult<Result<BooleanResult, forge_core::KernelError>> {
    dispatch(input, selected_test_pipeline())
}

/// Dispatch to the selected pipeline.
fn dispatch(
    input: BooleanInput,
    pipeline: TestPipeline,
) -> forge_core::OperationResult<Result<BooleanResult, forge_core::KernelError>> {
    match pipeline {
        TestPipeline::Adaptive => super::router::execute_boolean_adaptive(input),
        TestPipeline::Parametric => super::parametric::execute(input),
    }
}

// ── Assertions ────────────────────────────────────────────────────────────

/// Euler characteristic audit: returns (V, E, F, χ).
pub fn euler_audit(arena: &forge_topo::b_rep::TopologyArena) -> (usize, usize, usize, isize) {
    let v = arena.vertex_count();
    let e = arena.half_edge_count() / 2;
    let f = arena.face_count();
    let chi = v as isize - e as isize + f as isize;
    (v, e, f, chi)
}

/// Assert Euler formula holds per shell via forge-topo's structural validator.
pub fn assert_euler_formula_per_shell(arena: &forge_topo::b_rep::TopologyArena) {
    if arena.face_count() == 0 {
        return;
    }
    if let Err(e) =
        forge_topo::validate::validate_topology(arena, forge_topo::validate::ValidationLevel::Full)
    {
        panic!("Euler formula failed: {:?}", e);
    }
}

/// Compute face centroid for test assertions.
pub fn face_centroid(
    arena: &forge_topo::b_rep::TopologyArena,
    geom: &GeometryState,
    face: forge_topo::handles::FaceId,
) -> [f64; 3] {
    compute_face_centroid(arena, geom, face).unwrap()
}
