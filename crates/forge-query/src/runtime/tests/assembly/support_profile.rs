use super::super::support::*;
use crate::memory_workspace::ForgeQueryAspect;

#[test]
fn runtime_support_profiles_expose_facade_family_posture() {
    let memory_runtime = task_runtime();
    let bridge_runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("complete backend parts should build");

    for family in [
        ForgeQueryRuntimeFacadeFamily::Read,
        ForgeQueryRuntimeFacadeFamily::Live,
        ForgeQueryRuntimeFacadeFamily::Computed,
        ForgeQueryRuntimeFacadeFamily::Effect,
        ForgeQueryRuntimeFacadeFamily::BranchPreview,
        ForgeQueryRuntimeFacadeFamily::Write,
        ForgeQueryRuntimeFacadeFamily::Inspect,
    ] {
        assert_eq!(
            memory_runtime
                .support_profile()
                .support_for(family)
                .expect("memory support row should exist")
                .status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
        assert_eq!(
            bridge_runtime
                .support_profile()
                .support_for(family)
                .expect("bridge-backed support row should exist")
                .status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
    }

    assert_eq!(
        memory_runtime.support_profile().posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );
    assert_eq!(
        bridge_runtime.support_profile().posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );

    assert_eq!(
        bridge_runtime
            .support_profile()
            .support_for(ForgeQueryRuntimeFacadeFamily::Intent)
            .expect("intent support row should exist")
            .status(),
        ForgeQueryRuntimeFamilySupportStatus::Unsupported
    );
    assert!(bridge_runtime
        .support_profile()
        .support_for(ForgeQueryRuntimeFacadeFamily::Live)
        .expect("live support row should exist")
        .evidence()
        .iter()
        .any(|evidence| evidence == "test-subscription-activation"));
    let support_profile = bridge_runtime.support_profile();
    let inspect_support = support_profile
        .support_for(ForgeQueryRuntimeFacadeFamily::Inspect)
        .expect("inspect support row should exist");
    assert!(inspect_support
        .authority_lanes()
        .contains(&ForgeQueryAuthorityLane::BranchLocalTruth));
    assert!(inspect_support
        .authority_lanes()
        .contains(&ForgeQueryAuthorityLane::PendingWriteIntent));
}

#[test]
fn runtime_public_api_contract_marks_future_async_surfaces_as_deferred() {
    let runtime = task_runtime();
    let contract = runtime.public_api_contract();

    assert_eq!(
        contract.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );
    assert_eq!(contract.deferred_family_count(), 5);
    assert!(!contract.contract_digest().is_empty());

    for (family, expected_reason) in [
        (ForgeQueryRuntimeFacadeFamily::Temporal, "Milestone 9.4"),
        (
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
            "Milestone 9.5",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
            "Milestone 9.6",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
            "Milestone 10",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
            "Milestone 11",
        ),
    ] {
        let row = contract
            .family(family)
            .expect("future support gate row should exist");
        assert_eq!(
            row.status(),
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
        );
        assert!(
            row.reason()
                .is_some_and(|reason| reason.contains(expected_reason)),
            "deferred row for {family:?} must name its owning milestone"
        );
        assert!(row.authority_lanes().is_empty());
        assert!(
            row.evidence().is_empty(),
            "deferred future gates must not pretend runtime evidence exists"
        );
    }
}

#[test]
fn runtime_public_support_matrix_freezes_stable_deferred_and_unsupported_rows() {
    let workspace = task_runtime()
        .workspace("task.support-matrix")
        .expect("task runtime should open a named workspace");
    let matrix = workspace.public_support_matrix();
    let contract = workspace.public_api_contract();

    assert_eq!(
        matrix.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );
    assert_eq!(matrix.stable_row_count(), contract.stable_family_count());
    assert_eq!(
        matrix.deferred_row_count(),
        contract.deferred_family_count() + 1
    );
    assert_eq!(
        matrix.unsupported_row_count(),
        contract.unsupported_family_count()
    );
    assert_eq!(
        matrix.parallel_api_forbidden_row_count(),
        matrix.rows().len(),
        "every public support row must forbid sibling facade families"
    );
    assert_eq!(
        matrix.fail_closed_row_count(),
        matrix.deferred_row_count() + matrix.unsupported_row_count()
    );
    assert!(!matrix.matrix_digest().is_empty());

    for family in [
        ForgeQueryRuntimeFacadeFamily::Read,
        ForgeQueryRuntimeFacadeFamily::Live,
        ForgeQueryRuntimeFacadeFamily::Computed,
        ForgeQueryRuntimeFacadeFamily::Effect,
        ForgeQueryRuntimeFacadeFamily::BranchPreview,
        ForgeQueryRuntimeFacadeFamily::Write,
        ForgeQueryRuntimeFacadeFamily::Inspect,
    ] {
        let row = matrix
            .row_for_family(family)
            .expect("stable family should have matrix row");
        assert_eq!(
            row.status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
        assert_eq!(row.owner_milestone(), "Milestone 9.3");
        assert!(!row.admission_fail_closed());
        assert!(row.parallel_api_forbidden());
        assert!(row
            .extension_rule()
            .contains("handle-state-lane-aspect-inspection"));
        assert!(row.support_contract_digest().is_some());
    }

    for (surface, family, owner) in [
        (
            "temporal",
            ForgeQueryRuntimeFacadeFamily::Temporal,
            "Milestone 9.4",
        ),
        (
            "async-resource",
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
            "Milestone 9.5",
        ),
        (
            "mixed-cause-delivery",
            ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
            "Milestone 9.6",
        ),
        (
            "store-backed-execution",
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
            "Milestone 10",
        ),
        (
            "durable-artifacts",
            ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
            "Milestone 11",
        ),
    ] {
        let by_surface = matrix
            .row(surface)
            .expect("deferred family should have a named matrix row");
        let by_family = matrix
            .row_for_family(family)
            .expect("deferred family should have a family matrix row");
        assert_eq!(by_surface, by_family);
        assert_eq!(
            by_family.status(),
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
        );
        assert_eq!(by_family.owner_milestone(), owner);
        assert!(by_family.admission_fail_closed());
        assert!(by_family.parallel_api_forbidden());
        assert_eq!(
            by_family.extension_rule(),
            "must-extend-stabilized-handle-state-lane-aspect-inspection-facade"
        );
        assert!(by_family.support_contract_digest().is_some());
    }

    let certification = matrix
        .row("temporal-async-certification")
        .expect("9.7 certification gate must be explicit");
    assert_eq!(certification.facade_family(), None);
    assert_eq!(
        certification.status(),
        ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
    );
    assert_eq!(certification.owner_milestone(), "Milestone 9.7");
    assert!(certification.admission_fail_closed());
    assert!(certification.parallel_api_forbidden());
    assert!(certification.support_contract_digest().is_none());

    let intent = matrix
        .row_for_family(ForgeQueryRuntimeFacadeFamily::Intent)
        .expect("unsupported intent family should still be visible");
    assert_eq!(
        intent.status(),
        ForgeQueryRuntimeFamilySupportStatus::Unsupported
    );
    assert!(intent.admission_fail_closed());
}

#[test]
fn runtime_public_support_gate_denies_deferred_and_unsupported_families_before_use() {
    let workspace = task_runtime()
        .workspace("task.support-gate")
        .expect("task runtime should open a named workspace");

    let read = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Read)
        .expect("supported read family should admit");
    assert_eq!(read.family(), ForgeQueryRuntimeFacadeFamily::Read);
    assert_eq!(
        read.status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );

    for (family, expected_reason) in [
        (ForgeQueryRuntimeFacadeFamily::Temporal, "Milestone 9.4"),
        (
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
            "Milestone 9.5",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
            "Milestone 9.6",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
            "Milestone 10",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
            "Milestone 11",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::Intent,
            "intent commit strategies",
        ),
    ] {
        let error = workspace
            .admit_public_api_family(family)
            .expect_err("unsupported or deferred public API family should fail closed");
        match error {
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
                assert_eq!(denial.family(), family);
                assert!(
                    denial.reason().contains(expected_reason),
                    "denial for {family:?} should mention {expected_reason}, got {}",
                    denial.reason()
                );
            }
            other => panic!("expected typed public support denial, got {other:?}"),
        }
    }
}

#[test]
fn runtime_public_api_naming_contract_prefers_workspace_surface_names() {
    let contract = ForgeQueryRuntime::public_api_naming_contract();

    assert_eq!(contract.preferred_name_for("workspace"), Some("workspace"));
    assert_eq!(contract.preferred_name_for("live-view"), Some("live_view"));
    assert_eq!(contract.preferred_name_for("computed"), Some("computed"));
    assert_eq!(contract.preferred_name_for("effect"), Some("effect"));
    assert_eq!(contract.preferred_name_for("intent"), Some("intent"));
    assert_eq!(contract.preferred_name_for("read"), Some("read"));
    assert_eq!(contract.preferred_name_for("state"), Some("state"));
    assert_eq!(contract.preferred_name_for("observe"), Some("observe"));
    assert_eq!(contract.preferred_name_for("inspect"), Some("inspect"));
    assert!(contract.rows().iter().any(|row| {
        row.concept() == "computed"
            && row
                .compatibility_names()
                .iter()
                .any(|name| name == "computed_definition")
    }));
    assert!(contract.rows().iter().all(|row| {
        row.preferred_name() != "computed_declaration"
            && !row
                .compatibility_names()
                .iter()
                .any(|name| name == "computed_declaration")
    }));
    assert!(contract
        .rows()
        .iter()
        .all(|row| row.preferred_name() != "surface"));
    assert!(contract.preferred_entrypoint_count() >= 12);
    assert!(contract.compatibility_name_count() >= 10);
    assert!(!contract.contract_digest().is_empty());
}

#[test]
fn runtime_public_handle_contract_freezes_inspection_sections_and_future_state_lanes() {
    let workspace = task_runtime()
        .workspace("task.handle-contract")
        .expect("task runtime should open a named workspace");
    let contract = workspace.public_handle_contract();
    let support_contract = workspace.public_api_contract();

    assert_eq!(
        contract.support_contract_digest(),
        support_contract.contract_digest()
    );
    assert!(contract.inspectable_family_count() >= 13);
    assert!(contract.retained_artifact_family_count() >= 10);
    assert_eq!(contract.deferred_future_family_count(), 1);
    assert!(!contract.contract_digest().is_empty());

    for row in contract.rows() {
        assert!(
            row.inspection_sections()
                .iter()
                .any(|section| section == "authority-lane"),
            "{:?} must make authority lane inspectable",
            row.family()
        );
        assert!(
            row.inspection_sections()
                .iter()
                .any(|section| section == "basis-lane"),
            "{:?} must make basis lane inspectable",
            row.family()
        );
        assert!(
            row.inspection_sections()
                .iter()
                .any(|section| section == "support-posture"),
            "{:?} must make support posture inspectable",
            row.family()
        );
        assert!(
            row.inspection_sections()
                .iter()
                .any(|section| section == "inspection-digest"),
            "{:?} must expose a digest-bound inspection contract",
            row.family()
        );
        assert!(
            !row.authority_lanes().is_empty(),
            "{:?} must name at least one authority lane",
            row.family()
        );
        assert!(
            !row.basis_lanes().is_empty(),
            "{:?} must name at least one basis lane",
            row.family()
        );
    }

    let live_row = contract
        .row(ForgeQueryHandleContractFamily::LiveView)
        .expect("live handle contract row should exist");
    assert!(live_row.retained_artifact_required());
    assert_eq!(
        live_row.support_status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert!(live_row
        .authority_lanes()
        .contains(&ForgeQueryAuthorityLane::AuthoritativeTruth));
    assert!(live_row
        .basis_lanes()
        .contains(&ForgeQueryAuthorityLane::AuthoritativeTruth));
    assert!(live_row
        .inspection_sections()
        .iter()
        .any(|section| section == "subscription-lifecycle"));
    assert!(live_row
        .inspection_sections()
        .iter()
        .any(|section| section == "inspection-digest"));

    let computed_row = contract
        .row(ForgeQueryHandleContractFamily::ComputedView)
        .expect("computed handle contract row should exist");
    assert!(computed_row.retained_artifact_required());
    assert_eq!(
        computed_row.support_status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert!(computed_row
        .authority_lanes()
        .contains(&ForgeQueryAuthorityLane::DerivedRuntimeState));
    assert!(computed_row
        .basis_lanes()
        .contains(&ForgeQueryAuthorityLane::AuthoritativeTruth));
    assert!(computed_row
        .inspection_sections()
        .iter()
        .any(|section| section == "dependency-aspects"));
    assert!(computed_row
        .inspection_sections()
        .iter()
        .any(|section| section == "materialization"));

    let effect_row = contract
        .row(ForgeQueryHandleContractFamily::Effect)
        .expect("effect handle contract row should exist");
    assert_eq!(
        effect_row.support_status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert!(effect_row
        .authority_lanes()
        .contains(&ForgeQueryAuthorityLane::PendingWriteIntent));
    assert!(effect_row
        .inspection_sections()
        .iter()
        .any(|section| section == "feedback-phase-graph"));

    let branch_binding_row = contract
        .row(ForgeQueryHandleContractFamily::BranchBinding)
        .expect("branch binding contract row should exist");
    assert!(!branch_binding_row.retained_artifact_required());
    assert_eq!(
        branch_binding_row.support_status(),
        ForgeQueryRuntimeFamilySupportStatus::Unsupported
    );
    assert!(branch_binding_row
        .basis_lanes()
        .contains(&ForgeQueryAuthorityLane::BranchLocalTruth));
    assert!(branch_binding_row
        .inspection_sections()
        .iter()
        .any(|section| section == "unsupported-branch-handle-reuse"));

    let future_row = contract
        .row(ForgeQueryHandleContractFamily::TemporalAsyncCapableHandle)
        .expect("future temporal/async handle contract row should exist");
    assert!(!future_row.retained_artifact_required());
    assert!(future_row.deferred_future_posture());
    assert_eq!(
        future_row.support_status(),
        ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
    );
    assert!(future_row
        .authority_lanes()
        .contains(&ForgeQueryAuthorityLane::TemporalExecutionState));
    assert!(future_row
        .authority_lanes()
        .contains(&ForgeQueryAuthorityLane::AsyncResourceState));
    assert!(future_row
        .basis_lanes()
        .contains(&ForgeQueryAuthorityLane::TemporalExecutionState));
    assert!(future_row
        .inspection_sections()
        .iter()
        .any(|section| section == "deferred-support-posture"));
}

#[test]
fn runtime_workspace_rejects_empty_names_before_public_use() {
    let error = match task_runtime().workspace("  ") {
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
fn runtime_workspace_declares_observes_and_inspects_with_preferred_names() {
    let mut workspace = task_runtime()
        .workspace("task.workspace")
        .expect("task runtime should open a named workspace");
    let view: ForgeQueryLiveView<Value> = workspace
        .live_view_request("tasks.workspace-table", task_live_request(), task_schema())
        .expect("workspace live view should declare");

    let write = workspace
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Workspace facade" },
            }),
        })
        .expect("workspace write should execute");
    let patches = workspace.observe(&view);
    let inspection = workspace
        .inspect(&view)
        .expect("workspace inspect should use the unified inspection target");

    assert_eq!(workspace.name(), "task.workspace");
    assert_eq!(
        write.affected_live_view_ids(),
        &["tasks.workspace-table".to_string()]
    );
    assert_eq!(patches.query_delivery_batches.len(), 1);
    match inspection {
        ForgeQueryInspection::LiveView(live) => {
            assert_eq!(live.view_name(), "tasks.workspace-table");
            assert!(!live.installation_digest().is_empty());
            assert_eq!(
                live.authority_lane(),
                ForgeQueryAuthorityLane::AuthoritativeTruth
            );
        }
        other => panic!("expected workspace live inspection, got {other:?}"),
    }
}

#[test]
fn runtime_workspace_closure_builders_cover_live_computed_effect_dx() {
    let mut workspace = task_runtime()
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

    let write = workspace
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Builder DX" },
            }),
        })
        .expect("builder workspace write should route through declared surfaces");
    let live_patches = workspace.observe(&view);
    let computed_rows = workspace.materialize(&titles);
    let computed_patches = workspace.observe_computed("tasks.builder-title-list");
    let effect_inspection = workspace
        .inspect(&effect)
        .expect("builder effect should inspect through unified inspect");

    assert_eq!(
        write.affected_live_view_ids(),
        &["tasks.builder-table".to_string()]
    );
    assert_eq!(live_patches.query_delivery_batches.len(), 1);
    assert_eq!(computed_rows.len(), 1);
    assert_eq!(computed_patches.derived_patches.len(), 1);
    match effect_inspection {
        ForgeQueryInspection::Effect(effect) => {
            assert_eq!(effect.trigger_source(), "tasks.builder-title-list");
            assert_eq!(effect.pending_delivery_count(), 1);
            assert!(!effect.condition_digest().is_empty());
        }
        other => panic!("expected effect inspection, got {other:?}"),
    }
}

