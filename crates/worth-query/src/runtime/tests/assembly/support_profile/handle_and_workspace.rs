use super::super::super::support::*;
use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};
use crate::domain_installation::{
    WorthQueryDomainIdentityDeclaration, WorthQueryDomainIdentityName,
    WorthQueryDomainIdentityNamespace, WorthQueryDomainPackage, WorthQueryDomainSemanticVersion,
};
use crate::runtime::evidence_identities::{
    runtime_state_snapshot_basis_label_identity,
    runtime_state_snapshot_result_shape_label_identity,
    runtime_state_snapshot_test_subject_identity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TestRuntimeBasisDomain;

impl WorthQueryDomainEntryMarker for TestRuntimeBasisDomain {
    fn domain_key(&self) -> &'static str {
        "test.runtime.basis"
    }

    fn display_name(&self) -> &'static str {
        "TestRuntimeBasis"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TestRuntimeBasisContext;

impl WorthQueryDomainOperatingContext<TestRuntimeBasisDomain> for TestRuntimeBasisContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[]
    }

    fn context_identity_digest(&self) -> String {
        "operating:runtime-basis".to_string()
    }
}

fn test_runtime_basis_package() -> WorthQueryDomainPackage<TestRuntimeBasisDomain> {
    WorthQueryDomainPackage::declare(
        TestRuntimeBasisDomain,
        WorthQueryDomainIdentityDeclaration::new(
            WorthQueryDomainIdentityNamespace::new("test.runtime").unwrap(),
            WorthQueryDomainIdentityName::new("basis").unwrap(),
            WorthQueryDomainSemanticVersion::new(1, 0),
        ),
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
        .row(WorthQueryHandleContractFamily::LiveView)
        .expect("live handle contract row should exist");
    assert!(live_row.retained_artifact_required());
    assert_eq!(
        live_row.support_status(),
        WorthQueryRuntimeFamilySupportStatus::Supported
    );

    for family in [
        WorthQueryHandleContractFamily::TemporalCapableHandle,
        WorthQueryHandleContractFamily::AsyncResourceCapableHandle,
        WorthQueryHandleContractFamily::MixedCauseDeliveryCapableHandle,
    ] {
        let future_row = contract
            .row(family)
            .expect("future-capable handle contract row should exist");
        assert!(future_row.deferred_future_posture());
        assert_eq!(
            future_row.support_status(),
            WorthQueryRuntimeFamilySupportStatus::Supported
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
        WorthQueryRuntimeError::Workspace(error) => {
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
    let view: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view_request("tasks.workspace-table", task_live_request(), task_schema())
        .expect("workspace live view should declare");
    let inspection = workspace.inspect(&view).expect("inspect should succeed");

    assert_eq!(workspace.name(), "task.workspace");
    match inspection {
        WorthQueryInspection::LiveView(live) => {
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
    let view: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.builder-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                )
                .schema_basis("runtime-task-builder")
                .as_surface("tasks.builder-table")
        })
        .expect("workspace live builder should lower to a live view");
    let titles = workspace
        .computed::<WorthQueryNativeRow>(
            "tasks.builder-title-list",
            |c| {
                c.depends_on_live(&view)
                    .reads([test_aspect_touch("title.value")])
                    .produces([test_aspect_touch("runtime.title_list")])
            },
            TitleListMaintainer,
        )
        .expect("workspace computed builder should lower to a derived view");
    let effect = workspace
        .effect::<WorthQueryNativeRow>("tasks.builder-title-delivery", |e| {
            e.when_computed(&titles, [test_aspect_touch("runtime.title_list")])
                .condition_expression(
                    "expr.title-list-visible",
                    [test_aspect_touch("runtime.title_list")],
                    [test_aspect_touch("ui.title_list")],
                )
                .deliver("ui.title-list")
                .meaningful_change_suppression()
        })
        .expect("workspace effect builder should lower to an effect");

    workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-1"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("Builder DX"),
            )
        })
        .expect("builder workspace write should route through declared surfaces");
    let computed_patches = workspace.observe_computed(&titles);
    let effect_inspection = workspace.inspect(&effect).expect("inspect should succeed");

    assert_eq!(computed_patches.derived_patches.len(), 1);
    match effect_inspection {
        WorthQueryInspection::Effect(effect) => {
            assert_eq!(effect.trigger_source(), "tasks.builder-title-list");
        }
        other => panic!("expected effect inspection, got {other:?}"),
    }
}

