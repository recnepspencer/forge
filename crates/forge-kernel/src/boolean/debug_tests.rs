//! Regression tests for Boolean classification edge cases.

use forge_topo::classify::{classify_point_in_solid, PointClassification};
use forge_core::KernelError;

use super::test_helpers::{build_cube, face_centroid};

/// Verifies all inner cube face centroids classify as Inside
/// the outer cube for both axis-aligned and offset geometries.
#[test]
fn all_inner_faces_classify_inside_outer() {
    for center in [[0.0, 0.0, 0.0], [0.1, 0.2, 0.3], [1.5, -0.5, 0.7]] {
        let (topo_outer, geom_outer) = build_cube(center, 2.0);
        let (topo_inner, geom_inner) = build_cube(center, 1.0);

        let inside_count = count_faces_inside(
            &topo_inner, &geom_inner,
            &topo_outer, &geom_outer,
        );

        assert_eq!(
            inside_count, 6,
            "All 6 inner faces should be Inside outer, got {} for center {:?}",
            inside_count, center,
        );
    }
}

fn count_faces_inside(
    source_topo: &forge_topo::state::TopologyState,
    source_geom: &crate::geometry_store::GeometryStore,
    target_topo: &forge_topo::state::TopologyState,
    target_geom: &crate::geometry_store::GeometryStore,
) -> usize {
    source_topo.arena().iter_faces().filter(|(fid, _fdata)| {
        let centroid = face_centroid(source_topo.arena(), source_geom, *fid);

        let vertex_lookup = |index: u32| -> Result<[f64; 3], KernelError> {
            let gen = target_topo.arena().vertex_generation(index as usize).ok_or_else(|| {
                KernelError::InvalidInput {
                    message: format!("No vertex at index {}", index),
                    context: None,
                }
            })?;
            let vid = forge_topo::handles::VertexId::new(index, gen);
            target_geom.get_vertex_position(vid).copied().ok_or_else(|| {
                KernelError::InvalidInput {
                    message: format!("No position for vertex {}", index),
                    context: None,
                }
            })
        };

        let result = classify_point_in_solid(
            target_topo.arena(), &vertex_lookup, &centroid, 1e6, 1e-10,
        ).unwrap();
        matches!(result, PointClassification::Inside)
    }).count()
}
