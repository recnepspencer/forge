use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryLiveView, ForgeQueryMutationFamily,
    ForgeQueryWriteCommand,
};
use serde_json::Value;

mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn public_submission_lane_submit_replaces_direct_workspace_write() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.submission-lane.scalar")
        .expect("runtime should open a public workspace");
    let tasks = task_live_view(&mut workspace, "public-submission-lane-scalar-tasks");

    let receipt = workspace
        .submissions()
        .expect("submission lane should mint")
        .submit(task_insert_command(
            "task-submit-1",
            "Submitted scalar task",
        ))
        .expect("submission lane scalar write should execute");

    assert_eq!(receipt.mutation_family(), ForgeQueryMutationFamily::Insert);
    assert_eq!(receipt.declared_collection(), Some("Task"));

    let rows = workspace.read(&tasks);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].external_row()["identity"]["id"].as_str(),
        Some("task-submit-1")
    );
    assert_eq!(
        rows[0].external_row()["title"]["value"].as_str(),
        Some("Submitted scalar task")
    );
}

#[test]
fn public_submission_lane_submit_batch_replaces_direct_workspace_batch() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.submission-lane.batch")
        .expect("runtime should open a public workspace");
    let tasks = task_live_view(&mut workspace, "public-submission-lane-batch-tasks");

    let receipt = workspace
        .submissions()
        .expect("submission lane should mint")
        .submit_batch(vec![
            task_insert_command("task-batch-1", "Submitted batch one"),
            task_insert_command("task-batch-2", "Submitted batch two"),
        ])
        .expect("submission lane batch write should execute");

    assert_eq!(receipt.write_count(), 2);
    assert!(receipt
        .write_receipts()
        .iter()
        .all(|write| write.mutation_family() == ForgeQueryMutationFamily::Insert));

    let rows = workspace.read(&tasks);
    assert_eq!(rows.len(), 2);
    let mut titles = rows
        .iter()
        .map(|row| {
            row.external_row()["title"]["value"]
                .as_str()
                .expect("title should materialize")
        })
        .collect::<Vec<_>>();
    titles.sort_unstable();

    assert_eq!(titles, vec!["Submitted batch one", "Submitted batch two"]);
}

fn task_insert_command(id: &str, title: &str) -> ForgeQueryWriteCommand {
    ForgeQueryAspectMutationBuilder::new()
        .aspect("identity.id", id)
        .aspect("title.value", title)
        .build_insert("Task")
        .expect("task insert command should build")
}

fn task_live_view(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    name: &str,
) -> ForgeQueryLiveView<Value> {
    workspace
        .live_view(name, |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("identity.id")
                .schema_basis(format!("{name}-schema"))
        })
        .expect("task live view should declare")
}
