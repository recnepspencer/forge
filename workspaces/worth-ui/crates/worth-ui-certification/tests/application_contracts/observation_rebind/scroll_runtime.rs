#[path = "scroll_runtime/replacement.rs"]
mod replacement;
#[path = "scroll_runtime/scenario.rs"]
mod scenario;
#[path = "scroll_runtime/shared_owner_replacement.rs"]
mod shared_owner_replacement;

use scenario::{
    admit_scroll, publish_predecessor, publish_with_hit_coordinate, scroll_visual_source,
    with_scroll_mosaic,
};
use worth_ui::facade::observation_report::UiHostObservationPresentationBasis;
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_contract::{
    UiHostScrollDeltaPhase, UiHostScrollDeltaPrecision, UiHostScrollDeltaSource,
    UiHostScrollDeltaTargetAffinity,
};
use worth_ui_host_headless::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};
use worth_ui_test_support::{
    UiScrollObservationCertificationDenial, UiScrollObservationCertificationOutcome,
    WorthUiMountedIdentityCertificationExt, WorthUiServiceStateCertificationExt,
};

use crate::filesystem_contract_workspace::FilesystemContractWorkspace;
use crate::filesystem_mounted_world::{component_graph_nodes, establish_allocation};
use crate::mounted_application_lifecycle::known_empty_surface_world::profile;

