use worth_ui::facade::measurement_exchange::{
    UiMeasurementEvidenceFamily, UiViewportExtentRequest,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_egui::WorthUiHostEgui;
use worth_ui_runtime::facade::entry::UiMountedAllocationMeasurementRequest;
use worth_ui_runtime::facade::host::{
    UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext,
};
use worth_ui_runtime::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedFrameOutcome, UiMountedFrameRequest,
    UiPresentationDeadline,
};
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiFrameworkTurnCertificationExt,
    WorthUiMountedAllocationCertificationExt, WorthUiMountedFrameExecutionCertificationExt,
    WorthUiMountedIdentityCertificationExt, WorthUiMountedPublicationCertificationExt,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;
use super::mounted_application_lifecycle::known_empty_surface_world::profile;

#[test]
fn in_process_checked_in_pulse_produces_independently_expected_egui_shape() {
    let context = egui::Context::default();
    let host = WorthUiHostEgui::new(context.clone());
    let mut session = launch_and_mount_pulse(host);
    let mut outcome = None;
    let native = context.run(raw_input(), |_| {
        establish_viewport_allocation(&mut session);
        let prepared = session
            .execute_framework_turn(|_| {})
            .expect("no presentation lease is active")
            .into_execution()
            .unwrap_or_else(|_| panic!("pulse ordinary execution is admitted"))
            .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
            .expect("runtime completes the pulse mechanic");
        outcome = Some(session.present_prepared_mounted_frame(
            prepared,
            UiPresentationDeadline::at_tick(10),
            0,
        ));
    });

    let publication = match outcome {
        Some(UiMountedFrameOutcome::Published(publication)) => publication,
        Some(UiMountedFrameOutcome::RejectedBeforeEffects(rejected)) => {
            panic!(
                "native pulse rejected before effects: {:?}",
                rejected.rejections()[0].denial()
            )
        }
        _ => panic!("complete native pulse should publish"),
    };
    assert_eq!(native.shapes.len(), 1);
    assert_native_blue_viewport_rect(&native.shapes[0]);
    let adapter_cost = publication.cost_report().adapter();
    assert_eq!(adapter_cost.presented_surfaces(), 1);
    assert_eq!(adapter_cost.translated_rows(), 6);
    assert_eq!(adapter_cost.translated_bytes(), 560);
    assert_eq!(adapter_cost.native_resource_cache_hits(), 0);
    assert_eq!(adapter_cost.native_resource_cache_misses(), 0);
    assert_eq!(adapter_cost.asynchronous_handoffs(), 0);
    let _ = session.shutdown();
}

#[test]
fn in_process_public_native_shell_launches_without_mounted_construction_apis() {
    let context = egui::Context::default();
    let app = prepare_pulse_application(WorthUiHostEgui::new(context.clone()));
    let mut app = Some(app);
    let mut shell = None;
    let mut published = false;
    let native = context.run(raw_input(), |_| {
        let mut launched = app
            .take()
            .expect("egui callback launches the app once")
            .launch_native_surface()
            .expect("ordinary product facade launches one native surface");
        let _ = launched.generation_identity();
        published = matches!(
            launched.present_frame(10, 0),
            Ok(UiMountedFrameOutcome::Published(_))
        );
        shell = Some(launched);
    });
    assert!(published);
    assert_eq!(native.shapes.len(), 1);
    assert_native_blue_viewport_rect(&native.shapes[0]);
    shell.expect("native shell remains owned").shutdown();
}

#[test]
fn egui_host_replays_admitted_pulse_paint_until_the_shell_releases_it() {
    let context = egui::Context::default();
    let host = WorthUiHostEgui::new(context.clone());
    let app = prepare_pulse_application(host.clone());
    let mut app = Some(app);
    let mut shell = None;
    let first = context.run(raw_input(), |_| {
        let mut launched = app
            .take()
            .expect("pulse app launches once")
            .launch_native_surface()
            .expect("ordinary product facade launches one native surface");
        assert!(matches!(
            launched.present_frame(10, 0),
            Ok(UiMountedFrameOutcome::Published(_))
        ));
        shell = Some(launched);
    });
    assert_native_blue_viewport_rect(&first.shapes[0]);

    let retained = context.run(raw_input(), |_| host.repaint_retained_surfaces());
    assert_eq!(retained.shapes.len(), 1);
    assert_native_blue_viewport_rect(&retained.shapes[0]);

    let shutdown = shell.expect("native shell remains owned").shutdown();
    assert!(shutdown.host_session_released());
    let released = context.run(raw_input(), |_| host.repaint_retained_surfaces());
    assert!(released.shapes.is_empty());
}

fn launch_and_mount_pulse(
    host: WorthUiHostEgui,
) -> worth_ui::facade::app::WorthUiActiveApplicationSession {
    let mut session = prepare_pulse_application(host)
        .launch()
        .expect("query-free file-authored pulse launches");
    let surface = session.create_semantic_surface().unwrap();
    session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::NativeDisplay,
            profile(1),
        )
        .unwrap();
    for node in session.graph().node_identities().collect::<Vec<_>>() {
        let handle = session.mounted_graph_node(node).unwrap();
        session.mount_instance(handle, surface).unwrap();
    }
    session
}

fn prepare_pulse_application(host: WorthUiHostEgui) -> worth_ui::facade::app::WorthUiApp {
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-3-platform-pulse");
    let workspace = FilesystemContractWorkspace::new("phase-3-platform-pulse");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::platform_pulse_source_text(),
    );
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production filesystem provider reads the pulse source");
    let capabilities = scenario.platform_pulse_capability_application(host.clone());
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        snapshot,
        capabilities.capabilities(),
    );
    workspace.close();
    scenario.prepare_platform_pulse_application_with_host(submission, host)
}

fn establish_viewport_allocation(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let capability = session.host_measurement_capability();
    let assumptions = UiHostMeasurementAssumptionProfile::from_capability_report(
        capability.capability_report(),
        1,
        2,
        3,
        4,
    );
    let request = UiMountedAllocationMeasurementRequest::new(
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
        UiHostMeasurementNormalizationContext::viewport_logical_exact(assumptions),
    );
    session
        .establish_mounted_allocation_catalog(1, [request])
        .expect("egui viewport observation establishes the pulse allocation");
}

fn assert_native_blue_viewport_rect(shape: &egui::epaint::ClippedShape) {
    assert_eq!(
        shape.clip_rect,
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 96.0))
    );
    let egui::epaint::Shape::Rect(rect) = &shape.shape else {
        panic!("pulse native effect must be an egui rectangle");
    };
    assert_eq!(
        rect.rect,
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 96.0))
    );
    assert_eq!(rect.fill, egui::Color32::from_rgb(47, 129, 247));
}

fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(160.0, 96.0),
        )),
        ..Default::default()
    }
}
