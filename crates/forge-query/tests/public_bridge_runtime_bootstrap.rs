use forge_query::facade::{
    ForgeQueryExistingEntityTarget, ForgeQueryExistingTruthProbeDenialKind,
    ForgeQueryExistingTruthProbeMode, ForgeQueryLiveView, ForgeQueryRuntimeError,
};
use serde_json::{json, Value};

mod support;

use support::public_bridge_runtime::{
    public_bridge_runtime_bootstrap_invocation_count, public_graph_support_profile,
    reset_public_bridge_runtime_bootstrap_invocations, PublicBridgeRuntimeBootstrapPath,
    PublicBridgeRuntimeHarness,
};

fn public_entity_verified_profile() -> forge_query::facade::ForgeQueryRuntimeSupportProfile {
    public_graph_support_profile().with_bridge_backed_verification_support(
        "probe_existing",
        "direct_entity_identity",
        true,
        true,
        None,
    )
}

#[test]
fn public_bridge_runtime_common_bootstrap_lane_builds_runtime_backed_live_reads() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.bridge-runtime-bootstrap.common-lane")
        .expect("runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<Value> = workspace
        .live_view("public.bridge-runtime-bootstrap.common-lane.tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("public-bridge-runtime-bootstrap-common-lane-tasks")
        })
        .expect("task live view should declare");

    workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-bootstrap")
                .aspect("title.value", "Bootstrap task")
        })
        .expect("insert should execute through the public bridge-backed bootstrap lane");

    let rows = workspace.read(&tasks);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].external_row()["identity"]["id"].as_str(),
        Some("task-bootstrap")
    );
    assert_eq!(
        rows[0].external_row()["title"]["value"].as_str(),
        Some("Bootstrap task")
    );
}

#[test]
fn public_bridge_runtime_builder_lane_supports_seeded_existing_truth_probe() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness
        .bridge_backed_runtime_builder()
        .support_profile(public_entity_verified_profile())
        .build();
    let workspace = runtime
        .workspace("public.bridge-runtime-bootstrap.builder-lane")
        .expect("runtime should open a named workspace");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new("authority:task-bootstrap", "public-entity-1")
                .expect("existing entity target should build")
                .in_target_collection("Task")
                .expect("existing entity target collection should build"),
        )
        .expect("binding should build");
    harness.seed_existing_truth_value(&binding, "title.value", json!("Seeded bootstrap task"));

    let probe = workspace
        .probe_existing(binding, ["title.value"])
        .expect("probe should execute through the public builder bootstrap lane");

    assert_eq!(
        probe.mode(),
        ForgeQueryExistingTruthProbeMode::BackendVerifiedProbe
    );
    assert_eq!(
        probe
            .field("title.value")
            .expect("title field should exist")
            .external_value_json(),
        "\"Seeded bootstrap task\""
    );
}

#[test]
fn public_bridge_runtime_common_lane_fail_closes_existing_truth_probe_without_verification_support()
{
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let workspace = runtime
        .workspace("public.bridge-runtime-bootstrap.common-lane-unsupported-probe")
        .expect("runtime should open a named workspace");
    let binding = workspace
        .bind_existing_entity(
            ForgeQueryExistingEntityTarget::new("authority:task-bootstrap", "public-entity-1")
                .expect("existing entity target should build")
                .in_target_collection("Task")
                .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = workspace
        .probe_existing(binding, ["title.value"])
        .expect_err("common bootstrap lane should deny undeclared verification support");

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

#[test]
fn public_bridge_runtime_builder_lane_usage_stays_explicit() {
    reset_public_bridge_runtime_bootstrap_invocations();

    let harness = PublicBridgeRuntimeHarness::new();
    let _runtime = harness
        .bridge_backed_runtime_builder()
        .support_profile(public_graph_support_profile())
        .build();

    assert_eq!(
        public_bridge_runtime_bootstrap_invocation_count(PublicBridgeRuntimeBootstrapPath::Common),
        0
    );
    assert_eq!(
        public_bridge_runtime_bootstrap_invocation_count(PublicBridgeRuntimeBootstrapPath::Builder),
        1
    );
}

#[test]
fn public_bridge_runtime_common_and_builder_bootstrap_counts_stay_lane_local() {
    reset_public_bridge_runtime_bootstrap_invocations();

    let harness = PublicBridgeRuntimeHarness::new();
    let _common_runtime = harness.bridge_backed_runtime();
    let _builder_runtime = harness
        .bridge_backed_runtime_builder()
        .support_profile(public_entity_verified_profile())
        .build();
    let _second_common_runtime = harness.bridge_backed_runtime();

    assert_eq!(
        public_bridge_runtime_bootstrap_invocation_count(PublicBridgeRuntimeBootstrapPath::Common),
        2
    );
    assert_eq!(
        public_bridge_runtime_bootstrap_invocation_count(PublicBridgeRuntimeBootstrapPath::Builder),
        1
    );
}
