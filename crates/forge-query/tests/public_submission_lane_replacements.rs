use forge_foundational::facade::{
    AspectKey, AspectValue, CanonicalFieldPath, FieldKey, InternedString,
};
use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryAspectTouch, ForgeQueryLiveView,
    ForgeQueryMutationFamily, ForgeQueryNativeRow, ForgeQueryWriteCommand,
};

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
    assert_eq!(
        receipt.terminal_declared_collection_projection(),
        Some("Task")
    );

    let rows = workspace.read(&tasks);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].scalar_value_at(&field_path("identity.id")),
        Some(&text("task-submit-1"))
    );
    assert_eq!(
        rows[0].scalar_value_at(&field_path("title.value")),
        Some(&text("Submitted scalar task"))
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
            scalar_text(row.scalar_value_at(&field_path("title.value")))
                .expect("title should materialize")
                .to_string()
        })
        .collect::<Vec<_>>();
    titles.sort_unstable();

    assert_eq!(titles, vec!["Submitted batch one", "Submitted batch two"]);
}

fn task_insert_command(id: &str, title: &str) -> ForgeQueryWriteCommand {
    ForgeQueryAspectMutationBuilder::new()
        .aspect(touch("identity.id"), text(id))
        .aspect(touch("title.value"), text(title))
        .build_insert("Task")
        .expect("task insert command should build")
}

fn task_live_view(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    name: &str,
) -> ForgeQueryLiveView<ForgeQueryNativeRow> {
    workspace
        .live_view(name, |q| {
            q.from("Task")
                .select([
                    forge_query::facade::AspectFieldKey::new("identity", "id").unwrap(),
                    forge_query::facade::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(forge_query::facade::AspectFieldKey::new("identity", "id").unwrap())
                .schema_basis(format!("{name}-schema"))
        })
        .expect("task live view should declare")
}

fn touch(aspect_path: &str) -> ForgeQueryAspectTouch {
    let mut segments = aspect_path.split('.');
    let aspect = segments
        .next()
        .and_then(|segment| AspectKey::new(segment.to_string()))
        .expect("test aspect path aspect should admit");
    let fields = segments
        .map(|segment| {
            FieldKey::new(segment.to_string()).expect("test aspect path field should admit")
        })
        .collect::<Vec<_>>();
    if fields.is_empty() {
        ForgeQueryAspectTouch::aspect(aspect)
    } else {
        ForgeQueryAspectTouch::field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("test aspect path should have fields"),
        )
    }
}

fn text(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}

fn field_path(path: &str) -> CanonicalFieldPath {
    CanonicalFieldPath::new(
        path.split('.').map(|segment| {
            FieldKey::new(segment).expect("test field path segment should be valid")
        }),
    )
    .expect("test field path should be non-empty")
}

fn scalar_text(value: Option<&AspectValue>) -> Option<&str> {
    match value? {
        AspectValue::String(InternedString::Raw(value)) => Some(value.as_str()),
        AspectValue::String(InternedString::Symbol(_)) => None,
        _ => None,
    }
}
