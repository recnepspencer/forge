use super::super::support::*;

#[test]
fn probe_existing_returns_backend_verified_values_for_entity_targets() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.probe-existing-entity")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.probe-existing-entity-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-probe-existing-entity-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let probe = workspace
        .probe_existing(binding, ["identity.id", "title.value"])
        .expect("probe should execute");

    assert_eq!(
        probe.mode(),
        ForgeQueryExistingTruthProbeMode::BackendVerifiedProbe
    );
    assert_eq!(probe.fields().len(), 2);
    assert_eq!(
        probe
            .field("identity.id")
            .expect("identity field should exist")
            .external_value_json(),
        "\"task-1\""
    );
    assert_eq!(
        probe
            .field("title.value")
            .expect("title field should exist")
            .external_value_json(),
        "\"Seed title\""
    );
    assert!(!probe.probe_digest().is_empty());
}

#[test]
fn probe_existing_returns_backend_verified_values_for_relation_targets() {
    let runtime = stateful_bridge_task_relation_runtime();
    let mut workspace = runtime
        .workspace("tasks.probe-existing-relation")
        .expect("workspace should open");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.probe-existing-relation-table", |q| {
            q.from("TaskRelation")
                .select(["identity.id", "kind.value"])
                .order_by("kind.value")
                .schema_basis("tasks-probe-existing-relation-table")
        })
        .expect("relation live view should declare");
    let seed = workspace
        .insert("TaskRelation", |relation| {
            relation
                .aspect("identity.id", "rel-1")
                .aspect("kind.value", "depends_on")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing relation target should build")
            .in_target_collection("TaskRelation")
            .expect("existing relation target collection should build"),
        )
        .expect("relation binding should build");

    let probe = workspace
        .probe_existing(binding, ["kind.value"])
        .expect("relation probe should execute");

    assert_eq!(
        probe.binding().family(),
        ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
    );
    assert_eq!(
        probe
            .field("kind.value")
            .expect("kind field should exist")
            .external_value_json(),
        "\"depends_on\""
    );
}

#[test]
fn probe_existing_denies_missing_aspect_typed_and_early() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.probe-existing-missing-aspect")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.probe-existing-missing-aspect-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-probe-existing-missing-aspect-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .probe_existing(binding, ["status.value"])
        .expect_err("missing probed aspect should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect
            );
            assert_eq!(denial.probed_aspect_path(), Some("status.value"));
        }
        other => panic!("expected typed probe denial, got {other:?}"),
    }
}

#[test]
fn probe_existing_reports_the_actual_missing_aspect_in_multi_aspect_requests() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.probe-existing-multi-missing-aspect")
        .expect("task runtime should open a named workspace");
    let _: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.probe-existing-multi-missing-aspect-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-probe-existing-multi-missing-aspect-table")
        })
        .expect("live view should declare");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Seed title")
        })
        .expect("seed insert should execute");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(
                crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .probe_existing(binding, ["identity.id", "status.value", "title.value"])
        .expect_err("missing probed aspect should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect
            );
            assert_eq!(denial.probed_aspect_path(), Some("status.value"));
        }
        other => panic!("expected typed probe denial, got {other:?}"),
    }
}

#[test]
fn probe_existing_denies_unsupported_backend_typed_and_early() {
    let runtime = bridge_runtime_with_support(ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    ));
    let workspace = runtime
        .workspace("tasks.probe-existing-unsupported")
        .expect("workspace should open");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new(crate::runtime::ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::ForgeQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"), test_entity_identity("Task:1"))
                .expect("existing entity target should build")
                .in_target_collection("Task")
                .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .probe_existing(binding, ["title.value"])
        .expect_err("unsupported backend probe should deny");

    match error {
        ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported
            );
        }
        other => panic!("expected typed probe denial, got {other:?}"),
    }
}
