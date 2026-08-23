use worth_ui::facade::app::WorthUiVisibleRange;
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_contract::{UiMountedParticipationInput, WorthUiHeadlessMountedProjectionRecord};
use worth_ui_host_egui::{
    WorthUiEguiMountedProjectionPreparation, WorthUiEguiMountedResourceCache,
};
use worth_ui_host_headless::WorthUiHeadlessHost;
use worth_ui_runtime::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedAccessibilityProjection, UiMountedDiagnosticProjection,
    UiMountedFrameRequest, UiMountedOmissionReason, UiMountedParticipation,
    UiMountedParticipationFact, UiMountedParticipationStatus, UiMountedProjectionAudience,
    UiPreparedMountedFrame, WorthUiHeadlessMountedResourceCache,
};
use worth_ui_test_support::WorthUiMountedFrameExecutionCertificationExt;
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiFrameworkTurnCertificationExt,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;
use super::mounted_application_lifecycle::known_empty_surface_world::{
    first_node, mounted_application_with_host, profile,
};

#[test]
fn candidate_projection_is_effect_free_unpublished_and_audience_scoped() {
    let mut session =
        mounted_application_with_host("mounted-projection-audience", WorthUiHeadlessHost)
            .launch()
            .expect("real file-authored application launches");
    let accessibility_surface = session
        .create_semantic_surface_for(UiMountedProjectionAudience::new(true, false))
        .unwrap();
    let diagnostic_surface = session
        .create_semantic_surface_for(UiMountedProjectionAudience::new(false, true))
        .unwrap();
    let accessibility_binding = register(&mut session, accessibility_surface, 1);
    let diagnostic_binding = register(&mut session, diagnostic_surface, 1);
    let node = first_node(&session);
    session.mount_instance(node, accessibility_surface).unwrap();
    session.mount_instance(node, diagnostic_surface).unwrap();

    let candidate = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("empty source turn permits execution"))
        .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
        .expect("active truth prepares one mounted candidate");

    assert!(candidate.is_unpublished());
    assert!(session.inspect_mounted_identity().current_frame().is_none());
    let accessibility = projection_for(&candidate, accessibility_binding);
    let diagnostic = projection_for(&candidate, diagnostic_binding);
    assert_eq!(accessibility.nodes().len(), 1);
    assert_eq!(diagnostic.nodes().len(), 1);
    assert_eq!(
        accessibility.nodes()[0].diagnostic(),
        UiMountedDiagnosticProjection::Omitted(UiMountedOmissionReason::SurfacePolicyWithheld)
    );
    assert_eq!(
        diagnostic.nodes()[0].accessibility(),
        UiMountedAccessibilityProjection::Omitted(UiMountedOmissionReason::SurfacePolicyWithheld)
    );
    assert_adapter_participation(accessibility, authored_root_participation());
}

#[test]
fn canvas_bulk_storage_and_native_caches_do_not_scale_with_primitives() {
    let (mut scenario, workspace, mut session) = cross_lane_session();
    let surface = session.create_semantic_surface().unwrap();
    let first_binding = register(&mut session, surface, 1);
    mount_every_graph_node(&mut session, surface);
    admit_query_projection(&mut scenario, &mut session);
    let first = canvas_candidate(&mut session);
    let first_view = projection_for(&first, first_binding);
    let primitive_count = first_view.spatial_batches().rows()[0].primitive_count();

    assert!(primitive_count > first_view.nodes().len() as u32);
    assert_eq!(first_view.spatial_batches().rows().len(), 1);
    assert_eq!(first_view.resources().entries().len(), 1);
    let content_identity = first_view.resources().entries()[0].content_identity();
    let mut headless_cache = WorthUiHeadlessMountedResourceCache::default();
    let mut egui_cache = WorthUiEguiMountedResourceCache::default();
    headless_cache.reconcile(first_view).unwrap();
    egui_cache.reconcile(first_view).unwrap();
    let first_headless = headless_cache.handle_for(content_identity).unwrap();
    let first_egui = egui_cache.handle_for(content_identity).unwrap();

    let second_binding = session
        .rebind_host_surface(
            first_binding,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(2),
        )
        .unwrap()
        .binding_generation();
    let second = canvas_candidate(&mut session);
    let second_view = projection_for(&second, second_binding);
    assert_eq!(
        second_view.resources().entries()[0].content_identity(),
        content_identity
    );
    headless_cache.reconcile(second_view).unwrap();
    egui_cache.reconcile(second_view).unwrap();
    assert_ne!(
        headless_cache.handle_for(content_identity).unwrap(),
        first_headless
    );
    assert_ne!(egui_cache.handle_for(content_identity).unwrap(), first_egui);
    drop((first, second));
    let _ = session.shutdown();
    workspace.close();
}