#[test]
fn runtime_workspace_state_snapshots_are_async_safe_and_support_gated() {
    let mut workspace = task_runtime()
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
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "State DX" },
            }),
        })
        .expect("write should route through state surfaces");

    let live_state = workspace
        .state(&view)
        .expect("live handle state should snapshot retained subscription evidence");
    let computed_state = workspace
        .state(&titles)
        .expect("computed handle state should snapshot retained materialization evidence");
    let temporal_state = workspace
        .state(ForgeQueryRuntimeFacadeFamily::Temporal)
        .expect("deferred temporal family should report a typed state");
    let async_state = workspace
        .state(ForgeQueryRuntimeFacadeFamily::AsyncResource)
        .expect("deferred async family should report a typed state");
    let intent_state = workspace
        .state(ForgeQueryRuntimeFacadeFamily::Intent)
        .expect("unsupported intent family should report a typed state");

    assert_eq!(live_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        live_state.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert!(live_state.explanation().contains("retained subscription"));
    assert_eq!(computed_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        computed_state.authority_lane(),
        ForgeQueryAuthorityLane::DerivedRuntimeState
    );
    assert!(computed_state
        .explanation()
        .contains("retained materialization"));
    assert_ne!(live_state.state_digest(), computed_state.state_digest());

    assert_eq!(temporal_state.kind(), ForgeQueryRuntimeStateKind::Pending);
    assert_eq!(
        temporal_state.authority_lane(),
        ForgeQueryAuthorityLane::TemporalExecutionState
    );
    assert!(temporal_state.explanation().contains("Milestone 9.4"));
    assert_eq!(async_state.kind(), ForgeQueryRuntimeStateKind::Pending);
    assert_eq!(
        async_state.authority_lane(),
        ForgeQueryAuthorityLane::AsyncResourceState
    );
    assert!(async_state.explanation().contains("Milestone 9.5"));
    assert_eq!(intent_state.kind(), ForgeQueryRuntimeStateKind::Unsupported);
    assert_eq!(
        intent_state.authority_lane(),
        ForgeQueryAuthorityLane::PendingWriteIntent
    );
    assert_ne!(temporal_state.state_digest(), async_state.state_digest());
    assert_ne!(async_state.state_digest(), intent_state.state_digest());
}

