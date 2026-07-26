use std::time::Duration;
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiFrameworkTurnCertificationExt,
};

use worth_ui::facade::app::{
    WorthUiMountedApplicationReplacementOutcome, WorthUiMountedReplacementPreparationOutcome,
    WorthUiVisibleRange,
};
use worth_ui::facade::measurement_exchange::{
    UiMeasurementEvidenceFamily, UiViewportExtentObservation, UiViewportExtentRequest,
};
use worth_ui::facade::source::{WorthUiFilesystemSourceProvider, WorthUiFilesystemSourceWatcher};
use worth_ui_certification::scenario::application_authority_closure::candidate_catalog::admit_candidate_catalog;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_runtime::facade::entry::UiMountedAllocationMeasurementRequest;
use worth_ui_runtime::facade::host::{
    UiHeadlessRecorderCapacity, UiHeadlessUnperformedEffect, UiHostMeasurementAssumptionProfile,
    UiHostMeasurementNeed, UiHostMeasurementNormalizationContext, WorthUiHeadlessRecorder,
    WorthUiHostCapability,
};
use worth_ui_runtime::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedFrameOutcome, UiMountedFrameRequest,
    UiMountedLaneParticipation, UiPresentationDeadline, UiRequiredLaneContributionStatus,
};
use worth_ui_runtime::facade::{WorthUiMountedPreviewDisposition, WorthUiMountedPreviewOutcome};

use crate::filesystem_contract_workspace::FilesystemContractWorkspace;
use crate::mounted_application_lifecycle::adapter_projection_world::{
    preview_target, retire_query, submit_preview,
};
use crate::mounted_application_lifecycle::known_empty_surface_world::profile;

const SETTLEMENT_TIMEOUT: Duration = Duration::from_secs(5);
const SOURCE: &str = "app/main.wui";

#[test]
fn real_file_mount_measure_preview_and_watcher_edit_publish_one_mounted_successor() {
    let workspace = FilesystemContractWorkspace::new("phase-10-mounted-successor");
    workspace.write(
        SOURCE,
        &FilesystemApplicationLifecycleScenario::preview_cross_lane_source_text(false),
    );
    let mut watcher = WorthUiFilesystemSourceWatcher::start(WorthUiFilesystemSourceProvider::new(
        workspace.root(),
    ))
    .expect("production watcher registers the real .wui tree");
    let recorder = WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        UiViewportExtentObservation {
            width: 1024.0,
            height: 768.0,
        },
    );
    let mut scenario = FilesystemApplicationLifecycleScenario::new("phase-10-mounted-successor");
    let capabilities = scenario.preview_cross_lane_capability_application(recorder.clone());
    let initial = watcher
        .take_initial_snapshot()
        .expect("watcher owns the initial settled file bytes");
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        initial,
        capabilities.capabilities(),
    );
    let mut session = scenario
        .prepare_preview_cross_lane_application_with_host(submission, recorder.clone())
        .launch()
        .expect("real file-authored splitter cross-lane application launches");

    let (preview_target, preview_instance) = mount_all_nodes(&mut session);
    establish_first_allocation_catalog(&mut session);
    let preview = publish_preview(&mut session, preview_target, preview_instance);
    admit_query_projection(&mut scenario, &mut session);
    let ordinary = publish_all_lane_frame(&mut session);
    assert_eq!(ordinary.predecessor(), Some(preview.frame()));
    assert_translated_cross_lane_frame(&recorder, 1, ordinary.frame());
    let rust_authored =
        publish_equivalent_rust_authored_successor(&mut scenario, &mut session, &ordinary);
    assert_translated_cross_lane_frame(&recorder, 2, rust_authored.frame());

    workspace.write_atomic(
        SOURCE,
        &FilesystemApplicationLifecycleScenario::preview_cross_lane_source_text(true),
    );
    let settled = watcher
        .settle(SETTLEMENT_TIMEOUT)
        .expect("real operating-system watcher settles the atomic edit");
    let submission =
        FilesystemApplicationLifecycleScenario::lower_snapshot(settled, session.capabilities());
    let mut prepared = session
        .prepare_replacement(submission)
        .expect("watcher successor prepares through the public session");
    prepared
        .admit_candidate_settled_query_projection(scenario.settled_query_projection())
        .expect("candidate independently admits its exact settled Query projection");
    let catalog = admit_candidate_catalog(&session, &mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("watcher successor lowers");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("watcher successor stages");
    let boundary = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("empty turn yields replacement activation authority"))
        .into_activation_boundary();
    let replacement = match session
        .prepare_mounted_replacement(pending, catalog, boundary, None, all_lane_request())
        .expect("mounted watcher successor prepares")
    {
        WorthUiMountedReplacementPreparationOutcome::Prepared(replacement) => replacement,
        WorthUiMountedReplacementPreparationOutcome::SemanticNoOp(_) => {
            panic!("the authored successor component changes application meaning")
        }
    };
    assert_all_execution_lanes(replacement.frame());
    let (application, mounted) = match replacement.present(UiPresentationDeadline::at_tick(40), 3) {
        WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted,
        } => (application, mounted),
        _ => panic!("complete headless presentation must publish one mounted successor"),
    };

    assert_eq!(
        application.active_generation(),
        session.generation_identity()
    );
    assert_eq!(mounted.generation(), session.generation_identity());
    assert_eq!(mounted.predecessor(), Some(rust_authored.frame()));
    assert_eq!(session.current_mounted_publication(), Some(&mounted));
    assert!(session
        .inspect_mounted_identity()
        .mounted_instances()
        .iter()
        .any(|receipt| receipt.identity() == preview_instance));
    assert_translated_cross_lane_frame(&recorder, 3, mounted.frame());

    retire_query(&mut scenario, application.into_operation_live_retirement());
    retire_query(
        &mut scenario,
        session.shutdown().into_operation_live_retirement(),
    );
    let watcher_shutdown = watcher
        .shutdown()
        .expect("production watcher unregisters independently");
    assert!(watcher_shutdown.observed_notification_count() > 0);
    workspace.close();
}