#[test]
fn all_sealed_lanes_lower_to_specialized_tables_without_host_calls() {
    let (mut scenario, workspace, mut session) = cross_lane_session();
    let surface = session.create_semantic_surface().unwrap();
    let binding = register(&mut session, surface, 1);
    mount_every_graph_node(&mut session, surface);
    admit_query_projection(&mut scenario, &mut session);
    let candidate = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("empty source turn permits cross-lane execution"))
        .prepare_mounted_frame(
            UiMountedFrameRequest::all_bound_surfaces()
                .with_virtualized_range(WorthUiVisibleRange::rows(0, 1).unwrap()),
        )
        .unwrap();
    let view = projection_for(&candidate, binding);

    assert_eq!(view.paint_batches().rows().len(), 4);
    assert_eq!(view.spatial_batches().rows().len(), 1);
    assert_eq!(view.realtime_batches().rows().len(), 1);
    drop(candidate);
    let _ = session.shutdown();
    workspace.close();
}

fn register(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    surface: worth_ui_runtime::facade::mounted::UiSemanticSurfaceIdentity,
    epoch: u64,
) -> worth_ui_runtime::facade::mounted::UiSurfaceBindingGeneration {
    session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(epoch),
        )
        .unwrap()
        .binding_generation()
}

fn assert_adapter_participation(
    view: &worth_ui_runtime::facade::mounted::UiMountedProjectionView,
    expected: UiMountedParticipation,
) {
    let headless = WorthUiHeadlessMountedProjectionRecord::observe(view);
    let egui = WorthUiEguiMountedProjectionPreparation::prepare(view);
    assert_eq!(headless.nodes().len(), 1);
    assert_eq!(egui.nodes().len(), 1);
    assert_eq!(headless.nodes()[0].participation(), expected);
    assert_eq!(egui.nodes()[0].participation(), expected);
    assert_eq!(
        headless.nodes()[0].mounted_instance(),
        egui.nodes()[0].mounted_instance()
    );
}

fn authored_root_participation() -> UiMountedParticipation {
    let deferred = UiMountedParticipationFact::new(UiMountedParticipationStatus::Deferred);
    UiMountedParticipation::new(UiMountedParticipationInput {
        paint: deferred,
        clip: deferred,
        input: deferred,
        focus: deferred,
        hit_test: deferred,
        accessibility: deferred,
        motion: deferred,
        diagnostic: UiMountedParticipationFact::new(UiMountedParticipationStatus::Withheld),
    })
}

fn cross_lane_session() -> (
    FilesystemApplicationLifecycleScenario,
    FilesystemContractWorkspace,
    worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let scenario = FilesystemApplicationLifecycleScenario::new("mounted-projection-cross-lane");
    let workspace = FilesystemContractWorkspace::new("mounted-projection-cross-lane");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::cross_lane_source_text(),
    );
    let capabilities = scenario.cross_lane_capability_application(WorthUiHeadlessHost);
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .unwrap(),
        capabilities.capabilities(),
    );
    let session = scenario
        .prepare_cross_lane_application_with_host(submission, WorthUiHeadlessHost)
        .launch()
        .unwrap();
    (scenario, workspace, session)
}

fn mount_every_graph_node(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    surface: worth_ui_runtime::facade::mounted::UiSemanticSurfaceIdentity,
) {
    let nodes = session.graph().node_identities().collect::<Vec<_>>();
    for node in nodes {
        let handle = session.mounted_graph_node(node).unwrap();
        session.mount_instance(handle, surface).unwrap();
    }
}

fn admit_query_projection(
    scenario: &mut FilesystemApplicationLifecycleScenario,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let projection = scenario.settled_query_projection();
    let link = session.query_fact_link("inspector.measurements").unwrap();
    let completion = session
        .execute_framework_turn(|turn| {
            turn.query_projection(|source| {
                source.admit_settled(projection).unwrap();
                source.submit_settled(&link).unwrap();
            });
        })
        .expect("no mounted presentation lease is active");
    drop(completion.into_completion());
}

fn canvas_candidate(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> UiPreparedMountedFrame {
    session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("empty source turn permits canvas execution"))
        .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
        .unwrap()
}

fn projection_for(
    frame: &UiPreparedMountedFrame,
    binding: worth_ui_runtime::facade::mounted::UiSurfaceBindingGeneration,
) -> &worth_ui_runtime::facade::mounted::UiMountedProjectionView {
    frame
        .surfaces()
        .iter()
        .find(|surface| surface.requirement().binding() == binding)
        .expect("prepared manifest contains requested binding")
        .projection()
}
