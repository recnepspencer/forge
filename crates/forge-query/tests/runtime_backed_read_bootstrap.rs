use forge_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryLiveView, ForgeQueryNativeRow, QueryPatchGroupKind,
};

mod support;

use support::public_bridge_runtime::{
    public_bridge_runtime_bootstrap_invocation_count,
    reset_public_bridge_runtime_bootstrap_invocations, PublicBridgeRuntimeBootstrapPath,
    PublicBridgeRuntimeHarness,
};

#[test]
fn raw_runtime_read_bootstrap_simplicity_test() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.runtime-backed-read-bootstrap")
        .expect("runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("public.runtime-backed-read-bootstrap.tasks", |q| {
            q.from("Task")
                .select([
                    forge_query::facade::AspectFieldKey::new("identity", "id").unwrap(),
                    forge_query::facade::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(forge_query::facade::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("public-runtime-backed-read-bootstrap-tasks")
        })
        .expect("task live view should declare");

    workspace
        .insert("Task", |task| {
            task.aspect(touch("identity.id"), text("task-bootstrap"))
                .aspect(touch("title.value"), text("Bootstrap task"))
        })
        .expect("insert should execute through the public bootstrap lane");

    let rows = workspace.read(&tasks);
    let patches = workspace.observe(&tasks);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].scalar_value_at(&field_path("identity.id")),
        Some(&text("task-bootstrap"))
    );
    assert_eq!(
        rows[0].scalar_value_at(&field_path("title.value")),
        Some(&text("Bootstrap task"))
    );
    assert_eq!(patches.query_delivery_batches.len(), 1);
    assert_eq!(
        patches.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::CollectionMembershipPatchGroup
    );
    assert_eq!(patches.query_delivery_batches[0].sequence(), 1);
    assert!(patches.query_delivery_batches[0].has_relational_patch());
}

#[test]
fn runtime_backed_read_bootstrap_observe_is_a_drain_not_a_snapshot() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.runtime-backed-read-bootstrap-drain")
        .expect("runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("public.runtime-backed-read-bootstrap-drain.tasks", |q| {
            q.from("Task")
                .select([
                    forge_query::facade::AspectFieldKey::new("identity", "id").unwrap(),
                    forge_query::facade::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(forge_query::facade::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("public-runtime-backed-read-bootstrap-drain-tasks")
        })
        .expect("task live view should declare");

    workspace
        .insert("Task", |task| {
            task.aspect(touch("identity.id"), text("task-bootstrap-drain"))
                .aspect(touch("title.value"), text("Drain bootstrap task"))
        })
        .expect("insert should execute through the public bootstrap lane");

    let first = workspace.observe(&tasks);
    let second = workspace.observe(&tasks);

    assert_eq!(first.query_delivery_batches.len(), 1);
    assert!(second.query_delivery_batches.is_empty());
}

#[test]
fn runtime_backed_read_bootstrap_narrows_observation_to_touched_projection_meaning() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.runtime-backed-read-bootstrap-narrowing")
        .expect("runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view(
            "public.runtime-backed-read-bootstrap-narrowing.tasks",
            |q| {
                q.from("Task")
                    .select([
                        forge_query::facade::AspectFieldKey::new("identity", "id").unwrap(),
                        forge_query::facade::AspectFieldKey::new("title", "value").unwrap(),
                    ])
                    .order_by(forge_query::facade::AspectFieldKey::new("title", "value").unwrap())
                    .schema_basis("public-runtime-backed-read-bootstrap-narrowing-tasks")
            },
        )
        .expect("task live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect(touch("identity.id"), text("task-bootstrap-narrowing"))
                .aspect(touch("title.value"), text("Bootstrap title"))
                .aspect(touch("description.value"), text("hidden description"))
        })
        .expect("seed insert should execute through the public bootstrap lane");
    let _ = workspace.observe(&tasks);

    workspace
        .update(seed.deltas()[0].entity_identity().clone(), |task| {
            task.aspect(touch("title.value"), text("Bootstrap title updated"))
                .aspect(touch("description.value"), text("still hidden"))
        })
        .expect("projected update should execute through the public bootstrap lane");
    let projected = workspace.observe(&tasks);

    workspace
        .update(seed.deltas()[0].entity_identity().clone(), |task| {
            task.aspect(touch("description.value"), text("hidden again"))
        })
        .expect("hidden-only update should execute through the public bootstrap lane");
    let hidden_only = workspace.observe(&tasks);
    let rows = workspace.read(&tasks);

    assert_eq!(projected.query_delivery_batches.len(), 1);
    assert_eq!(
        projected.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::DetailFieldPatchGroup
    );
    assert!(hidden_only.query_delivery_batches.is_empty());
    assert_eq!(
        rows[0].scalar_value_at(&field_path("title.value")),
        Some(&text("Bootstrap title updated"))
    );
}

#[test]
fn runtime_backed_read_bootstrap_removes_deleted_members_from_read_and_observe() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.runtime-backed-read-bootstrap-delete")
        .expect("runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("public.runtime-backed-read-bootstrap-delete.tasks", |q| {
            q.from("Task")
                .select([
                    forge_query::facade::AspectFieldKey::new("identity", "id").unwrap(),
                    forge_query::facade::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(forge_query::facade::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("public-runtime-backed-read-bootstrap-delete-tasks")
        })
        .expect("task live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect(touch("identity.id"), text("task-bootstrap-delete"))
                .aspect(touch("title.value"), text("Delete bootstrap task"))
        })
        .expect("seed insert should execute through the public bootstrap lane");
    let _ = workspace.observe(&tasks);

    workspace
        .delete(seed.deltas()[0].entity_identity().clone())
        .expect("delete should execute through the public bootstrap lane");

    let patches = workspace.observe(&tasks);
    let rows = workspace.read(&tasks);

    assert_eq!(patches.query_delivery_batches.len(), 1);
    assert_eq!(
        patches.query_delivery_batches[0].patch_group_kind(),
        QueryPatchGroupKind::CollectionMembershipPatchGroup
    );
    assert!(rows.is_empty());
}

#[test]
fn ordinary_runtime_backed_read_bootstrap_stays_on_common_lane_without_builder_scaffolding() {
    reset_public_bridge_runtime_bootstrap_invocations();

    let harness = PublicBridgeRuntimeHarness::new();
    let _runtime = harness.bridge_backed_runtime();

    assert_eq!(
        public_bridge_runtime_bootstrap_invocation_count(PublicBridgeRuntimeBootstrapPath::Common),
        1
    );
    assert_eq!(
        public_bridge_runtime_bootstrap_invocation_count(PublicBridgeRuntimeBootstrapPath::Builder),
        0
    );
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