#[test]
fn runtime_public_declaration_builders_support_downstream_vocab_layers() {
    let live = WorthQueryLiveViewBuilder::surface("tasks.external-table")
        .from("Task")
        .select([
            crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id").unwrap(),
            crate::authoring::AspectFieldKey::from_authoring_parts("title", "value").unwrap(),
        ])
        .order_by(crate::authoring::AspectFieldKey::from_authoring_parts("title", "value").unwrap())
        .allow_traversal_relation("manager", 2)
        .schema_basis("external-task-table")
        .build()
        .expect("public live declaration builder should lower without a workspace");
    let computed = WorthQueryComputedBuilder::surface("tasks.external-summary")
        .reads([test_aspect_touch("title.value")])
        .produces([test_aspect_touch("summary.value")])
        .whole_refresh_fallback()
        .build()
        .expect("public computed declaration builder should lower without a workspace");

    assert_eq!(live.request().target(), "Task");
    assert_eq!(live.request().traversal().len(), 1);
    assert_eq!(
        live.request().traversal()[0].relation_name().as_str(),
        "manager"
    );
    assert_eq!(live.request().traversal()[0].depth(), 2);
    assert_eq!(computed.name(), "tasks.external-summary");
    assert!(!computed.incremental());
}

#[test]
fn runtime_workspace_state_snapshots_are_async_safe_and_support_gated() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("task.state-workspace")
        .expect("task runtime should open a named workspace");
    let view: WorthQueryLiveView<WorthQueryNativeRow> = workspace
        .live_view("tasks.state-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                        .unwrap(),
                ])
                .schema_basis("runtime-task-state")
        })
        .expect("workspace live builder should lower to a live view");
    let titles = workspace
        .computed::<WorthQueryNativeRow>(
            "tasks.state-title-list",
            |c| {
                c.depends_on_live(&view)
                    .reads([test_aspect_touch("title.value")])
                    .produces([test_aspect_touch("runtime.title_list")])
            },
            TitleListMaintainer,
        )
        .expect("workspace computed builder should lower to a derived view");

    workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-1"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("State DX"),
            )
        })
        .expect("write should route through state surfaces");

    let live_state = workspace.state(&view).expect("live state should snapshot");
    let computed_state = workspace
        .state(&titles)
        .expect("computed state should snapshot");
    let temporal_state = workspace
        .state(WorthQueryRuntimeFacadeFamily::Temporal)
        .expect("runtime-backed temporal family should report a typed state");

    assert_eq!(live_state.kind(), WorthQueryRuntimeStateKind::Ready);
    assert_eq!(computed_state.kind(), WorthQueryRuntimeStateKind::Ready);
    assert_eq!(temporal_state.kind(), WorthQueryRuntimeStateKind::Ready);
}

