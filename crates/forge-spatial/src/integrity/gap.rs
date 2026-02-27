//! Face-to-face gap measurement via Halton-sampled point projection.
//!
//! DOMAIN: Measure the geometric distance between two faces by
//! projecting quasi-random sample points onto the opposing face's plane.
//!
//! DEPENDENCIES: `forge-topo` (arena, handles, traversal),
//!               `forge-geom` (Plane, signed_distance).
//! INVARIANTS:
//!   - Non-planar faces must be filtered by the caller.
//!   - Sample density maps to fixed counts for reproducible results.
//!   - `has_overlap = min_gap_mm < 0` (negative = penetration).

use forge_core::KernelError;
use forge_geom::primitives::plane::{signed_distance, Plane};
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;

/// Statistical summary of the sampled gap between two faces.
#[derive(Debug, Clone)]
pub struct GapReport {
    /// Smallest signed distance across all samples (mm).
    /// Negative means penetration/overlap.
    min_gap_mm: f64,
    /// Largest signed distance across all samples (mm).
    max_gap_mm: f64,
    /// Mean signed distance across all samples (mm).
    mean_gap_mm: f64,
    /// Total number of samples taken.
    sample_count: usize,
    /// True when `min_gap_mm < 0` (faces interpenetrate).
    has_overlap: bool,
}

impl GapReport {
    /// Smallest signed distance across all samples.
    pub fn get_min_gap_mm(&self) -> f64 {
        self.min_gap_mm
    }

    /// Largest signed distance across all samples.
    pub fn get_max_gap_mm(&self) -> f64 {
        self.max_gap_mm
    }

    /// Mean signed distance across all samples.
    pub fn get_mean_gap_mm(&self) -> f64 {
        self.mean_gap_mm
    }

    /// Total number of samples taken.
    pub fn get_sample_count(&self) -> usize {
        self.sample_count
    }

    /// True when faces interpenetrate.
    pub fn has_overlap(&self) -> bool {
        self.has_overlap
    }
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
            GapSampleDensity::Fine => 400,
        }
    }
}

/// Measure the face-to-face gap between `face_a` and `face_b` using Halton sampling.
///
/// Collects vertex positions of `face_a` via `position_fn`, computes a bounding
/// rectangle in the dominant projection axis of `plane_b`, generates quasi-random
/// Halton samples over that rectangle, and measures their signed distance to `plane_b`.
///
/// `plane_fn` must return the supporting plane for the given face. If the plane
/// is missing, `KernelError::InvalidInput` is returned.
pub fn measure_gap(
    face_a: FaceId,
    arena_a: &TopologyArena,
    face_b: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    plane_fn: &dyn Fn(FaceId) -> Option<Plane>,
    density: GapSampleDensity,
) -> Result<GapReport, KernelError> {
    let plane_b = plane_fn(face_b).ok_or_else(|| KernelError::InvalidInput {
        message: format!("measure_gap: no plane registered for face {:?}", face_b),
        context: None,
    })?;

    let edges: Vec<_> = FaceEdgeIterator::new(arena_a, face_a)
        .map_err(|e| e.with_phase("measure_gap.face_a_iter"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.with_phase("measure_gap.face_a_collect"))?;

    let mut poly_pts: Vec<[f64; 3]> = Vec::with_capacity(edges.len());
    for he in &edges {
        let v = arena_a.get_half_edge(*he)?.origin();
        if let Some(pos) = position_fn(v) {
            poly_pts.push(pos);
        }
    }

    if poly_pts.len() < 3 {
        return Err(KernelError::InvalidInput {
            message: format!("measure_gap: face {:?} has fewer than 3 vertices", face_a),
            context: None,
        });
    }

    let n = plane_b.normal();
    let dominant = if n[0].abs() >= n[1].abs() && n[0].abs() >= n[2].abs() {
        0
    } else if n[1].abs() >= n[2].abs() {
        1
    } else {
        2
    };
    let (u_axis, v_axis) = other_axes(dominant);

    let mut u_min = f64::MAX;
    let mut u_max = f64::MIN;
    let mut v_min = f64::MAX;
    let mut v_max = f64::MIN;
    for p in &poly_pts {
        let pu = p[u_axis];
        let pv = p[v_axis];
        if pu < u_min {
            u_min = pu;
        }
        if pu > u_max {
            u_max = pu;
        }
        if pv < v_min {
            v_min = pv;
        }
        if pv > v_max {
            v_max = pv;
        }
    }

    let n_samples = density.sample_count();
    let mut distances: Vec<f64> = Vec::with_capacity(n_samples);

    for i in 0..n_samples {
        let hu = halton(i + 1, 2);
        let hv = halton(i + 1, 3);

        let su = u_min + hu * (u_max - u_min);
        let sv = v_min + hv * (v_max - v_min);

        let mut sample = [0.0_f64; 3];
        sample[u_axis] = su;
        sample[v_axis] = sv;
        sample[dominant] = 0.0;

        let dist = signed_distance(&plane_b, &sample);
        distances.push(dist);
    }

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
        if idx == 0 {
            break;
        }
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
