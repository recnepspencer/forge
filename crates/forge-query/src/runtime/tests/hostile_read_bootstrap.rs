use super::support::*;

#[test]
fn ordinary_runtime_backed_read_path_keeps_forbidden_fallback_seams_at_exact_zero() {
    let runtime = stateful_bridge_task_runtime();
    let mut workspace = runtime
        .workspace("runtime.tests.hostile-read-bootstrap")
        .expect("task runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<Value> = workspace
        .live_view("runtime.tests.hostile-read-bootstrap.tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("runtime-tests-hostile-read-bootstrap-tasks")
        })
        .expect("task live view should declare");

    reset_forbidden_fallback_seam_invocations();

    workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-hostile-bootstrap")
                .aspect("title.value", "Hostile bootstrap task")
        })
        .expect("insert should execute through the ordinary bridge-backed lane");

    let rows = workspace.read(&tasks);
    let patches = workspace.observe(&tasks);

    assert_eq!(rows.len(), 1);
    assert_eq!(patches.query_delivery_batches.len(), 1);
    for seam in [
        ForgeQueryForbiddenFallbackSeam::ConsumeScalarFields,
        ForgeQueryForbiddenFallbackSeam::DecodeRowPair,
        ForgeQueryForbiddenFallbackSeam::DecodeRowTriple,
        ForgeQueryForbiddenFallbackSeam::VerifyScalarAlignment,
        ForgeQueryForbiddenFallbackSeam::ReadLiveArtifactBundle,
        ForgeQueryForbiddenFallbackSeam::BindLiveArtifact,
        ForgeQueryForbiddenFallbackSeam::ReadLiveArtifactBinding,
    ] {
        assert_eq!(forbidden_fallback_seam_invocation_count(seam), 0);
    }
}
