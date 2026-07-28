#[path = "visual_identity/filesystem_mounted_world.rs"]
mod filesystem_mounted_world;
#[path = "visual_identity/overlay_oracle.rs"]
mod overlay_oracle;
#[path = "visual_identity/point_oracle.rs"]
mod point_oracle;
#[path = "visual_identity/projection_oracle.rs"]
mod projection_oracle;
#[path = "visual_identity/region_oracle.rs"]
mod region_oracle;
#[path = "visual_identity/resource_bounds.rs"]
mod resource_bounds;

use worth_ui::facade::inspection::{
    UiGeometryOnly, UiVisualCapturePoll, UiVisualSnapshotOutcome, UiVisualSnapshotRequest,
};
use worth_ui_host_contract::{
    UiMountedFilledRectMechanic, UiMountedHitTestMechanic, UiMountedLayerRow,
    UiMountedNodeProjectionView, UiMountedPaintBatchRow,
};
use worth_ui_host_egui::WorthUiHostEgui;
use worth_ui_runtime::facade::mounted::{
    UiMountedFrameOutcome, UiMountedFramePreparationDenial, UiMountedInspectionReceipt,
    UiMountedInspectionRequest, UiMountedProjectionDenial, UiPresentationDeadline,
};
use worth_ui_test_support::{
    WorthUiMountedIdentityCertificationExt, WorthUiMountedPublicationCertificationExt,
};

use filesystem_mounted_world::{
    establish_allocation, launch_native_region_world, launch_native_region_world_with_policy,
    launch_native_world, launch_native_world_with_policy, launch_world, prepare_frame,
    HitOrderProfile,
};
use point_oracle::assert_four_way_point_adjudication;
use projection_oracle::assert_four_way_projection;
use region_oracle::{
    assert_frontmost_inset_region_adjudication, assert_region_candidate_truncation,
    assert_region_result_truncation,
};

#[test]
fn real_filesystem_world_keeps_paint_and_hit_mechanics_non_substitutable() {
    let mut session = launch_world(HitOrderProfile::Canonical);
    establish_allocation(&mut session, 3);
    let prepared = prepare_frame(&mut session).expect("the four-way world completes projection");

    assert_four_way_projection(prepared.surfaces()[0].projection());
}

#[test]
fn duplicate_hit_order_is_rejected_before_a_candidate_frame_can_publish() {
    let mut session = launch_world(HitOrderProfile::Duplicate);
    establish_allocation(&mut session, 3);
    let prior = session.inspect_mounted_identity().current_frame();
    let denial = match prepare_frame(&mut session) {
        Err(denial) => denial,
        Ok(_) => panic!("duplicate hit order must deny projection"),
    };

    assert!(matches!(
        denial,
        UiMountedFramePreparationDenial::Projection(
            UiMountedProjectionDenial::DuplicateHitTestOrder { order, .. }
        ) if order.rank() == 1
    ));
    assert_eq!(session.inspect_mounted_identity().current_frame(), prior);
}

#[test]
fn egui_cost_counts_runtime_mounted_hit_mechanics() {
    let context = egui::Context::default();
    let mut session = launch_native_world(WorthUiHostEgui::new(context.clone()));
    let mut outcome = None;
    let native = context.run(raw_input(), |_| {
        establish_allocation(&mut session, 3);
        let prepared = prepare_frame(&mut session).expect("egui receives a complete projection");
        let projection = prepared.surfaces()[0].projection();
        assert_four_way_projection(projection);
        assert!(projection.clips().rows().is_empty());
        assert!(projection.spatial_batches().rows().is_empty());
        assert!(projection.realtime_batches().rows().is_empty());
        assert!(projection.resources().entries().is_empty());
        outcome = Some(session.present_prepared_mounted_frame(
            prepared,
            UiPresentationDeadline::at_tick(10),
            0,
        ));
    });
    let publication = match outcome {
        Some(UiMountedFrameOutcome::Published(publication)) => publication,
        Some(_) => panic!("complete egui visual identity frame must publish"),
        None => panic!("egui callback must produce a presentation outcome"),
    };
    let adapter_cost = publication.cost_report().adapter();
    assert_eq!(adapter_cost.translated_rows(), 10);
    assert_eq!(
        adapter_cost.translated_bytes(),
        u64::try_from(
            4 * std::mem::size_of::<UiMountedNodeProjectionView>()
                + std::mem::size_of::<UiMountedLayerRow>()
                + std::mem::size_of::<UiMountedPaintBatchRow>()
                + 2 * std::mem::size_of::<UiMountedFilledRectMechanic>()
                + 2 * std::mem::size_of::<UiMountedHitTestMechanic>()
        )
        .unwrap()
    );
    assert_eq!(native.shapes.len(), 2);
    let _ = session.shutdown();
}

