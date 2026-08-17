use worth_ui::facade::measurement_exchange::{
    UiMeasurementEvidenceFamily, UiViewportExtentRequest,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_contract::UiMountedPaintCommand;
use worth_ui_host_egui::WorthUiHostEgui;
use worth_ui_runtime::facade::entry::UiMountedAllocationMeasurementRequest;
use worth_ui_runtime::facade::host::{
    UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext,
};
use worth_ui_runtime::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedFrameOutcome, UiMountedFrameRequest,
    UiMountedInspectionReceipt, UiMountedInspectionRequest, UiPresentationDeadline,
};
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiFrameworkTurnCertificationExt,
    WorthUiMountedAllocationCertificationExt, WorthUiMountedFrameExecutionCertificationExt,
    WorthUiMountedIdentityCertificationExt, WorthUiMountedPublicationCertificationExt,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;
use super::mounted_application_lifecycle::known_empty_surface_world::profile;
use super::mounted_host_protocol::scripted_host::ScriptedPresentationHost;

#[test]
fn in_process_checked_in_pulse_produces_independently_expected_egui_shape() {
    let context = egui::Context::default();
    let host = WorthUiHostEgui::new(context.clone());
    let mut session = launch_and_mount_pulse(host);
    let mut outcome = None;
    let mut projection_counts = None;
    let native = context.run_ui(raw_input(), |_| {
        establish_viewport_allocation(&mut session);
        let prepared = session
            .execute_framework_turn(|_| {})
            .expect("no presentation lease is active")
            .into_execution()
            .unwrap_or_else(|_| panic!("pulse ordinary execution is admitted"))
            .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
            .expect("runtime completes the pulse mechanic");
        let projection = prepared.surfaces()[0].projection();
        projection_counts = Some([
            projection.nodes().len(),
            projection.clips().rows().len(),
            projection.layers().rows().len(),
            projection.filled_rects().rows().len(),
            projection.hit_tests().rows().len(),
            projection.paint_batches().rows().len(),
            projection.spatial_batches().rows().len(),
            projection.realtime_batches().rows().len(),
            projection.resources().entries().len(),
        ]);
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
    assert_native_pulse(&native.shapes, egui::Color32::from_rgb(47, 129, 247));
    let adapter_cost = publication.cost_report().adapter();
    assert_eq!(projection_counts.unwrap(), [4, 0, 1, 2, 0, 1, 0, 0, 0]);
    assert_eq!(adapter_cost.presented_surfaces(), 1);
    assert_eq!(adapter_cost.translated_rows(), 6);
    assert_eq!(
        adapter_cost.translated_bytes(),
        u64::try_from(2 * std::mem::size_of::<UiMountedPaintCommand>())
            .expect("the exact pulse projection tables fit the cost receipt")
    );
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
    let native = context.run_ui(raw_input(), |_| {
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
    assert_native_pulse(&native.shapes, egui::Color32::from_rgb(47, 129, 247));
    shell.expect("native shell remains owned").shutdown();
}

#[test]
fn egui_host_replays_admitted_pulse_paint_until_the_shell_releases_it() {
    let context = egui::Context::default();
    let host = WorthUiHostEgui::new(context.clone());
    let app = prepare_pulse_application(host.clone());
    let mut app = Some(app);
    let mut shell = None;
    let first = context.run_ui(raw_input(), |_| {
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
    assert_native_pulse(&first.shapes, egui::Color32::from_rgb(47, 129, 247));

    let retained = context.run_ui(raw_input(), |_| host.repaint_retained_surfaces());
    assert_native_pulse(&retained.shapes, egui::Color32::from_rgb(47, 129, 247));

    let shutdown = shell.expect("native shell remains owned").shutdown();
    assert!(shutdown.host_session_released());
    let released = context.run_ui(raw_input(), |_| host.repaint_retained_surfaces());
    assert!(released.shapes.is_empty());
}

#[test]
fn clipped_to_zero_native_delta_advances_without_a_new_physical_epoch() {
    let host = ScriptedPresentationHost::native_display();
    let mut session = prepare_pulse_application(host.clone())
        .launch()
        .expect("the native-display scripted host launches the production pulse graph");
    let surface = session.create_semantic_surface().unwrap();
    session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::NativeDisplay,
            profile(1),
        )
        .unwrap();
    let target_node = session
        .graph()
        .node_identities()
        .find(|node| {
            session
                .graph()
                .lookup()
                .graph_node(*node)
                .is_some_and(|lookup| {
                    lookup
                        .value()
                        .declaration_identity()
                        .authored_semantic_name()
                        == "component:platform.pulse.component.identity_target"
                })
        })
        .expect("the authored inset identity target belongs to the pulse graph");
    let mut target_instance = None;
    for node in session.graph().node_identities().collect::<Vec<_>>() {
        let handle = session.mounted_graph_node(node).unwrap();
        let instance = session.mount_instance(handle, surface).unwrap();
        if node == target_node {
            target_instance = Some(instance);
        }
    }
    establish_viewport_allocation(&mut session);

    let physical_epoch = worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(41);
    host.push_presentation(native_completion(physical_epoch, true));
    let initial = present_prepared(&mut session);
    assert_presentation(&session, initial.frame(), physical_epoch, true);

    session
        .unmount_instance(target_instance.expect("the inset target was mounted"))
        .unwrap();
    host.push_presentation(native_completion(physical_epoch, false));
    let effect_free_delta = present_prepared(&mut session);
    assert_presentation(&session, effect_free_delta.frame(), physical_epoch, false);
    assert_eq!(
        session.inspect_mounted_identity().current_frame(),
        Some(effect_free_delta.frame())
    );

    host.push_presentation(native_completion(physical_epoch, false));
    let unchanged = present_prepared(&mut session);
    assert_presentation(&session, unchanged.frame(), physical_epoch, false);
    assert_eq!(
        session.inspect_mounted_identity().current_frame(),
        Some(unchanged.frame())
    );
    assert_eq!(host.presentation_calls(), 3);
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P3-CLIPPED-DELTA-01\":1}}");
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P3-CLIPPED-DELTA-01\":\"zero-paint-as-indeterminate\"}}"
    );
    let _ = session.shutdown();
}

fn present_prepared(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui_runtime::facade::mounted::UiMountedFramePublicationReceipt {
    let prepared = session
        .execute_framework_turn(|_| {})
        .expect("no presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("the pulse turn remains executable"))
        .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
        .unwrap();
    match session.present_prepared_mounted_frame(prepared, UiPresentationDeadline::at_tick(10), 0) {
        UiMountedFrameOutcome::Published(publication) => publication,
        _ => panic!("native effect-free progression must publish"),
    }
}

fn assert_presentation(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    epoch: worth_ui_host_contract::UiHostPresentationEpoch,
    painted: bool,
) {
    let inspected = match session.inspect_mounted_frame(UiMountedInspectionRequest::frame(frame)) {
        UiMountedInspectionReceipt::Available(inspected) => inspected,
        other => panic!("published native frame must remain inspectable, observed {other:?}"),
    };
    let [surface] = inspected.presentation().surfaces() else {
        panic!("the pulse frame has exactly one native surface");
    };
    assert_eq!(surface.epoch(), epoch);
    assert_eq!(
        surface
            .effects()
            .families()
            .contains(&worth_ui_host_contract::UiMountedEffectFamily::NativePaint),
        painted
    );
}

fn native_completion(
    epoch: worth_ui_host_contract::UiHostPresentationEpoch,
    painted: bool,
) -> worth_ui_host_contract::UiHostSurfacePresentationOutcome {
    let effects = painted
        .then_some(worth_ui_host_contract::UiMountedEffectFamily::NativePaint)
        .into_iter()
        .collect();
    let cost = worth_ui_host_contract::UiHostPresentationCostReport::from_adapter(
        worth_ui_host_contract::UiHostPresentationCostInput {
            presented_surfaces: u64::from(painted),
            presented_pixels: u64::from(painted) * 800 * 600,
            gpu_writes: u64::from(painted),
            render_passes: u64::from(painted),
            surface_copies: u64::from(painted),
            surface_acquisitions: u64::from(painted),
            queue_submissions: u64::from(painted),
            presents: u64::from(painted),
            ..Default::default()
        },
    );
    worth_ui_host_contract::UiHostSurfacePresentationOutcome::Presented(
        worth_ui_host_contract::UiMountedSurfacePresentationCompletion::new(
            UiHostSurfacePresentationMode::NativeDisplay,
            epoch,
            worth_ui_host_contract::UiMountedCompletedEffects::new(effects),
            cost,
        ),
    )
}

pub(super) fn launch_and_mount_pulse(
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

fn prepare_pulse_application<Host>(host: Host) -> worth_ui::facade::app::WorthUiApp
where
    Host: worth_ui_certification::scenario::application_authority_closure::fixed_host::FixedCertificationHostBinding
        + Clone,
{
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

pub(super) fn establish_viewport_allocation(
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

fn assert_native_pulse(shapes: &[egui::epaint::ClippedShape], expected_background: egui::Color32) {
    assert_eq!(shapes.len(), 2);
    let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 96.0));
    let target = egui::Rect::from_min_size(egui::pos2(48.0, 24.0), egui::vec2(64.0, 48.0));
    let observed = shapes
        .iter()
        .map(|shape| {
            let egui::epaint::Shape::Rect(rect) = &shape.shape else {
                panic!("pulse native effects must be egui rectangles");
            };
            assert_eq!(shape.clip_rect, rect.rect);
            (rect.rect, rect.fill)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        observed,
        vec![
            (viewport, expected_background),
            (target, egui::Color32::from_rgb(242, 204, 96)),
        ]
    );
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
