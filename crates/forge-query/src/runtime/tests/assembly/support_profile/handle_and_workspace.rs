use super::super::super::support::*;
use crate::runtime::evidence_identities::{
    runtime_state_snapshot_basis_label_identity,
    runtime_state_snapshot_result_shape_label_identity,
    runtime_state_snapshot_test_subject_identity,
};
use crate::evidence_identity::{forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

fn test_runtime_world_basis() -> crate::application::ForgeQueryAdmittedWorldBasis {
    crate::application::ForgeQueryAdmittedWorldBasis::new(
        "test.runtime.basis",
        "TestRuntimeBasis",
        "operating:runtime-basis".to_string(),
        forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(ForgeQueryEvidenceTag::new("test_handle"), "handle:runtime-basis")
            .seal(),
        "support:snapshot".to_string(),
        crate::query_basis_lifecycle::query_basis_lifecycle_support_report().report_identity(),
    )
}
#[test]
fn runtime_public_handle_contract_freezes_inspection_sections_and_future_state_lanes() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.handle-contract")
        .expect("task runtime should open a named workspace");
    let contract = workspace.public_handle_contract();
    let support_contract = workspace.public_api_contract();

    assert_eq!(
        contract.support_contract_digest(),
        support_contract.contract_digest()
    );
    assert!(contract.inspectable_family_count() >= 15);
    assert!(contract.retained_artifact_family_count() >= 10);
    assert_eq!(contract.deferred_future_family_count(), 3);

    let live_row = contract
        .row(ForgeQueryHandleContractFamily::LiveView)
        .expect("live handle contract row should exist");
    assert!(live_row.retained_artifact_required());
    assert_eq!(
        live_row.support_status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );

    for family in [
        ForgeQueryHandleContractFamily::TemporalCapableHandle,
        ForgeQueryHandleContractFamily::AsyncResourceCapableHandle,
        ForgeQueryHandleContractFamily::MixedCauseDeliveryCapableHandle,
    ] {
        let future_row = contract
            .row(family)
            .expect("future-capable handle contract row should exist");
        assert!(future_row.deferred_future_posture());
        assert_eq!(
            future_row.support_status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
        let mut sections = future_row.inspection_sections().to_vec();
        let original_count = sections.len();
        sections.sort();
        sections.dedup();
        assert_eq!(
            sections.len(),
            original_count,
            "future-capable handle contract rows must not repeat inspection sections"
        );
    }
}

#[test]
fn runtime_workspace_rejects_empty_names_before_public_use() {
    let error = match stateful_bridge_task_runtime().workspace("  ") {
        Ok(_) => panic!("empty workspace names should not enter the public API"),
        Err(error) => error,
    };

    match error {
        ForgeQueryRuntimeError::Workspace(error) => {
            assert!(error
                .to_string()
                .contains("workspace name may not be empty"));
        }
        other => panic!("expected workspace validation error, got {other:?}"),
    }
}

#[test]
fn runtime_workspace_declares_and_inspects_with_preferred_names() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("task.workspace")
        .expect("task runtime should open a named workspace");
    let view: ForgeQueryLiveView<Value> = workspace
        .live_view_request("tasks.workspace-table", task_live_request(), task_schema())
        .expect("workspace live view should declare");
    let inspection = workspace.inspect(&view).expect("inspect should succeed");

    assert_eq!(workspace.name(), "task.workspace");
    match inspection {
        ForgeQueryInspection::LiveView(live) => {
            assert_eq!(live.view_name(), "tasks.workspace-table");
        }
        other => panic!("expected workspace live inspection, got {other:?}"),
    }
}

