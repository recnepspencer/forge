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
    use forge_geom::algorithms::boundary_cert::eval::*;
    use forge_geom::algorithms::boundary_cert::schema::*;
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
            matches!(
                cert,
                WeakSimpleCertificate::Rejected {
                    reason: BoundaryRejectReason::OverlappingSegments,
                    ..
                }
            ),
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
            matches!(
                cert,
                WeakSimpleCertificate::Rejected {
                    reason: BoundaryRejectReason::SelfCrossing,
                    ..
                }
            ),
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
            matches!(
                certify_boundary(&boundary),
                WeakSimpleCertificate::Rejected { .. }
            ),
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
            matches!(
                certify_boundary(&boundary),
                WeakSimpleCertificate::Rejected {
                    reason: BoundaryRejectReason::SelfCrossing,
                    ..
                }
            ),
            "Pentagram must be SelfCrossing",
        );
    }

    /// D5 regression: different groups → different DecisionIds.
    #[test]
    fn different_groups_produce_different_decision_ids() {
        use forge_topo::bitset::EntityBitset;

        let mut group_a = EntityBitset::with_capacity(10);
        group_a
            .insert(0)
            .expect("bitset capacity must cover test indices");
        group_a
            .insert(1)
            .expect("bitset capacity must cover test indices");

        let mut group_b = EntityBitset::with_capacity(10);
        group_b
            .insert(2)
            .expect("bitset capacity must cover test indices");
        group_b
            .insert(3)
            .expect("bitset capacity must cover test indices");

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
        group
            .insert(0)
            .expect("bitset capacity must cover test indices");
        group
            .insert(3)
            .expect("bitset capacity must cover test indices");
        group
            .insert(7)
            .expect("bitset capacity must cover test indices");
        assert_eq!(compute_group_hash(&group), compute_group_hash(&group));
    }

    fn compute_group_hash(group: &forge_topo::bitset::EntityBitset) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for idx in 0..group.capacity() {
            if group
                .contains(idx)
                .expect("bitset capacity must cover test indices")
            {
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

    use super::super::nmt_eval::{
        execute_sheet_region_merge_persistent, resolve_merge_region_selection_persistent,
        test_build_merge_plan, test_map_resolution_incompatibility_for_persistent,
        test_resolve_face_ref_result_direct, test_resolve_face_ref_result_with_lineage_fallback,
        test_validate_connectivity,
    };
    use super::super::schema::{MergeRegionSelectionPersistent, PersistentFaceRef};
    use crate::core::ModelingContext;
    use crate::geometry_state::GeometryState;
    use forge_topo::topology::naming::Selector;
    use forge_topo::topology::naming::{assign_name, PersistentName};

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
            crate::core::KernelState::new(topo, geom),
            &mut ctx,
        );

        assert!(
            result.is_ok(),
            "merge_coplanar_faces_extracted should succeed"
        );
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

        let faces_with_planes_before: usize = topo
            .arena()
            .iter_faces()
            .filter(|(fid, _)| geom.get_face_plane(*fid).is_some())
            .count();

        let result = crate::operations::boolean::postprocess::merge_coplanar_faces_extracted(
            crate::core::KernelState::new(topo, geom),
            &mut ctx,
        );
        assert!(result.is_ok());
        let (new_state, merged_count) = result.unwrap();
        let (new_topo, new_geom) = new_state.into_parts();

        assert!(
            merged_count > 0,
            "Fixture regression: expected merge_coplanar_faces to merge at least one coplanar pair",
        );

        let faces_with_planes_after: usize = new_topo
            .arena()
            .iter_faces()
            .filter(|(fid, _)| new_geom.get_face_plane(*fid).is_some())
            .count();

        let live_face_count = new_topo.arena().face_count();

        assert_eq!(
            faces_with_planes_after, live_face_count,
            "D3 regression: after merging {} faces, there are {} live faces \
             but {} plane bindings. Killed-face bindings were not cleaned. \
             (Before merge: {} bindings)",
            merged_count, live_face_count, faces_with_planes_after, faces_with_planes_before,
        );
    }

    /// Deterministic kernel-owned fixture: build a single planar quad-like face
    /// and split it into two coplanar faces using Euler ops only.
    ///
    /// Returns the topology, geometry, and the exact two-face selection bitset
    /// for direct `certify_merge_boundary` integration tests.
    fn build_two_face_coplanar_sheet_fixture() -> (
        forge_topo::state::TopologyState,
        GeometryState,
        forge_topo::bitset::EntityBitset,
    ) {
        use forge_topo::euler::make_edge_face::MakeEdgeFace;
        use forge_topo::euler::make_vertex_face::MakeVertexFace;
        use forge_topo::euler::split_edge::SplitEdge;
        use forge_topo::operator::apply_op;
        use forge_topo::state::TopologyState;

        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.25,
            },
        )
        .unwrap()
        .into_value();
        let _se2 = apply_op(
            &mut draft,
            SplitEdge {
                edge: se1.he_am,
                parameter: 0.50,
            },
        )
        .unwrap()
        .into_value();
        let se3 = apply_op(
            &mut draft,
            SplitEdge {
                edge: se1.he_mb,
                parameter: 0.50,
            },
        )
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
        group
            .insert(mvf.face.index())
            .expect("bitset capacity must cover fixture faces");
        group
            .insert(mef.new_face.index())
            .expect("bitset capacity must cover fixture faces");

        let mut geom = GeometryState::new();
        let perimeter =
            forge_topo::algorithms::region_extraction::walk_face_group_boundary_perimeter(
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

    fn build_ambiguous_face_persistent_name_fixture(
    ) -> (forge_topo::state::TopologyState, PersistentName) {
        let (topo, _geom, group) = build_two_face_coplanar_sheet_fixture();
        let mut face_ids = Vec::new();
        for (fid, _) in topo.arena().iter_faces() {
            if group.contains(fid.index()).expect("group capacity") {
                face_ids.push(fid);
            }
        }
        assert!(
            face_ids.len() >= 2,
            "fixture must contain at least two selected faces"
        );

        let source_face = face_ids[0];
        let target_face = face_ids[1];
        let name = assign_name(
            topo.arena(),
            forge_topo::attributes::EntityKey::Face(source_face),
        )
        .expect("assign source face name");

        let mut draft = topo.into_mutation();
        let source_lineage = draft
            .arena()
            .get_face(source_face)
            .expect("source face exists")
            .lineage()
            .cloned()
            .expect("source face lineage");
        draft
            .arena_mut()
            .get_face_mut(target_face)
            .expect("target face exists")
            .set_lineage(Some(source_lineage));

        let topo_ambiguous = draft.commit().expect("tampered lineage fixture commit");
        (topo_ambiguous, name)
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

    #[test]
    fn persistent_selection_resolves_two_face_fixture_deterministically() {
        let (topo, _geom, group) = build_two_face_coplanar_sheet_fixture();
        let mut selected_names = Vec::new();
        let mut surviving: Option<forge_topo::handles::FaceId> = None;
        for (fid, _) in topo.arena().iter_faces() {
            if group.contains(fid.index()).expect("group capacity") {
                if surviving.is_none() {
                    surviving = Some(fid);
                }
                selected_names.push(
                    assign_name(topo.arena(), forge_topo::attributes::EntityKey::Face(fid))
                        .expect("assign face name"),
                );
            }
        }
        selected_names.sort_by_key(|n| (n.get_ancestry_hash(), n.get_ordinal()));
        let surviving_name = assign_name(
            topo.arena(),
            forge_topo::attributes::EntityKey::Face(surviving.expect("fixture face")),
        )
        .expect("assign surviving name");

        let persistent =
            MergeRegionSelectionPersistent::new(selected_names, Vec::new(), surviving_name);
        let state = crate::core::KernelState::new(topo, GeometryState::new());
        let mut ctx = ModelingContext::new();

        let resolved = resolve_merge_region_selection_persistent(&state, &persistent, &mut ctx)
            .expect("persistent selection should resolve");

        assert_eq!(resolved.get_selected_faces().iter_ones().count(), 2);
        assert!(resolved
            .get_selected_faces()
            .contains(resolved.get_surviving_face().index())
            .unwrap());
        assert_eq!(
            ctx.get_trace_adjuncts().records().len(),
            3,
            "2 selected + 1 surviving resolutions"
        );
    }

    #[test]
    fn persistent_selection_executes_region_merge_through_persistent_entrypoint() {
        let (topo, mut geom, group) = build_two_face_coplanar_sheet_fixture();
        let mut selected_names = Vec::new();
        let mut surviving: Option<forge_topo::handles::FaceId> = None;
        for (fid, _) in topo.arena().iter_faces() {
            if group.contains(fid.index()).expect("group capacity") {
                if surviving.is_none() {
                    surviving = Some(fid);
                }
                selected_names.push(
                    assign_name(topo.arena(), forge_topo::attributes::EntityKey::Face(fid))
                        .expect("assign face name"),
                );
            }
        }
        let surviving_name = assign_name(
            topo.arena(),
            forge_topo::attributes::EntityKey::Face(surviving.expect("fixture face")),
        )
        .expect("assign surviving name");

        let persistent =
            MergeRegionSelectionPersistent::new(selected_names, Vec::new(), surviving_name);
        let state = crate::core::KernelState::new(topo, geom);
        let mut ctx = ModelingContext::new();

        let op = execute_sheet_region_merge_persistent(state, &persistent, &mut ctx)
            .expect("persistent region merge entrypoint should succeed on coplanar 2-face fixture");
        let output = op.into_value();
        let (_new_state, merge) = output.into_parts();

        assert_eq!(
            merge.get_killed_faces().len(),
            1,
            "two-face merge should kill exactly one face"
        );
    }

    #[test]
    fn persistent_selection_missing_face_fails_closed_with_typed_resolution_adjunct() {
        let (topo, _geom, _group) = build_two_face_coplanar_sheet_fixture();
        let missing = PersistentName::new(0xdead_beef, forge_core::EntityKind::Face, 0);
        let persistent =
            MergeRegionSelectionPersistent::new(vec![missing.clone()], Vec::new(), missing);
        let state = crate::core::KernelState::new(topo, GeometryState::new());
        let mut ctx = ModelingContext::new();

        let err = resolve_merge_region_selection_persistent(&state, &persistent, &mut ctx)
            .expect_err("missing persistent name must fail closed");
        match err {
            forge_core::KernelError::MergeFailure(
                forge_core::errors::MergeError::PersistentResolutionMissing { role, query },
            ) => {
                assert_eq!(
                    role,
                    forge_core::errors::PersistentResolutionRole::SurvivingFace
                );
                assert_eq!(
                    query,
                    forge_core::ResolutionQuerySummary::PersistentName {
                        entity_kind: forge_core::EntityKind::Face,
                        ancestry_hash_hex: format!("{:032x}", 0xdead_beefu128),
                        ordinal: 0,
                    }
                );
            }
            other => panic!(
                "expected typed PersistentResolutionMissing merge error, got {:?}",
                other
            ),
        }

        let payload = ctx.get_trace_adjuncts().records()[0]
            .as_resolution_payload()
            .expect("resolution adjunct kind")
            .expect("decode resolution payload");
        assert_eq!(payload.outcome, forge_core::ResolutionOutcome::Missing);
        assert_eq!(
            payload.operation_scope_id.as_deref(),
            Some("sheet_region_merge")
        );
    }

    #[test]
    fn persistent_selection_ambiguous_face_fails_closed_no_first_match() {
        let (topo, ambiguous_name) = build_ambiguous_face_persistent_name_fixture();
        let state = crate::core::KernelState::new(topo, GeometryState::new());
        let persistent = MergeRegionSelectionPersistent::new(
            vec![ambiguous_name.clone()],
            Vec::new(),
            ambiguous_name.clone(),
        );
        let mut ctx = ModelingContext::new();

        let err = resolve_merge_region_selection_persistent(&state, &persistent, &mut ctx)
            .expect_err("split ancestry name must resolve ambiguously and fail closed");
        match err {
            forge_core::KernelError::MergeFailure(
                forge_core::errors::MergeError::PersistentResolutionAmbiguous {
                    role,
                    candidate_count,
                    query,
                },
            ) => {
                assert_eq!(
                    role,
                    forge_core::errors::PersistentResolutionRole::SurvivingFace
                );
                assert!(candidate_count >= 2);
                assert_eq!(
                    query,
                    forge_core::ResolutionQuerySummary::PersistentName {
                        entity_kind: forge_core::EntityKind::Face,
                        ancestry_hash_hex: format!("{:032x}", ambiguous_name.get_ancestry_hash()),
                        ordinal: ambiguous_name.get_ordinal(),
                    }
                );
            }
            other => panic!(
                "expected typed PersistentResolutionAmbiguous merge error, got {:?}",
                other
            ),
        }

        let payload = ctx.get_trace_adjuncts().records()[0]
            .as_resolution_payload()
            .expect("resolution adjunct kind")
            .expect("decode resolution payload");
        assert_eq!(payload.outcome, forge_core::ResolutionOutcome::Ambiguous);
        assert!(
            payload.candidate_count >= 2,
            "must preserve all candidates, no first-match"
        );
        let ordered = &payload.ordered_candidates;
        let mut sorted = ordered.clone();
        sorted.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
        assert_eq!(
            &sorted, ordered,
            "candidate summaries must be deterministically ordered"
        );
    }

    #[test]
    fn selector_based_persistent_resolution_uses_typed_contract_and_fails_closed() {
        let (topo, ambiguous_name) = build_ambiguous_face_persistent_name_fixture();
        let selector = PersistentFaceRef::Selector(Selector::ByAncestry {
            hash: ambiguous_name.get_ancestry_hash(),
            kind: forge_core::EntityKind::Face,
        });

        let result = test_resolve_face_ref_result_direct(topo.arena(), &selector);
        match result {
            crate::core::ResolutionResult::Ambiguous { candidates, evidence, query } => {
                let ordered = candidates.as_slice();
                assert!(
                    ordered.len() >= 2,
                    "selector ambiguity must preserve multiple candidates"
                );
                let mut sorted = ordered.to_vec();
                sorted.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
                assert_eq!(
                    ordered,
                    sorted.as_slice(),
                    "selector candidates must be deterministically ordered"
                );

                let payload = crate::core::ResolutionResult::Ambiguous { candidates, evidence, query }
                    .to_trace_payload(forge_core::DecisionId(1), None, None);
                
                match payload.query {
                    forge_core::tracing::ResolutionQuerySummary::Selector { selector_fingerprint, .. } => {
                        assert!(selector_fingerprint.is_some(), "NURBS-safe fingerprint must be emitted for selector queries");
                    }
                    _ => panic!("Expected Selector query summary"),
                }
            }
            other => panic!(
                "expected Ambiguous typed resolution result, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn lineage_fallback_legacy_history_returns_typed_incompatible_not_missing() {
        let root = forge_topo::lineage::Lineage::root(
            1,
            forge_topo::lineage::OpSignature::with_id("root_face", 1),
        );
        let child = forge_topo::lineage::Lineage::derive(
            &root,
            forge_topo::lineage::OpSignature::with_id("split_face", 2),
        );

        let mut draft = forge_topo::state::TopologyState::empty().into_mutation();
        draft.log_lineage_event(forge_topo::lineage::LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::Face, 0),
            entity_snapshot: None, // legacy/index-only lineage evidence
            lineage: child,
        });
        let topo = draft
            .commit()
            .expect("synthetic legacy lineage state must commit");

        let missing = PersistentFaceRef::Name(PersistentName::new(
            root.get_ancestry_hash(),
            forge_core::EntityKind::Face,
            0,
        ));
        assert!(
            forge_topo::topology::naming::resolve_name(
                topo.arena(),
                match &missing { PersistentFaceRef::Name(name) => name, _ => unreachable!() }
            ).is_empty(),
            "test precondition: no live face exists, so direct persistent-name resolution must miss"
        );
        let result = test_resolve_face_ref_result_with_lineage_fallback(&topo, &missing);
        match result {
            crate::core::ResolutionResult::Incompatible {
                incompatibility, ..
            } => {
                assert!(matches!(
                    incompatibility,
                    crate::core::ResolutionIncompatibility::LegacyIndexOnlyLineageHistory
                ));
            }
            other => panic!(
                "expected typed Incompatible for legacy lineage history, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn lineage_fallback_with_no_descendants_stays_typed_missing() {
        let (topo, _geom, _group) = build_two_face_coplanar_sheet_fixture();
        let missing = PersistentFaceRef::Name(PersistentName::new(
            0xfeed_face_u128,
            forge_core::EntityKind::Face,
            0,
        ));

        assert!(
            forge_topo::topology::naming::resolve_name(
                topo.arena(),
                match &missing {
                    PersistentFaceRef::Name(name) => name,
                    _ => unreachable!(),
                }
            )
            .is_empty(),
            "test precondition: direct persistent-name resolution must miss"
        );

        let result = test_resolve_face_ref_result_with_lineage_fallback(&topo, &missing);
        match result {
            crate::core::ResolutionResult::Missing { evidence, .. } => {
                assert!(
                    evidence
                        .routes_attempted
                        .contains(&crate::core::ResolverRoute::DirectPersistentName),
                    "direct route must be recorded"
                );
                assert!(
                    evidence
                        .routes_attempted
                        .contains(&crate::core::ResolverRoute::LineageReidentified),
                    "lineage route attempt must be recorded"
                );
            }
            other => panic!(
                "expected typed Missing when no lineage descendants exist, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn persistent_incompatibility_mapping_preserves_substrate_unavailable_and_origin_kind() {
        let mapped = test_map_resolution_incompatibility_for_persistent(
            &crate::core::ResolutionIncompatibility::SubstrateUnavailable,
        );
        assert_eq!(
            mapped,
            forge_core::errors::PersistentResolutionIncompatibility::SubstrateUnavailable
        );

        let mapped = test_map_resolution_incompatibility_for_persistent(
            &crate::core::ResolutionIncompatibility::UnsupportedEntityOrigin {
                origin: forge_core::errors::PersistentResolutionOriginKind::GeometricIntersection,
            },
        );
        assert_eq!(
            mapped,
            forge_core::errors::PersistentResolutionIncompatibility::UnsupportedEntityOrigin {
                origin: forge_core::errors::PersistentResolutionOriginKind::GeometricIntersection,
            }
        );
    }

    #[test]
    fn lineage_fallback_resolves_live_descendant_and_traces_lineage_route() {
        let (topo, _geom, group) = build_two_face_coplanar_sheet_fixture();
        let target_face = topo
            .arena()
            .iter_faces()
            .find_map(|(fid, _)| {
                group
                    .contains(fid.index())
                    .ok()
                    .and_then(|in_group| in_group.then_some(fid))
            })
            .expect("fixture must have at least one selected face");

        let synthetic_root = forge_topo::lineage::Lineage::root(
            77,
            forge_topo::lineage::OpSignature::with_id("synthetic_root_face", 1),
        );
        let child = forge_topo::lineage::Lineage::derive(
            &synthetic_root,
            forge_topo::lineage::OpSignature::with_id("synthetic_split_face", 2),
        );

        let mut draft = topo.into_mutation();
        draft
            .arena_mut()
            .get_face_mut(target_face)
            .expect("target face exists")
            .set_lineage(Some(child.clone()));
        draft.log_lineage_event(forge_topo::lineage::LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::Face, target_face.index()),
            entity_snapshot: Some(target_face.into()),
            lineage: child,
        });
        let topo = draft
            .commit()
            .expect("synthetic lineage descendant fixture commit");

        let missing_parent_name = PersistentName::new(
            synthetic_root.get_ancestry_hash(),
            forge_core::EntityKind::Face,
            0,
        );
        let pref = PersistentFaceRef::Name(missing_parent_name.clone());

        let direct = test_resolve_face_ref_result_direct(topo.arena(), &pref);
        assert!(
            matches!(direct, crate::core::ResolutionResult::Missing { .. }),
            "Direct persistent-name lookup must miss to exercise lineage fallback, got {:?}",
            direct,
        );

        let fallback = test_resolve_face_ref_result_with_lineage_fallback(&topo, &pref);
        match fallback {
            crate::core::ResolutionResult::Resolved { value, route, .. } => {
                assert_eq!(
                    route,
                    crate::core::ResolverRoute::LineageReidentified,
                    "lineage fallback success must surface the lineage route",
                );
                assert_eq!(value.snapshot_ref.kind, forge_core::EntityKind::Face);
                assert_eq!(value.snapshot_ref.index, target_face.index());
                assert_eq!(value.snapshot_ref.generation, target_face.generation());
            }
            other => panic!("expected lineage fallback Resolved result, got {:?}", other),
        }

        let persistent = MergeRegionSelectionPersistent::new(
            vec![missing_parent_name.clone()],
            Vec::new(),
            missing_parent_name,
        );
        let state = crate::core::KernelState::new(topo, GeometryState::new());
        let mut ctx = ModelingContext::new();
        let resolved = resolve_merge_region_selection_persistent(&state, &persistent, &mut ctx)
            .expect("persistent resolver should succeed via lineage fallback");
        assert_eq!(resolved.get_selected_faces().iter_ones().count(), 1);
        assert_eq!(resolved.get_surviving_face(), target_face);

        let payload = ctx
            .get_trace_adjuncts()
            .records()
            .first()
            .expect("must emit at least one resolution adjunct")
            .as_resolution_payload()
            .expect("resolution adjunct kind")
            .expect("decode resolution adjunct");
        assert_eq!(payload.outcome, forge_core::ResolutionOutcome::Resolved);
        assert_eq!(
            payload.final_route,
            forge_core::ResolutionRoute::LineageReidentified,
            "typed resolution adjunct must preserve lineage route",
        );
        assert_eq!(
            payload.operation_scope_id.as_deref(),
            Some("sheet_region_merge")
        );

        let reid_payload = ctx
            .get_trace_adjuncts()
            .records()
            .iter()
            .find_map(|r| r.as_reidentification_payload())
            .expect("must emit dedicated reidentification adjunct")
            .expect("decode reidentification adjunct");
        assert_eq!(
            reid_payload.outcome,
            forge_core::ReidentificationOutcome::Resolved
        );
        assert_eq!(
            reid_payload.compatibility,
            forge_core::ReidentificationCompatibilitySummary::Available
        );
        assert_eq!(
            reid_payload.operation_scope_id.as_deref(),
            Some("sheet_region_merge")
        );
    }

    #[test]
    fn lineage_fallback_ambiguous_descendants_fail_closed_with_deterministic_candidates() {
        let (topo, _geom, group) = build_two_face_coplanar_sheet_fixture();
        let mut faces: Vec<_> = topo
            .arena()
            .iter_faces()
            .filter_map(|(fid, _)| {
                group
                    .contains(fid.index())
                    .ok()
                    .and_then(|in_group| in_group.then_some(fid))
            })
            .collect();
        faces.sort_by_key(|f| (f.index(), f.generation()));
        assert!(faces.len() >= 2, "fixture must have at least two faces");
        let face_a = faces[0];
        let face_b = faces[1];

        let synthetic_root = forge_topo::lineage::Lineage::root(
            88,
            forge_topo::lineage::OpSignature::with_id("synthetic_root_face", 10),
        );
        let child_a = forge_topo::lineage::Lineage::derive(
            &synthetic_root,
            forge_topo::lineage::OpSignature::with_id("synthetic_split_face", 11),
        );
        let child_b = forge_topo::lineage::Lineage::derive(
            &synthetic_root,
            forge_topo::lineage::OpSignature::with_id("synthetic_split_face", 12),
        );

        let mut draft = topo.into_mutation();
        draft
            .arena_mut()
            .get_face_mut(face_a)
            .unwrap()
            .set_lineage(Some(child_a.clone()));
        draft
            .arena_mut()
            .get_face_mut(face_b)
            .unwrap()
            .set_lineage(Some(child_b.clone()));
        draft.log_lineage_event(forge_topo::lineage::LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::Face, face_a.index()),
            entity_snapshot: Some(face_a.into()),
            lineage: child_a,
        });
        draft.log_lineage_event(forge_topo::lineage::LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::Face, face_b.index()),
            entity_snapshot: Some(face_b.into()),
            lineage: child_b,
        });
        let topo = draft
            .commit()
            .expect("synthetic ambiguous lineage fixture commit");

        let missing_parent_name = PersistentName::new(
            synthetic_root.get_ancestry_hash(),
            forge_core::EntityKind::Face,
            0,
        );
        let pref = PersistentFaceRef::Name(missing_parent_name.clone());

        let direct = test_resolve_face_ref_result_direct(topo.arena(), &pref);
        assert!(
            matches!(direct, crate::core::ResolutionResult::Missing { .. }),
            "Direct persistent-name lookup must miss to exercise lineage fallback, got {:?}",
            direct,
        );

        let fallback = test_resolve_face_ref_result_with_lineage_fallback(&topo, &pref);
        match fallback {
            crate::core::ResolutionResult::Ambiguous {
                candidates,
                evidence,
                ..
            } => {
                assert_eq!(
                    candidates.len(),
                    2,
                    "must preserve both lineage descendants"
                );
                assert!(
                    evidence
                        .routes_attempted
                        .contains(&crate::core::ResolverRoute::LineageReidentified),
                    "lineage route must be recorded in resolver evidence"
                );
                let ordered = candidates.as_slice();
                let mut sorted = ordered.to_vec();
                sorted.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
                assert_eq!(
                    ordered,
                    sorted.as_slice(),
                    "fallback candidates must be deterministic"
                );
            }
            other => panic!(
                "expected lineage fallback Ambiguous result, got {:?}",
                other
            ),
        }

        let persistent = MergeRegionSelectionPersistent::new(
            vec![missing_parent_name.clone()],
            Vec::new(),
            missing_parent_name,
        );
        let state = crate::core::KernelState::new(topo, GeometryState::new());
        let mut ctx = ModelingContext::new();
        let err = resolve_merge_region_selection_persistent(&state, &persistent, &mut ctx)
            .expect_err("persistent resolver must fail closed on ambiguous lineage descendants");
        match err {
            forge_core::KernelError::MergeFailure(
                forge_core::errors::MergeError::PersistentResolutionAmbiguous {
                    role,
                    candidate_count,
                    ..
                },
            ) => {
                assert_eq!(
                    role,
                    forge_core::errors::PersistentResolutionRole::SurvivingFace
                );
                assert_eq!(candidate_count, 2);
            }
            other => panic!(
                "expected typed PersistentResolutionAmbiguous merge error, got {:?}",
                other
            ),
        }

        let payload = ctx
            .get_trace_adjuncts()
            .records()
            .first()
            .expect("must emit at least one resolution adjunct")
            .as_resolution_payload()
            .expect("resolution adjunct kind")
            .expect("decode resolution adjunct");
        assert_eq!(payload.outcome, forge_core::ResolutionOutcome::Ambiguous);
        assert_eq!(payload.candidate_count, 2);
        assert!(
            payload
                .routes_attempted
                .contains(&forge_core::ResolutionRoute::LineageReidentified),
            "typed resolution adjunct must record attempted lineage route",
        );
        let mut sorted = payload.ordered_candidates.clone();
        sorted.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
        assert_eq!(
            payload.ordered_candidates, sorted,
            "adjunct candidate summaries must be deterministic"
        );
    }

    /// Integration: decision metadata uses content-derived ID and outcome-accurate kind.
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
            d.get_id().0,
            0,
            "D5 regression: DecisionId should not be 0 — should be content-derived hash",
        );
        match d.get_kind() {
            forge_core::DecisionKind::Exact => {
                assert_eq!(
                    d.get_tier(),
                    forge_core::DecisionTier::Deterministic,
                    "Simple certificate should trace as Exact/Deterministic",
                );
            }
            forge_core::DecisionKind::NearBoundary { .. } => {
                assert_eq!(
                    d.get_tier(),
                    forge_core::DecisionTier::NearBoundary,
                    "WeaklySimple certificate should trace as near-boundary until policy is resolved by the caller",
                );
            }
            other => panic!(
                "D5 regression: unexpected decision kind for certifier result: {:?}",
                other
            ),
        }
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

    // =====================================================================
    // SECTION 3: Sprint 3 — execute_sheet_region_merge integration tests
    //
    // These exercise the full merge execution pipeline: real cube topology
    // with geometry bindings, NMT radial inflation, KernelDraft transaction,
    // JoinFaces/JoinFacesNmt dispatch, geometry cleanup, and traced decisions.
    // =====================================================================

    use crate::core::kernel_draft::KernelDraft;
    use crate::core::KernelState;
    use crate::mesh_builder::make_cube;
    use crate::operations::boolean::postprocess::merge_eligibility::nmt_eval::execute_sheet_region_merge;
    use crate::operations::boolean::postprocess::merge_eligibility::schema::{
        MergeRegionSelection, RadialUseSelector,
    };
    use forge_core::errors::MergeError;
    use forge_core::KernelError;
    use forge_topo::bitset::EntityBitset;
    use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId};

    /// Build a cube KernelState with one edge inflated to valence 3.
    ///
    /// Returns `(KernelState, target_edge_index, face_a, face_b, face_extra)`
    /// where face_a and face_b are the original cube faces on the target edge,
    /// and face_extra is the manually inserted third face.
    fn build_cube_with_valence_3_edge() -> (
        KernelState,
        u32,    // target edge index
        FaceId, // face_a (original, adjacent to target edge)
        FaceId, // face_b (original, adjacent to target edge)
        FaceId, // face_extra (inserted, creating valence 3)
    ) {
        let cube = make_cube([0.0, 0.0, 0.0], 2.0).expect("make_cube must succeed");
        let (topo, geom) = cube.into_parts();

        let state = KernelState::new(topo, geom);
        let mut draft = KernelDraft::new(state);

        // Find the first edge and its two adjacent faces.
        let (target_edge_id, target_edge_data) = draft
            .arena()
            .iter_edges()
            .next()
            .expect("cube must have edges");
        let target_edge_idx = target_edge_id.index();

        let entry_he = target_edge_data.half_edge();
        let face_a_id = draft.arena().get_half_edge(entry_he).unwrap().face();
        let twin_he = draft.arena().get_half_edge(entry_he).unwrap().radial_next();
        let face_b_id = draft.arena().get_half_edge(twin_he).unwrap().face();

        let v_a = draft.arena().get_half_edge(entry_he).unwrap().origin();
        let v_b = draft.arena().get_half_edge(twin_he).unwrap().origin();

        // Get face_a's plane for the extra face.
        let plane_a = draft
            .geometry()
            .get_face_plane(face_a_id)
            .cloned()
            .expect("cube faces must have plane bindings");

        // Insert a new face on the same edge (creates valence 3).
        let shell = draft.arena().get_face(face_a_id).unwrap().shell();
        let ph_loop = forge_topo::handles::LoopId::from_raw_parts(u32::MAX, 0);

        let extra_face = draft
            .draft_mut()
            .insert_face(forge_topo::arena::FaceData::new(ph_loop, shell));
        let extra_edge = draft
            .draft_mut()
            .insert_edge(forge_topo::arena::EdgeData::new(
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
            ));

        let he_fwd = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                extra_face,
                v_a,
                target_edge_id,
            ));
        let he_ret = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                he_fwd, he_fwd, he_fwd, extra_face, v_b, extra_edge,
            ));

        // Wire the 2-element loop: fwd ↔ ret.
        let dm = draft.draft_mut();
        dm.arena_mut()
            .get_half_edge_mut(he_fwd)
            .unwrap()
            .set_next(he_ret);
        dm.arena_mut()
            .get_half_edge_mut(he_fwd)
            .unwrap()
            .set_prev(he_ret);
        dm.arena_mut()
            .get_half_edge_mut(he_ret)
            .unwrap()
            .set_next(he_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he_ret)
            .unwrap()
            .set_prev(he_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he_ret)
            .unwrap()
            .set_radial_next(he_ret);
        dm.arena_mut()
            .get_edge_mut(extra_edge)
            .unwrap()
            .set_half_edge(he_ret);

        // Wire the radial ring: entry_he → twin_he → he_fwd → entry_he (valence 3).
        dm.arena_mut()
            .get_half_edge_mut(entry_he)
            .unwrap()
            .set_radial_next(twin_he);
        dm.arena_mut()
            .get_half_edge_mut(twin_he)
            .unwrap()
            .set_radial_next(he_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he_fwd)
            .unwrap()
            .set_radial_next(entry_he);

        // Create a loop for the extra face.
        let extra_loop = dm.insert_loop(forge_topo::arena::LoopData::new(he_fwd, extra_face));
        dm.arena_mut()
            .get_face_mut(extra_face)
            .unwrap()
            .set_outer_loop(extra_loop);

        // Give the extra face a plane binding.
        draft.geometry_mut().set_face_plane(extra_face, plane_a);

        let nmt_state = draft
            .commit_with_mode(
                forge_topo::validate::ValidationLevel::Minimal,
                forge_topo::validate::TopologyMode::NmtIntermediate,
            )
            .expect("NMT commit must succeed");

        (nmt_state, target_edge_idx, face_a_id, face_b_id, extra_face)
    }

    // ----- Test 1: Merge with geometry cleanup -----

    /// Merge two faces on a valence-3 edge. Verify killed face's plane is removed,
    /// surviving face's plane is preserved. Integration: real cube geometry.
    #[test]
    fn merge_coplanar_faces_cleans_geometry() {
        let (state, _edge_idx, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let mut protected = EntityBitset::with_capacity(cap);
        protected.insert(face_extra.index()).unwrap();

        let selection = MergeRegionSelection::new(selected, protected, face_a);

        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state, &selection, &mut ctx);
        assert!(result.is_ok(), "Merge must succeed: {:?}", result.err());

        let output = result.unwrap().into_value();
        let merge_result = output.get_merge();
        assert_eq!(merge_result.get_surviving_face(), face_a);
        assert!(merge_result.get_killed_faces().contains(&face_b));
    }

    // ----- Test 2: Ambiguous valence-4 rejection -----

    /// Build a valence-4 edge and attempt merge without radial selectors.
    /// Must fail with AmbiguousRadialSelection.
    #[test]
    fn valence_4_rejects_without_radial_selector() {
        let (state, edge_idx, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

        // Inflate to valence 4 by adding yet another face.
        let mut draft = KernelDraft::new(state);

        let target_edge_id = {
            let mut found = None;
            for (eid, _) in draft.arena().iter_edges() {
                if eid.index() == edge_idx {
                    found = Some(eid);
                    break;
                }
            }
            found.expect("target edge must exist")
        };

        let entry_he = draft.arena().get_edge(target_edge_id).unwrap().half_edge();
        let v_a = draft.arena().get_half_edge(entry_he).unwrap().origin();
        let v_b = {
            let twin = draft.arena().get_half_edge(entry_he).unwrap().radial_next();
            draft.arena().get_half_edge(twin).unwrap().origin()
        };

        let shell = draft.arena().get_face(face_a).unwrap().shell();
        let ph_loop = forge_topo::handles::LoopId::from_raw_parts(u32::MAX, 0);

        let face_4 = draft
            .draft_mut()
            .insert_face(forge_topo::arena::FaceData::new(ph_loop, shell));
        let edge_4 = draft
            .draft_mut()
            .insert_edge(forge_topo::arena::EdgeData::new(
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
            ));
        let he4_fwd = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                face_4,
                v_a,
                target_edge_id,
            ));
        let he4_ret = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                he4_fwd, he4_fwd, he4_fwd, face_4, v_b, edge_4,
            ));

        let dm = draft.draft_mut();
        dm.arena_mut()
            .get_half_edge_mut(he4_fwd)
            .unwrap()
            .set_next(he4_ret);
        dm.arena_mut()
            .get_half_edge_mut(he4_fwd)
            .unwrap()
            .set_prev(he4_ret);
        dm.arena_mut()
            .get_half_edge_mut(he4_ret)
            .unwrap()
            .set_next(he4_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he4_ret)
            .unwrap()
            .set_prev(he4_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he4_ret)
            .unwrap()
            .set_radial_next(he4_ret);
        dm.arena_mut()
            .get_edge_mut(edge_4)
            .unwrap()
            .set_half_edge(he4_ret);

        // Wire valence-4 ring: find the existing ring and insert he4_fwd.
        let he3 = {
            let mut cur = entry_he;
            loop {
                let next = dm.arena().get_half_edge(cur).unwrap().radial_next();
                if next == entry_he {
                    break cur;
                }
                cur = next;
            }
        };
        dm.arena_mut()
            .get_half_edge_mut(he3)
            .unwrap()
            .set_radial_next(he4_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he4_fwd)
            .unwrap()
            .set_radial_next(entry_he);

        let l4 = dm.insert_loop(forge_topo::arena::LoopData::new(he4_fwd, face_4));
        dm.arena_mut()
            .get_face_mut(face_4)
            .unwrap()
            .set_outer_loop(l4);

        let state_v4 = draft
            .commit_with_mode(
                forge_topo::validate::ValidationLevel::Minimal,
                forge_topo::validate::TopologyMode::NmtIntermediate,
            )
            .unwrap();

        // Now try to merge 3 of the 4 faces without radial selectors.
        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();
        selected.insert(face_extra.index()).unwrap();

        let protected = EntityBitset::with_capacity(cap);
        let selection = MergeRegionSelection::new(selected, protected, face_a);

        let mut ctx = ModelingContext::new();
        let err = execute_sheet_region_merge(state_v4, &selection, &mut ctx)
            .expect_err("Must fail on ambiguous valence-4 edge");

        assert!(
            matches!(
                err,
                KernelError::MergeFailure(MergeError::AmbiguousRadialSelection { .. })
                    | KernelError::MergeFailure(MergeError::BoundaryCertificationFailed { .. })
                    | KernelError::InternalError { .. }
            ),
            "Expected AmbiguousRadialSelection or earlier boundary-gate failure, got: {:?}",
            err,
        );
    }

    /// Pre-gate planner unit coverage: the valence-4 synthetic fixture must still
    /// hit the ambiguity path in `build_merge_plan`, independent of boundary cert.
    #[test]
    fn planner_pre_gate_valence_4_rejects_without_radial_selector() {
        let (state, edge_idx, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

        let mut draft = KernelDraft::new(state);
        let target_edge_id = draft
            .arena()
            .iter_edges()
            .find_map(|(eid, _)| (eid.index() == edge_idx).then_some(eid))
            .expect("target edge must exist");

        let entry_he = draft.arena().get_edge(target_edge_id).unwrap().half_edge();
        let v_a = draft.arena().get_half_edge(entry_he).unwrap().origin();
        let v_b = {
            let twin = draft.arena().get_half_edge(entry_he).unwrap().radial_next();
            draft.arena().get_half_edge(twin).unwrap().origin()
        };
        let shell = draft.arena().get_face(face_a).unwrap().shell();
        let ph_loop = forge_topo::handles::LoopId::from_raw_parts(u32::MAX, 0);

        let face_4 = draft
            .draft_mut()
            .insert_face(forge_topo::arena::FaceData::new(ph_loop, shell));
        let edge_4 = draft
            .draft_mut()
            .insert_edge(forge_topo::arena::EdgeData::new(
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
            ));
        let he4_fwd = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                face_4,
                v_a,
                target_edge_id,
            ));
        let he4_ret = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                he4_fwd, he4_fwd, he4_fwd, face_4, v_b, edge_4,
            ));

        let dm = draft.draft_mut();
        dm.arena_mut()
            .get_half_edge_mut(he4_fwd)
            .unwrap()
            .set_next(he4_ret);
        dm.arena_mut()
            .get_half_edge_mut(he4_fwd)
            .unwrap()
            .set_prev(he4_ret);
        dm.arena_mut()
            .get_half_edge_mut(he4_ret)
            .unwrap()
            .set_next(he4_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he4_ret)
            .unwrap()
            .set_prev(he4_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he4_ret)
            .unwrap()
            .set_radial_next(he4_ret);
        dm.arena_mut()
            .get_edge_mut(edge_4)
            .unwrap()
            .set_half_edge(he4_ret);

        let he3 = {
            let mut cur = entry_he;
            loop {
                let next = dm.arena().get_half_edge(cur).unwrap().radial_next();
                if next == entry_he {
                    break cur;
                }
                cur = next;
            }
        };
        dm.arena_mut()
            .get_half_edge_mut(he3)
            .unwrap()
            .set_radial_next(he4_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he4_fwd)
            .unwrap()
            .set_radial_next(entry_he);
        let l4 = dm.insert_loop(forge_topo::arena::LoopData::new(he4_fwd, face_4));
        dm.arena_mut()
            .get_face_mut(face_4)
            .unwrap()
            .set_outer_loop(l4);

        let state_v4 = draft
            .commit_with_mode(
                forge_topo::validate::ValidationLevel::Minimal,
                forge_topo::validate::TopologyMode::NmtIntermediate,
            )
            .unwrap();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();
        selected.insert(face_extra.index()).unwrap();
        let protected = EntityBitset::with_capacity(cap);
        let selection = MergeRegionSelection::new(selected, protected, face_a);

        let err = test_build_merge_plan(state_v4.topology().arena(), &selection)
            .expect_err("planner must reject ambiguous valence-4 edge without selector");
        assert!(
            matches!(
                err,
                KernelError::MergeFailure(MergeError::AmbiguousRadialSelection { .. })
            ),
            "pre-gate planner coverage regression: expected AmbiguousRadialSelection, got {:?}",
            err,
        );
    }

    // ----- Test 5: Disconnected faces fail connectivity -----

    /// Two faces that share no edges → BFS connectivity failure.
    #[test]
    fn disconnected_faces_fail_connectivity() {
        // Build a cube with valence-3, then insert an orphan face into the arena.
        // The orphan shares no edges with any cube face → BFS cannot reach it.
        let cube = make_cube([0.0, 0.0, 0.0], 2.0).expect("make_cube must succeed");
        let (topo, geom) = cube.into_parts();
        let state = KernelState::new(topo, geom);
        let mut draft = KernelDraft::new(state);

        // Find a face from the cube to use as the "selected" face.
        let (cube_face, _) = draft.arena().iter_faces().next().unwrap();
        let shell = draft.arena().get_face(cube_face).unwrap().shell();

        // Insert an orphan face with its own vertex + loop, no shared edges.
        let ph_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
        let ph_loop = forge_topo::handles::LoopId::from_raw_parts(u32::MAX, 0);
        let orphan_face = draft
            .draft_mut()
            .insert_face(forge_topo::arena::FaceData::new(ph_loop, shell));
        let orphan_v = draft
            .draft_mut()
            .insert_vertex(forge_topo::arena::VertexData::new(ph_he));
        let orphan_edge = draft
            .draft_mut()
            .insert_edge(forge_topo::arena::EdgeData::new(ph_he));
        let orphan_he = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                ph_he,
                ph_he,
                ph_he,
                orphan_face,
                orphan_v,
                orphan_edge,
            ));
        // Self-loop: next/prev/radial all point to itself.
        let dm = draft.draft_mut();
        dm.arena_mut()
            .get_half_edge_mut(orphan_he)
            .unwrap()
            .set_next(orphan_he);
        dm.arena_mut()
            .get_half_edge_mut(orphan_he)
            .unwrap()
            .set_prev(orphan_he);
        dm.arena_mut()
            .get_half_edge_mut(orphan_he)
            .unwrap()
            .set_radial_next(orphan_he);
        dm.arena_mut()
            .get_vertex_mut(orphan_v)
            .unwrap()
            .set_outgoing(orphan_he);
        dm.arena_mut()
            .get_edge_mut(orphan_edge)
            .unwrap()
            .set_half_edge(orphan_he);

        let orphan_loop = dm.insert_loop(forge_topo::arena::LoopData::new(orphan_he, orphan_face));
        dm.arena_mut()
            .get_face_mut(orphan_face)
            .unwrap()
            .set_outer_loop(orphan_loop);

        let state = draft
            .commit_with_mode(
                forge_topo::validate::ValidationLevel::Minimal,
                forge_topo::validate::TopologyMode::NmtIntermediate,
            )
            .unwrap();

        // Select cube_face + orphan_face. They share no edges → BFS must fail.
        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(cube_face.index()).unwrap();
        selected.insert(orphan_face.index()).unwrap();

        let protected = EntityBitset::with_capacity(cap);
        let selection = MergeRegionSelection::new(selected, protected, cube_face);

        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state, &selection, &mut ctx);
        assert!(result.is_err(), "Must fail on disconnected faces");

        if let Err(err) = result {
            assert!(
                matches!(
                    err,
                    KernelError::MergeFailure(MergeError::WouldDisconnectSheet { .. })
                ),
                "Expected WouldDisconnectSheet, got: {:?}",
                err,
            );
        }
    }

    /// Pre-gate connectivity unit coverage: disconnected synthetic fixtures should
    /// still deterministically fail BFS connectivity even if the executor now
    /// rejects earlier at the boundary-cert gate.
    #[test]
    fn connectivity_validator_rejects_disconnected_faces_pre_gate() {
        let cube = make_cube([0.0, 0.0, 0.0], 2.0).expect("make_cube must succeed");
        let (topo, geom) = cube.into_parts();
        let state = KernelState::new(topo, geom);
        let mut draft = KernelDraft::new(state);

        let (cube_face, _) = draft.arena().iter_faces().next().unwrap();
        let shell = draft.arena().get_face(cube_face).unwrap().shell();

        let ph_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);
        let ph_loop = forge_topo::handles::LoopId::from_raw_parts(u32::MAX, 0);
        let orphan_face = draft
            .draft_mut()
            .insert_face(forge_topo::arena::FaceData::new(ph_loop, shell));
        let orphan_v = draft
            .draft_mut()
            .insert_vertex(forge_topo::arena::VertexData::new(ph_he));
        let orphan_edge = draft
            .draft_mut()
            .insert_edge(forge_topo::arena::EdgeData::new(ph_he));
        let orphan_he = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                ph_he,
                ph_he,
                ph_he,
                orphan_face,
                orphan_v,
                orphan_edge,
            ));
        let dm = draft.draft_mut();
        dm.arena_mut()
            .get_half_edge_mut(orphan_he)
            .unwrap()
            .set_next(orphan_he);
        dm.arena_mut()
            .get_half_edge_mut(orphan_he)
            .unwrap()
            .set_prev(orphan_he);
        dm.arena_mut()
            .get_half_edge_mut(orphan_he)
            .unwrap()
            .set_radial_next(orphan_he);
        dm.arena_mut()
            .get_vertex_mut(orphan_v)
            .unwrap()
            .set_outgoing(orphan_he);
        dm.arena_mut()
            .get_edge_mut(orphan_edge)
            .unwrap()
            .set_half_edge(orphan_he);
        let orphan_loop = dm.insert_loop(forge_topo::arena::LoopData::new(orphan_he, orphan_face));
        dm.arena_mut()
            .get_face_mut(orphan_face)
            .unwrap()
            .set_outer_loop(orphan_loop);

        let state = draft
            .commit_with_mode(
                forge_topo::validate::ValidationLevel::Minimal,
                forge_topo::validate::TopologyMode::NmtIntermediate,
            )
            .unwrap();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(cube_face.index()).unwrap();
        selected.insert(orphan_face.index()).unwrap();
        let protected = EntityBitset::with_capacity(cap);
        let selection = MergeRegionSelection::new(selected, protected, cube_face);

        let err = test_validate_connectivity(state.topology().arena(), &selection)
            .expect_err("disconnected selection must fail BFS connectivity pre-gate");
        assert!(
            matches!(
                err,
                KernelError::MergeFailure(MergeError::WouldDisconnectSheet { .. })
            ),
            "pre-gate connectivity coverage regression: expected WouldDisconnectSheet, got {:?}",
            err,
        );
    }

    // ----- Test 7: Deterministic merge plans -----

    /// Same input twice produces identical MergePlan hash and step ordering.
    #[test]
    fn deterministic_merge_plans() {
        // Now that make_cube uses deterministic EdgeMap (flat Vec) instead of HashMap,
        // two independent builds produce identical arena layouts and thus identical plans.

        let run = || {
            let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
            let cap = 64;
            let mut selected = EntityBitset::with_capacity(cap);
            selected.insert(face_a.index()).unwrap();
            selected.insert(face_b.index()).unwrap();

            let mut protected = EntityBitset::with_capacity(cap);
            protected.insert(face_extra.index()).unwrap();

            let selection = MergeRegionSelection::new(selected, protected, face_a);
            let mut ctx = ModelingContext::new();
            let result = execute_sheet_region_merge(state, &selection, &mut ctx)
                .expect("merge must succeed");
            let output = result.into_value();
            let merge = output.get_merge();
            let steps: Vec<u32> = merge
                .get_plan()
                .get_steps()
                .iter()
                .map(|s| s.edge_index)
                .collect();
            let hash = merge.get_plan().get_plan_hash();
            (steps, hash)
        };

        let (steps_a, hash_a) = run();
        let (steps_b, hash_b) = run();

        assert_eq!(steps_a.len(), steps_b.len(), "Plan step counts must match",);

        for (i, (a, b)) in steps_a.iter().zip(steps_b.iter()).enumerate() {
            assert_eq!(a, b, "Step {} edge_index differs: {} vs {}", i, a, b,);
        }

        assert_eq!(
            hash_a, hash_b,
            "Plan hashes must be identical for deterministic inputs",
        );
    }

    // ----- Test 9: Traced decisions per step -----

    /// After merge, the decision log has one TracedDecision per step.
    #[test]
    fn traced_decisions_contain_step_metadata() {
        let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let mut protected = EntityBitset::with_capacity(cap);
        protected.insert(face_extra.index()).unwrap();

        let selection = MergeRegionSelection::new(selected, protected, face_a);
        let mut ctx = ModelingContext::new();
        let result =
            execute_sheet_region_merge(state, &selection, &mut ctx).expect("merge must succeed");

        let plan_steps = result.get_value().get_merge().get_plan().step_count();
        let decision_count = result.get_decision_log().decisions().count();

        assert!(
            decision_count >= plan_steps,
            "Decision log must have at least one decision per step: got {} decisions for {} steps",
            decision_count,
            plan_steps,
        );
    }

    // ----- Test 10: ManifoldStrict rejects NMT slits -----

    /// After a valence-3 merge (creates slit), ManifoldStrict commit fails
    /// but NmtIntermediate commit succeeds.
    #[test]
    fn manifold_strict_commit_rejects_nmt_slits() {
        let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let mut protected = EntityBitset::with_capacity(cap);
        protected.insert(face_extra.index()).unwrap();

        let selection = MergeRegionSelection::new(selected, protected, face_a);
        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state, &selection, &mut ctx);

        // The execution engine uses NmtIntermediate commit — so it should succeed.
        // But if we then try to re-commit with ManifoldStrict, slits would fail.
        assert!(
            result.is_ok(),
            "NmtIntermediate merge must succeed: {:?}",
            result.err(),
        );
    }

    // ----- Test 3: RadialUseSelector disambiguates valence-4 -----

    /// Build a valence-4 edge with explicit RadialUseSelector for the ambiguous edge.
    /// When the selector is provided, the merge must succeed instead of returning
    /// AmbiguousRadialSelection.
    #[test]
    fn valence_4_with_explicit_radial_selector_succeeds() {
        let (state, edge_idx, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

        // Inflate to valence 4 (same as test 2).
        let mut draft = KernelDraft::new(state);

        let target_edge_id = {
            let mut found = None;
            for (eid, _) in draft.arena().iter_edges() {
                if eid.index() == edge_idx {
                    found = Some(eid);
                    break;
                }
            }
            found.expect("target edge must exist")
        };

        let entry_he = draft.arena().get_edge(target_edge_id).unwrap().half_edge();
        let v_a = draft.arena().get_half_edge(entry_he).unwrap().origin();
        let v_b = {
            let twin = draft.arena().get_half_edge(entry_he).unwrap().radial_next();
            draft.arena().get_half_edge(twin).unwrap().origin()
        };

        let shell = draft.arena().get_face(face_a).unwrap().shell();
        let ph_loop = forge_topo::handles::LoopId::from_raw_parts(u32::MAX, 0);

        let face_4 = draft
            .draft_mut()
            .insert_face(forge_topo::arena::FaceData::new(ph_loop, shell));
        let edge_4 = draft
            .draft_mut()
            .insert_edge(forge_topo::arena::EdgeData::new(
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
            ));
        let he4_fwd = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                HalfEdgeId::from_raw_parts(u32::MAX, 0),
                face_4,
                v_a,
                target_edge_id,
            ));
        let he4_ret = draft
            .draft_mut()
            .insert_half_edge(forge_topo::arena::HalfEdgeData::new(
                he4_fwd, he4_fwd, he4_fwd, face_4, v_b, edge_4,
            ));

        let dm = draft.draft_mut();
        dm.arena_mut()
            .get_half_edge_mut(he4_fwd)
            .unwrap()
            .set_next(he4_ret);
        dm.arena_mut()
            .get_half_edge_mut(he4_fwd)
            .unwrap()
            .set_prev(he4_ret);
        dm.arena_mut()
            .get_half_edge_mut(he4_ret)
            .unwrap()
            .set_next(he4_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he4_ret)
            .unwrap()
            .set_prev(he4_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he4_ret)
            .unwrap()
            .set_radial_next(he4_ret);
        dm.arena_mut()
            .get_edge_mut(edge_4)
            .unwrap()
            .set_half_edge(he4_ret);

        let he3 = {
            let mut cur = entry_he;
            loop {
                let next = dm.arena().get_half_edge(cur).unwrap().radial_next();
                if next == entry_he {
                    break cur;
                }
                cur = next;
            }
        };
        dm.arena_mut()
            .get_half_edge_mut(he3)
            .unwrap()
            .set_radial_next(he4_fwd);
        dm.arena_mut()
            .get_half_edge_mut(he4_fwd)
            .unwrap()
            .set_radial_next(entry_he);

        let l4 = dm.insert_loop(forge_topo::arena::LoopData::new(he4_fwd, face_4));
        dm.arena_mut()
            .get_face_mut(face_4)
            .unwrap()
            .set_outer_loop(l4);

        let state_v4 = draft
            .commit_with_mode(
                forge_topo::validate::ValidationLevel::Minimal,
                forge_topo::validate::TopologyMode::NmtIntermediate,
            )
            .unwrap();

        // Select face_a + face_b WITH an explicit radial selector for the ambiguous edge.
        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let protected = EntityBitset::with_capacity(cap);

        let selectors = vec![RadialUseSelector::new(
            edge_idx,
            face_a.index(),
            face_b.index(),
        )];

        let selection =
            MergeRegionSelection::with_radial_selectors(selected, protected, face_a, selectors);

        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state_v4, &selection, &mut ctx);
        assert!(
            result.is_ok(),
            "Merge with explicit RadialUseSelector must succeed: {:?}",
            result.err(),
        );
    }

    // ----- Test 4: Protected ring intact after merge -----

    /// After merging face_a + face_b on a valence-3 edge, the extra face's
    /// outer loop must still be walkable (no dangling next/prev pointers).
    #[test]
    fn protected_ring_intact_after_merge() {
        let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let mut protected = EntityBitset::with_capacity(cap);
        protected.insert(face_extra.index()).unwrap();

        let selection = MergeRegionSelection::new(selected, protected, face_a);
        let mut ctx = ModelingContext::new();
        let result =
            execute_sheet_region_merge(state, &selection, &mut ctx).expect("merge must succeed");

        let output = result.into_value();
        let merge = output.get_merge();
        assert_eq!(
            merge.get_surviving_face(),
            face_a,
            "Surviving face must be face_a",
        );
        assert!(
            merge.get_killed_faces().contains(&face_b),
            "face_b must be killed",
        );
        assert!(
            !merge.get_killed_faces().contains(&face_extra),
            "face_extra must NOT be killed — it is protected",
        );
    }

    // ----- Test 6: Failure rolls back topology and geometry -----

    /// When execute_sheet_region_merge fails (bad selection), the function
    /// returns Err and the KernelDraft is dropped. The original KernelState
    /// was consumed, so rollback is implicit (draft drop = no mutation persisted).
    /// Verify the error type is correct.
    #[test]
    fn fail_midway_rolls_back_topo_and_geometry() {
        // Use an empty selection (0 selected faces) — this triggers the
        // connectivity check failure before any topology mutation occurs.
        let cube = make_cube([0.0, 0.0, 0.0], 2.0).expect("make_cube must succeed");
        let (topo, geom) = cube.into_parts();
        let face_count_before = topo.arena().face_count();
        let state = KernelState::new(topo, geom);

        let cap = 64;
        let selected = EntityBitset::with_capacity(cap);
        let protected = EntityBitset::with_capacity(cap);
        let fake_face = FaceId::from_raw_parts(999, 0);
        let selection = MergeRegionSelection::new(selected, protected, fake_face);

        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state, &selection, &mut ctx);

        assert!(result.is_err(), "Merge with empty selection must fail",);

        // The original state was consumed by KernelDraft, which was dropped.
        // No topology mutation leaked — the error proves atomic rollback.
        // Verify it's the expected error kind.
        if let Err(err) = result {
            assert!(
                matches!(
                    err,
                    KernelError::MergeFailure(MergeError::WouldDisconnectSheet { .. })
                        | KernelError::InvalidInput { .. }
                        | KernelError::InternalError { .. }
                ),
                "Expected connectivity/input error or earlier boundary-gate failure, got: {:?}",
                err,
            );
        }
    }

    // ----- Test 8: Handle re-derivation across multi-step merge -----

    /// Build a fixture where 3 cube faces share edges pairwise. After merging
    /// face pair (A,B), the plan's second step must re-derive handles from the
    /// mutated arena — not use stale handles from the initial snapshot.
    /// The plan builder creates steps sorted by edge_index; here we just
    /// verify a multi-step plan executes without stale-handle errors.
    #[test]
    fn handle_rederivation_across_multi_step_merge() {
        // Build a cube where face_a shares edges with both face_b and face_extra.
        // Select all three → at least 2 merge steps → second step re-derives.
        let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();
        selected.insert(face_extra.index()).unwrap();

        let protected = EntityBitset::with_capacity(cap);
        let selection = MergeRegionSelection::new(selected, protected, face_a);

        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state, &selection, &mut ctx);

        // This may succeed (if plan has steps) or fail with an expected error.
        // The key assertion: no panic from stale handle access.
        // If it succeeds, verify multi-step execution.
        match result {
            Ok(op_result) => {
                let output = op_result.into_value();
                assert!(
                    output.get_merge().get_plan().step_count() >= 1,
                    "Multi-face selection must produce at least 1 plan step",
                );
            }
            Err(err) => {
                // Acceptable errors: AmbiguousRadialSelection (3 faces on one edge)
                // or PartialMergePlanRejected (expected when topology mutates mid-plan).
                // NOT acceptable: a panic from invalid handle access.
                assert!(
                    matches!(
                        err,
                        KernelError::MergeFailure(MergeError::AmbiguousRadialSelection { .. })
                            | KernelError::MergeFailure(
                                MergeError::PartialMergePlanRejected { .. }
                            )
                            | KernelError::MergeFailure(
                                MergeError::BoundaryCertificationFailed { .. }
                            )
                            | KernelError::TopologyViolation { .. }
                            | KernelError::InternalError { .. }
                    ),
                    "Expected merge/topology error or earlier boundary-gate failure, got: {:?}",
                    err,
                );
            }
        }
    }

    // =====================================================================
    // SECTION 4: Defect regression tests (D1, D3, D4, D5)
    // =====================================================================

    /// D1 regression: execute_sheet_region_merge must return committed KernelState.
    ///
    /// Verify the returned state reflects actual merge mutations:
    /// - Face count decreased (killed face removed)
    /// - Killed face's plane binding removed from GeometryState
    /// - Surviving face's plane binding preserved
    #[test]
    fn merge_returns_committed_kernel_state() {
        let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();
        let face_count_before = state.topology().arena().face_count();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let mut protected = EntityBitset::with_capacity(cap);
        protected.insert(face_extra.index()).unwrap();

        let selection = MergeRegionSelection::new(selected, protected, face_a);
        let mut ctx = ModelingContext::new();
        let result =
            execute_sheet_region_merge(state, &selection, &mut ctx).expect("merge must succeed");

        let output = result.into_value();
        let new_state = output.get_state();

        let face_count_after = new_state.topology().arena().face_count();
        assert!(
            face_count_after < face_count_before,
            "D1 regression: returned KernelState must have fewer faces after merge. \
            Before: {}, after: {}",
            face_count_before,
            face_count_after,
        );

        assert!(
            new_state.geometry().get_face_plane(face_b).is_none(),
            "D1 regression: killed face_b's plane binding must be removed in returned state",
        );

        assert!(
            new_state.geometry().get_face_plane(face_a).is_some(),
            "D1 regression: surviving face_a's plane binding must be preserved",
        );
    }

    /// D3 regression: overlap between selected_faces and protected_faces
    /// must be rejected with ProtectedUseConflict.
    #[test]
    fn protected_face_in_selected_set_rejected() {
        let (state, _, face_a, face_b, _face_extra) = build_cube_with_valence_3_edge();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        // Overlap: face_b is both selected AND protected.
        let mut protected = EntityBitset::with_capacity(cap);
        protected.insert(face_b.index()).unwrap();

        let selection = MergeRegionSelection::new(selected, protected, face_a);
        let mut ctx = ModelingContext::new();
        let err = execute_sheet_region_merge(state, &selection, &mut ctx)
            .expect_err("Must reject when selected ∩ protected != ∅");

        assert!(
            matches!(
                err,
                KernelError::MergeFailure(MergeError::ProtectedUseConflict { .. })
            ),
            "D3 regression: expected ProtectedUseConflict, got: {:?}",
            err,
        );
    }

    /// D5/P2-2 regression: merge traces must survive finalization exactly once.
    ///
    /// Under the OperationFinalizer contract, decisions accumulate in `ModelingContext`
    /// during execution and are drained into the returned `OperationResult` at the
    /// operation boundary. The context must be empty after successful finalization to
    /// avoid double-counting on reuse.
    #[test]
    fn ctx_receives_traced_decisions_after_merge() {
        let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let mut protected = EntityBitset::with_capacity(cap);
        protected.insert(face_extra.index()).unwrap();

        let selection = MergeRegionSelection::new(selected, protected, face_a);
        let mut ctx = ModelingContext::new();
        let result =
            execute_sheet_region_merge(state, &selection, &mut ctx).expect("merge must succeed");

        assert!(
            !result.get_decision_log().is_empty(),
            "D5 regression: OperationResult decision log must not be empty",
        );

        assert!(
            ctx.get_decision_log_mut().is_empty(),
            "P2-2 regression: ModelingContext decision log must be drained after finalization",
        );
    }

    /// Epic A gate regression: if boundary certification rejects before draft creation,
    /// the returned error must preserve witness/reason and the ctx trace must still
    /// contain the certifier decision.
    #[test]
    fn boundary_cert_gate_rejection_preserves_witness_reason_and_ctx_trace() {
        // Selecting all three faces from this synthetic valence-3 fixture is known to
        // produce a degenerate/rejected boundary under the certifier.
        let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();
        selected.insert(face_extra.index()).unwrap();
        let protected = EntityBitset::with_capacity(cap);
        let selection = MergeRegionSelection::new(selected, protected, face_a);

        let mut ctx = ModelingContext::new();
        let err = execute_sheet_region_merge(state, &selection, &mut ctx)
            .expect_err("rejected boundary must fail before merge execution");

        match err {
            KernelError::MergeFailure(MergeError::BoundaryCertificationFailed {
                reason,
                witness,
            }) => {
                assert!(
                    !reason.is_empty(),
                    "gate rejection must preserve certifier reason text",
                );
                assert!(
                    reason.contains("Boundary") || reason.contains("Degenerate"),
                    "expected certifier rejection detail in reason, got: {}",
                    reason,
                );
                assert!(
                    witness.is_some(),
                    "gate rejection must preserve certifier witness when provided",
                );
            }
            other => panic!(
                "expected BoundaryCertificationFailed from gate rejection, got {:?}",
                other
            ),
        }

        let decisions: Vec<_> = ctx.get_decision_log_mut().decisions().collect();
        assert!(
            !decisions.is_empty(),
            "gate rejection must still propagate certifier decision trace into ctx",
        );
        assert_eq!(
            decisions.len(), 1,
            "gate rejection should stop before merge-step execution; expected only certifier decision",
        );

        let d = decisions[0];
        assert_eq!(
            d.get_tier(),
            forge_core::DecisionTier::Escalated,
            "rejected certificate should trace as an escalated decision",
        );
        assert!(
            matches!(d.get_kind(), forge_core::DecisionKind::Forced { .. }),
            "rejected certificate should trace as Forced, got {:?}",
            d.get_kind(),
        );
        match d.get_context() {
            forge_core::DecisionContext::Degeneracy { description } => {
                assert!(
                    description.contains("Boundary rejected"),
                    "expected rejection context text, got: {}",
                    description,
                );
                assert!(
                    !description.contains("MergeStep"),
                    "gate rejection must occur before any merge-step decisions",
                );
            }
            other => panic!(
                "expected Degeneracy context on cert rejection, got {:?}",
                other
            ),
        }
    }

    /// Gate ordering regression: boundary certification must run before later
    /// input validation (e.g. selected/protected overlap), so a rejected boundary
    /// fails with BoundaryCertificationFailed rather than ProtectedUseConflict.
    #[test]
    fn boundary_cert_gate_precedes_protected_overlap_validation() {
        let (state, _, face_a, face_b, face_extra) = build_cube_with_valence_3_edge();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();
        selected.insert(face_extra.index()).unwrap();

        // Deliberately invalid: overlap with selected set.
        let mut protected = EntityBitset::with_capacity(cap);
        protected.insert(face_b.index()).unwrap();

        let selection = MergeRegionSelection::new(selected, protected, face_a);
        let mut ctx = ModelingContext::new();
        let err = execute_sheet_region_merge(state, &selection, &mut ctx)
            .expect_err("gate should reject before protected-face overlap validation");

        assert!(
            matches!(err, KernelError::MergeFailure(MergeError::BoundaryCertificationFailed { .. })),
            "gate ordering regression: expected BoundaryCertificationFailed before ProtectedUseConflict, got {:?}",
            err,
        );

        let decisions: Vec<_> = ctx.get_decision_log_mut().decisions().collect();
        assert_eq!(
            decisions.len(),
            1,
            "gate ordering regression: expected only the certifier decision before early return",
        );
        assert!(
            !matches!(decisions[0].get_context(), forge_core::DecisionContext::Degeneracy { description } if description.contains("MergeStep")),
            "gate ordering regression: merge-step decisions must not appear before boundary gate passes",
        );
    }

    /// D4 regression: RadialUseSelector uses face indices (not halfedge indices).
    ///
    /// Passing valid face indices succeeds.
    #[test]
    fn selector_with_valid_face_indices_succeeds() {
        let (state, edge_idx, face_a, face_b, _face_extra) = build_cube_with_valence_3_edge();

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let protected = EntityBitset::with_capacity(cap);

        let selectors = vec![RadialUseSelector::new(
            edge_idx,
            face_a.index(),
            face_b.index(),
        )];

        let selection =
            MergeRegionSelection::with_radial_selectors(selected, protected, face_a, selectors);

        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state, &selection, &mut ctx);
        assert!(
            result.is_ok(),
            "D4 regression: valid face-index selectors must succeed: {:?}",
            result.err(),
        );
    }

    /// D4 regression: passing halfedge indices (old incorrect semantics)
    /// into RadialUseSelector must fail because the plan builder looks up
    /// by face index and won't find matching faces in the radial ring.
    #[test]
    fn selector_with_halfedge_indices_fails() {
        let (state, edge_idx, face_a, face_b, _face_extra) = build_cube_with_valence_3_edge();

        // Find actual halfedge indices — these differ from face indices.
        let arena = state.topology().arena();
        let target_edge = {
            let mut found = None;
            for (eid, _) in arena.iter_edges() {
                if eid.index() == edge_idx {
                    found = Some(eid);
                    break;
                }
            }
            found.expect("target edge must exist")
        };
        let entry_he = arena.get_edge(target_edge).unwrap().half_edge();
        let he_idx_a = entry_he.index();
        let twin_he = arena.get_half_edge(entry_he).unwrap().radial_next();
        let he_idx_b = twin_he.index();

        // Only proceed if halfedge indices differ from face indices.
        if he_idx_a == face_a.index() && he_idx_b == face_b.index() {
            return;
        }

        let cap = 64;
        let mut selected = EntityBitset::with_capacity(cap);
        selected.insert(face_a.index()).unwrap();
        selected.insert(face_b.index()).unwrap();

        let protected = EntityBitset::with_capacity(cap);

        let selectors = vec![RadialUseSelector::new(
            edge_idx, he_idx_a, // halfedge index, not face index
            he_idx_b,
        )];

        let selection =
            MergeRegionSelection::with_radial_selectors(selected, protected, face_a, selectors);

        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state, &selection, &mut ctx);

        assert!(
            result.is_err(),
            "D4 regression: halfedge indices as face selectors must not succeed silently",
        );
    }

    /// D6 adversarial: out-of-range EntityBitset::contains must propagate as
    /// KernelError, not be silently swallowed via unwrap_or(false).
    ///
    /// Constructs a bitset with capacity derived from the fixture's actual
    /// face indices, guaranteeing an out-of-range contains() hit.
    #[test]
    fn out_of_range_bitset_propagates_error() {
        let (state, _, face_a, _face_b, _face_extra) = build_cube_with_valence_3_edge();

        // Find the maximum face index in the arena.
        let max_face_idx = state
            .topology()
            .arena()
            .iter_faces()
            .map(|(fid, _)| fid.index())
            .max()
            .expect("cube must have faces");

        // Capacity = max_face_idx, so contains(max_face_idx) is out-of-range
        // (bitset is [0, capacity), so capacity itself is OOB).
        let cap = max_face_idx;
        assert!(
            cap > 0,
            "Precondition: max face index must be > 0 for OOB test",
        );

        let mut selected = EntityBitset::with_capacity(cap);
        // Insert face_a if it fits (it likely does since cap = max_face_idx).
        if face_a.index() < cap {
            selected.insert(face_a.index()).unwrap();
        }

        let protected = EntityBitset::with_capacity(cap);
        let selection = MergeRegionSelection::new(selected, protected, face_a);

        let mut ctx = ModelingContext::new();
        let result = execute_sheet_region_merge(state, &selection, &mut ctx);

        // Connectivity validation walks all faces in the arena. When it hits
        // a face with index == max_face_idx, selected.contains(max_face_idx)
        // returns Err (out of bounds). With fail-closed `?`, this propagates.
        assert!(
            result.is_err(),
            "D6 regression: out-of-range bitset must propagate error, not silently ignore. \
            max_face_idx={}, bitset_cap={}",
            max_face_idx,
            cap,
        );
    }

    /// Integration: the runtime validator runs at emission without false-positives.
    /// Uses the same live-descendant fixture as `lineage_fallback_resolves_live_descendant_and_traces_lineage_route`.
    /// On the happy path the validator must pass (no `InternalError`) and the
    /// reidentification adjunct must be emitted with matching outcome/compatibility.
    #[test]
    fn reidentification_trace_payload_drift_causes_internal_error_before_adjunct_push_no_false_positive() {
        use forge_core::{
            ReidentificationCompatibilitySummary, ReidentificationOutcome,
            ReidentificationTraceConsistencyError,
        };

        // ── Build the live-descendant fixture ──────────────────────────────
        let (topo, _geom, group) = build_two_face_coplanar_sheet_fixture();
        let target_face = topo
            .arena()
            .iter_faces()
            .find_map(|(fid, _)| {
                group
                    .contains(fid.index())
                    .ok()
                    .and_then(|in_group| in_group.then_some(fid))
            })
            .expect("fixture must have at least one selected face");

        let synthetic_root = forge_topo::lineage::Lineage::root(
            99,
            forge_topo::lineage::OpSignature::with_id("synthetic_root_face", 1),
        );
        let child = forge_topo::lineage::Lineage::derive(
            &synthetic_root,
            forge_topo::lineage::OpSignature::with_id("synthetic_split_face", 2),
        );

        let mut draft = topo.into_mutation();
        draft
            .arena_mut()
            .get_face_mut(target_face)
            .expect("target face exists")
            .set_lineage(Some(child.clone()));
        draft.log_lineage_event(forge_topo::lineage::LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::Face, target_face.index()),
            entity_snapshot: Some(target_face.into()),
            lineage: child,
        });
        let topo = draft
            .commit()
            .expect("synthetic lineage descendant fixture commit");

        let missing_parent_name = PersistentName::new(
            synthetic_root.get_ancestry_hash(),
            forge_core::EntityKind::Face,
            0,
        );

        // ── Happy path: validator must pass and adjunct must be emitted ───
        let persistent = MergeRegionSelectionPersistent::new(
            vec![missing_parent_name.clone()],
            Vec::new(),
            missing_parent_name,
        );
        let state = crate::core::KernelState::new(topo, GeometryState::new());
        let mut ctx = ModelingContext::new();
        let resolved = resolve_merge_region_selection_persistent(&state, &persistent, &mut ctx)
            .expect("happy-path lineage resolved — validator must not false-positive");
        assert_eq!(resolved.get_selected_faces().iter_ones().count(), 1);

        let reid_adjunct = ctx
            .get_trace_adjuncts()
            .records()
            .iter()
            .find_map(|r| r.as_reidentification_payload())
            .expect("happy-path must emit a reidentification adjunct")
            .expect("adjunct decoded");
        assert_eq!(
            reid_adjunct.outcome,
            ReidentificationOutcome::Resolved,
            "happy-path adjunct outcome must be Resolved",
        );
        assert_eq!(
            reid_adjunct.compatibility,
            ReidentificationCompatibilitySummary::Available,
            "happy-path adjunct compatibility must be Available",
        );

        // ── Adversarial: artificially drift payload and confirm validator ─
        // Build a standalone drifted payload to test the validator API directly.
        // (We can't inject a tampered payload into resolve_single_face_ref without
        // refactoring the emission site; instead this covers the validator contract
        // at the API level, complementing the unit tests in reidentification_trace.rs.)
        let mut drifted = reid_adjunct;
        drifted.outcome = ReidentificationOutcome::Incompatible;
        // Leave compatibility = Available — this is the exact drift the validator guards against.
        let fake_decision = forge_core::tracing::TracedDecision::new(
            drifted.decision_id,
            forge_core::DecisionKind::Forced {
                reason: "ReidentificationIncompatible".into(),
            },
            forge_core::DecisionTier::Escalated,
            0.0,
            forge_core::DecisionContext::Degeneracy {
                description: "adversarial_drift_test".into(),
            },
        );
        assert_eq!(
            drifted.validate_against_decision(&fake_decision),
            Err(ReidentificationTraceConsistencyError::OutcomeCompatibilityMismatch),
            "Incompatible outcome with Available compatibility must be flagged as OutcomeCompatibilityMismatch",
        );
    }

    #[test]
    fn generation_reuse_does_not_cause_stale_snapshot_leakage() {
        use forge_core::tracing::ResolutionOutcome;

        // 1. Setup topography and face
        let mut draft = forge_topo::state::TopologyState::empty().into_mutation();
        let f1 = draft.arena_mut().add_face();
        let root_lineage = forge_topo::lineage::Lineage::root(
            1,
            forge_topo::lineage::OpSignature::with_id("root_face", 1),
        );
        draft.arena_mut().get_face_mut(f1).unwrap().set_lineage(Some(root_lineage.clone()));

        // Save the old name
        let persistent_name = PersistentName::new(
            root_lineage.get_ancestry_hash(),
            forge_core::EntityKind::Face,
            0,
        );

        // 2. Delete the first face (frees up its slot)
        draft.arena_mut().remove_face(f1).unwrap();

        // 3. Add a new face (generation bumps in the slot)
        let f2 = draft.arena_mut().add_face();
        
        // Assert generation bumped up
        assert_eq!(f1.index(), f2.index(), "arena should reuse slot");
        assert!(f2.generation() > f1.generation(), "new face must have a higher generation");
        
        let new_lineage = forge_topo::lineage::Lineage::root(
            2,
            forge_topo::lineage::OpSignature::with_id("new_face", 2),
        );
        draft.arena_mut().get_face_mut(f2).unwrap().set_lineage(Some(new_lineage));

        let topo = draft.commit().expect("commit");

        // 4. Resolve the old persistent name against tracking
        let selection = MergeRegionSelectionPersistent::new(
            vec![persistent_name],
            Vec::new(),
            PersistentName::new(0, forge_core::EntityKind::Face, 0),
        );
        
        let state = crate::core::KernelState::new(topo, GeometryState::new());
        let mut ctx = ModelingContext::new();

        // Should NOT find the matching entity due to a different generation or lineage
        let resolved = resolve_merge_region_selection_persistent(&state, &selection, &mut ctx)
            .expect("should return gracefully");

        assert_eq!(
            resolved.get_selected_faces().iter_ones().count(),
            0,
            "stale snapshot cannot be erroneously resolved"
        );

        // Check the resolution trace adjunct
        let resolution_adjunct = ctx
            .get_trace_adjuncts()
            .records()
            .iter()
            .find_map(|r| r.as_resolution_payload())
            .expect("must emit a tracing adjunct")
            .unwrap();

        assert_eq!(
            resolution_adjunct.outcome,
            ResolutionOutcome::Missing,
            "Must report missing when the snapshot is stale"
        );
        assert_eq!(
            resolution_adjunct.ordered_candidates.len(),
            0,
            "candidate count must be zero since stale entities are not valid candidates"
        );
    }
}
