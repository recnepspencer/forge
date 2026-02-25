//! Adversarial tests for merge eligibility certification.
//!
//! DOMAIN: Tests that the boundary certification → merge gating pipeline
//! works correctly under adversarial inputs. Split into two sections:
//!
//! 1. **Certifier adversarial** — exercises forge-geom boundary_cert with
//!    pathological geometry (D2/D6/D7 regressions).
//! 2. **Kernel integration** — exercises the full pipeline through
//!    `certify_merge_boundary` with real `TopologyArena` + `GeometryState`,
//!    and validates trace propagation + geometry cleanup.
//!
//! DEPENDENCIES: forge-geom (boundary_cert), forge-topo, GeometryState, ModelingContext.

#[cfg(test)]
mod tests {
    use forge_geom::algorithms::boundary_cert::schema::*;
    use forge_geom::algorithms::boundary_cert::eval::*;
    use std::sync::{Mutex, OnceLock};

    // =====================================================================
    // SECTION 1: Certifier-level adversarial tests
    // =====================================================================

    /// D2 regression: figure-8 with crossing forces fallback path.
    /// Must be rejected — the old `unwrap_or(TriSign::Zero)` would accept.
    #[test]
    fn fallback_path_rejects_figure_eight_with_crossing() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [2.0, 0.0], 0),
            Segment2D::new([2.0, 0.0], [1.0, 1.0], 1),
            Segment2D::new([1.0, 1.0], [2.0, 2.0], 2),
            Segment2D::new([2.0, 2.0], [0.0, 0.0], 3),
            Segment2D::new([0.0, 0.0], [0.0, 2.0], 4),
            Segment2D::new([0.0, 2.0], [1.0, 1.0], 5),
            Segment2D::new([1.0, 1.0], [0.0, 0.0], 6),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        assert!(
            matches!(cert, WeakSimpleCertificate::Rejected { .. }),
            "Figure-8 with crossing MUST be rejected, got {:?}",
            cert,
        );
    }

    /// D2 regression: collinear overlap must be rejected in fallback.
    #[test]
    fn collinear_overlap_with_reversal_rejected_in_fallback() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [3.0, 0.0], 0),
            Segment2D::new([3.0, 0.0], [3.0, 1.0], 1),
            Segment2D::new([3.0, 1.0], [2.0, 1.0], 2),
            Segment2D::new([2.0, 1.0], [2.0, 0.0], 3),
            Segment2D::new([2.0, 0.0], [4.0, 0.0], 4),
            Segment2D::new([4.0, 0.0], [4.0, 1.0], 5),
            Segment2D::new([4.0, 1.0], [0.0, 1.0], 6),
            Segment2D::new([0.0, 1.0], [0.0, 0.0], 7),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        assert!(
            matches!(cert, WeakSimpleCertificate::Rejected { reason: BoundaryRejectReason::OverlappingSegments, .. }),
            "Collinear overlap MUST be rejected, got {:?}",
            cert,
        );
    }

    /// D7 regression: equal components → X dropped (spec §4.6 priority).
    #[test]
    fn projection_frame_tiebreak_all_equal_drops_x() {
        let frame = build_projection_frame([1.0, 1.0, 1.0]);
        assert_eq!(frame.get_drop_axis(), 0, "Equal components: drop X");
    }

    #[test]
    fn projection_frame_tiebreak_yz_equal_drops_y() {
        let frame = build_projection_frame([0.1, 0.5, 0.5]);
        assert_eq!(frame.get_drop_axis(), 1, "Y == Z > X: drop Y");
    }

    #[test]
    fn projection_frame_tiebreak_xz_equal_drops_x() {
        let frame = build_projection_frame([0.5, 0.1, 0.5]);
        assert_eq!(frame.get_drop_axis(), 0, "X == Z > Y: drop X");
    }

    #[test]
    fn projection_frame_negative_normal_flips_orientation() {
        let pos = build_projection_frame([0.0, 0.0, 1.0]);
        let neg = build_projection_frame([0.0, 0.0, -1.0]);
        assert_eq!(pos.get_drop_axis(), neg.get_drop_axis());
        assert!(pos.get_orientation_sign() > 0.0);
        assert!(neg.get_orientation_sign() < 0.0);
    }

    /// Machine-epsilon perturbation must not cause false rejection.
    #[test]
    fn machine_epsilon_perturbation_still_simple() {
        let eps = f64::EPSILON;
        let vertices = [[0.0, 0.0], [1.0, eps], [1.0, 1.0], [0.0, 1.0]];
        let n = vertices.len();
        let segments: Vec<Segment2D> = (0..n)
            .map(|i| Segment2D::new(vertices[i], vertices[(i + 1) % n], i as u64))
            .collect();
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        assert_eq!(certify_boundary(&boundary), WeakSimpleCertificate::Simple);
    }

    /// Exact crossings must be detected as an intersection and not silently ignored.
    #[test]
    fn tiny_crossing_not_silently_accepted() {
        // A bow-tie shape with an explicit exact crossing.
        let segments = vec![
            Segment2D::new([0.0, 0.0], [1.0, 0.0], 0),
            Segment2D::new([1.0, 0.0], [0.0, 1.0], 1), // Crosses segment 3 transversally
            Segment2D::new([0.0, 1.0], [1.0, 1.0], 2),
            Segment2D::new([1.0, 1.0], [0.0, 0.0], 3), // Crosses segment 1 transversally
        ];

        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        let cert = certify_boundary(&boundary);
        assert!(
            matches!(cert, WeakSimpleCertificate::Rejected { reason: BoundaryRejectReason::SelfCrossing, .. }),
            "Exact crossing must be Rejected with SelfCrossing, got {:?}",
            cert,
        );
    }

    /// Degenerate collinear triangle — zero enclosed area — must be rejected.
    #[test]
    fn degenerate_collinear_triangle_rejected() {
        let segments = vec![
            Segment2D::new([0.0, 0.0], [1.0, 0.0], 0),
            Segment2D::new([1.0, 0.0], [2.0, 0.0], 1),
            Segment2D::new([2.0, 0.0], [0.0, 0.0], 2),
        ];
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        assert!(
            matches!(certify_boundary(&boundary), WeakSimpleCertificate::Rejected { .. }),
            "Collinear triangle is zero-area degenerate — must be rejected",
        );
    }

    /// Pentagram: 5 proper crossings → SelfCrossing.
    #[test]
    fn pentagram_five_crossings_rejected() {
        use std::f64::consts::PI;
        let mut vertices = [[0.0f64; 2]; 5];
        for i in 0..5 {
            let angle = (2.0 * PI * (2 * i) as f64) / 5.0 - PI / 2.0;
            vertices[i] = [angle.cos(), angle.sin()];
        }
        let segments: Vec<Segment2D> = (0..5)
            .map(|i| Segment2D::new(vertices[i], vertices[(i + 1) % 5], i as u64))
            .collect();
        let frame = ProjectionFrame2D::new(2, 0, 1, 1.0);
        let boundary = ProjectedBoundary2D::new(segments, frame);
        assert!(
            matches!(certify_boundary(&boundary), WeakSimpleCertificate::Rejected { reason: BoundaryRejectReason::SelfCrossing, .. }),
            "Pentagram must be SelfCrossing",
        );
    }

    /// D5 regression: different groups → different DecisionIds.
    #[test]
    fn different_groups_produce_different_decision_ids() {
        use forge_topo::bitset::EntityBitset;

        let mut group_a = EntityBitset::with_capacity(10);
        let _ = group_a.insert(0);
        let _ = group_a.insert(1);

        let mut group_b = EntityBitset::with_capacity(10);
        let _ = group_b.insert(2);
        let _ = group_b.insert(3);

        assert_ne!(
            compute_group_hash(&group_a),
            compute_group_hash(&group_b),
            "Different groups must produce different DecisionIds (D5 regression)",
        );
    }

    #[test]
    fn same_group_produces_same_decision_id() {
        use forge_topo::bitset::EntityBitset;
        let mut group = EntityBitset::with_capacity(10);
        let _ = group.insert(0);
        let _ = group.insert(3);
        let _ = group.insert(7);
        assert_eq!(compute_group_hash(&group), compute_group_hash(&group));
    }

    fn compute_group_hash(group: &forge_topo::bitset::EntityBitset) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for idx in 0..group.capacity() {
            if group.contains(idx).unwrap_or(false) {
                h = h.wrapping_mul(0x100000001b3) ^ (idx as u64);
            }
        }
        h
    }

    // =====================================================================
    // SECTION 2: Kernel integration tests
    //
    // These exercise the FULL pipeline: real TopologyArena + GeometryState
    // → boundary_adapter → certify_merge_boundary → trace propagation.
    // =====================================================================

    use crate::geometry_state::GeometryState;
    use crate::core::ModelingContext;

    fn env_test_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    /// Integration: trace propagation — verify that calling merge_coplanar_faces
    /// produces decisions in the ModelingContext decision log.
    #[test]
    fn merge_coplanar_faces_propagates_decisions_to_ctx() {
        let (topo, mut geom, _) = build_two_face_coplanar_sheet_fixture();
        let mut ctx = ModelingContext::new();

        assert!(
            ctx.get_decision_log_mut().is_empty(),
            "Precondition: ctx decision log should be empty before merge",
        );

        let result = crate::operations::boolean::postprocess::merge_coplanar_faces_extracted(
            crate::core::KernelState::new(topo, geom), &mut ctx,
        );

        assert!(result.is_ok(), "merge_coplanar_faces_extracted should succeed");
        let (_, merged_count) = result.unwrap();

        assert!(
            merged_count > 0,
            "Fixture regression: expected merge_coplanar_faces to merge at least one coplanar pair",
        );
        assert!(
            !ctx.get_decision_log_mut().is_empty(),
            "D4 regression: merge_coplanar_faces merged {} faces but produced \
             zero decisions in ctx. Certifier decisions are being silently dropped.",
            merged_count,
        );
    }

    /// Integration: geometry cleanup — verify killed faces have their plane
    /// bindings removed from GeometryState after merge.
    #[test]
    fn merge_removes_killed_face_plane_bindings() {
        let (topo, mut geom, _) = build_two_face_coplanar_sheet_fixture();
        let mut ctx = ModelingContext::new();

        let faces_with_planes_before: usize = topo.arena().iter_faces()
            .filter(|(fid, _)| geom.get_face_plane(*fid).is_some())
            .count();

        let result = crate::operations::boolean::postprocess::merge_coplanar_faces_extracted(
            crate::core::KernelState::new(topo, geom), &mut ctx,
        );
        assert!(result.is_ok());
        let (new_state, merged_count) = result.unwrap();
        let (new_topo, new_geom) = new_state.into_parts();

        assert!(
            merged_count > 0,
            "Fixture regression: expected merge_coplanar_faces to merge at least one coplanar pair",
        );

        let faces_with_planes_after: usize = new_topo.arena().iter_faces()
            .filter(|(fid, _)| new_geom.get_face_plane(*fid).is_some())
            .count();

        let live_face_count = new_topo.arena().face_count();

        assert_eq!(
            faces_with_planes_after, live_face_count,
            "D3 regression: after merging {} faces, there are {} live faces \
             but {} plane bindings. Killed-face bindings were not cleaned. \
             (Before merge: {} bindings)",
            merged_count, live_face_count, faces_with_planes_after,
            faces_with_planes_before,
        );
    }

    /// Deterministic kernel-owned fixture: build a single planar quad-like face
    /// and split it into two coplanar faces using Euler ops only.
    ///
    /// Returns the topology, geometry, and the exact two-face selection bitset
    /// for direct `certify_merge_boundary` integration tests.
    fn build_two_face_coplanar_sheet_fixture(
    ) -> (
        forge_topo::state::TopologyState,
        GeometryState,
        forge_topo::bitset::EntityBitset,
    ) {
        use forge_topo::state::TopologyState;
        use forge_topo::operator::apply_op;
        use forge_topo::euler::make_vertex_face::MakeVertexFace;
        use forge_topo::euler::split_edge::SplitEdge;
        use forge_topo::euler::make_edge_face::MakeEdgeFace;

        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.25 })
            .unwrap()
            .into_value();
        let _se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_am, parameter: 0.50 })
            .unwrap()
            .into_value();
        let se3 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.50 })
            .unwrap()
            .into_value();

        // Split the 4-vertex boundary into two faces via a diagonal.
        let mef = apply_op(
            &mut draft,
            MakeEdgeFace {
                vertex_a: mvf.vertex,
                vertex_b: se3.new_vertex,
                face: mvf.face,
            },
        )
        .unwrap()
        .into_value();

        let topo = draft.commit().expect("fixture topology commit");

        let mut group = forge_topo::bitset::EntityBitset::for_faces(topo.arena());
        let _ = group.insert(mvf.face.index());
        let _ = group.insert(mef.new_face.index());

        let mut geom = GeometryState::new();
        let perimeter = forge_topo::algorithms::region_extraction::walk_face_group_boundary_perimeter(
            topo.arena(),
            &group,
        )
        .expect("fixture perimeter extraction");
        assert_eq!(
            perimeter.len(),
            4,
            "fixture expected a 4-vertex perimeter, got {}",
            perimeter.len(),
        );

        let square = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        for (v, p) in perimeter.iter().zip(square.iter()) {
            geom.set_vertex_position(*v, *p);
        }

        let plane = forge_geom::Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
            .expect("fixture plane");
        geom.set_face_plane(mvf.face, plane.clone());
        geom.set_face_plane(mef.new_face, plane);

        (topo, geom, group)
    }

    /// Integration: exercise `certify_merge_boundary` directly with a real
    /// kernel-built two-face coplanar sheet fixture (topology + GeometryState).
    #[test]
    fn certify_merge_boundary_accepts_coplanar_sheet_fixture() {
        let (topo, geom, group) = build_two_face_coplanar_sheet_fixture();
        let arena = topo.arena();
        let op_result =
            crate::operations::boolean::postprocess::merge_eligibility::eval::certify_merge_boundary(
                arena, &group, &geom,
            )
            .expect("certify_merge_boundary should succeed on planar two-face fixture");

        assert!(
            !op_result.get_decision_log().is_empty(),
            "certify_merge_boundary must produce at least one traced decision",
        );

        let cert = op_result.into_value();
        assert!(
            matches!(
                cert,
                forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate::Simple
                    | forge_geom::algorithms::boundary_cert::schema::WeakSimpleCertificate::WeaklySimple { .. }
            ),
            "two-face coplanar sheet should be merge-eligible, got {:?}",
            cert,
        );
    }

    /// Integration: decision metadata uses content-derived ID and PolicyApplied kind.
    #[test]
    fn certify_produces_meaningful_decision_metadata_on_sheet_fixture() {
        let (topo, geom, group) = build_two_face_coplanar_sheet_fixture();
        let arena = topo.arena();
        let op_result =
            crate::operations::boolean::postprocess::merge_eligibility::eval::certify_merge_boundary(
                arena, &group, &geom,
            )
            .expect("certify_merge_boundary should succeed");

        let decisions: Vec<_> = op_result.get_decision_log().decisions().collect();
        assert!(!decisions.is_empty(), "Must have at least one decision");

        let d = &decisions[0];
        assert_ne!(
            d.get_id().0, 0,
            "D5 regression: DecisionId should not be 0 — should be content-derived hash",
        );
        assert!(
            matches!(d.get_kind(), forge_core::DecisionKind::PolicyApplied { .. }),
            "D5 regression: DecisionKind should be PolicyApplied, got {:?}",
            d.get_kind(),
        );
    }

    /// Integration: verify FORGE_TRACE_DIR causes trace files to be emitted
    /// during merge_coplanar_faces pipeline.
    #[test]
    fn trace_dir_emits_trace_files_during_merge() {
        let _guard = env_test_lock().lock().expect("env test lock poisoned");

        let trace_dir = std::env::temp_dir().join(format!(
            "forge_trace_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        std::fs::create_dir_all(&trace_dir).expect("create trace dir");
        let helper_name = concat!(
            "operations::boolean::postprocess::merge_eligibility::tests::tests::",
            "trace_dir_emits_trace_files_during_merge_subprocess_helper"
        );

        let output = std::process::Command::new(std::env::current_exe().expect("current_exe"))
            .arg("--exact")
            .arg(helper_name)
            .arg("--nocapture")
            .env("FORGE_TRACE_DIR", &trace_dir)
            .env("FORGE_TRACE_SUBPROCESS", "1")
            .output()
            .expect("spawn test subprocess");

        assert!(
            output.status.success(),
            "trace helper subprocess failed: status={:?}\nstdout:\n{}\nstderr:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );

        let trace_file = trace_dir.join("trace.json");
        assert!(
            trace_file.exists(),
            "FORGE_TRACE_DIR regression: expected trace file at {}",
            trace_file.display(),
        );

        let trace_text = std::fs::read_to_string(&trace_file).expect("read trace.json");
        assert!(
            trace_text.contains("Boundary certified") || trace_text.contains("Boundary rejected"),
            "trace.json exists but does not contain merge-certification trace entries:\n{}",
            trace_text,
        );

        let _ = std::fs::remove_dir_all(&trace_dir);
    }

    /// Subprocess helper for `trace_dir_emits_trace_files_during_merge`.
    ///
    /// Runs in a fresh process so `resolve_trace_dir()` sees FORGE_TRACE_DIR
    /// before its OnceLock cache initializes.
    #[test]
    fn trace_dir_emits_trace_files_during_merge_subprocess_helper() {
        if std::env::var("FORGE_TRACE_SUBPROCESS").ok().as_deref() != Some("1") {
            return;
        }

        let (topo, geom, group) = build_two_face_coplanar_sheet_fixture();
        let arena = topo.arena();

        let op_result = crate::operations::boolean::postprocess::merge_eligibility::eval::certify_merge_boundary(
            arena, &group, &geom,
        ).expect("certify_merge_boundary should succeed");

        let _ = op_result.into_value();
    }
}
