use super::support::*;

#[test]
fn workspace_reads_live_artifact_binding_as_one_snapshot_coherent_named_pack() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("live.artifact.binding")
        .expect("task runtime should open a named workspace");
    let entities: ForgeQueryLiveView<Value> = workspace
        .live_view_request("tasks.live-entities", task_live_request(), task_schema())
        .expect("entity live view should declare");
    let names: ForgeQueryLiveView<Value> = workspace
        .live_view_request("tasks.live-names", task_live_request(), task_schema())
        .expect("naming live view should declare");

    workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Live artifact")
        })
        .expect("workspace write should execute");

    let binding = workspace
        .read_live_artifact_binding(
            "tasks.historical.naming-pack",
            [(&entities).into(), (&names).into()],
        )
        .expect("live artifact binding should materialize");

    assert_eq!(binding.artifact_name(), "tasks.historical.naming-pack");
    assert_eq!(binding.target_count(), 2);
    assert_eq!(
        binding.read(&entities).expect("entity rows").rows().len(),
        1
    );
    assert_eq!(binding.read(&names).expect("name rows").rows().len(), 1);
    assert_eq!(
        binding
            .read(&entities)
            .expect("entity rows")
            .receipt()
            .snapshot_identity(),
        binding
            .read(&names)
            .expect("name rows")
            .receipt()
            .snapshot_identity(),
    );
    assert_eq!(
        binding.snapshot_identity(),
        binding
            .read(&entities)
            .expect("entity rows")
            .receipt()
            .snapshot_identity(),
    );
}
