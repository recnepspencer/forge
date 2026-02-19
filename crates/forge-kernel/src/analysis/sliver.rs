//! Sliver face detection and area computation.
//!
//! DOMAIN: Post-boolean quality analysis — detecting degenerate thin faces
//! that indicate precision issues or require explicit policy waivers.
//!
//! DEPENDENCIES: `forge-topo` (arena, handles), `geometry_store` (vertex positions)

use forge_core::KernelError;
use forge_topo::handles::FaceId;
use forge_topo::state::TopologyState;
use crate::geometry_store::GeometryStore;

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
/// Walks each face loop, collects vertex positions, computes the polygon
/// area via the Newell method (sum of cross products), and classifies
/// faces below `min_face_area` as slivers.
pub fn analyze_slivers(
    topo: &TopologyState,
    geom: &GeometryStore,
    min_face_area: f64,
) -> Result<SliverReport, KernelError> {
    let arena = topo.arena();
    let mut face_areas: Vec<(FaceId, f64)> = Vec::new();
    let mut sliver_count = 0;

    for (face_id, face_data) in arena.iter_faces() {
        let vertices = collect_loop_positions(topo, geom, face_data.outer_loop())?;
        let area = forge_geom::primitives::polygon::compute_polygon_area(&vertices);

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

/// Collect vertex positions around a face loop.
fn collect_loop_positions(
    topo: &TopologyState,
    geom: &GeometryStore,
    loop_id: forge_topo::handles::LoopId,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let arena = topo.arena();
    let loop_data = arena.get_loop(loop_id)?;
    let start_he = loop_data.half_edge();
    let mut positions = Vec::new();
    let mut current_he = start_he;

    loop {
        let he_data = arena.get_half_edge(current_he)?;

        if let Some(pos) = geom.get_vertex_position(he_data.origin()) {
            positions.push(*pos);
        }

        current_he = he_data.next();
        if current_he == start_he {
            break;
        }

        if positions.len() > 10000 {
            return Err(KernelError::InternalError {
                message: "Loop traversal exceeded 10000 edges (infinite loop?)".to_string(),
                context: None,
            });
        }
    }

    Ok(positions)
}


#[cfg(test)]
mod tests {
    use crate::geometry_store::GeometryStore;
    use forge_topo::state::TopologyState;
    use super::analyze_slivers;

    // Unit tests for polygon area are now in forge-geom.
    // We only test the integration here if needed.

    #[test]
    fn unit_square_area_is_one() {
        let verts = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let area = forge_geom::primitives::polygon::compute_polygon_area(&verts);
        assert!((area - 1.0).abs() < 1e-10, "Expected 1.0, got {area}");
    }

    #[test]
    fn triangle_area() {
        let verts = [
            [0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [0.0, 3.0, 0.0],
        ];
        let area = forge_geom::primitives::polygon::compute_polygon_area(&verts);
        assert!((area - 3.0).abs() < 1e-10, "Expected 3.0, got {area}");
    }

    #[test]
    fn degenerate_line_has_zero_area() {
        let verts = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ];
        let area = forge_geom::primitives::polygon::compute_polygon_area(&verts);
        assert!(area < 1e-15, "Expected 0, got {area}");
    }

    #[test]
    fn sliver_rectangle_area() {
        let verts = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1e-12, 0.0],
            [0.0, 1e-12, 0.0],
        ];
        let area = forge_geom::primitives::polygon::compute_polygon_area(&verts);
        assert!(area < 1e-10, "Sliver area {area} should be tiny");
        assert!(area > 0.0, "Sliver area should be positive");
    }
}
