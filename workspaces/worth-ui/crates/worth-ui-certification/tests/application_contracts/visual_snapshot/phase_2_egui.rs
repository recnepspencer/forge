use std::sync::Arc;

use worth_ui_host_egui::WorthUiHostEgui;
use worth_ui_runtime::facade::host::WorthUiOperationalHostAdapter;
use worth_ui_runtime::facade::mounted::UiMountedFrameRequest;
use worth_ui_test_support::{
    WorthUiFrameworkTurnCertificationExt, WorthUiMountedFrameExecutionCertificationExt,
};

use super::super::platform_pulse::{establish_viewport_allocation, launch_and_mount_pulse};
use super::support::current_target;
use super::*;

#[test]
fn egui_exact_screenshot_event_completes_required_pixel_capture() {
    let (context, host, mut session) = presented_egui_world("egui-visual-exact");
    assert_eq!(
        host.visual_capture_capability(),
        worth_ui_host_contract::UiHostCaptureCapability::Pixels {
            maximum_bytes: 64 * 1024 * 1024,
            exact_presentation_epoch: true,
        }
    );
    let target = current_target(&session);
    let expected_frame = target.frame();
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
                .artifacts(UiPixelsRequired::policy()),
        )
        .expect("real egui capture is admitted");

    let (pending, user_data, command_count) = request_screenshot(&context, &mut session, pending);
    assert_eq!(command_count, 1);
    let event_input = input_with_screenshot(user_data, egui::Color32::from_rgb(7, 11, 13));
    let mut completed = None;
    let mut pending = Some(pending);
    let _ = context.run_ui(event_input, |_| {
        completed = Some(
            session.poll_visual_snapshot(pending.take().expect("the event frame consumes once"), 1),
        );
    });
    let receipt = match completed.expect("the event frame polls once") {
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Captured(receipt)) => receipt,
        _ => panic!("the exact event must complete required pixels"),
    };
    assert_eq!(
        receipt.affinity().frame(),
        expected_frame.diagnostic_value()
    );
    assert_eq!(
        receipt.affinity().relation(),
        worth_ui::facade::inspection::UiVisualSnapshotRelation::Current
    );
    assert_eq!(receipt.coordinates().native_client_origin(), [10, 15]);
    assert_eq!(
        receipt.coordinates().client_physical_dimensions(),
        [200, 120]
    );
    assert_eq!(
        receipt.coordinates().viewport_logical_dimensions(),
        [160.0, 96.0]
    );
    assert_eq!(receipt.coordinates().scale(), [1.25, 1.25]);
    assert_eq!(receipt.pixel_artifact().bytes().len(), 200 * 120 * 4);
    assert_eq!(&receipt.pixel_artifact().bytes()[..4], &[7, 11, 13, 255]);
}

#[test]
fn egui_wrong_correlation_stays_pending_and_cancellation_is_after_effect() {
    let (context, _host, mut session) = presented_egui_world("egui-visual-correlation");
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .expect("real egui capture is admitted");
    let (pending, _user_data, _) = request_screenshot(&context, &mut session, pending);

    let wrong = egui::UserData::new("foreign-screenshot-correlation");
    let event_input = input_with_screenshot(wrong, egui::Color32::WHITE);
    let mut next = None;
    let mut pending = Some(pending);
    let _ = context.run_ui(event_input, |_| {
        next = Some(session.poll_visual_snapshot(
            pending.take().expect("the wrong-event frame consumes once"),
            1,
        ));
    });
    let pending = match next.expect("wrong event frame polls once") {
        UiVisualCapturePoll::Pending(pending) => pending,
        UiVisualCapturePoll::Completed(_) => {
            panic!("an unrelated screenshot cannot complete this request")
        }
    };
    let cancelled = session.cancel_visual_snapshot(pending);
    assert_eq!(
        cancelled.posture(),
        UiVisualCancellationPosture::ReadbackMayHaveBegun
    );
}

#[test]
fn egui_epoch_advance_before_event_returns_superseded_without_pixels() {
    let (context, _host, mut session) = presented_egui_world("egui-visual-epoch");
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .expect("real egui capture is admitted");
    let (pending, user_data, _) = request_screenshot(&context, &mut session, pending);
    present_successor(&context, &mut session);

    let event_input = input_with_screenshot(user_data, egui::Color32::WHITE);
    let mut outcome = None;
    let mut pending = Some(pending);
    let _ = context.run_ui(event_input, |_| {
        outcome = Some(
            session.poll_visual_snapshot(
                pending
                    .take()
                    .expect("the successor event frame consumes once"),
                2,
            ),
        );
    });
    assert!(matches!(
        outcome.expect("successor event frame polls once"),
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Superseded(value))
            if !value.predecessor_artifact_copied()
    ));
}

