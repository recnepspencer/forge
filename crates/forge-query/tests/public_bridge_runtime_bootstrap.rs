use forge_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryEntityIdentity, ForgeQueryExistingEntityTarget,
    ForgeQueryExistingTruthBindingAuthorityLabel, ForgeQueryExistingTruthProbeDenialKind,
    ForgeQueryExistingTruthProbeMode, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryLiveView, ForgeQueryMutationAuthorityIdentity,
    ForgeQueryNativeRow, ForgeQueryRuntimeError,
};
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

fn existing_authority(label: &str) -> ForgeQueryMutationAuthorityIdentity {
    ForgeQueryMutationAuthorityIdentity::existing_truth_binding_authority(
        ForgeQueryExistingTruthBindingAuthorityLabel::new(label)
            .expect("existing-truth authority label"),
    )
    .expect("existing-truth authority identity")
}

fn existing_task_binding(
    authority_label: &str,
    entity_token: &str,
) -> ForgeQueryExistingTruthTargetBinding {
    ForgeQueryExistingTruthTargetBinding::from_entity_target(
        ForgeQueryExistingEntityTarget::new(
            existing_authority(authority_label),
            ForgeQueryEntityIdentity::admit_authored_entity_token(
                forge_query::facade::QueryExternalIdentityToken::new(std::sync::Arc::from(
                    entity_token,
                )),
            ),
        )
        .expect("existing entity target should build")
        .in_target_collection("Task")
        .expect("existing entity target collection should build"),
    )
    .expect("binding should build")
}

#[test]
fn public_bridge_runtime_common_bootstrap_lane_builds_runtime_backed_live_reads() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public.bridge-runtime-bootstrap.common-lane")
        .expect("runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("public.bridge-runtime-bootstrap.common-lane.tasks", |q| {
            q.from("Task")
                .select([
                    forge_query::facade::AspectFieldKey::new("identity", "id").unwrap(),
                    forge_query::facade::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(forge_query::facade::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("public-bridge-runtime-bootstrap-common-lane-tasks")
        })
        .expect("task live view should declare");

    workspace
        .insert("Task", |task| {
            task.aspect(touch("identity.id"), text("task-bootstrap"))
                .aspect(touch("title.value"), text("Bootstrap task"))
        })
        .expect("insert should execute through the public bridge-backed bootstrap lane");

    let rows = workspace.read(&tasks);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].scalar_value_at(&field_path("identity.id")),
        Some(&text("task-bootstrap"))
    );
    assert_eq!(
        rows[0].scalar_value_at(&field_path("title.value")),
        Some(&text("Bootstrap task"))
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
    let binding = existing_task_binding("authority:task-bootstrap", "public-entity-1");
    let seed = harness.seed_backend_authoritative_truth(
        &binding,
        "title.value",
        text("Seeded bootstrap task"),
    );
    assert_eq!(seed.binding_digest(), binding.binding_digest());
    assert_eq!(seed.target_collection(), "Task");
    assert_eq!(seed.terminal_aspect_path_projection(), "title.value");

    let probe = workspace
        .probe_existing_intent(
            ForgeQueryExistingTruthProbeRequest::new(binding, [touch("title.value")])
                .expect("probe request should build"),
        )
        .execute()
        .expect("probe should execute through the public builder bootstrap lane")
        .probe()
        .clone();

    assert_eq!(
        probe.mode(),
        ForgeQueryExistingTruthProbeMode::BackendVerifiedProbe
    );
    assert_eq!(
        probe
            .field_for_touch(&touch("title.value"))
            .expect("title field should exist")
            .foundational_value(),
        &text("Seeded bootstrap task")
    );
}

#[test]
fn public_bridge_runtime_builder_lane_missing_existing_truth_probe_fails_closed() {
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness
        .bridge_backed_runtime_builder()
        .support_profile(public_entity_verified_profile())
        .build();
    let workspace = runtime
        .workspace("public.bridge-runtime-bootstrap.builder-lane-missing-probe")
        .expect("runtime should open a named workspace");
    let binding =
        existing_task_binding("authority:task-bootstrap-missing", "public-entity-missing");

    let error = workspace
        .probe_existing_intent(
            ForgeQueryExistingTruthProbeRequest::new(binding, [touch("title.value")])
                .expect("probe request should build"),
        )
        .execute()
        .expect_err("backend-verified probe must deny missing authoritative truth");

    match error {
        ForgeQueryRuntimeError::ExistingTruthProbeDenied(denial) => {
            assert_eq!(
                denial.kind(),
                ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect
            );
        }
        other => panic!("expected typed missing probe denial, got {other:?}"),
    }
}

#[test]
fn public_bridge_runtime_common_lane_fail_closes_existing_truth_probe_without_verification_support()
{
    let harness = PublicBridgeRuntimeHarness::new();
    let runtime = harness.bridge_backed_runtime();
    let workspace = runtime
        .workspace("public.bridge-runtime-bootstrap.common-lane-unsupported-probe")
        .expect("runtime should open a named workspace");
    let binding = existing_task_binding("authority:task-bootstrap", "public-entity-1");

    let error = workspace
        .probe_existing_intent(
            ForgeQueryExistingTruthProbeRequest::new(binding, [touch("title.value")])
                .expect("probe request should build"),
        )
        .execute()
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
