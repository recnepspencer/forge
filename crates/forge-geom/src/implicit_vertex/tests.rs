//! Tests for the ImplicitVertex primitive.

#[cfg(test)]
mod tests {
    use crate::plane::Plane;
    use crate::PlaneSet;
    use crate::implicit_vertex::{ImplicitVertex, PlaneRef, resolve_position, select_best_triple};

    /// Default residual tolerance for tests.
    const TEST_RESIDUAL: f64 = 1e-8;
    /// Default degeneracy threshold for tests.
    const TEST_DEGENERACY: f64 = 1e-15;

    fn cube_planes() -> Vec<Plane> {
        vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),    // x = 0
            Plane::try_new([-1.0, 0.0, 0.0], 1.0).unwrap(),   // x = 1
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),    // y = 0
            Plane::try_new([0.0, -1.0, 0.0], 1.0).unwrap(),   // y = 1
            Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),    // z = 0
            Plane::try_new([0.0, 0.0, -1.0], 1.0).unwrap(),   // z = 1
        ]
    }

    #[test]
    fn reject_fewer_than_three_planes() {
        let result = ImplicitVertex::try_new(vec![PlaneRef::new(0), PlaneRef::new(1)]);
        assert!(result.is_none());
    }

    #[test]
    fn three_axis_aligned_planes_at_origin() {
        let planes = cube_planes();
        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(2), PlaneRef::new(4),
        ]).unwrap();

        let pos = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY).unwrap();
        assert!((pos[0]).abs() < 1e-10);
        assert!((pos[1]).abs() < 1e-10);
        assert!((pos[2]).abs() < 1e-10);
    }

    #[test]
    fn cube_vertex_at_one_one_one() {
        let planes = cube_planes();
        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(1), PlaneRef::new(3), PlaneRef::new(5),
        ]).unwrap();

        let pos = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY).unwrap();
        assert!((pos[0] - 1.0).abs() < 1e-10);
        assert!((pos[1] - 1.0).abs() < 1e-10);
        assert!((pos[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn all_eight_cube_vertices_resolve() {
        let planes = cube_planes();
        let triples: [(usize, usize, usize); 8] = [
            (0, 2, 4), (0, 2, 5), (0, 3, 4), (0, 3, 5),
            (1, 2, 4), (1, 2, 5), (1, 3, 4), (1, 3, 5),
        ];

        for (a, b, c) in triples {
            let vertex = ImplicitVertex::try_new(vec![
                PlaneRef::new(a), PlaneRef::new(b), PlaneRef::new(c),
            ]).unwrap();
            let pos = resolve_position(&vertex, &PlaneSet::new(planes.clone()), TEST_RESIDUAL, TEST_DEGENERACY);
            assert!(pos.is_ok(), "Failed to resolve vertex ({}, {}, {})", a, b, c);
        }
    }

    #[test]
    fn overconstrained_apex_consistent() {
        let planes = vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),
            Plane::try_new([1.0, 1.0, 1.0], 0.0).unwrap(),
        ];

        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(1), PlaneRef::new(2), PlaneRef::new(3),
        ]).unwrap();

        assert!(vertex.is_overconstrained());

        let pos = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY).unwrap();
        assert!((pos[0]).abs() < 1e-10);
        assert!((pos[1]).abs() < 1e-10);
        assert!((pos[2]).abs() < 1e-10);
    }

    #[test]
    fn overconstrained_inconsistent_returns_error() {
        let planes = vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),
            Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap(),
        ];

        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(1), PlaneRef::new(2), PlaneRef::new(3),
        ]).unwrap();

        let result = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY);
        assert!(result.is_err());
    }

    #[test]
    fn select_best_triple_picks_well_conditioned() {
        let planes = vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),
            Plane::try_new([1.0, 1.0, 0.0], 0.0).unwrap(),
        ];

        let refs: Vec<PlaneRef> = (0..4).map(PlaneRef::new).collect();
        let (i, j, k) = select_best_triple(&refs, &PlaneSet::new(planes)).unwrap();

        assert!(i < j);
        assert!(j < k);
    }

    #[test]
    fn implicit_vertex_plane_count() {
        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(1), PlaneRef::new(2),
        ]).unwrap();

        assert_eq!(vertex.plane_count(), 3);
        assert!(!vertex.is_overconstrained());
    }

    #[test]
    fn out_of_bounds_plane_ref_returns_error() {
        let planes = vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
        ];

        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(1), PlaneRef::new(99),
        ]).unwrap();

        let result = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY);
        assert!(result.is_err());
    }
}
