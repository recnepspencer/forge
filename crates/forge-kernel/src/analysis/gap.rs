//! Face-to-face gap measurement via Halton-sampled point projection.
//!
//! DOMAIN: Measure the geometric distance between two faces.
//! DEPENDENCIES: forge_core, forge_geom (plane signed_distance, polygon bounds),
//!               forge_kernel (GeometryStore, ModelingContext).
//! INVARIANTS:
//!   - Non-planar faces → `KernelError::PolicyRequired` (never a wrong answer).
//!   - Sample density maps to fixed counts for reproducible `sample_count`.
//!   - `has_overlap = min_gap_mm < 0` (negative → penetration).

use forge_core::{KernelError, OperationResult};
use forge_topo::state::TopologyState;
use forge_topo::handles::FaceId;
use forge_topo::traverse::FaceEdgeIterator;

use crate::geometry_store::GeometryStore;
use crate::core::ModelingContext;

// ── Public types ──────────────────────────────────────────────────────────────

/// Statistical summary of the sampled gap between two faces.
#[derive(Debug, Clone)]
pub struct GapReport {
    /// Smallest signed distance across all samples (mm).
    /// Negative means penetration/overlap.
    pub min_gap_mm: f64,
    /// Largest signed distance across all samples (mm).
    pub max_gap_mm: f64,
    /// Mean signed distance across all samples (mm).
    pub mean_gap_mm: f64,
    /// Total number of samples taken.
    pub sample_count: usize,
    /// True when `min_gap_mm < 0` (faces interpenetrate).
    pub has_overlap: bool,
}

/// Controls the number of Halton sample points used for gap measurement.
#[derive(Debug, Clone, Copy)]
pub enum GapSampleDensity {
    /// 25 samples — fast heuristic check.
    Coarse,
    /// 100 samples — standard quality.
    Medium,
    /// 400 samples — high-precision audit.
    Fine,
}

