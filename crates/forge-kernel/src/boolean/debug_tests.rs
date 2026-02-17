//! Regression tests for Boolean classification edge cases.

use forge_geom::bsp::{build_convex_polyhedron, BspConfig};
use forge_geom::plane::Plane;
use crate::core::ModelingContext;
use crate::mesh_builder::build_halfedge_mesh;
use forge_topo::classify::{classify_point_in_solid, PointClassification};
use forge_core::KernelError;

fn build_cube(
    center: [f64; 3],
    half_size: f64,
) -> (forge_topo::state::TopologyState, crate::geometry_store::GeometryStore) {
    let planes = vec![
        Plane::from_point_normal(
            [center[0] + half_size, center[1], center[2]],
            [1.0, 0.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0] - half_size, center[1], center[2]],
            [-1.0, 0.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1] + half_size, center[2]],
            [0.0, 1.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1] - half_size, center[2]],
            [0.0, -1.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1], center[2] + half_size],
            [0.0, 0.0, 1.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1], center[2] - half_size],
            [0.0, 0.0, -1.0],
        ).unwrap(),
    ];
    let cell = build_convex_polyhedron(&planes, &BspConfig::default()).unwrap();
    let mut ctx = ModelingContext::new();
    build_halfedge_mesh(&cell, &mut ctx).unwrap().into_parts()
}

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
        let centroid = face_centroid(source_topo, source_geom, *fid);

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
            target_topo.arena(), &vertex_lookup, &centroid, 1e6,
        ).unwrap();
        matches!(result, PointClassification::Inside)
    }).count()
}

fn face_centroid(
    topo: &forge_topo::state::TopologyState,
    geom: &crate::geometry_store::GeometryStore,
    face: forge_topo::handles::FaceId,
) -> [f64; 3] {
    let fdata = topo.arena().get_face(face).unwrap();
    let ld = topo.arena().get_loop(fdata.outer_loop).unwrap();
    let start = ld.half_edge;
    let mut current = start;
    let mut sum = [0.0; 3];
    let mut count = 0u32;
    loop {
        let he = topo.arena().get_half_edge(current).unwrap();
        let pos = geom.get_vertex_position(he.origin).unwrap();
        sum[0] += pos[0];
        sum[1] += pos[1];
        sum[2] += pos[2];
        count += 1;
        current = he.next;
        if current == start { break; }
    }
    let inv = 1.0 / f64::from(count);
    [sum[0] * inv, sum[1] * inv, sum[2] * inv]
}
