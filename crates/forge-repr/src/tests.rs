//! Tests for forge-repr types and traits.

use crate::{TriangleMesh, Viewable, Tessellatable};

#[test]
fn triangle_mesh_construction() {
    let mut mesh = TriangleMesh::new();
    let v0 = mesh.add_vertex([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let v1 = mesh.add_vertex([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let v2 = mesh.add_vertex([0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
    mesh.add_triangle(v0, v1, v2);

    assert_eq!(mesh.vertex_count(), 3);
    assert_eq!(mesh.triangle_count(), 1);
}

#[test]
fn viewable_trait_is_object_safe() {
    struct UnitSphere;
    impl Viewable for UnitSphere {
        fn evaluate_sdf(&self, point: [f64; 3]) -> f64 {
            let r = (point[0] * point[0]
                + point[1] * point[1]
                + point[2] * point[2])
                .sqrt();
            r - 1.0
        }
        fn bounding_box(&self) -> ([f64; 3], [f64; 3]) {
            ([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0])
        }
    }

    let sphere = UnitSphere;
    let _: &dyn Viewable = &sphere;
    assert!(sphere.evaluate_sdf([0.0, 0.0, 0.0]) < 0.0);
    assert!(sphere.evaluate_sdf([2.0, 0.0, 0.0]) > 0.0);
}

#[test]
fn tessellatable_trait_is_object_safe() {
    struct EmptyGeom;
    impl Tessellatable for EmptyGeom {
        fn tessellate(&self, _tolerance: f64) -> TriangleMesh {
            TriangleMesh::new()
        }
    }

    let geom = EmptyGeom;
    let _: &dyn Tessellatable = &geom;
    let mesh = geom.tessellate(0.1);
    assert_eq!(mesh.vertex_count(), 0);
}
