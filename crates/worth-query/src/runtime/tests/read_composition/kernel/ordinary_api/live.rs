use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::ordinary::live::{
    declare_live, WorthQueryLiveDeclarationStopKind, WorthQueryLiveOpenOutcome,
    WorthQueryManagedLiveCloseOutcome, WorthQueryManagedLiveHandle,
};
use crate::ordinary::read::{current, declare, WorthQueryReadStopSource};
use crate::runtime::tests::support::{
    complete_backend_from_parts_builder, insert_command, stateful_bridge_task_runtime, task_schema,
    test_string_aspect_value, test_update_string_aspect_command, TestSourceAdapter,
};
use crate::runtime::{
    WorthQueryReadBuilder, WorthQueryReadDenial, WorthQueryRuntimeError, WorthQueryWorkspace,
};

#[test]
fn managed_live_open_preserves_one_shot_meaning_and_authority_journey() {
    let mut workspace = task_workspace("managed-live-parity");
    workspace
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("First")),
            ],
        ))
        .expect("seed write should execute");
    let one_shot = one_shot_task_result(&mut workspace);

    let opened = match declare_live("tasks.managed", task_collection_read)
        .expect("managed task live declaration should build")
        .using(current())
        .open(&mut workspace)
    {
        WorthQueryLiveOpenOutcome::Opened(opened) => opened,
        WorthQueryLiveOpenOutcome::Stopped(stop) => {
            panic!(
                "managed live open unexpectedly stopped: {:?}",
                stop.source()
            )
        }
    };
    let live = opened
        .handle()
        .read(&mut workspace)
        .expect("managed live read should execute");

    assert_eq!(live.rows(), one_shot.rows());
    assert_eq!(
        live.receipt().snapshot_identity(),
        one_shot.receipt().snapshot_identity()
    );
    assert_eq!(opened.journey_counters().planning_attempt_count(), 1);
    assert_eq!(
        opened
            .journey_counters()
            .lower_runtime_execution_completed_count(),
        1
    );
}

#[test]
fn managed_live_declaration_rejects_an_empty_resource_name_before_runtime_contact() {
    let stop = declare_live(" ", task_collection_read)
        .expect_err("empty managed resource name must stop during declaration");

    assert_eq!(
        stop.kind(),
        WorthQueryLiveDeclarationStopKind::EmptyResourceName
    );
    assert_eq!(
        stop.next_action(),
        crate::ordinary::read::WorthQueryReadNextAction::ReviseDeclaration
    );
}

#[test]
fn managed_live_resource_routes_admitted_deltas_and_suppresses_irrelevant_updates() {
    let mut workspace = task_workspace("managed-live-delivery");
    let handle = open_task_resource(&mut workspace, "tasks.delivery");
    let insert = workspace
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("First")),
            ],
        ))
        .expect("insert should route through managed resource");
    assert_eq!(
        handle
            .drain(&mut workspace)
            .expect("insert delivery should drain")
            .batches()
            .len(),
        1
    );
    assert_eq!(
        handle
            .read(&mut workspace)
            .expect("live result should remain readable after insert")
            .rows(),
        one_shot_task_result(&mut workspace).rows()
    );

    workspace
        .write(test_update_string_aspect_command(
            insert.deltas()[0].entity_identity.clone(),
            "description.value",
            "irrelevant",
        ))
        .expect("irrelevant update should execute");
    assert!(handle
        .drain(&mut workspace)
        .expect("suppressed delivery drain should succeed")
        .is_empty());
    assert_eq!(
        handle
            .read(&mut workspace)
            .expect("suppressed update must preserve live meaning")
            .rows(),
        one_shot_task_result(&mut workspace).rows()
    );

    workspace
        .write(test_update_string_aspect_command(
            insert.deltas()[0].entity_identity.clone(),
            "title.value",
            "Renamed",
        ))
        .expect("projected update should execute");
    let delivery = handle
        .drain(&mut workspace)
        .expect("projected delivery should drain");
    assert_eq!(delivery.batches().len(), 1);
    assert_eq!(delivery.batches()[0].sequence(), 2);
    assert_eq!(
        handle
            .read(&mut workspace)
            .expect("updated live result should remain readable")
            .rows(),
        one_shot_task_result(&mut workspace).rows()
    );
}

#[test]
fn managed_live_close_detaches_shared_consumers_and_removes_final_resource() {
    let mut workspace = task_workspace("managed-live-shared-close");
    let first = open_task_resource(&mut workspace, "tasks.first");
    let second = open_task_resource(&mut workspace, "tasks.second");

    let first_close = closed(first.close(&mut workspace));
    assert!(!first_close.lane_terminal());
    assert!(workspace
        .resolve_live_artifact_target("tasks.first")
        .is_err());

    let second_close = closed(second.close(&mut workspace));
    assert!(second_close.lane_terminal());
    assert!(workspace
        .resolve_live_artifact_target("tasks.second")
        .is_err());

    let write = workspace
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("After close")),
            ],
        ))
        .expect("write after close should execute without orphan delivery");
    assert!(write
        .terminal_affected_live_view_ids_projection()
        .is_empty());
}