#[test]
fn runtime_state_snapshot_is_digest_bound_to_basis_shape_lane_and_state() {
    let ready = ForgeQueryRuntimeStateSnapshot::ready(
        "basis:current",
        "shape:table",
        ForgeQueryAuthorityLane::AuthoritativeTruth,
        "sync runtime-backed rows are ready",
    );
    let pending = ForgeQueryRuntimeStateSnapshot::deferred(
        ForgeQueryRuntimeStateKind::Pending,
        "basis:current",
        "shape:table",
        ForgeQueryAuthorityLane::BridgeExternalState,
        "async/resource family is deferred",
    );

    assert_eq!(ready.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(ready.basis_digest(), "basis:current");
    assert_eq!(ready.result_shape_digest(), "shape:table");
    assert_eq!(
        ready.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(pending.kind(), ForgeQueryRuntimeStateKind::Pending);
    assert_eq!(
        pending.authority_lane(),
        ForgeQueryAuthorityLane::BridgeExternalState
    );
    assert_ne!(ready.state_digest(), pending.state_digest());
    assert!(
        pending.explanation().contains("deferred"),
        "pending/deferred state must carry a localizable explanation"
    );
}

#[test]
#[should_panic(expected = "ready state should use ForgeQueryRuntimeStateSnapshot::ready")]
fn runtime_state_snapshot_rejects_ready_state_through_deferred_constructor() {
    let _ = ForgeQueryRuntimeStateSnapshot::deferred(
        ForgeQueryRuntimeStateKind::Ready,
        "basis:current",
        "shape:table",
        ForgeQueryAuthorityLane::AuthoritativeTruth,
        "ready state must not enter through the deferred constructor",
    );
}

#[test]
fn compatibility_memory_backend_constructor_is_explicit_and_runtime_builder_matches_it() {
    let backend = ForgeQueryMemoryApp::compatibility_backend([ForgeQueryCollection::new(
        "Task",
        [ForgeQueryAspect::new("title", "title.value")],
    )])
    .expect("compatibility backend should build");
    assert_eq!(
        crate::runtime::ForgeQueryRuntimeBackend::support_profile(&backend).posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );

    let runtime = ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([ForgeQueryCollection::new(
            "Task",
            [ForgeQueryAspect::new("title", "title.value")],
        )])
        .build()
        .expect("compatibility in-memory runtime should build");
    assert_eq!(
        runtime.support_profile().posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );
}

#[test]
fn runtime_support_denies_unsupported_write_family_before_execution() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Write,
                "test backend disabled write authority",
            ),
        ),
    );

    let error = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "external-1" },
                "title": { "value": "Should not write" },
            }),
        })
        .expect_err("unsupported write family should deny before write authority");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Write);
            assert_eq!(denial.reason(), "test backend disabled write authority");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_builder_rejects_support_profiles_that_overclaim_unimplemented_families() {
    let profile = ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
        ForgeQueryRuntimeFamilySupport::supported(
            ForgeQueryRuntimeFacadeFamily::Intent,
            [ForgeQueryAuthorityLane::PendingWriteIntent],
            [ForgeQueryEffectPolicy::AuthoritativeAllowed],
            ["fake-intent-adapter"],
        ),
    );

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .support_profile(profile)
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("support profile must not claim unimplemented facade support"),
        Err(error) => error,
    };

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Intent);
            assert!(denial.reason().contains("intent authority adapter"));
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_denies_unsupported_computed_family_before_registration() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Computed,
                "test backend disabled computed resources",
            ),
        ),
    );

    let error = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("task_titles.unsupported", ["title".to_string()]),
            TitleListMaintainer,
        )
        .expect_err("unsupported computed family should deny before registration");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Computed);
            assert_eq!(denial.reason(), "test backend disabled computed resources");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_denies_unsupported_preview_and_branch_sessions_without_panicking() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::BranchPreview,
                "test backend disabled branch and preview sessions",
            ),
        ),
    );

    let preview_error = match runtime.preview("unsupported preview") {
        Ok(_) => panic!("unsupported preview should return a typed denial"),
        Err(error) => error,
    };
    match preview_error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(
                denial.family(),
                ForgeQueryRuntimeFacadeFamily::BranchPreview
            );
            assert_eq!(
                denial.reason(),
                "test backend disabled branch and preview sessions"
            );
        }
        other => panic!("expected unsupported preview family denial, got {other:?}"),
    }

    let branch_error = match runtime.branch("unsupported branch") {
        Ok(_) => panic!("unsupported branch should return a typed denial"),
        Err(error) => error,
    };
    match branch_error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(
                denial.family(),
                ForgeQueryRuntimeFacadeFamily::BranchPreview
            );
            assert_eq!(
                denial.reason(),
                "test backend disabled branch and preview sessions"
            );
        }
        other => panic!("expected unsupported branch family denial, got {other:?}"),
    }
}