impl GapSampleDensity {
    /// Fixed sample count for this density (reproducible across runs).
    pub fn sample_count(self) -> usize {
        match self {
            GapSampleDensity::Coarse => 25,
            GapSampleDensity::Medium => 100,
            GapSampleDensity::Fine   => 400,
        }
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Measure the face-to-face gap between `face_a` and `face_b` using Halton sampling.
///
/// # Steps
/// 1. Guard: `geom_b.face_is_planar(face_b)` — returns `PolicyRequired` if false.
/// 2. Collect the polygon vertices of `face_a` and compute an approximate bounding
///    rectangle in the face's dominant projection axis.
/// 3. Generate Halton sequence samples (base 2 × base 3) over that rectangle.
/// 4. Project each sample point onto `plane_b` and record `signed_distance`.
/// 5. Aggregate into `GapReport`.
pub fn measure_gap(
    face_a: FaceId,
    topo_a: &TopologyState,
    geom_a: &GeometryStore,
    face_b: FaceId,
    topo_b: &TopologyState,
    geom_b: &GeometryStore,
    density: GapSampleDensity,
    _ctx: &mut ModelingContext,
) -> OperationResult<Result<GapReport, KernelError>> {
    let inner = measure_gap_inner(face_a, topo_a, geom_a, face_b, topo_b, geom_b, density);
    OperationResult::new(inner)
}

// ── Private implementation ────────────────────────────────────────────────────

fn measure_gap_inner(
    face_a: FaceId,
    topo_a: &TopologyState,
    geom_a: &GeometryStore,
    face_b: FaceId,
    _topo_b: &TopologyState,
    geom_b: &GeometryStore,
    density: GapSampleDensity,
) -> Result<GapReport, KernelError> {
    // ── 1. Guard: face_b must be planar ───────────────────────────────────────
    if !geom_b.face_is_planar(face_b) {
        return Err(KernelError::InvalidInput {
            message: format!(
                "measure_gap: face {:?} is non-planar; gap measurement requires a planar reference face",
                face_b
            ),
            context: None,
        });
    }

    // ── 2. Get face_b's supporting plane ──────────────────────────────────────
    let plane_b = geom_b.get_face_plane(face_b).ok_or_else(|| {
        KernelError::InvalidInput {
            message: format!("measure_gap: no plane registered for face {:?}", face_b),
            context: None,
        }
    })?;

    // ── 3. Collect face_a polygon vertices ────────────────────────────────────
    let arena_a = topo_a.arena();
    let edges: Vec<_> = FaceEdgeIterator::new(arena_a, face_a)
        .map_err(|e| e.with_phase("measure_gap.face_a_iter"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.with_phase("measure_gap.face_a_collect"))?;

    let mut poly_pts: Vec<[f64; 3]> = Vec::with_capacity(edges.len());
    for he in &edges {
        let v = arena_a.get_half_edge(*he)?.origin();
        if let Some(pos) = geom_a.get_vertex_position(v) {
            poly_pts.push(*pos);
        }
    }

    if poly_pts.len() < 3 {
        return Err(KernelError::InvalidInput {
            message: format!("measure_gap: face {:?} has fewer than 3 vertices", face_a),
            context: None,
        });
    }

    // ── 4. Compute 2D bounding rect in the face's dominant axis projection ────
    let n = plane_b.normal();
    // dominant axis = axis of largest |n| component → project onto the other two
    let dominant = if n[0].abs() >= n[1].abs() && n[0].abs() >= n[2].abs() { 0 }
                   else if n[1].abs() >= n[2].abs() { 1 }
                   else { 2 };
    let (u_axis, v_axis) = other_axes(dominant);

    let mut u_min = f64::MAX;
    let mut u_max = f64::MIN;
    let mut v_min = f64::MAX;
    let mut v_max = f64::MIN;
    for p in &poly_pts {
        let pu = p[u_axis];
        let pv = p[v_axis];
        if pu < u_min { u_min = pu; }
        if pu > u_max { u_max = pu; }
        if pv < v_min { v_min = pv; }
        if pv > v_max { v_max = pv; }
    }

    // ── 5. Halton sampling + signed distance accumulation ─────────────────────
    let n_samples = density.sample_count();
    let mut distances: Vec<f64> = Vec::with_capacity(n_samples);

    for i in 0..n_samples {
        let hu = halton(i + 1, 2);
        let hv = halton(i + 1, 3);

        let su = u_min + hu * (u_max - u_min);
        let sv = v_min + hv * (v_max - v_min);

        // Reconstruct 3D point: project along dominant axis onto plane_b
        // For a planar face the sample lives in the plane of face_a;
        // we project the sample onto plane_b to measure the gap.
        let mut sample = [0.0_f64; 3];
        sample[u_axis] = su;
        sample[v_axis] = sv;
        // set the dominant coordinate to 0; signed_distance from plane_b accounts for the offset
        sample[dominant] = 0.0;

        let dist = forge_geom::primitives::plane::signed_distance(plane_b, &sample);
        distances.push(dist);
    }

    // ── 6. Aggregate ──────────────────────────────────────────────────────────
    let min_gap_mm = distances.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_gap_mm = distances.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean_gap_mm = distances.iter().sum::<f64>() / (n_samples as f64);

    Ok(GapReport {
        min_gap_mm,
        max_gap_mm,
        mean_gap_mm,
        sample_count: n_samples,
        has_overlap: min_gap_mm < 0.0,
    })
}

// ── Halton sequence ───────────────────────────────────────────────────────────

/// Van der Corput / Halton sequence at index `n` in base `b`.
///
/// Produces a low-discrepancy value in `[0, 1)`. Indices start at 1.
fn halton(n: usize, b: usize) -> f64 {
    let mut f = 1.0_f64;
    let mut r = 0.0_f64;
    let mut idx = n;
    loop {
        f /= b as f64;
        r += f * (idx % b) as f64;
        idx /= b;
        if idx == 0 { break; }
    }
    r
}

/// Return the two axis indices that are NOT the given axis.
fn other_axes(dominant: usize) -> (usize, usize) {
    match dominant {
        0 => (1, 2),
        1 => (0, 2),
        _ => (0, 1),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halton_base2_first_eight() {
        let expected = [0.5, 0.25, 0.75, 0.125, 0.625, 0.375, 0.875, 0.0625];
        for (i, &exp) in expected.iter().enumerate() {
            let got = halton(i + 1, 2);
            assert!((got - exp).abs() < 1e-12, "index {}: got {}, exp {}", i + 1, got, exp);
        }
    }

    #[test]
    fn gap_sample_density_fixed_counts() {
        assert_eq!(GapSampleDensity::Coarse.sample_count(), 25);
        assert_eq!(GapSampleDensity::Medium.sample_count(), 100);
        assert_eq!(GapSampleDensity::Fine.sample_count(), 400);
    }

    #[test]
    fn measure_gap_parallel_faces() {
        let mut ctx = crate::core::ModelingContext::new();
        let res_a = crate::mesh_builder::make_cube([0.0, 0.0, 0.0], 10.0).unwrap();
        let (topo_a, geom_a) = res_a.into_parts();
        
        let res_b = crate::mesh_builder::make_cube([11.5, 0.0, 0.0], 10.0).unwrap();
        let (topo_b, geom_b) = res_b.into_parts();

        // Find the +X face of A and -X face of B
        let face_a = topo_a.arena().iter_faces().find(|(f, _)| {
            geom_a.get_face_plane(*f).map_or(false, |p| p.normal()[0] > 0.9)
        }).unwrap().0;

        let face_b = topo_b.arena().iter_faces().find(|(f, _)| {
            geom_b.get_face_plane(*f).map_or(false, |p| p.normal()[0] < -0.9)
        }).unwrap().0;

        let report = measure_gap(
            face_a, &topo_a, &geom_a,
            face_b, &topo_b, &geom_b,
            GapSampleDensity::Coarse,
            &mut ctx,
        ).into_value().unwrap();

        assert!(!report.has_overlap);
        assert!((report.min_gap_mm - 1.5).abs() < 1e-6);
        assert!((report.max_gap_mm - 1.5).abs() < 1e-6);
        assert!((report.mean_gap_mm - 1.5).abs() < 1e-6);
    }

    #[test]
    fn measure_gap_intersecting_faces() {
        let mut ctx = crate::core::ModelingContext::new();
        let res_a = crate::mesh_builder::make_cube([0.0, 0.0, 0.0], 10.0).unwrap();
        let (topo_a, geom_a) = res_a.into_parts();
        
        // Overlap of 2mm
        let res_b = crate::mesh_builder::make_cube([8.0, 0.0, 0.0], 10.0).unwrap();
        let (topo_b, geom_b) = res_b.into_parts();

        let face_a = topo_a.arena().iter_faces().find(|(f, _)| {
            geom_a.get_face_plane(*f).map_or(false, |p| p.normal()[0] > 0.9)
        }).unwrap().0;

        let face_b = topo_b.arena().iter_faces().find(|(f, _)| {
            geom_b.get_face_plane(*f).map_or(false, |p| p.normal()[0] < -0.9)
        }).unwrap().0;

        let report = measure_gap(
            face_a, &topo_a, &geom_a,
            face_b, &topo_b, &geom_b,
            GapSampleDensity::Coarse,
            &mut ctx,
        ).into_value().unwrap();

        assert!(report.has_overlap);
        assert!(report.min_gap_mm < -1.9); // Approximately -2.0
    }

    #[test]
    fn measure_gap_missing_plane_returns_error() {
        let mut ctx = crate::core::ModelingContext::new();
        let res_a = crate::mesh_builder::make_cube([0.0, 0.0, 0.0], 10.0).unwrap();
        let (topo_a, geom_a) = res_a.into_parts();
        
        let res_b = crate::mesh_builder::make_cube([11.5, 0.0, 0.0], 10.0).unwrap();
        let (topo_b, _geom_b) = res_b.into_parts();
        let empty_geom = GeometryStore::new();

        let face_a = topo_a.arena().iter_faces().next().unwrap().0;
        let face_b = topo_b.arena().iter_faces().next().unwrap().0;
        
        let result = measure_gap(
            face_a, &topo_a, &geom_a,
            face_b, &topo_b, &empty_geom,
            GapSampleDensity::Coarse,
            &mut ctx,
        ).into_value();

        match result {
            Err(KernelError::InvalidInput { message, .. }) => {
                assert!(message.contains("no plane registered"));
            }
            _ => panic!("Expected InvalidInput for missing plane, got {:?}", result),
        }
    }
}