#[test]
fn runtime_workspace_closure_builders_cover_live_computed_effect_dx() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("task.builder-workspace")
        .expect("task runtime should open a named workspace");
    let view: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.builder-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("runtime-task-builder")
                .as_surface("tasks.builder-table")
        })
        .expect("workspace live builder should lower to a live view");
    let titles = workspace
        .computed::<Value>(
            "tasks.builder-title-list",
            |c| {
                c.depends_on_live(&view)
                    .reads(["title.value"])
                    .produces(["runtime.title_list"])
            },
            TitleListMaintainer,
        )
        .expect("workspace computed builder should lower to a derived view");
    let effect = workspace
        .effect::<Value>("tasks.builder-title-delivery", |e| {
            e.when_computed(&titles, ["runtime.title_list"])
                .condition_expression(
                    "expr.title-list-visible",
                    ["runtime.title_list"],
                    ["ui.title_list"],
                )
                .deliver("ui.title-list")
                .meaningful_change_suppression()
        })
        .expect("workspace effect builder should lower to an effect");

    workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "Builder DX")
        })
        .expect("builder workspace write should route through declared surfaces");
    let computed_patches = workspace.observe_computed("tasks.builder-title-list");
    let effect_inspection = workspace.inspect(&effect).expect("inspect should succeed");

    assert_eq!(computed_patches.derived_patches.len(), 1);
    match effect_inspection {
        ForgeQueryInspection::Effect(effect) => {
            assert_eq!(effect.trigger_source(), "tasks.builder-title-list");
        }
        other => panic!("expected effect inspection, got {other:?}"),
    }
}

#[test]
fn runtime_public_declaration_builders_support_downstream_vocab_layers() {
    let live = ForgeQueryLiveViewBuilder::surface("tasks.external-table")
        .from("Task")
        .select(["identity.id", "title.value"])
        .order_by("title.value")
        .allow_traversal_relation("manager", 2)
        .schema_basis("external-task-table")
        .build()
        .expect("public live declaration builder should lower without a workspace");
    let computed = ForgeQueryComputedBuilder::surface("tasks.external-summary")
        .reads(["title.value"])
        .produces(["summary.value"])
        .whole_refresh_fallback()
        .build()
        .expect("public computed declaration builder should lower without a workspace");

    assert_eq!(live.request().target(), "Task");
    assert_eq!(live.request().traversal().len(), 1);
    assert_eq!(live.request().traversal()[0].relation(), "manager");
    assert_eq!(live.request().traversal()[0].depth(), 2);
    assert_eq!(computed.name(), "tasks.external-summary");
    assert!(!computed.incremental());
}