fn publish_equivalent_rust_authored_successor(
    scenario: &mut FilesystemApplicationLifecycleScenario,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    predecessor: &worth_ui_runtime::facade::mounted::UiMountedFramePublicationReceipt,
) -> worth_ui_runtime::facade::mounted::UiMountedFramePublicationReceipt {
    let submission = FilesystemApplicationLifecycleScenario::preview_cross_lane_rust_submission(
        session.capabilities(),
    );
    let mut prepared = session
        .prepare_replacement(submission)
        .expect("equivalent Rust-authored candidate prepares");
    let active_nodes = session.graph().node_identities().collect::<Vec<_>>();
    let candidate_nodes = prepared
        .candidate_graph()
        .node_identities()
        .collect::<Vec<_>>();
    assert_eq!(
        session
            .graph()
            .compare_to(prepared.candidate_graph())
            .kind(),
        worth_ui::facade::graph::UiGraphWorldDifferenceKind::SameWorldSuccessor,
        "equivalent Rust meaning is rebased as the direct graph successor; \
         active={active_nodes:?}, candidate={candidate_nodes:?}"
    );
    prepared
        .admit_candidate_settled_query_projection(scenario.settled_query_projection())
        .expect("equivalent Rust candidate admits the same settled Query projection");
    let catalog = admit_candidate_catalog(&*session, &mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("equivalent Rust candidate lowers");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("equivalent Rust candidate stages");
    let boundary = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("empty turn yields replacement activation authority"))
        .into_activation_boundary();
    let outcome = session
        .prepare_mounted_replacement(pending, catalog, boundary, None, all_lane_request())
        .expect("equivalent Rust candidate reaches one mounted decision");
    let WorthUiMountedReplacementPreparationOutcome::Prepared(replacement) = outcome else {
        panic!("mounted eligibility makes the equivalent authored graph a prepared successor")
    };
    assert_all_execution_lanes(replacement.frame());
    let (application, mounted) = match replacement.present(UiPresentationDeadline::at_tick(30), 2) {
        WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted,
        } => (application, mounted),
        _ => panic!("equivalent Rust-authored mounted successor must publish"),
    };
    assert_eq!(mounted.predecessor(), Some(predecessor.frame()));
    assert_eq!(mounted.generation(), session.generation_identity());
    retire_query(scenario, application.into_operation_live_retirement());
    mounted
}

pub(super) fn mount_all_nodes(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> (
    worth_ui::facade::graph::UiGraphNodeIdentity,
    worth_ui_runtime::facade::mounted::UiMountedInstanceIdentity,
) {
    let surface = session.create_semantic_surface().unwrap();
    session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        )
        .unwrap();
    let target = preview_target(session);
    let nodes = session.graph().node_identities().collect::<Vec<_>>();
    let mut preview_instance = None;
    for node in nodes {
        let handle = session.mounted_graph_node(node).unwrap();
        let instance = session.mount_instance(handle, surface).unwrap();
        if node == target {
            preview_instance = Some(instance);
        }
    }
    (
        target,
        preview_instance.expect("file-authored splitter node is mounted"),
    )
}

