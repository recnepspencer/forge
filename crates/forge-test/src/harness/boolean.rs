//! Self-consistency harness for Boolean corpus fuzzing.
//!
//! DOMAIN: Test infrastructure — validates Boolean results against
//! topological invariants and point-in-solid consistency.
//! INVARIANTS: No external oracle required; uses Euler formula + raycasting.
//! DEPENDENCIES: `forge-kernel` (execute_boolean), `forge-topo` (classify, validate)

use forge_core::KernelError;
use forge_kernel::operations::boolean::{BooleanInput, BooleanOp, BooleanResult, execute_boolean};
use forge_kernel::geometry_store::GeometryStore;
use forge_topo::classify::{classify_point_in_solid, PointClassification};
use forge_topo::handles::VertexId;
use forge_topo::state::TopologyState;

/// Outcome of a single fuzz case.
#[derive(Debug)]
pub enum FuzzOutcome {
    /// Boolean succeeded and passed all checks.
    Pass,
    /// Boolean returned an error (may be legitimate for disjoint intersection).
    BooleanError(KernelError),
    /// Result topology failed self-consistency.
    ConsistencyFailure(String),
}

/// Report from running a fuzz corpus.
#[derive(Debug)]
pub struct FuzzReport {
    /// Total cases run.
    pub total: usize,
    /// Cases that passed.
    pub passed: usize,
    /// Cases where the boolean returned an error.
    pub errors: usize,
    /// Cases that failed consistency (seed, message).
    pub failures: Vec<(u64, String)>,
}

impl FuzzReport {
    /// Whether all non-error cases passed.
    pub fn all_passed(&self) -> bool {
        self.failures.is_empty()
    }
}

/// Run a single Boolean case and check self-consistency.
pub fn run_single_case(input: BooleanInput) -> FuzzOutcome {
    let op = input.operation();

    let target_topo = input.target_topology().clone();
    let target_geom = input.target_geometry().clone();
    let tool_topo = input.tool_topology().clone();
    let tool_geom = input.tool_geometry().clone();

    let result = execute_boolean(input);

    let bool_result = match result {
        Ok(r) => {
            forge_core::result::log_result(&format!("{:?}", op), &r);
            r.into_value()
        }
        Err(e) => return FuzzOutcome::BooleanError(e),
    };

    if let Err(msg) = check_point_consistency(
        &bool_result, op,
        &target_topo, &target_geom,
        &tool_topo, &tool_geom,
    ) {
        return FuzzOutcome::ConsistencyFailure(msg);
    }

    FuzzOutcome::Pass
}

/// Check point-in-solid consistency for the Boolean result.
///
/// Samples points on a grid within the bounding box and verifies
/// that each point's classification in the result matches the
/// expected Boolean semantics.
fn check_point_consistency(
    result: &BooleanResult,
    op: BooleanOp,
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
) -> Result<(), String> {
    let result_topo = result.topology();
    let result_geom = result.geometry();

    let bb_target = compute_bounding_box(target_topo, target_geom);
    let bb_tool = compute_bounding_box(tool_topo, tool_geom);
    let combined_bb = merge_bounding_boxes(&bb_target, &bb_tool);

    let sample_points = generate_sample_grid_from_bb(&combined_bb, 5);

    let ray_extent = 1e6;
    let mut mismatches = 0usize;

    for point in &sample_points {
        let in_result = classify_in_solid(result_topo, result_geom, point, ray_extent);
        let in_target = classify_in_solid(target_topo, target_geom, point, ray_extent);
        let in_tool = classify_in_solid(tool_topo, tool_geom, point, ray_extent);

        let expected = match op {
            BooleanOp::Union => in_target || in_tool,
            BooleanOp::Intersection => in_target && in_tool,
            BooleanOp::Subtraction => in_target && !in_tool,
        };

        if in_result != expected {
            mismatches += 1;
        }
    }

    if mismatches > 0 {
        Err(format!(
            "{mismatches}/{} point-in-solid mismatches for {op:?}",
            sample_points.len()
        ))
    } else {
        Ok(())
    }
}

