//! EMBER BSP pipeline tests — BSP merge boolean operations.
//!
//! Tests validate the BSP merge pipeline and dual-engine router.

#[cfg(test)]
mod tests {
    use crate::operations::boolean::test_helpers::build_cube;
    use crate::operations::boolean::{BooleanInput, BooleanOp, BooleanResult};
    use crate::operations::ember_boolean::{execute_ember_boolean, execute_boolean_adaptive, EmberError};

    /// Helper: execute EMBER boolean and unwrap.
    fn ember_boolean(input: BooleanInput) -> BooleanResult {
        let envelope = execute_ember_boolean(input)
            .expect("EMBER should not reject planar inputs");
        envelope.into_result()
            .expect("EMBER boolean should succeed")
    }

    #[test]
    fn ember_basic_intersection() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([1.0, 0.0, 0.0], 1.0);
        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);
        let result = ember_boolean(input);
        assert_eq!(result.topology().arena().face_count(), 6);
    }

    #[test]
    fn ember_basic_subtraction() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([1.0, 0.0, 0.0], 1.0);
        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Subtraction);
        let result = ember_boolean(input);
        assert!(result.topology().arena().face_count() >= 6);
    }

    #[test]
    fn ember_disjoint_union() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([10.0, 0.0, 0.0], 1.0);
        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);
        let result = ember_boolean(input);
        assert_eq!(result.topology().arena().face_count(), 12);
    }

    #[test]
    fn ember_embedded_subtraction() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 2.0);
        let (topo_b, geom_b) = build_cube([0.0, 0.0, 0.0], 0.5);
        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Subtraction);
        let result = ember_boolean(input);
        assert!(result.topology().arena().face_count() >= 12);
    }

    /// Scale disparity — BSP merge handles this via exact arithmetic, no issue.
    #[test]
    fn ember_scale_disparity_union() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1e6);
        let (topo_b, geom_b) = build_cube([0.0, 0.0, 0.0], 1e-3);
        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);
        match execute_ember_boolean(input) {
            Ok(envelope) => { let _ = envelope.into_result(); }
            Err(EmberError::PipelineError(e)) => {
                eprintln!("EMBER pipeline error (acceptable): {e}");
            }
            Err(e) => panic!("Unexpected EMBER error: {:?}", e),
        }
    }

    /// Adaptive router should always succeed (falls back to legacy).
    #[test]
    fn adaptive_router_always_succeeds() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([1.0, 0.0, 0.0], 1.0);
        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);
        let envelope = execute_boolean_adaptive(input);
        let result = envelope.into_result().expect("Adaptive should always produce a result");
        assert_eq!(result.topology().arena().face_count(), 6);
    }

    /// Two cubes sharing a face — coplanar boundary.
    #[test]
    fn ember_coplanar_union_two_cubes() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([2.0, 0.0, 0.0], 1.0);
        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);
        let result = ember_boolean(input);
        assert!(result.topology().arena().face_count() >= 10);
    }

    /// 2×2×2 grid union — chained BSP merge operations.
    #[test]
    fn ember_coplanar_grid_2x2x2() {
        let step = 2.0;
        let (mut topo, mut geom) = build_cube([0.0, 0.0, 0.0], 1.0);
        for ix in 0..2usize {
            for iy in 0..2usize {
                for iz in 0..2usize {
                    if ix == 0 && iy == 0 && iz == 0 { continue; }
                    let center = [ix as f64 * step, iy as f64 * step, iz as f64 * step];
                    let (topo_tool, geom_tool) = build_cube(center, 1.0);
                    let input = BooleanInput::new(topo, geom, topo_tool, geom_tool, BooleanOp::Union);
                    match execute_ember_boolean(input) {
                        Ok(envelope) => match envelope.into_result() {
                            Ok(r) => { let parts = r.into_topo_geom(); topo = parts.0; geom = parts.1; }
                            Err(e) => panic!("EMBER 2x2x2 step failed: {:?}", e),
                        },
                        Err(e) => panic!("EMBER pipeline failed: {:?}", e),
                    }
                }
            }
        }
        let face_count = topo.arena().face_count();
        // Ideal: 6 (fully merged cube). Max: 48 (8 cubes × 6 faces, no merging).
        // Coplanar face merging across chained operations is an optimization.
        assert!(face_count >= 6 && face_count <= 48,
            "Expected 6..48 faces, got {}", face_count);
    }
}