#[test]
fn real_egui_capture_seals_distinct_visible_and_hit_indexes() {
    let context = egui::Context::default();
    let (mut session, receipt) = capture_four_way_snapshot(&context);
    assert_eq!(receipt.visible_region_count(), 2);
    assert_eq!(receipt.hit_test_region_count(), 2);
    assert_eq!(
        receipt.visible_region_index_identity().diagnostic_value(),
        receipt.identity().diagnostic_value()
    );
    assert_eq!(
        receipt.hit_test_region_index_identity().diagnostic_value(),
        receipt.identity().diagnostic_value()
    );
    assert_ne!(
        receipt.visible_region_index_identity().structural_digest(),
        receipt.hit_test_region_index_identity().structural_digest()
    );
    assert_eq!(receipt.cost().counters()[0], 4);
    assert!(receipt.cost().retained_structural_bytes() > 0);
    let disposed = session.dispose_visual_snapshot(receipt);
    assert!(disposed.released_registered_resource());
    let _ = session.shutdown();
}

#[test]
fn real_egui_snapshot_adjudicates_four_way_identity_from_retained_receipts() {
    let context = egui::Context::default();
    let (mut session, receipt) = capture_four_way_snapshot(&context);

    assert_four_way_point_adjudication(&receipt);

    let disposed = session.dispose_visual_snapshot(receipt);
    assert!(disposed.released_registered_resource());
    let _ = session.shutdown();
}

#[test]
fn identity_overlay_is_a_managed_mounted_successor_and_clear() {
    let context = egui::Context::default();
    let (host, session, receipt) = capture_four_way_snapshot_with_host(&context);
    overlay_oracle::assert_managed_overlay_successors(&context, &host, session, receipt);
}

#[test]
fn overlay_publication_and_clear_failures_return_retryable_linear_handles() {
    let context = egui::Context::default();
    let (session, receipt) = capture_four_way_snapshot(&context);
    overlay_oracle::assert_overlay_rollbacks_preserve_linear_authority(&context, session, receipt);
}

#[test]
fn pending_overlay_is_enumerated_and_disposed_at_shutdown() {
    let context = egui::Context::default();
    let (session, receipt) = capture_four_way_snapshot(&context);
    overlay_oracle::assert_pending_overlay_is_enumerated_at_shutdown(session, receipt);
}

#[test]
fn dropping_pending_overlay_is_no_effect_cancellation() {
    let context = egui::Context::default();
    let (host, session, receipt) = capture_four_way_snapshot_with_host(&context);
    overlay_oracle::assert_pending_drop_is_no_effect_cancellation(
        &context, &host, session, receipt,
    );
}

#[test]
fn foreign_session_cannot_register_a_retained_overlay_target() {
    let owner_context = egui::Context::default();
    let foreign_context = egui::Context::default();
    let (owner, owner_receipt) = capture_four_way_snapshot(&owner_context);
    let (foreign, foreign_receipt) = capture_four_way_snapshot(&foreign_context);
    overlay_oracle::assert_foreign_session_rejects_overlay_before_registration(
        owner,
        owner_receipt,
        foreign,
        foreign_receipt,
    );
}

#[test]
fn superseded_receipts_and_prederived_targets_are_denied_distinctly() {
    let context = egui::Context::default();
    let (session, receipt) = capture_four_way_snapshot(&context);
    overlay_oracle::assert_superseded_overlay_sources_are_denied(&context, session, receipt);
}

#[test]
fn snapshot_authority_expires_when_its_session_is_gone() {
    let context = egui::Context::default();
    let (session, receipt) = capture_four_way_snapshot(&context);
    overlay_oracle::assert_expired_snapshot_cannot_derive_overlay_target(session, receipt);
}

#[test]
fn overlay_registry_capacity_returns_a_typed_denial() {
    let context = egui::Context::default();
    let (session, receipt) = capture_four_way_snapshot(&context);
    overlay_oracle::assert_overlay_registry_capacity_is_typed(session, receipt);
}

#[test]
fn dropping_a_published_handle_does_not_clear_the_overlay() {
    let context = egui::Context::default();
    let (session, receipt) = capture_four_way_snapshot(&context);
    overlay_oracle::assert_published_drop_persists_until_shutdown(&context, session, receipt);
}

#[test]
fn real_egui_point_budget_exhaustion_is_typed_incomplete() {
    let context = egui::Context::default();
    let policy = query_policy(1, 1);
    let session = launch_native_world_with_policy(WorthUiHostEgui::new(context.clone()), policy);
    let (mut session, receipt) = capture_snapshot_from_session(&context, session);

    receipt.with_coordinate_scope(|scope| {
        let point = worth_ui::facade::inspection::UiClientPhysicalPixel::new(100, 50).unwrap();
        let result = scope
            .adjudicate_point(scope.client_pixel(point).unwrap())
            .unwrap();
        assert!(matches!(
            result.visible(),
            worth_ui::facade::inspection::UiVisualVisibleOutcome::Incomplete(budget)
                if budget.maximum_candidates() == 1
        ));
        assert!(matches!(
            result.hit_test(),
            worth_ui::facade::inspection::UiVisualHitTestOutcome::Incomplete(budget)
                if budget.maximum_candidates() == 1
        ));
        assert_eq!(result.cost().candidates_considered(), 1);
    });

    let _ = session.dispose_visual_snapshot(receipt);
    let _ = session.shutdown();
}

