//! Sliver face detection.
//!
//! DOMAIN: Detect degenerate thin faces whose area falls below a threshold.
//! Slivers indicate precision issues or require explicit policy waivers.
//!
//! DEPENDENCIES: `forge-topo` (arena, handles, traversal),
//!               `forge-geom` (polygon area).
//! INVARIANTS: No topology mutation. Read-only spatial query.

use forge_core::KernelError;
use forge_geom::primitives::polygon::compute_polygon_area;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;

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
        let vertices = collect_face_positions(arena, face_id, position_fn)?;
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

/// Collect vertex positions around a face via halfedge traversal.
fn collect_face_positions(
    arena: &TopologyArena,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions = Vec::new();
    let mut count = 0;

    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        let v = he.origin();
        if let Some(pos) = position_fn(v) {
            positions.push(pos);
        }
        count += 1;
        if count > 10000 {
            return Err(KernelError::InternalError {
                message: "Face traversal exceeded 10000 edges (infinite loop?)".to_string(),
                context: None,
            });
        }
    }

    Ok(positions)
}
