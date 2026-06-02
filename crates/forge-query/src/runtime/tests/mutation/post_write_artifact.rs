use super::super::support::*;

#[test]
fn batch_write_retained_artifact_keeps_receipt_inspection_and_exact_retained_binding_together() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.post-write-artifact")
        .expect("task runtime should open a named workspace");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.post-write-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-post-write-table")
        })
        .expect("live view should declare");
    let computed: ForgeQueryDerivedViewHandle<Value> = workspace
        .computed(
            "tasks.post-write-summary",
            |c| {
                c.depends_on_live(&live)
                    .reads(["title.value"])
                    .produces(["ui.batch_summary"])
            },
            TitleListMaintainer,
        )
        .expect("computed view should declare");

    let receipt = workspace
        .batch(|batch| {
            batch.insert("Task", |task| {
                task.aspect("identity.id", "task-1")
                    .aspect("title.value", "Buy milk")
            })
        })
        .expect("batch should execute");

    let artifact = workspace
        .materialize_batch_write_artifact_binding(
            &receipt,
            "tasks.post-write-artifact",
            [(&computed).into()],
        )
        .expect("post-write retained artifact should build");
    let inspection = workspace.inspect(&receipt).expect("receipt should inspect");
    let expected_row = workspace.materialize(&computed)[0].clone();
    let retained_row: Value = artifact
        .retained_artifact()
        .decode_single_row(&computed)
        .expect("retained artifact should decode the exact retained row");

    assert_eq!(artifact.receipt().batch_digest(), receipt.batch_digest());
    match inspection {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => {
            assert_eq!(
                artifact.inspection().inspection_digest(),
                inspection.inspection_digest()
            );
        }
        other => panic!("expected batch-write receipt inspection, got {other:?}"),
    }
    assert_eq!(
        artifact.retained_artifact().artifact_name(),
        "tasks.post-write-artifact"
    );
    assert_eq!(artifact.retained_artifact().target_count(), 1);
    assert_eq!(
        artifact
            .retained_artifact()
            .target_view_names()
            .collect::<Vec<_>>(),
        vec!["tasks.post-write-summary"]
    );
    assert_eq!(retained_row, expected_row);
}