#[test]
fn egui_wrong_viewport_event_is_affinity_indeterminate() {
    let (context, _host, mut session) = presented_egui_world("egui-visual-viewport");
    let grant = session.visual_inspection_authority().issue_pixel_grant();
    let pending = session
        .begin_visual_pixel_snapshot(
            &grant,
            UiVisualSnapshotRequest::for_local_development_unredacted_frame(current_target(
                &session,
            ))
            .artifacts(UiPixelsRequired::policy()),
        )
        .expect("real egui capture is admitted");
    let (pending, user_data, _) = request_screenshot(&context, &mut session, pending);
    let mut event_input = raw_input();
    event_input.events.push(egui::Event::Screenshot {
        viewport_id: egui::ViewportId::from_hash_of("foreign-viewport"),
        user_data,
        image: Arc::new(egui::ColorImage::filled([200, 120], egui::Color32::WHITE)),
    });
    let mut pending = Some(pending);
    let mut outcome = None;
    let _ = context.run_ui(event_input, |_| {
        outcome = Some(
            session.poll_visual_snapshot(
                pending
                    .take()
                    .expect("the wrong-viewport frame consumes once"),
                1,
            ),
        );
    });
    assert!(matches!(
        outcome.expect("wrong-viewport event frame polls once"),
        UiVisualCapturePoll::Completed(UiVisualSnapshotOutcome::Indeterminate(
            UiVisualSnapshotIndeterminate::CaptureAffinity
        ))
    ));
}

#[test]
fn egui_emits_no_screenshot_command_without_explicit_capture_request() {
    let context = egui::Context::default();
    let host = WorthUiHostEgui::new(context.clone());
    let mut session = launch_and_mount_pulse(host);
    let output = context.run_ui(raw_input(), |_| {
        establish_viewport_allocation(&mut session);
        publish_frame(&mut session);
    });
    assert_eq!(screenshot_commands(&output).count(), 0);
}

type PendingRequired = worth_ui::facade::inspection::UiPendingVisualCapture<
    worth_ui::facade::inspection::UiCurrentPresentedSurfaceTarget,
    UiPixelsRequired,
>;

fn presented_egui_world(
    label: &str,
) -> (
    egui::Context,
    WorthUiHostEgui,
    worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let context = egui::Context::default();
    let host = WorthUiHostEgui::new(context.clone());
    let mut session = launch_and_mount_pulse(host.clone());
    let _ = context.run_ui(raw_input(), |_| {
        establish_viewport_allocation(&mut session);
        publish_frame(&mut session);
    });
    let _ = label;
    (context, host, session)
}

fn present_successor(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let _ = context.run_ui(raw_input(), |_| publish_frame(session));
}

fn publish_frame(session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession) {
    let prepared = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("ordinary mounted execution is admitted"))
        .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
        .expect("pulse frame prepares");
    assert!(matches!(
        session.present_prepared_mounted_frame(prepared, UiPresentationDeadline::at_tick(10), 0),
        UiMountedFrameOutcome::Published(_)
    ));
}

fn request_screenshot(
    context: &egui::Context,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    pending: PendingRequired,
) -> (PendingRequired, egui::UserData, usize) {
    let mut poll = None;
    let mut pending = Some(pending);
    let output = context.run_ui(raw_input(), |_| {
        poll = Some(
            session
                .poll_visual_snapshot(pending.take().expect("the request frame consumes once"), 0),
        );
    });
    let pending = match poll.expect("request frame polls once") {
        UiVisualCapturePoll::Pending(pending) => pending,
        UiVisualCapturePoll::Completed(_) => panic!("first egui poll submits screenshot work"),
    };
    let commands = screenshot_commands(&output).collect::<Vec<_>>();
    let user_data = (*commands.first().expect("one screenshot command is emitted")).clone();
    (pending, user_data, commands.len())
}

fn screenshot_commands(output: &egui::FullOutput) -> impl Iterator<Item = &egui::UserData> {
    output
        .viewport_output
        .values()
        .flat_map(|viewport| viewport.commands.iter())
        .filter_map(|command| match command {
            egui::ViewportCommand::Screenshot(user_data) => Some(user_data),
            _ => None,
        })
}

fn input_with_screenshot(user_data: egui::UserData, color: egui::Color32) -> egui::RawInput {
    let mut input = raw_input();
    input.events.push(egui::Event::Screenshot {
        viewport_id: egui::ViewportId::ROOT,
        user_data,
        image: Arc::new(egui::ColorImage::filled([200, 120], color)),
    });
    input
}

fn raw_input() -> egui::RawInput {
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
