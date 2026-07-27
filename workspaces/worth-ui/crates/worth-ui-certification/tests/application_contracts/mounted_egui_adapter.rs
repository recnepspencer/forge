use worth_ui_host_egui::WorthUiHostEgui;
use worth_ui_runtime::facade::host::{WorthUiHostCapability, WorthUiOperationalHostAdapter};
use worth_ui_runtime::facade::mounted::{
    UiHostSurfacePresentationDenial, UiHostSurfacePresentationMode, UiMountedEffectFamily,
    UiMountedFrameOutcome, UiMountedFrameRequest, UiMountedPaintProjection,
    UiMountedParticipationStatus, UiPresentationDeadline,
};
use worth_ui_test_support::WorthUiFrameworkTurnCertificationExt;
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;
use worth_ui_test_support::{
    WorthUiMountedFrameExecutionCertificationExt, WorthUiMountedPublicationCertificationExt,
};

use super::mounted_application_lifecycle::known_empty_surface_world::{
    first_node, mounted_application_with_host, profile,
};

#[test]
fn real_wui_no_effect_frame_publishes_without_synthetic_egui_shapes() {
    let context = egui::Context::default();
    let host = WorthUiHostEgui::new(context.clone());
    assert_eq!(
        host.operational_capability_report().observed_capabilities(),
        &[
            WorthUiHostCapability::DpiObservation,
            WorthUiHostCapability::NativePaint,
            WorthUiHostCapability::ViewportObservation,
        ]
    );
    let mut session = mounted_application_with_host("mounted-egui-empty", host.clone())
        .launch()
        .expect("real file-authored application launches");
    let surface = session.create_semantic_surface().unwrap();
    session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::NativeDisplay,
            profile(1),
        )
        .unwrap();
    assert_eq!(host.registered_surface_count(), 1);
    let node = first_node(&session);
    session.mount_instance(node, surface).unwrap();
    let frame = prepare(&mut session);
    assert_no_native_effect_precondition(&frame);

    let mut frame = Some(frame);
    let mut outcome = None;
    let native = context.run(raw_input(), |_| {
        outcome = Some(session.present_prepared_mounted_frame(
            frame.take().expect("egui callback consumes the frame once"),
            UiPresentationDeadline::at_tick(10),
            0,
        ));
    });
    assert!(matches!(outcome, Some(UiMountedFrameOutcome::Published(_))));
    assert!(
        native.shapes.is_empty(),
        "a no-effect mounted frame must not produce an adapter-owned debug shape"
    );
    let shutdown = session.shutdown();
    assert!(matches!(
        shutdown.host_session_release(),
        Some(worth_ui_runtime::facade::host::UiHostSessionReleaseOutcome::Released(receipt))
            if receipt.released_surface_count() == 1
    ));
    assert_eq!(host.registered_surface_count(), 0);
}

#[test]
fn real_wui_record_only_mode_denies_before_egui_effects() {
    let context = egui::Context::default();
    let host = WorthUiHostEgui::new(context.clone());
    let mut session = mounted_application_with_host("mounted-egui-recording-denial", host)
        .launch()
        .expect("real file-authored application launches");
    let surface = session.create_semantic_surface().unwrap();
    session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        )
        .unwrap();
    let node = first_node(&session);
    session.mount_instance(node, surface).unwrap();
    let frame = prepare(&mut session);
    assert_eq!(
        frame.surfaces()[0].requirement().presentation_mode(),
        UiHostSurfacePresentationMode::RecordOnly
    );

    let mut frame = Some(frame);
    let mut outcome = None;
    let native = context.run(raw_input(), |_| {
        outcome = Some(session.present_prepared_mounted_frame(
            frame.take().expect("egui callback consumes the frame once"),
            UiPresentationDeadline::at_tick(10),
            0,
        ));
    });
    let rejected = match outcome {
        Some(UiMountedFrameOutcome::RejectedBeforeEffects(rejected)) => rejected,
        _ => panic!("external native mechanics must reject before egui effects"),
    };
    assert_eq!(
        rejected.rejections()[0].denial(),
        UiHostSurfacePresentationDenial::UnsupportedEffect(
            UiMountedEffectFamily::RecordedProjection
        )
    );
    assert!(native.shapes.is_empty());
    assert!(session.inspect_mounted_identity().current_frame().is_none());
    let _ = session.shutdown();
}

fn assert_no_native_effect_precondition(
    frame: &worth_ui_runtime::facade::mounted::UiPreparedMountedFrame,
) {
    assert_eq!(frame.surfaces().len(), 1);
    let projection = frame.surfaces()[0].projection();
    assert!(projection
        .nodes()
        .iter()
        .all(|node| matches!(node.paint(), UiMountedPaintProjection::Omitted(_))));
    assert!(
        !projection.paint_batches().rows().is_empty(),
        "the control must contain count-only paint evidence that egui refuses to invent"
    );
    assert!(projection.nodes().iter().all(|node| {
        node.participation().focus().status() != UiMountedParticipationStatus::Admitted
    }));
    assert!(projection.spatial_batches().rows().is_empty());
    assert!(projection.realtime_batches().rows().is_empty());
}

fn prepare(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui_runtime::facade::mounted::UiPreparedMountedFrame {
    session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("ordinary mounted execution is admitted"))
        .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
        .unwrap()
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