#[test]
fn real_egui_region_adjudication_is_many_to_many_and_complete_postured() {
    let context = egui::Context::default();
    let session = launch_native_region_world(WorthUiHostEgui::new(context.clone()));
    let (mut session, receipt) = capture_snapshot_from_session(&context, session);

    assert_frontmost_inset_region_adjudication(&receipt);

    let _ = session.dispose_visual_snapshot(receipt);
    let _ = session.shutdown();
}

#[test]
fn real_egui_region_budgets_distinguish_candidate_and_result_truncation() {
    let candidate_context = egui::Context::default();
    let candidate_session = launch_native_region_world_with_policy(
        WorthUiHostEgui::new(candidate_context.clone()),
        query_policy(1, 1),
    );
    let (mut candidate_session, candidate_receipt) =
        capture_snapshot_from_session(&candidate_context, candidate_session);
    assert_region_candidate_truncation(&candidate_receipt);
    let _ = candidate_session.dispose_visual_snapshot(candidate_receipt);
    let _ = candidate_session.shutdown();

    let result_context = egui::Context::default();
    let result_session = launch_native_region_world_with_policy(
        WorthUiHostEgui::new(result_context.clone()),
        query_policy(1, 2),
    );
    let (mut result_session, result_receipt) =
        capture_snapshot_from_session(&result_context, result_session);
    assert_region_result_truncation(&result_receipt);
    let _ = result_session.dispose_visual_snapshot(result_receipt);
    let _ = result_session.shutdown();
}

fn query_policy(
    maximum_results: u16,
    maximum_candidates: u16,
) -> worth_ui::facade::inspection::UiVisualInspectionPolicy {
    worth_ui::facade::inspection::UiVisualInspectionPolicy::bounded(
        worth_ui::facade::inspection::UiVisualInspectionDisclosure::local_development_unredacted(),
        worth_ui::facade::inspection::UiVisualInspectionCapacity::bounded(
            2,
            maximum_results,
            maximum_candidates,
        ),
        worth_ui::facade::inspection::UiVisualInspectionRegionCapacity::bounded(65_536, 65_536),
        worth_ui::facade::inspection::UiVisualInspectionByteBudget::bounded(
            4_096,
            4_096,
            64 << 10,
            128 << 10,
        ),
    )
    .unwrap()
}

pub(super) fn capture_four_way_snapshot(
    context: &egui::Context,
) -> (
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    worth_ui::facade::inspection::UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let (_, session, receipt) = capture_four_way_snapshot_with_host(context);
    (session, receipt)
}

fn capture_four_way_snapshot_with_host(
    context: &egui::Context,
) -> (
    WorthUiHostEgui,
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    worth_ui::facade::inspection::UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let host = WorthUiHostEgui::new(context.clone());
    let session = launch_native_world(host.clone());
    let (session, receipt) = capture_snapshot_from_session(context, session);
    (host, session, receipt)
}

fn capture_snapshot_from_session(
    context: &egui::Context,
    mut session: worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> (
    worth_ui::facade::app::WorthUiActiveApplicationSession,
    worth_ui::facade::inspection::UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let _ = context.run(raw_input(), |_| {
        establish_allocation(&mut session, 3);
        let prepared = prepare_frame(&mut session).expect("the four-way projection completes");
        let outcome = session.present_prepared_mounted_frame(
            prepared,
            UiPresentationDeadline::at_tick(10),
            0,
        );
        assert!(matches!(outcome, UiMountedFrameOutcome::Published(_)));
    });
    let target = match session.inspect_mounted_frame(UiMountedInspectionRequest::current()) {
        UiMountedInspectionReceipt::Available(frame) => frame
            .current_visual_target()
            .expect("the real world presents exactly one surface"),
        other => panic!("the published frame must be inspectable, got {other:?}"),
    };
    let grant = session.visual_inspection_authority().issue_geometry_grant();
    let pending = session
        .begin_visual_geometry_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiGeometryOnly::policy()),
        )
        .expect("the retained presented surface admits geometry capture");
    let mut pending = Some(pending);
    let mut poll = None;
    let _ = context.run(raw_input(), |_| {
        poll = Some(session.poll_visual_snapshot(
            pending.take().expect("the callback consumes one capture"),
            1,
        ));
    });
    let receipt = match poll.expect("egui callback polls the capture") {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("geometry-only egui capture should complete immediately"),
    };
    (session, receipt)
}

pub(super) fn raw_input() -> egui::RawInput {
    let mut input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(160.0, 96.0),
        )),
        ..Default::default()
    };
    let viewport = input
        .viewports
        .get_mut(&egui::ViewportId::ROOT)
        .expect("raw input carries the root viewport");
    viewport.native_pixels_per_point = Some(1.25);
    viewport.inner_rect = Some(egui::Rect::from_min_size(
        egui::pos2(8.0, 12.0),
        egui::vec2(160.0, 96.0),
    ));
    input
}
