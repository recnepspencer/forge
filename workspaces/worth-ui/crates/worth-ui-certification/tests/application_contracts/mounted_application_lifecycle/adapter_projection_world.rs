use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::application_authority_closure::fixed_host::FixedCertificationHostBinding;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_headless::WorthUiHeadlessHost;
use worth_ui_runtime::facade::mounted::{
    UiMountedLaneParticipation, UiRequiredLaneContributionStatus,
};
use worth_ui_runtime::facade::{
    runtime_handoff::{UiResizeLogicalExtent, UiResizePreviewSample},
    WorthUiPendingMountedPreview,
};
use worth_ui_test_support::WorthUiFrameworkTurnCertificationExt;

use crate::filesystem_contract_workspace::FilesystemContractWorkspace;

pub(crate) fn preview_application_with_host<Host>(
    responsibility: &str,
    host: Host,
) -> (
    WorthUiActiveApplicationSession,
    FilesystemContractWorkspace,
    FilesystemApplicationLifecycleScenario,
)
where
    Host: FixedCertificationHostBinding + 'static,
{
    preview_application_from_sources(
        responsibility,
        host,
        FilesystemApplicationLifecycleScenario::preview_source_text(false),
        FilesystemApplicationLifecycleScenario::preview_source_text(true),
    )
}

pub(crate) fn preview_application_from_sources<Host>(
    responsibility: &str,
    host: Host,
    initial_source: String,
    successor_source: String,
) -> (
    WorthUiActiveApplicationSession,
    FilesystemContractWorkspace,
    FilesystemApplicationLifecycleScenario,
)
where
    Host: FixedCertificationHostBinding + 'static,
{
    let mut scenario = FilesystemApplicationLifecycleScenario::new(responsibility);
    let workspace = FilesystemContractWorkspace::new(responsibility);
    workspace.write("app/main.wui", &initial_source);
    let filesystem = WorthUiFilesystemSourceProvider::new(workspace.root());
    let initial_snapshot = filesystem
        .read()
        .expect("real splitter preview source bytes should freeze");
    let capability_app = scenario.preview_capability_application(WorthUiHeadlessHost);
    let initial_submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        initial_snapshot,
        capability_app.capabilities(),
    );
    let mut session = scenario
        .prepare_preview_application_with_host(initial_submission, host)
        .launch()
        .expect("real splitter preview application should launch");

    workspace.write_atomic("app/main.wui", &successor_source);
    let successor_snapshot = filesystem
        .read()
        .expect("edited splitter preview source bytes should freeze");
    let successor_submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        successor_snapshot,
        session.capabilities(),
    );
    let cutover = FilesystemApplicationLifecycleScenario::activate_submission(
        &mut session,
        successor_submission,
    );
    retire_query(&mut scenario, cutover.into_operation_live_retirement());
    (session, workspace, scenario)
}

pub(crate) fn retire_query(
    scenario: &mut FilesystemApplicationLifecycleScenario,
    retirement: worth_ui_query_binding::WorthUiOperationLiveRetirement,
) {
    match scenario.close_query_retirement(retirement) {
        worth_ui_query_binding::WorthUiOperationLiveRetirementCloseOutcome::Closed(_) => {}
        worth_ui_query_binding::WorthUiOperationLiveRetirementCloseOutcome::Stopped(stop) => {
            panic!("preview Query retirement stopped: {:?}", stop.query_error())
        }
    }
}

pub(crate) fn submit_preview<'session>(
    session: &'session mut WorthUiActiveApplicationSession,
    target: worth_ui::facade::graph::UiGraphNodeIdentity,
    pixels: f32,
) -> WorthUiPendingMountedPreview<'session> {
    let extent = UiResizeLogicalExtent::try_from_logical_pixels(pixels).unwrap();
    session
        .execute_framework_turn(|turn| {
            turn.resize_preview(|source| {
                source
                    .admit_and_submit(UiResizePreviewSample::new(target, extent))
                    .unwrap();
            });
        })
        .expect("no mounted presentation lease is active")
        .into_mounted_preview()
        .unwrap_or_else(|other| {
            panic!(
                "typed resize preview must produce mounted preview authority: {:?}",
                (*other).into_completion()
            )
        })
}

pub(crate) fn cell_status(
    frame: &worth_ui_runtime::facade::mounted::UiPreparedMountedFrame,
    lane: UiMountedLaneParticipation,
) -> UiRequiredLaneContributionStatus {
    frame
        .manifest()
        .lane_contributions()
        .iter()
        .find(|cell| cell.lane() == lane)
        .unwrap()
        .status()
}

pub(crate) fn preview_target(
    session: &WorthUiActiveApplicationSession,
) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    let expected = worth_ui::facade::declaration::MosaicSizingContractId::new(
        FilesystemApplicationLifecycleScenario::preview_sizing_contract_id(),
    )
    .unwrap();
    let graph = session.graph();
    let matches = graph
        .node_identities()
        .filter(|identity| {
            graph
                .lookup()
                .topology_node(*identity)
                .and_then(|lookup| {
                    lookup
                        .value_ref()
                        .containment_claim()
                        .mosaic_sizing_contract_id()
                        .cloned()
                })
                .is_some_and(|observed| observed == expected)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "the real .wui world must author exactly one node with the preview sizing contract"
    );
    matches[0]
}

pub(crate) fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(800.0, 600.0),
        )),
        ..Default::default()
    }
}