#[test]
fn runtime_workspace_state_snapshots_are_async_safe_and_support_gated() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("task.state-workspace")
        .expect("task runtime should open a named workspace");
    let view: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.state-table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .schema_basis("runtime-task-state")
        })
        .expect("workspace live builder should lower to a live view");
    let titles = workspace
        .computed::<Value>(
            "tasks.state-title-list",
            |c| {
                c.depends_on_live(&view)
                    .reads(["title.value"])
                    .produces(["runtime.title_list"])
            },
            TitleListMaintainer,
        )
        .expect("workspace computed builder should lower to a derived view");

    workspace
        .insert("Task", |task| {
            task.aspect("identity.id", "task-1")
                .aspect("title.value", "State DX")
        })
        .expect("write should route through state surfaces");

    let live_state = workspace.state(&view).expect("live state should snapshot");
    let computed_state = workspace
        .state(&titles)
        .expect("computed state should snapshot");
    let temporal_state = workspace
        .state(ForgeQueryRuntimeFacadeFamily::Temporal)
        .expect("runtime-backed temporal family should report a typed state");

    assert_eq!(live_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(computed_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(temporal_state.kind(), ForgeQueryRuntimeStateKind::Ready);
}

#[test]
fn runtime_state_snapshot_is_digest_bound_to_basis_shape_lane_and_state() {
    let ready = ForgeQueryRuntimeStateSnapshot::ready(
        runtime_state_snapshot_basis_label_identity(&runtime_state_snapshot_test_subject_identity(
            "basis:current",
        )),
        runtime_state_snapshot_result_shape_label_identity(
            &runtime_state_snapshot_test_subject_identity("shape:table"),
        ),
        ForgeQueryAuthorityLane::AuthoritativeTruth,
        "sync runtime-backed rows are ready",
    );
    let pending = ForgeQueryRuntimeStateSnapshot::deferred(
        ForgeQueryRuntimeStateKind::Pending,
        runtime_state_snapshot_basis_label_identity(&runtime_state_snapshot_test_subject_identity(
            "basis:current",
        )),
        runtime_state_snapshot_result_shape_label_identity(
            &runtime_state_snapshot_test_subject_identity("shape:table"),
        ),
        ForgeQueryAuthorityLane::BridgeExternalState,
        "async/resource family is deferred",
    );

    assert_ne!(ready.state_digest(), pending.state_digest());
    assert!(pending.explanation().contains("deferred"));
}

#[test]
fn runtime_workspace_states_basis_lifecycle_surfaces() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.basis-state-workspace")
        .expect("task runtime should open a named workspace");
    let current = crate::query_basis_lifecycle::admit_observation_basis_intent(
        crate::query_basis_lifecycle::RawBasisIntent::current_head(
            crate::query_basis_lifecycle::BasisOperationLaneRequest::Observation,
        ),
    )
    .expect("current-head observation should admit");
    let preview = crate::query_basis_lifecycle::admit_observation_basis_intent(
        crate::query_basis_lifecycle::RawBasisIntent::preview(
            forge_runtime_bridge::facade::BridgePreviewSessionIdentity::from_stable_name(
                "preview:state-1",
            )
            .evidence_identity(),
            crate::query_basis_lifecycle::BasisOperationLaneRequest::Observation,
        ),
    )
    .expect("preview observation should admit as advisory");

    let current_state = workspace
        .state(&current)
        .expect("basis capability should snapshot");
    let preview_state = workspace
        .state(&preview)
        .expect("preview basis capability should snapshot");

    assert_eq!(current_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(preview_state.kind(), ForgeQueryRuntimeStateKind::Pending);
}

#[test]
fn runtime_workspace_inspection_surfaces_basis_lifecycle_artifacts() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.basis-inspection-workspace")
        .expect("task runtime should open a named workspace");
    let current = crate::query_basis_lifecycle::admit_observation_basis_intent(
        crate::query_basis_lifecycle::RawBasisIntent::current_head(
            crate::query_basis_lifecycle::BasisOperationLaneRequest::Observation,
        ),
    )
    .expect("current-head observation should admit");
    let world_basis = test_runtime_world_basis();

    let capability_inspection = workspace
        .inspect(&current)
        .expect("basis capability should inspect");
    let world_inspection = workspace
        .inspect(&world_basis)
        .expect("retained world basis should inspect");

    match capability_inspection {
        ForgeQueryInspection::BasisLifecycle(inspection) => {
            assert_eq!(inspection.subject_label(), "observation_basis_capability");
            assert_eq!(inspection.state_kind(), ForgeQueryRuntimeStateKind::Ready);
            assert_eq!(
                inspection.authority_lane(),
                ForgeQueryAuthorityLane::AuthoritativeTruth
            );
            assert_eq!(
                inspection
                    .family()
                    .expect("family should be present")
                    .as_str(),
                "current_head"
            );
        }
        other => panic!("expected basis lifecycle inspection, got {other:?}"),
    }

    match world_inspection {
        ForgeQueryInspection::BasisLifecycle(inspection) => {
            assert_eq!(inspection.subject_label(), "admitted_world_basis");
            assert_eq!(inspection.state_kind(), ForgeQueryRuntimeStateKind::Ready);
            assert_eq!(
                inspection.support_digest(),
                Some(world_basis.support_snapshot_digest())
            );
            assert_eq!(
                inspection.shape_digest(),
                world_basis.handle_identity_for_reporting()
            );
        }
        other => panic!("expected admitted world basis inspection, got {other:?}"),
    }
}

#[test]
#[should_panic(expected = "ready state should use ForgeQueryRuntimeStateSnapshot::ready")]
fn runtime_state_snapshot_rejects_ready_state_through_deferred_constructor() {
    let _ = ForgeQueryRuntimeStateSnapshot::deferred(
        ForgeQueryRuntimeStateKind::Ready,
        runtime_state_snapshot_basis_label_identity(&runtime_state_snapshot_test_subject_identity(
            "basis:current",
        )),
        runtime_state_snapshot_result_shape_label_identity(
            &runtime_state_snapshot_test_subject_identity("shape:table"),
        ),
        ForgeQueryAuthorityLane::AuthoritativeTruth,
        "ready state must not enter through the deferred constructor",
    );
}