pub(super) fn establish_first_allocation_catalog(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let capability = session.host_measurement_capability();
    assert!(capability
        .capability_report()
        .supports(WorthUiHostCapability::ViewportObservation));
    let assumptions = UiHostMeasurementAssumptionProfile::from_capability_report(
        capability.capability_report(),
        1,
        2,
        3,
        4,
    );
    let input = UiMountedAllocationMeasurementRequest::new(
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
        UiHostMeasurementNormalizationContext::viewport_logical_exact(assumptions),
    );
    let receipt = session
        .establish_mounted_allocation_catalog(1, [input])
        .expect("mounted graph and real host measurement establish the first catalog");
    let committed = receipt.committed();
    assert!(!committed.receipts().is_empty());
    assert_eq!(
        usize::from(committed.counters().committed_receipts()),
        committed.receipts().len()
    );
}

fn publish_preview(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    target: worth_ui::facade::graph::UiGraphNodeIdentity,
    instance: worth_ui_runtime::facade::mounted::UiMountedInstanceIdentity,
) -> worth_ui_runtime::facade::mounted::UiMountedFramePublicationReceipt {
    let prepared = submit_preview(session, target, 320.0)
        .prepare(instance)
        .unwrap_or_else(|_| panic!("mounted splitter prepares preview"));
    let resolved = match prepared.present(UiPresentationDeadline::at_tick(10), 0) {
        WorthUiMountedPreviewOutcome::Resolved(resolved) => resolved,
        _ => panic!("headless preview resolves synchronously"),
    };
    match resolved.disposition() {
        WorthUiMountedPreviewDisposition::Published(publication) => publication.clone(),
        _ => panic!("mounted preview publishes before the first edit"),
    }
}

pub(super) fn admit_query_projection(
    scenario: &mut FilesystemApplicationLifecycleScenario,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let projection = scenario.settled_query_projection();
    let link = session
        .query_fact_link("inspector.measurements")
        .expect("file-authored binding resolves");
    drop(
        session
            .execute_framework_turn(|turn| {
                turn.query_projection(|source| {
                    source.admit_settled(projection).unwrap();
                    source.submit_settled(&link).unwrap();
                });
            })
            .expect("query projection enters outside mounted presentation")
            .into_completion(),
    );
}

pub(super) fn publish_all_lane_frame(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui_runtime::facade::mounted::UiMountedFramePublicationReceipt {
    let outcome = session
        .execute_mounted_frame(
            all_lane_request(),
            UiPresentationDeadline::at_tick(20),
            1,
            |_| {},
        )
        .unwrap_or_else(|_| panic!("public mounted facade executes the all-lane frame"));
    match outcome {
        UiMountedFrameOutcome::Published(publication) => {
            assert_all_execution_lanes_from_publication(session, &publication);
            publication
        }
        _ => panic!("first ordinary all-lane frame must publish"),
    }
}

pub(super) fn all_lane_request() -> UiMountedFrameRequest {
    UiMountedFrameRequest::all_bound_surfaces()
        .with_virtualized_range(WorthUiVisibleRange::rows(0, 1).unwrap())
}

fn assert_all_execution_lanes(frame: &worth_ui_runtime::facade::mounted::UiPreparedMountedFrame) {
    for expected in [
        UiMountedLaneParticipation::Ordinary,
        UiMountedLaneParticipation::Virtualized,
        UiMountedLaneParticipation::CanvasSpatial,
        UiMountedLaneParticipation::Realtime,
    ] {
        assert!(frame.manifest().lane_contributions().iter().any(|cell| {
            cell.lane() == expected && cell.status() == UiRequiredLaneContributionStatus::Admitted
        }));
    }
}

fn assert_all_execution_lanes_from_publication(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    publication: &worth_ui_runtime::facade::mounted::UiMountedFramePublicationReceipt,
) {
    let inspection = session.inspect_mounted_identity();
    let frame = inspection
        .frame_receipts()
        .iter()
        .find(|receipt| receipt.frame_identity() == publication.frame())
        .expect("published frame remains inspectable");
    assert_eq!(frame.frame_identity(), publication.frame());
}

fn assert_translated_cross_lane_frame(
    recorder: &WorthUiHeadlessRecorder,
    transcript_index: usize,
    frame: worth_ui_runtime::facade::mounted::UiMountedFrameIdentity,
) {
    let transcripts = recorder.observed_transcripts();
    let transcript = &transcripts[transcript_index];
    assert_eq!(transcript.frame(), frame);
    assert!(transcript
        .unperformed_effects()
        .iter()
        .any(|effect| matches!(
            effect,
            UiHeadlessUnperformedEffect::CanvasSpatial {
                primitive_count: 64,
                ..
            }
        )));
    assert!(transcript
        .unperformed_effects()
        .iter()
        .any(|effect| matches!(
            effect,
            UiHeadlessUnperformedEffect::Realtime {
                overlay_row_count: 2,
                ..
            }
        )));
}
use worth_ui_test_support::WorthUiMountedAllocationCertificationExt;
