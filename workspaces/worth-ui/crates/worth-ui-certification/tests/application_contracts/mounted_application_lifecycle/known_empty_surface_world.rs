use worth_ui::facade::host::WorthUiOperationalHostAdapter;
use worth_ui::facade::mounted::{
    UiHostSurfacePresentationMode, UiSurfaceBindingCoordinatePosture, UiSurfaceBindingProfile,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use crate::filesystem_contract_workspace::FilesystemContractWorkspace;

pub(crate) fn active_session() -> worth_ui::facade::app::WorthUiActiveApplicationSession {
    mounted_application("mounted-identity")
        .launch()
        .expect("runtime should launch from the real filesystem-authored world")
}

fn mounted_application(label: &str) -> worth_ui::facade::app::WorthUiApp {
    let scenario = FilesystemApplicationLifecycleScenario::new(label);
    let submission = mounted_submission(
        label,
        &FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
        &scenario,
    );
    scenario.prepare_application_with_host(
        submission,
        worth_ui::facade::host::WorthUiHeadlessRecorder::default(),
    )
}

pub(crate) fn mounted_application_with_host<Host>(
    label: &str,
    host: Host,
) -> worth_ui::facade::app::WorthUiApp
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    let scenario = FilesystemApplicationLifecycleScenario::new(label);
    let submission = mounted_submission(
        label,
        &FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
        &scenario,
    );
    scenario.prepare_application_with_host(submission, host)
}

pub(crate) fn mounted_application_with_host_and_retention_budget<Host>(
    label: &str,
    host: Host,
    retention_budget: worth_ui::facade::mounted::UiMountedFrameRetentionBudget,
) -> worth_ui::facade::app::WorthUiApp
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    let scenario = FilesystemApplicationLifecycleScenario::new(label);
    let submission = mounted_submission(
        label,
        &FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
        &scenario,
    );
    scenario.prepare_application_with_host_and_retention_budget(submission, host, retention_budget)
}

fn mounted_submission(
    label: &str,
    source: &str,
    scenario: &FilesystemApplicationLifecycleScenario,
) -> worth_ui::facade::source::WorthUiWatchedCandidateSubmission {
    let workspace = FilesystemContractWorkspace::new(label);
    workspace.write("app/main.wui", source);
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production filesystem acquisition should read actual .wui bytes");
    workspace.close();
    let capabilities = scenario.capability_application();
    FilesystemApplicationLifecycleScenario::lower_snapshot(snapshot, capabilities.capabilities())
}

pub(crate) fn registered_surface(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui::facade::mounted::UiSemanticSurfaceIdentity {
    let surface = session.create_semantic_surface().unwrap();
    session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(1),
        )
        .unwrap();
    surface
}

pub(crate) fn first_node(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui::facade::mounted::UiMountedGraphNodeHandle {
    let node = session.graph().node_identities().next().unwrap();
    session.mounted_graph_node(node).unwrap()
}

pub(crate) fn profile(epoch: u64) -> UiSurfaceBindingProfile {
    UiSurfaceBindingProfile::new(
        1_000,
        UiSurfaceBindingCoordinatePosture::LogicalPoints,
        epoch,
    )
    .unwrap()
}
