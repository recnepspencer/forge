use worth_ui::facade::host::WorthUiHeadlessHost;
use worth_ui::facade::runtime::{WorthUiHandleResolutionOutcome, WorthUiRealtimeFrameTarget};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;
use super::filesystem_replacement_support::activate_current_filesystem_candidate;

const REALTIME_DECLARATION: &str = "component workspace.component.cross_lane_realtime {}\n";

#[test]
fn public_replacement_retires_and_remints_exact_renderer_surface_generation() {
    let scenario = FilesystemApplicationLifecycleScenario::new("realtime-surface-generation");
    let workspace = FilesystemContractWorkspace::new("realtime-surface-generation");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::cross_lane_source_text(),
    );
    let capabilities = scenario.cross_lane_capability_application(WorthUiHeadlessHost);
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .expect("predecessor source settles from disk"),
        capabilities.capabilities(),
    );
    let mut session = scenario
        .prepare_cross_lane_application_with_host(submission, WorthUiHeadlessHost)
        .launch()
        .expect("predecessor cross-lane application launches");
    let stale = session
        .first_realtime_renderer_surface()
        .expect("predecessor renderer surface");
    let predecessor = session
        .inspect_realtime_target(stale)
        .expect("predecessor renderer-surface summary");

    let without_realtime = FilesystemApplicationLifecycleScenario::cross_lane_source_text()
        .replace(REALTIME_DECLARATION, "");
    workspace.write("app/main.wui", &without_realtime);
    let removal = activate_current_filesystem_candidate(&workspace, &mut session)
        .expect("realtime removal activates")
        .into_activation()
        .expect("realtime removal changes executable meaning");
    assert!(removal.query_retirement().is_empty());
    assert_eq!(
        session
            .inspect_realtime_target(stale)
            .expect_err("retired surface cannot inspect successor truth")
            .outcome(),
        WorthUiHandleResolutionOutcome::TargetMissing
    );

    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::cross_lane_source_text(),
    );
    let reinsertion = activate_current_filesystem_candidate(&workspace, &mut session)
        .expect("realtime reinsertion activates")
        .into_activation()
        .expect("realtime reinsertion changes executable meaning");
    assert!(reinsertion.query_retirement().is_empty());
    assert_eq!(
        session
            .inspect_realtime_target(stale)
            .expect_err("predecessor surface generation cannot resolve the reminted slot")
            .outcome(),
        WorthUiHandleResolutionOutcome::TargetMissing
    );
    let fresh = session
        .first_realtime_renderer_surface()
        .expect("successor renderer surface");
    assert_ne!(fresh, stale);
    let successor = session
        .inspect_realtime_target(fresh)
        .expect("successor renderer-surface summary");
    assert_eq!(
        successor.host_session_identity(),
        predecessor.host_session_identity()
    );
    assert_ne!(
        successor.plan_basis_digest(),
        predecessor.plan_basis_digest()
    );

    let execution = session
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("successor execution turn"));
    let denial = execution
        .execute_realtime_frame(WorthUiRealtimeFrameTarget::renderer_surface(stale))
        .expect_err("stale renderer surface denies before successor draw work");
    assert_eq!(denial.counters().frame_synchronized_pass_count(), 0);
    assert_eq!(denial.counters().renderer_surface_handoff_count(), 0);
    drop(execution);

    let _ = session.shutdown();
    workspace.close();
}
