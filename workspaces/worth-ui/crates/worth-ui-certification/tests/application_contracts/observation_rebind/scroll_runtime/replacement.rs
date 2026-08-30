use super::scenario::{
    admit_scroll, publish_predecessor, publish_with_hit_coordinate,
    reduced_sibling_scroll_visual_source, sibling_scroll_visual_source, with_scroll_mosaic,
};
use worth_ui::facade::app::{
    WorthUiMountedApplicationReplacementOutcome, WorthUiMountedReplacementPreparationOutcome,
};
use worth_ui::facade::observation_report::{
    UiHostObservationPresentationBasis, UiHostSurfacePosition,
    UI_HOST_SURFACE_POSITION_SUBPIXELS_PER_UNIT,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_contract::{
    UiHostScrollDeltaPhase, UiHostScrollDeltaPrecision, UiHostScrollDeltaSource,
    UiHostScrollDeltaTargetAffinity,
};
use worth_ui_host_headless::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_runtime::facade::mounted::{UiMountedFrameRequest, UiPresentationDeadline};
use worth_ui_test_support::{
    UiScrollObservationCertificationOutcome, WorthUiMountedIdentityCertificationExt,
    WorthUiServiceStateCertificationExt,
};

use crate::filesystem_contract_workspace::FilesystemContractWorkspace;
use crate::filesystem_mounted_world::{component_graph_nodes, establish_allocation};
use crate::mounted_application_lifecycle::known_empty_surface_world::profile;
use crate::mounted_application_lifecycle::published_mounted_world::{
    presented_epoch, PresentedObservationBasis,
};
use crate::mounted_publication::stage_replacement;

#[test]
fn mounted_replacement_retires_one_live_scroll_sibling_and_routes_the_retained_sibling() {
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-315-scroll-replacement");
    let recorder = WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        worth_ui::facade::measurement_exchange::UiViewportExtentObservation {
            width: 160.0,
            height: 96.0,
        },
    );
    let capabilities =
        with_scroll_mosaic(scenario.visual_identity_application_builder(recorder.clone()))
            .freeze()
            .expect("scroll replacement capabilities freeze");
    let initial_workspace = FilesystemContractWorkspace::new("phase-315-scroll-replacement");
    initial_workspace.write("app/main.wui", &sibling_scroll_visual_source());
    let snapshot = WorthUiFilesystemSourceProvider::new(initial_workspace.root())
        .read()
        .expect("production source provider reads sibling scroll source");
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        snapshot,
        capabilities.capabilities(),
    );
    let application =
        with_scroll_mosaic(scenario.visual_identity_application_builder(recorder.clone()))
            .with_candidate_submission(submission)
            .freeze()
            .expect("sibling scroll application freezes");
    initial_workspace.close();

    let component_nodes = component_graph_nodes(&application);
    let retained_graph_node = component_nodes[1];
    let reduced_graph_node = component_nodes[2];
    let mut session = application.launch().expect("scroll application launches");
    let surface = session.create_semantic_surface().unwrap();
    let binding = session
        .register_host_surface(
            surface,
            worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        )
        .unwrap()
        .binding_generation();
    let mut retained_target = None;
    for graph_node in component_nodes {
        let handle = session.mounted_graph_node(graph_node).unwrap();
        let mounted = session.mount_instance(handle, surface).unwrap();
        if graph_node == retained_graph_node {
            retained_target = Some(mounted);
        }
    }
    establish_allocation(&mut session, 3);
    publish_predecessor(&mut session);
    let _ = publish_with_hit_coordinate(
        &mut session,
        binding,
        retained_target.expect("retained scroll target is mounted"),
    );
    let before = session.inspect_scroll_runtime_for_certification();
    assert_eq!(before.owner_geometry().len(), 4);
    assert_eq!(
        before
            .owner_geometry()
            .iter()
            .filter(|row| row.graph_node_digest() == Some(retained_graph_node.digest()))
            .count(),
        2
    );
    assert_eq!(
        before
            .owner_geometry()
            .iter()
            .filter(|row| row.graph_node_digest() == Some(reduced_graph_node.digest()))
            .count(),
        2
    );
    let expected_retained_geometry = before
        .owner_geometry()
        .iter()
        .filter(|row| row.graph_node_digest() == Some(retained_graph_node.digest()))
        .copied()
        .collect::<Vec<_>>();

    let replacement_workspace = FilesystemContractWorkspace::new("phase-315-scroll-successor");
    replacement_workspace.write("app/main.wui", &reduced_sibling_scroll_visual_source());
    let (pending, catalog, boundary) = stage_replacement(&replacement_workspace, &mut session);
    let prepared = match session
        .prepare_mounted_replacement(
            pending,
            catalog,
            boundary,
            None,
            UiMountedFrameRequest::all_bound_surfaces(),
        )
        .unwrap()
    {
        WorthUiMountedReplacementPreparationOutcome::Prepared(prepared) => prepared,
        WorthUiMountedReplacementPreparationOutcome::SemanticNoOp(_) => {
            panic!("removing one live scroll owner requires replacement")
        }
    };
    let hit_row = prepared.frame().surfaces()[0]
        .projection()
        .hit_tests()
        .rows()
        .iter()
        .find(|row| row.mounted_instance() == retained_target.unwrap())
        .copied()
        .expect("retained scroll target remains hit-testable in the successor");
    let unit = UI_HOST_SURFACE_POSITION_SUBPIXELS_PER_UNIT as f32;
    let coordinate = UiHostSurfacePosition::viewport_logical(
        ((hit_row.clip_bounds().x() + 1.0) * unit).round() as i64,
        ((hit_row.clip_bounds().y() + 1.0) * unit).round() as i64,
    );
    let mounted = match prepared.present(UiPresentationDeadline::at_tick(2_000), 1) {
        WorthUiMountedApplicationReplacementOutcome::Published { mounted, .. } => mounted,
        _ => panic!("complete scroll successor publishes"),
    };
    replacement_workspace.close();

    let after = session.inspect_scroll_runtime_for_certification();
    assert_eq!(after.owner_geometry().len(), 3);
    assert_eq!(
        after
            .owner_geometry()
            .iter()
            .filter(|row| row.graph_node_digest() == Some(retained_graph_node.digest()))
            .copied()
            .collect::<Vec<_>>(),
        expected_retained_geometry,
        "the unaffected live sibling keeps both of its installed owners"
    );
    assert_eq!(
        after
            .owner_geometry()
            .iter()
            .filter(|row| row.graph_node_digest() == Some(reduced_graph_node.digest()))
            .count(),
        1,
        "replacement retires only the removed nested owner"
    );
    let mut expected_mounted = session
        .inspect_mounted_identity()
        .mounted_instances()
        .iter()
        .map(|entry| entry.identity().diagnostic_value())
        .collect::<Vec<_>>();
    expected_mounted.sort_unstable();
    assert_eq!(after.ownership_mounted_instances(), expected_mounted);
    assert_eq!(
        after.ownership_resolutions(),
        before.ownership_resolutions()
            + u64::try_from(expected_mounted.len()).expect("bounded mounted count")
    );
    assert!(after.ownership_graph_nodes_visited() > before.ownership_graph_nodes_visited());
    assert!(after.ownership_plan_nodes_visited() > before.ownership_plan_nodes_visited());

    let binding_view = session.inspect_mounted_identity().surface_bindings()[0];
    let current = PresentedObservationBasis {
        host_surface: binding_view.host_surface_identity(),
        frame: mounted.frame(),
        epoch: presented_epoch(&session, mounted.frame(), binding),
        instance: hit_row.mounted_instance(),
        receipt: hit_row.node_receipt(),
    };
    let presentation = UiHostObservationPresentationBasis::new(
        current.host_surface,
        current.frame,
        binding,
        current.epoch,
    );
    let resolution_before_delta = after.ownership_resolutions();
    assert_eq!(
        admit_scroll(
            &mut session,
            binding,
            &current,
            1,
            UiHostScrollDeltaPhase::Updated,
            UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, coordinate),
            0,
            -1_000_000_000,
        ),
        UiScrollObservationCertificationOutcome::Applied {
            source: UiHostScrollDeltaSource::PointerWheel,
            phase: UiHostScrollDeltaPhase::Updated,
            precision: UiHostScrollDeltaPrecision::Pixel,
            requested_inline_subpixels: 0,
            requested_block_subpixels: 1_000_000_000,
            owners_visited: 2,
        }
    );
    assert_eq!(
        session
            .inspect_scroll_runtime_for_certification()
            .ownership_resolutions(),
        resolution_before_delta,
        "the retained sibling routes immediately without rediscovery"
    );
    let _ = session.shutdown();
    drop(capabilities);
}