#[test]
fn managed_live_handle_cannot_close_a_resource_in_another_workspace() {
    let mut owner = task_workspace("managed-live-owner");
    let mut unrelated = task_workspace("managed-live-unrelated");
    let handle = open_task_resource(&mut owner, "tasks.same-name");
    let unrelated_handle = open_task_resource(&mut unrelated, "tasks.same-name");

    assert!(handle.read(&mut unrelated).is_err());
    assert!(handle.drain(&mut unrelated).is_err());
    assert!(handle.observe(&mut unrelated).is_err());
    let handle = match handle.close(&mut unrelated) {
        WorthQueryManagedLiveCloseOutcome::Stopped(stop) => stop.into_handle(),
        WorthQueryManagedLiveCloseOutcome::Closed(_) => {
            panic!("unrelated workspace must not close another workspace's resource")
        }
    };
    assert!(handle.read(&mut owner).is_ok());
    assert!(closed(handle.close(&mut owner)).lane_terminal());
    assert!(closed(unrelated_handle.close(&mut unrelated)).lane_terminal());
}

#[test]
fn managed_live_resource_names_are_unique_before_a_second_subscription_is_opened() {
    let mut workspace = task_workspace("managed-live-name-uniqueness");
    let handle = open_task_resource(&mut workspace, "tasks.unique");
    let second = declare_live("tasks.unique", task_collection_read)
        .expect("second declaration authors the same requested resource name")
        .using(current())
        .open(&mut workspace);

    match second {
        WorthQueryLiveOpenOutcome::Stopped(stop) => {
            assert!(matches!(
                stop.source(),
                WorthQueryReadStopSource::Runtime(
                    WorthQueryRuntimeError::LiveSubscriptionInstallation { stage, .. }
                ) if *stage == "managed-resource-name-admission"
            ));
        }
        WorthQueryLiveOpenOutcome::Opened(_) => {
            panic!("duplicate managed resource name must be denied")
        }
    }
    assert!(handle.read(&mut workspace).is_ok());
    assert!(closed(handle.close(&mut workspace)).lane_terminal());
}

#[test]
fn failed_backend_close_preserves_the_managed_resource_for_retry() {
    let runtime = complete_backend_from_parts_builder()
        .source_adapter(TestSourceAdapter::fail_close())
        .build_backend_from_parts()
        .build()
        .expect("close-failing runtime should build");
    let mut workspace = runtime
        .workspace("managed-live-close-retry")
        .expect("close-failing workspace should open");
    let handle = open_task_resource(&mut workspace, "tasks.close-retry");

    let handle = match handle.close(&mut workspace) {
        WorthQueryManagedLiveCloseOutcome::Stopped(stop) => stop.into_handle(),
        WorthQueryManagedLiveCloseOutcome::Closed(_) => {
            panic!("backend close failure must stop managed resource disposal")
        }
    };

    assert!(handle.read(&mut workspace).is_ok());
    assert!(handle.drain(&mut workspace).is_ok());
    assert!(workspace
        .resolve_live_artifact_target("tasks.close-retry")
        .is_ok());
}

pub(super) fn task_collection_read<Output>(
    read: WorthQueryReadBuilder<Output>,
) -> Result<Output, WorthQueryReadDenial> {
    read.local_collection(
        "Task",
        task_schema(),
        |query| {
            query
                .project(
                    AspectFieldSelector::new("identity", "id")
                        .expect("identity selector should build"),
                )
                .project(
                    AspectFieldSelector::new("title", "value")
                        .expect("title selector should build"),
                )
        },
        |shape| {
            shape
                .field(
                    AuthoredResultShapeField::new("identity", "id", "identity.id")
                        .expect("identity result field should build"),
                )
                .field(
                    AuthoredResultShapeField::new("title", "value", "title")
                        .expect("title result field should build"),
                )
        },
    )
}

pub(super) fn task_workspace(name: &str) -> WorthQueryWorkspace {
    stateful_bridge_task_runtime()
        .workspace(name)
        .expect("task workspace should open")
}

pub(super) fn one_shot_task_result(
    workspace: &mut WorthQueryWorkspace,
) -> crate::runtime::WorthQueryReadResult {
    declare(task_collection_read)
        .expect("one-shot parity read should declare")
        .using(current())
        .run(workspace)
        .into_result()
        .expect("one-shot parity read should execute")
        .into_result()
}

pub(super) fn open_task_resource(
    workspace: &mut WorthQueryWorkspace,
    name: &str,
) -> WorthQueryManagedLiveHandle {
    match declare_live(name, task_collection_read)
        .expect("managed task resource should declare")
        .using(current())
        .open(workspace)
    {
        WorthQueryLiveOpenOutcome::Opened(opened) => opened.into_handle(),
        WorthQueryLiveOpenOutcome::Stopped(stop) => {
            panic!(
                "managed task resource unexpectedly stopped: {:?}",
                stop.source()
            )
        }
    }
}

pub(super) fn closed(
    outcome: WorthQueryManagedLiveCloseOutcome,
) -> crate::ordinary::live::WorthQueryManagedLiveCloseReceipt {
    match outcome {
        WorthQueryManagedLiveCloseOutcome::Closed(receipt) => receipt,
        WorthQueryManagedLiveCloseOutcome::Stopped(stop) => {
            panic!(
                "managed resource close unexpectedly stopped: {:?}",
                stop.error()
            )
        }
    }
}