#[test]
fn exact_coordinate_scroll_retains_sign_cancellation_and_ambiguous_denial() {
    let scenario = FilesystemApplicationLifecycleScenario::new("phase-315-scroll-ingress");
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
            .expect("scroll visual capabilities freeze");
    let workspace = FilesystemContractWorkspace::new("phase-315-scroll-ingress");
    workspace.write("app/main.wui", &scroll_visual_source());
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production source provider reads the scroll visual source");
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        snapshot,
        capabilities.capabilities(),
    );
    let application =
        with_scroll_mosaic(scenario.visual_identity_application_builder(recorder.clone()))
            .with_candidate_submission(submission)
            .freeze()
            .expect("source-authored scroll visual application freezes");
    workspace.close();
    let component_nodes = component_graph_nodes(&application);
    let scroll_target_node = component_nodes[1];
    let mut session = application
        .launch()
        .expect("mosaic-authored application launches");
    let surface = session.create_semantic_surface().unwrap();
    let binding = session
        .register_host_surface(
            surface,
            worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        )
        .unwrap()
        .binding_generation();
    let mut scroll_target = None;
    for graph_node in component_nodes {
        let handle = session.mounted_graph_node(graph_node).unwrap();
        let mounted = session.mount_instance(handle, surface).unwrap();
        if graph_node == scroll_target_node {
            scroll_target = Some(mounted);
        }
    }
    establish_allocation(&mut session, 3);
    publish_predecessor(&mut session);
    let (current, coordinate) = publish_with_hit_coordinate(
        &mut session,
        binding,
        scroll_target.expect("hit-only component is mounted"),
    );
    let presentation = UiHostObservationPresentationBasis::new(
        current.host_surface,
        current.frame,
        binding,
        current.epoch,
    );
    let ownership_before_deltas = session.inspect_scroll_runtime_for_certification();
    assert!(ownership_before_deltas.ownership_resolutions() > 0);
    assert!(ownership_before_deltas.ownership_graph_nodes_visited() > 0);
    assert!(ownership_before_deltas.ownership_plan_nodes_visited() > 0);
    let updated = admit_scroll(
        &mut session,
        binding,
        &current,
        1,
        UiHostScrollDeltaPhase::Updated,
        UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, coordinate),
        125,
        -250,
    );
    assert_eq!(
        updated,
        UiScrollObservationCertificationOutcome::Applied {
            source: UiHostScrollDeltaSource::PointerWheel,
            phase: UiHostScrollDeltaPhase::Updated,
            precision: UiHostScrollDeltaPrecision::Pixel,
            requested_inline_subpixels: -125,
            requested_block_subpixels: 250,
            owners_visited: 2,
        }
    );

    let cancelled = admit_scroll(
        &mut session,
        binding,
        &current,
        2,
        UiHostScrollDeltaPhase::Cancelled,
        UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, coordinate),
        500,
        -750,
    );
    assert_eq!(
        cancelled,
        UiScrollObservationCertificationOutcome::Applied {
            source: UiHostScrollDeltaSource::PointerWheel,
            phase: UiHostScrollDeltaPhase::Cancelled,
            precision: UiHostScrollDeltaPrecision::Pixel,
            requested_inline_subpixels: 0,
            requested_block_subpixels: 0,
            owners_visited: 1,
        }
    );

    let saturated = admit_scroll(
        &mut session,
        binding,
        &current,
        3,
        UiHostScrollDeltaPhase::Updated,
        UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, coordinate),
        0,
        -1_000_000_000,
    );
    assert_eq!(
        saturated,
        UiScrollObservationCertificationOutcome::Applied {
            source: UiHostScrollDeltaSource::PointerWheel,
            phase: UiHostScrollDeltaPhase::Updated,
            precision: UiHostScrollDeltaPrecision::Pixel,
            requested_inline_subpixels: 0,
            requested_block_subpixels: 1_000_000_000,
            owners_visited: 2,
        }
    );
    let saturated_geometry = session.inspect_scroll_runtime_for_certification();
    assert_eq!(saturated_geometry.owner_geometry().len(), 2);
    assert!(
        saturated_geometry.owner_geometry().iter().all(|owner| {
            owner.graph_node_digest() == Some(scroll_target_node.digest())
                && owner.plan_region_index().is_some()
                && owner.max_block_subpixels() > 0
                && owner.block_offset_subpixels() == owner.max_block_subpixels()
        }),
        "nested owner geometry: {:?}",
        saturated_geometry.owner_geometry(),
    );
    assert_ne!(
        saturated_geometry.owner_geometry()[0].plan_region_index(),
        saturated_geometry.owner_geometry()[1].plan_region_index(),
    );

    let returned = admit_scroll(
        &mut session,
        binding,
        &current,
        4,
        UiHostScrollDeltaPhase::Updated,
        UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, coordinate),
        0,
        1_000_000_000,
    );
    assert_eq!(
        returned,
        UiScrollObservationCertificationOutcome::Applied {
            source: UiHostScrollDeltaSource::PointerWheel,
            phase: UiHostScrollDeltaPhase::Updated,
            precision: UiHostScrollDeltaPrecision::Pixel,
            requested_inline_subpixels: 0,
            requested_block_subpixels: -1_000_000_000,
            owners_visited: 2,
        }
    );
    let returned_geometry = session.inspect_scroll_runtime_for_certification();
    assert!(returned_geometry
        .owner_geometry()
        .iter()
        .all(|owner| owner.block_offset_subpixels() == 0));

    let inner = returned_geometry
        .owner_geometry()
        .iter()
        .min_by_key(|owner| owner.plan_region_index())
        .copied()
        .expect("nested scroll geometry contains the inner owner");
    let outer = returned_geometry
        .owner_geometry()
        .iter()
        .max_by_key(|owner| owner.plan_region_index())
        .copied()
        .expect("nested scroll geometry contains the outer owner");
    assert!(inner.max_block_subpixels() > 1);
    assert!(outer.max_block_subpixels() > 1);
    let outer_share = (outer.max_block_subpixels() / 2).max(1);
    assert!(outer_share < outer.max_block_subpixels());
    let routed_delta = inner
        .max_block_subpixels()
        .checked_add(outer_share)
        .expect("nested routed delta remains representable");

    let partially_routed = admit_scroll(
        &mut session,
        binding,
        &current,
        5,
        UiHostScrollDeltaPhase::Updated,
        UiHostScrollDeltaTargetAffinity::exact_coordinate(presentation, coordinate),
        0,
        -routed_delta,
    );
    assert_eq!(
        partially_routed,
        UiScrollObservationCertificationOutcome::Applied {
            source: UiHostScrollDeltaSource::PointerWheel,
            phase: UiHostScrollDeltaPhase::Updated,
            precision: UiHostScrollDeltaPrecision::Pixel,
            requested_inline_subpixels: 0,
            requested_block_subpixels: routed_delta,
            owners_visited: 2,
        }
    );
    let partially_routed_geometry = session.inspect_scroll_runtime_for_certification();
    let routed_inner = partially_routed_geometry
        .owner_geometry()
        .iter()
        .find(|owner| owner.plan_region_index() == inner.plan_region_index())
        .copied()
        .expect("inner owner remains observable after partial routing");
    let routed_outer = partially_routed_geometry
        .owner_geometry()
        .iter()
        .find(|owner| owner.plan_region_index() == outer.plan_region_index())
        .copied()
        .expect("outer owner remains observable after partial routing");
    assert_eq!(
        routed_inner.block_offset_subpixels(),
        inner.max_block_subpixels()
    );
    assert_eq!(routed_outer.block_offset_subpixels(), outer_share);

    let fallback = admit_scroll(
        &mut session,
        binding,
        &current,
        6,
        UiHostScrollDeltaPhase::Updated,
        UiHostScrollDeltaTargetAffinity::presented_surface_fallback(presentation),
        1,
        1,
    );
    assert_eq!(
        fallback,
        UiScrollObservationCertificationOutcome::Denied(
            UiScrollObservationCertificationDenial::PresentedSurfaceFallbackIsAmbiguous,
        )
    );
    let ownership_after_deltas = session.inspect_scroll_runtime_for_certification();
    assert_eq!(
        ownership_after_deltas.ownership_resolutions(),
        ownership_before_deltas.ownership_resolutions(),
        "host delta routing must use the mounted ownership index"
    );
    assert_eq!(
        ownership_after_deltas.ownership_graph_nodes_visited(),
        ownership_before_deltas.ownership_graph_nodes_visited(),
        "host delta routing must not rediscover ownership through the graph"
    );
    assert_eq!(
        ownership_after_deltas.ownership_plan_nodes_visited(),
        ownership_before_deltas.ownership_plan_nodes_visited(),
        "host delta routing must not rediscover ownership through mosaic plan ranges"
    );

    let _ = session.shutdown();
    drop(capabilities);
}
