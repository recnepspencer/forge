use worth_ui::facade::host::WorthUiHeadlessHost;
use worth_ui::facade::mounted::{
    UiMountedAllocationProjection, UiMountedFrameRequest, UiMountedOmissionReason,
    UiMountedParticipationStatus,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;
use super::mounted_application_lifecycle::known_empty_surface_world::{
    first_node, registered_surface,
};

#[derive(Debug, Eq, PartialEq)]
struct AuthoredMountedOracle {
    paint: UiMountedParticipationStatus,
    input: UiMountedParticipationStatus,
    focus: UiMountedParticipationStatus,
    hit_test: UiMountedParticipationStatus,
    diagnostic: UiMountedParticipationStatus,
    allocation_omission: UiMountedOmissionReason,
}

#[test]
fn real_query_free_and_query_backed_paths_match_the_authored_ui_oracle() {
    let query_free = project_query_free();
    let query_backed = project_query_backed();
    let expected = AuthoredMountedOracle {
        paint: UiMountedParticipationStatus::Deferred,
        input: UiMountedParticipationStatus::Deferred,
        focus: UiMountedParticipationStatus::Deferred,
        hit_test: UiMountedParticipationStatus::Deferred,
        diagnostic: UiMountedParticipationStatus::Withheld,
        allocation_omission: UiMountedOmissionReason::NoCommittedAllocation,
    };

    assert_eq!(query_free, expected);
    assert_eq!(query_backed, expected);
}

fn project_query_free() -> AuthoredMountedOracle {
    let scenario = FilesystemApplicationLifecycleScenario::new("mounted-parity-query-free");
    let workspace = FilesystemContractWorkspace::new("mounted-parity-query-free");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
    );
    let capabilities = scenario.capability_application();
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .unwrap(),
        capabilities.capabilities(),
    );
    let mut session = scenario.prepare_application(submission).launch().unwrap();
    let oracle = project_first_node(&mut session);
    let _ = session.shutdown();
    workspace.close();
    oracle
}

fn project_query_backed() -> AuthoredMountedOracle {
    let mut scenario = FilesystemApplicationLifecycleScenario::new("mounted-parity-query-backed");
    let workspace = FilesystemContractWorkspace::new("mounted-parity-query-backed");
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
    let mut session = scenario
        .prepare_cross_lane_application_with_host(submission, WorthUiHeadlessHost)
        .launch()
        .unwrap();
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
    let oracle = project_first_node(&mut session);
    let _ = session.shutdown();
    workspace.close();
    oracle
}

fn project_first_node(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> AuthoredMountedOracle {
    let surface = registered_surface(session);
    let node = first_node(session);
    session.mount_instance(node, surface).unwrap();
    let binding = session.inspect_mounted_identity().surface_bindings()[0].binding_generation();
    let candidate = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("empty source turn permits mounted projection"))
        .prepare_mounted_frame(UiMountedFrameRequest::all_bound_surfaces())
        .unwrap();
    let view = candidate
        .surfaces()
        .iter()
        .find(|receipt| receipt.requirement().binding() == binding)
        .unwrap()
        .projection();
    let node = &view.nodes()[0];
    let participation = node.participation();
    let allocation_omission = match node.allocation() {
        UiMountedAllocationProjection::Omitted(reason) => reason,
        other => panic!("authored scenario has no committed allocation, got {other:?}"),
    };
    AuthoredMountedOracle {
        paint: participation.paint().status(),
        input: participation.input().status(),
        focus: participation.focus().status(),
        hit_test: participation.hit_test().status(),
        diagnostic: participation.diagnostic().status(),
        allocation_omission,
    }
}