#[test]
fn runtime_state_snapshot_is_digest_bound_to_basis_shape_lane_and_state() {
    let ready = WorthQueryRuntimeStateSnapshot::ready(
        runtime_state_snapshot_basis_label_identity(&runtime_state_snapshot_test_subject_identity(
            "basis:current",
        )),
        runtime_state_snapshot_result_shape_label_identity(
            &runtime_state_snapshot_test_subject_identity("shape:table"),
        ),
        WorthQueryAuthorityLane::AuthoritativeTruth,
        "sync runtime-backed rows are ready",
    );
    let pending = WorthQueryRuntimeStateSnapshot::deferred(
        WorthQueryRuntimeStateKind::Pending,
        runtime_state_snapshot_basis_label_identity(&runtime_state_snapshot_test_subject_identity(
            "basis:current",
        )),
        runtime_state_snapshot_result_shape_label_identity(
            &runtime_state_snapshot_test_subject_identity("shape:table"),
        ),
        WorthQueryAuthorityLane::BridgeExternalState,
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
    let current = crate::basis_lifecycle::basis_lifecycle()
        .current_head()
        .observe()
        .expect("current-head observation should admit");
    let branch = crate::basis_lifecycle::basis_lifecycle()
        .branch_head("branch:state-1", true)
        .observe()
        .expect("branch-head observation should admit");

    let current_state = workspace
        .state(&current)
        .expect("basis capability should snapshot");
    let branch_state = workspace
        .state(&branch)
        .expect("branch basis capability should snapshot");

    assert_eq!(current_state.kind(), WorthQueryRuntimeStateKind::Ready);
    assert_eq!(branch_state.kind(), WorthQueryRuntimeStateKind::Ready);
    assert_eq!(
        branch_state.authority_lane(),
        WorthQueryAuthorityLane::BranchLocalTruth
    );
}

#[test]
fn runtime_workspace_inspection_surfaces_basis_lifecycle_artifacts() {
    let runtime = stateful_bridge_task_runtime_with_domain(test_runtime_basis_package());
    let installed = runtime
        .domain(TestRuntimeBasisDomain)
        .expect("test basis domain should be installed");
    let world_basis = installed
        .declarations(&runtime, TestRuntimeBasisContext)
        .expect("installed test basis context should admit")
        .retained_world_basis();
    let workspace = runtime
        .workspace("task.basis-inspection-workspace")
        .expect("task runtime should open a named workspace");
    let current = crate::basis_lifecycle::basis_lifecycle()
        .current_head()
        .observe()
        .expect("current-head observation should admit");
    let capability_inspection = workspace
        .inspect(&current)
        .expect("basis capability should inspect");
    let world_inspection = workspace
        .inspect(&world_basis)
        .expect("retained world basis should inspect");

    match capability_inspection {
        WorthQueryInspection::BasisLifecycle(inspection) => {
            assert_eq!(inspection.subject_label(), "scoped_observation_basis");
            assert_eq!(inspection.state_kind(), WorthQueryRuntimeStateKind::Ready);
            assert_eq!(
                inspection.authority_lane(),
                WorthQueryAuthorityLane::AuthoritativeTruth
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
        WorthQueryInspection::BasisLifecycle(inspection) => {
            assert_eq!(inspection.subject_label(), "admitted_world_basis");
            assert_eq!(inspection.state_kind(), WorthQueryRuntimeStateKind::Ready);
            assert_eq!(
                inspection.support_digest(),
                Some(world_basis.support_snapshot_digest())
            );
            assert_eq!(
                inspection.shape_digest(),
                runtime_state_snapshot_result_shape_label_identity(world_basis.handle_identity())
                    .as_str()
            );
        }
        other => panic!("expected admitted world basis inspection, got {other:?}"),
    }
}

#[test]
#[should_panic(expected = "ready state should use WorthQueryRuntimeStateSnapshot::ready")]
fn runtime_state_snapshot_rejects_ready_state_through_deferred_constructor() {
    let _ = WorthQueryRuntimeStateSnapshot::deferred(
        WorthQueryRuntimeStateKind::Ready,
        runtime_state_snapshot_basis_label_identity(&runtime_state_snapshot_test_subject_identity(
            "basis:current",
        )),
        runtime_state_snapshot_result_shape_label_identity(
            &runtime_state_snapshot_test_subject_identity("shape:table"),
        ),
        WorthQueryAuthorityLane::AuthoritativeTruth,
        "ready state must not enter through the deferred constructor",
    );
}