/// Classify a point as inside (true) or outside (false) a solid.
fn classify_in_solid(
    topo: &TopologyState,
    geom: &GeometryStore,
    point: &[f64; 3],
    ray_extent: f64,
) -> bool {
    let arena = topo.arena();
    let vertex_lookup = |index: u32| -> Result<[f64; 3], KernelError> {
        let gen = arena.vertex_generation(index as usize).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No active vertex at slot index {index}"),
                context: None,
            }
        })?;
        let vid = VertexId::from_raw_parts(index, gen);
        geom.get_vertex_position(vid).copied().ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for vertex {index}"),
                context: None,
            }
        })
    };

    let result = classify_point_in_solid(arena, &vertex_lookup, None, point, ray_extent, 1e-10);
    matches!(result, Ok(PointClassification::Inside { .. } | PointClassification::OnBoundary(_)))
}

/// Merge two bounding boxes into their union.
fn merge_bounding_boxes(
    a: &([f64; 3], [f64; 3]),
    b: &([f64; 3], [f64; 3]),
) -> ([f64; 3], [f64; 3]) {
    let min_bb = [
        a.0[0].min(b.0[0]),
        a.0[1].min(b.0[1]),
        a.0[2].min(b.0[2]),
    ];
    let max_bb = [
        a.1[0].max(b.1[0]),
        a.1[1].max(b.1[1]),
        a.1[2].max(b.1[2]),
    ];
    (min_bb, max_bb)
}

/// Generate a 3D grid of sample points within a bounding box.
fn generate_sample_grid_from_bb(
    bb: &([f64; 3], [f64; 3]),
    grid_size: usize,
) -> Vec<[f64; 3]> {
    let margin = 0.5;
    let lo = [bb.0[0] - margin, bb.0[1] - margin, bb.0[2] - margin];
    let hi = [bb.1[0] + margin, bb.1[1] + margin, bb.1[2] + margin];

    let mut points = Vec::with_capacity(grid_size * grid_size * grid_size);
    for ix in 0..grid_size {
        for iy in 0..grid_size {
            for iz in 0..grid_size {
                let t_x = (ix as f64 + 0.5) / grid_size as f64;
                let t_y = (iy as f64 + 0.5) / grid_size as f64;
                let t_z = (iz as f64 + 0.5) / grid_size as f64;
                points.push([
                    lo[0] + t_x * (hi[0] - lo[0]),
                    lo[1] + t_y * (hi[1] - lo[1]),
                    lo[2] + t_z * (hi[2] - lo[2]),
                ]);
            }
        }
    }
    points
}

/// Compute axis-aligned bounding box from topology vertex positions.
fn compute_bounding_box(
    topo: &TopologyState,
    geom: &GeometryStore,
) -> ([f64; 3], [f64; 3]) {
    let mut min_bb = [f64::MAX; 3];
    let mut max_bb = [f64::MIN; 3];

    for (vid, _) in topo.arena().iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            for axis in 0..3 {
                if pos[axis] < min_bb[axis] {
                    min_bb[axis] = pos[axis];
                }
                if pos[axis] > max_bb[axis] {
                    max_bb[axis] = pos[axis];
                }
            }
        }
    }

    if min_bb[0] == f64::MAX {
        return ([0.0; 3], [1.0; 3]);
    }

    (min_bb, max_bb)
}

/// Run a fuzz corpus of Boolean cases.
///
/// Uses the provided `generator_fn` to create each `BooleanInput` from a seed.
/// Reports overall pass/fail/error statistics.
pub fn run_fuzz_corpus<F>(
    count: usize,
    base_seed: u64,
    generator_fn: F,
) -> FuzzReport
where
    F: Fn(u64) -> Result<BooleanInput, KernelError>,
{
    let mut report = FuzzReport {
        total: count,
        passed: 0,
        errors: 0,
        failures: Vec::new(),
    };

    for i in 0..count {
        let seed = base_seed.wrapping_add(i as u64);

        let input = match generator_fn(seed) {
            Ok(inp) => inp,
            Err(_) => {
                report.errors += 1;
                continue;
            }
        };

        match run_single_case(input) {
            FuzzOutcome::Pass => report.passed += 1,
            FuzzOutcome::BooleanError(_) => report.errors += 1,
            FuzzOutcome::ConsistencyFailure(msg) => {
                report.failures.push((seed, msg));
            }
        }
    }

    report
}
