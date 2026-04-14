//! Sliver face detection.
//!
//! DOMAIN: Detect degenerate thin faces whose area falls below a threshold.
//! Slivers indicate precision issues or require explicit policy waivers.
//!
//! DEPENDENCIES: `forge-topo` (arena, handles, traversal),
//!               `worth-geom` (polygon area).
//! INVARIANTS: No topology mutation. Read-only spatial query.

use forge_core::KernelError;
use worth_geom::compute_polygon_area;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};

use super::utils;

/// Result of sliver analysis on a topology.
#[derive(Debug, Clone)]
pub struct SliverReport {
    /// Total number of faces analyzed.
    face_count: usize,
    /// Number of faces with area below the threshold.
    sliver_count: usize,
    /// Per-face areas (sorted ascending).
    face_areas: Vec<(FaceId, f64)>,
    /// The threshold used for sliver classification.
    threshold: f64,
}

impl SliverReport {
    /// Total faces analyzed.
    pub fn get_face_count(&self) -> usize {
        self.face_count
    }

    /// Number of sliver faces (area < threshold).
    pub fn get_sliver_count(&self) -> usize {
        self.sliver_count
    }

    /// All face areas, sorted ascending.
    pub fn get_face_areas(&self) -> &[(FaceId, f64)] {
        &self.face_areas
    }

    /// The threshold used for classification.
    pub fn get_threshold(&self) -> f64 {
        self.threshold
    }

    /// The smallest face area, or None if no faces.
    pub fn get_min_area(&self) -> Option<f64> {
        self.face_areas.first().map(|(_, a)| *a)
    }

    /// Face IDs of all slivers.
    pub fn get_sliver_face_ids(&self) -> Vec<FaceId> {
        self.face_areas
            .iter()
            .filter(|(_, area)| *area < self.threshold)
            .map(|(fid, _)| *fid)
            .collect()
    }
}

/// Analyze a topology for sliver faces.
///
/// Walks each face loop, collects vertex positions via `position_fn`,
/// computes the polygon area via the Newell method (sum of cross products),
/// and classifies faces below `min_face_area` as slivers.
pub fn analyze_slivers(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    min_face_area: f64,
) -> Result<SliverReport, KernelError> {
    let mut face_areas: Vec<(FaceId, f64)> = Vec::new();
    let mut sliver_count = 0;

    for (face_id, _) in arena.iter_faces() {
        let vertices = utils::collect_face_positions(arena, face_id, position_fn)?;
        if vertices.len() < 3 {
            continue;
        }

        let area = compute_polygon_area(&vertices);

        if area < min_face_area {
            sliver_count += 1;
        }

        face_areas.push((face_id, area));
    }

    face_areas.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    Ok(SliverReport {
        face_count: face_areas.len(),
        sliver_count,
        face_areas,
        threshold: min_face_area,
    })
}
