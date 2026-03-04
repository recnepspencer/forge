use super::*;
// =====================================================================
// SECTION 2: Kernel integration tests
//
// These exercise the FULL pipeline: real TopologyArena + GeometryState
// → boundary_adapter → certify_merge_boundary → trace propagation.
// =====================================================================

use super::super::super::nmt_eval::{
    execute_sheet_region_merge_persistent, resolve_merge_region_selection_persistent,
    NmtEvalTestApi,
};
use super::super::super::schema::{MergeRegionSelectionPersistent, PersistentFaceRef};
use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;
use forge_topo::persistent_naming::Selector;
use forge_topo::persistent_naming::{assign_name, PersistentName};
use crate::lineage::{LineageEvent, OpSignature};

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

    let result =
        crate::operations::boolean::_deprecated::parametric::postprocess::merge_coplanar_faces_extracted(
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

    let result =
        crate::operations::boolean::_deprecated::parametric::postprocess::merge_coplanar_faces_extracted(
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
    forge_topo::transactions::TopologyState,
    GeometryState,
    forge_topo::bitset::EntityBitset,
) {
    use forge_topo::entity_lifecycle::make_edge_face::MakeEdgeFace;
    use forge_topo::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use forge_topo::entity_lifecycle::split_edge::SplitEdge;
    use forge_topo::operator::apply_op;
    use forge_topo::transactions::TopologyState;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft.execute(MakeVertexFace).unwrap().into_value();
    let se1 = apply_op(
        &mut draft,
        SplitEdge {
            edge: mvf.half_edge,
        },
    )
    .unwrap()
    .into_value();
    let _se2 = apply_op(
        &mut draft,
        SplitEdge {
            edge: se1.he_am,
        },
    )
    .unwrap()
    .into_value();
    let se3 = apply_op(
        &mut draft,
        SplitEdge {
            edge: se1.he_mb,
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

    let plane = Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
        .expect("fixture plane");
    geom.set_face_plane(mvf.face, plane.clone());
    geom.set_face_plane(mef.new_face, plane);

    (topo, geom, group)
}

fn build_ambiguous_face_persistent_name_fixture(
) -> (forge_topo::transactions::TopologyState, PersistentName) {
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
        crate::operations::boolean::_deprecated::parametric::postprocess::merge_eligibility::eval::certify_merge_boundary(
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
            WeakSimpleCertificate::Simple
                | WeakSimpleCertificate::WeaklySimple { .. }
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

    let result = NmtEvalTestApi::resolve_face_ref_direct(topo.arena(), &selector);
    match result {
        crate::core::ResolutionResult::Ambiguous {
            candidates,
            evidence,
            query,
        } => {
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

            let payload = crate::core::ResolutionResult::Ambiguous {
                candidates,
                evidence,
                query,
            }
            .to_trace_payload(forge_core::DecisionId(1), None);

            match payload.query {
                forge_core::tracing::ResolutionQuerySummary::Selector {
                    selector_fingerprint,
                    ..
                } => {
                    assert!(
                        selector_fingerprint.is_some(),
                        "NURBS-safe fingerprint must be emitted for selector queries"
                    );
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

    let mut draft = forge_topo::transactions::TopologyState::empty().into_mutation();
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
        forge_topo::persistent_naming::resolve_name(
            topo.arena(),
            match &missing { PersistentFaceRef::Name(name) => name, _ => unreachable!() }
        ).is_empty(),
        "test precondition: no live face exists, so direct persistent-name resolution must miss"
    );
    let result = NmtEvalTestApi::resolve_face_ref_with_lineage_fallback(&topo, &missing);
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
        forge_topo::persistent_naming::resolve_name(
            topo.arena(),
            match &missing {
                PersistentFaceRef::Name(name) => name,
                _ => unreachable!(),
            }
        )
        .is_empty(),
        "test precondition: direct persistent-name resolution must miss"
    );

    let result = NmtEvalTestApi::resolve_face_ref_with_lineage_fallback(&topo, &missing);
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
    let mapped = NmtEvalTestApi::map_resolution_incompatibility(
        &crate::core::ResolutionIncompatibility::SubstrateUnavailable,
    );
    assert_eq!(
        mapped,
        forge_core::errors::PersistentResolutionIncompatibility::SubstrateUnavailable
    );

    let mapped = NmtEvalTestApi::map_resolution_incompatibility(
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

    let direct = NmtEvalTestApi::resolve_face_ref_direct(topo.arena(), &pref);
    assert!(
        matches!(direct, crate::core::ResolutionResult::Missing { .. }),
        "Direct persistent-name lookup must miss to exercise lineage fallback, got {:?}",
        direct,
    );

    let fallback = NmtEvalTestApi::resolve_face_ref_with_lineage_fallback(&topo, &pref);
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

    let direct = NmtEvalTestApi::resolve_face_ref_direct(topo.arena(), &pref);
    assert!(
        matches!(direct, crate::core::ResolutionResult::Missing { .. }),
        "Direct persistent-name lookup must miss to exercise lineage fallback, got {:?}",
        direct,
    );

    let fallback = NmtEvalTestApi::resolve_face_ref_with_lineage_fallback(&topo, &pref);
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
        crate::operations::boolean::_deprecated::parametric::postprocess::merge_eligibility::eval::certify_merge_boundary(
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

    let op_result = crate::operations::boolean::_deprecated::parametric::postprocess::merge_eligibility::eval::certify_merge_boundary(
        arena, &group, &geom,
    ).expect("certify_merge_boundary should succeed");

    let _ = op_result.into_value();
}

