use worth_ui_host_egui::WorthUiHostEgui;
use worth_ui_runtime::facade::host::WorthUiHeadlessRecorder;
use worth_ui_runtime::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedAllocationProjection, UiMountedFrameOutcome,
    UiMountedFrameRequest, UiMountedOmissionReason, UiMountedParticipationStatus,
    UiPresentationDeadline,
};
use worth_ui_test_support::{
    WorthUiFrameworkTurnCertificationExt, WorthUiMountedFrameExecutionCertificationExt,
    WorthUiMountedIdentityCertificationExt, WorthUiMountedPublicationCertificationExt,
};

use super::mounted_application_lifecycle::known_empty_surface_world::{
    first_node, mounted_application_with_host, profile,
};

#[derive(Debug, Eq, PartialEq)]
struct IndependentMountedMeaning {
    node_count: usize,
    paint: UiMountedParticipationStatus,
    input: UiMountedParticipationStatus,
    focus: UiMountedParticipationStatus,
    hit_test: UiMountedParticipationStatus,
    diagnostic: UiMountedParticipationStatus,
    allocation_omission: UiMountedOmissionReason,
    paint_batch_count: usize,
    spatial_batch_count: usize,
    realtime_batch_count: usize,
}

#[test]
fn headless_and_egui_consume_equivalent_sealed_meaning_with_independent_consequences() {
    let recorder = WorthUiHeadlessRecorder::default();
    let mut headless =
        mounted_application_with_host("phase7-adapter-parity-headless", recorder.clone())
            .launch()
            .expect("headless file-authored application launches");
    mount_one(&mut headless, UiHostSurfacePresentationMode::RecordOnly);
    let headless_frame = prepare(&mut headless);

    let context = egui::Context::default();
    let egui_host = WorthUiHostEgui::new(context.clone());
    let mut egui = mounted_application_with_host("phase7-adapter-parity-egui", egui_host.clone())
        .launch()
        .expect("egui file-authored application launches");
    mount_one(&mut egui, UiHostSurfacePresentationMode::NativeDisplay);
    let egui_frame = prepare(&mut egui);

    let headless_meaning = independent_meaning(&headless_frame);
    let egui_meaning = independent_meaning(&egui_frame);
    assert_eq!(headless_meaning, egui_meaning);

    let headless_publication = published(headless.present_prepared_mounted_frame(
        headless_frame,
        UiPresentationDeadline::at_tick(10),
        0,
    ));
    let transcripts = recorder.observed_transcripts();
    assert_eq!(transcripts.len(), 1);
    assert_eq!(transcripts[0].frame(), headless_publication.frame());
    assert_eq!(transcripts[0].nodes().len(), headless_meaning.node_count);
    assert_eq!(
        transcripts[0].paint_batches().len(),
        headless_meaning.paint_batch_count
    );

    let mut egui_frame = Some(egui_frame);
    let mut egui_outcome = None;
    let native = context.run_ui(raw_input(), |_| {
        egui_outcome = Some(egui.present_prepared_mounted_frame(
            egui_frame.take().expect("egui consumes the frame once"),
            UiPresentationDeadline::at_tick(10),
            0,
        ));
    });
    let egui_publication = published(egui_outcome.expect("egui frame returns one outcome"));
    assert!(
        native.shapes.is_empty(),
        "the independent egui consequence for sealed no-effect meaning is no native shape"
    );
    assert_eq!(egui.current_mounted_publication(), Some(&egui_publication));

    let headless_shutdown = headless.shutdown();
    assert!(matches!(
        headless_shutdown.host_session_release(),
        Some(worth_ui_runtime::facade::host::UiHostSessionReleaseOutcome::Released(receipt))
            if receipt.released_surface_count() == 1
    ));
    let egui_shutdown = egui.shutdown();
    assert!(matches!(
        egui_shutdown.host_session_release(),
        Some(worth_ui_runtime::facade::host::UiHostSessionReleaseOutcome::Released(receipt))
            if receipt.released_surface_count() == 1
    ));
    assert_eq!(egui_host.registered_surface_count(), 0);
}

fn independent_meaning(
    frame: &worth_ui_runtime::facade::mounted::UiPreparedMountedFrame,
) -> IndependentMountedMeaning {
    let projection = frame.surfaces()[0].projection();
    let node = &projection.nodes()[0];
    let participation = node.participation();
    let allocation_omission = match node.allocation() {
        UiMountedAllocationProjection::Omitted(reason) => reason,
        _ => panic!("the authored parity fixture has no committed allocation"),
    };
    IndependentMountedMeaning {
        node_count: projection.nodes().len(),
        paint: participation.paint().status(),
        input: participation.input().status(),
        focus: participation.focus().status(),
        hit_test: participation.hit_test().status(),
        diagnostic: participation.diagnostic().status(),
        allocation_omission,
        paint_batch_count: projection.paint_batches().rows().len(),
        spatial_batch_count: projection.spatial_batches().rows().len(),
        realtime_batch_count: projection.realtime_batches().rows().len(),
    }
}

fn mount_one(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    mode: UiHostSurfacePresentationMode,
) {
    let surface = session
        .create_semantic_surface()
        .expect("semantic surface mints");
    session
        .register_host_surface(surface, mode, profile(1))
        .expect("adapter surface registers");
    let node = first_node(session);
    session
        .mount_instance(node, surface)
        .expect("one graph node mounts");
}

fn prepare(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui_runtime::facade::mounted::UiPreparedMountedFrame {
    session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("active application lends mounted execution"))
        .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
        .expect("sealed mounted meaning prepares")
}

fn published(
    outcome: UiMountedFrameOutcome,
) -> worth_ui_runtime::facade::mounted::UiMountedFramePublicationReceipt {
    match outcome {
        UiMountedFrameOutcome::Published(publication) => publication,
        _ => panic!("adapter must publish the admitted no-effect frame"),
    }
}

fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(800.0, 600.0),
        )),
        ..Default::default()
    }
}
