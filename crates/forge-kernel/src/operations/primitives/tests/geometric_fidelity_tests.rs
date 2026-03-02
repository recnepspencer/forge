//! Geometric fidelity tests — vertex positions and symbolic planes.

use crate::context::ModelingContext;
use crate::geometry::facade::GeometryView;
use crate::operations::primitives::make_cube;
use super::{test_config, OperationScope};

#[test]
fn cube_vertex_positions_are_corners() {
    let cfg = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let size = 2.0;
    let hs = size / 2.0;
    let r = make_cube([0.0; 3], size, &mut scope).unwrap();

    let mut corners: Vec<[f64; 3]> = Vec::new();
    for (vid, _) in r.topology().arena().iter_vertices() {
        corners.push(*r.geometry().get_vertex_position(vid).unwrap());
    }
    assert_eq!(corners.len(), 8, "Cube must have 8 vertices");
    for corner in &corners {
        for coord in corner {
            assert!(
                (coord.abs() - hs).abs() < 1e-10,
                "Cube vertex {corner:?}: each coord should be ±{hs}"
            );
        }
    }
}

#[test]
fn cube_vertices_have_symbolic_planes() {
    let cfg = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let r = make_cube([0.0; 3], 2.0, &mut scope).unwrap();
    for (vid, _) in r.topology().arena().iter_vertices() {
        assert!(
            r.geometry().get_vertex_exact(vid)
                .and_then(|ep| ep.symbolic_planes()).is_some(),
            "V#{} should have symbolic plane indices", vid.index()
        );
    }
}

#[test]
fn cube_offset_vertex_positions_correct() {
    let cfg = test_config();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let center = [5.0, -3.0, 7.0];
    let size = 4.0;
    let hs = size / 2.0;
    let r = make_cube(center, size, &mut scope).unwrap();

    for (vid, _) in r.topology().arena().iter_vertices() {
        let pos = r.geometry().get_vertex_position(vid).unwrap();
        for (i, c) in center.iter().enumerate() {
            let offset = (pos[i] - c).abs();
            assert!(
                (offset - hs).abs() < 1e-10,
                "V#{}: coord[{i}]={} should be {}±{hs}",
                vid.index(), pos[i], c
            );
        }
    }
}
