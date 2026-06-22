use super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField};
use crate::basis::{
    preflight_execution_basis, resolve_snapshot_basis, BasisResolutionMode, ExecutionBasisIntent,
    SnapshotLineageClass,
};
use crate::query_context::{
    admit_query_basis_context, bind_query_basis_context, QueryBasisContextRequest,
    QueryContextBindingSource,
};
use crate::schema_view::SchemaRelationView;

mod hardening;

#[test]
fn read_family_dispatch_fires_for_matching_declared_collection() {
    let mut workspace = workspace_with_read_obligation(
        "read-family-matching",
        "user",
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::ReadFamily,
        ),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let family = identity_read_family(&mut workspace, "tasks");

    let result = workspace
        .execute_read_family(&family)
        .expect("matching read-family obligation should allow execution");
    let dispatch = result
        .receipt()
        .graph_obligation_dispatch()
        .expect("read result should retain graph obligation dispatch");

    assert_eq!(dispatch.selection().matched_obligation_count(), 1);
    assert_eq!(
        dispatch.envelope().unwrap().context().kind(),
        ForgeQueryGraphObligationDispatchContextKind::ReadFamily
    );
    assert_eq!(
        dispatch
            .selection()
            .counters()
            .registration_full_scan_count(),
        0
    );
    assert_eq!(
        result.receipt().graph_obligation_envelope_digest(),
        dispatch.envelope_digest()
    );
}

#[test]
fn unrelated_read_family_keeps_selection_counters_without_fake_envelope() {
    let mut workspace = workspace_with_read_obligation(
        "read-family-non-matching",
        "unrelated",
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::ReadFamily,
        ),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let family = identity_read_family(&mut workspace, "tasks");

    let result = workspace
        .read_family_intent(&family)
        .execute()
        .expect("unrelated read-family obligation should not fire");
    let dispatch = result
        .receipt()
        .graph_obligation_dispatch()
        .expect("descriptor-backed read should retain no-match selection evidence");

    assert_eq!(dispatch.selection().matched_obligation_count(), 0);
    assert!(dispatch.envelope().is_none());
    assert_eq!(dispatch.envelope_digest(), None);
    assert!(
        dispatch
            .selection()
            .counters()
            .attempted_bucket_lookup_count()
            > 0
    );
}

#[test]
fn read_family_helper_fronts_retain_equivalent_obligation_evidence() {
    let mut execute_workspace = workspace_with_read_obligation(
        "read-family-execute-front",
        "user",
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::ReadFamily,
        ),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let execute_family = identity_read_family(&mut execute_workspace, "tasks");
    let execute_result = execute_workspace
        .execute_read_family(&execute_family)
        .expect("execute_read_family should execute");

    let mut intent_workspace = workspace_with_read_obligation(
        "read-family-intent-front",
        "user",
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::ReadFamily,
        ),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let intent_family = identity_read_family(&mut intent_workspace, "tasks");
    let intent_result = intent_workspace
        .read_family_intent(&intent_family)
        .execute()
        .expect("read_family_intent should execute");

    assert_eq!(
        execute_result.receipt().graph_obligation_envelope_digest(),
        intent_result.receipt().graph_obligation_envelope_digest()
    );
    assert_eq!(
        execute_result
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .selection()
            .matched_obligation_count(),
        intent_result
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .selection()
            .matched_obligation_count()
    );
}

#[test]
fn blocking_read_family_obligation_denies_before_execution() {
    let mut workspace = workspace_with_read_obligation(
        "read-family-blocked",
        "user",
        ForgeQueryGraphObligationSupportPosture::unsupported(
            ForgeQueryGraphObligationSupportLane::ReadFamily,
        ),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let family = identity_read_family(&mut workspace, "tasks");
    let error = workspace
        .read_family_intent(&family)
        .execute()
        .expect_err("unsupported selected read obligation should block execution");

    match error {
        ForgeQueryRuntimeError::GraphObligationDenied(denial) => {
            assert_eq!(denial.blocking_count(), 1);
        }
        other => panic!("unexpected read obligation denial: {other:?}"),
    }
}

#[test]
fn live_read_helper_fronts_retain_equivalent_obligation_evidence() {
    let mut intent_workspace = workspace_with_live_read_obligation("live-read-intent-front");
    let intent_view = live_view(&mut intent_workspace);
    let intent_result = intent_workspace
        .read_live_intent(&intent_view)
        .execute()
        .expect("live read intent should execute");

    let mut helper_workspace = workspace_with_live_read_obligation("live-read-helper-front");
    let helper_view = live_view(&mut helper_workspace);
    let helper_result = helper_workspace
        .read_live_result(&helper_view)
        .expect("read_live_result should execute");

    assert_eq!(
        intent_result.receipt().graph_obligation_envelope_digest(),
        helper_result.receipt().graph_obligation_envelope_digest()
    );
    assert_eq!(
        helper_result
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .envelope()
            .unwrap()
            .context()
            .kind(),
        ForgeQueryGraphObligationDispatchContextKind::LiveRead
    );
}

#[test]
fn read_family_in_basis_context_uses_basis_operating_world() {
    let mut workspace = workspace_with_read_obligation(
        "read-family-branch-world",
        "user",
        ForgeQueryGraphObligationSupportPosture::supported(
            ForgeQueryGraphObligationSupportLane::ReadFamily,
        ),
        ForgeQueryGraphObligationOperatingWorldSelector::branch(),
    );
    let family = identity_read_family(&mut workspace, "tasks");
    let branch_context = branch_context_for_family(&family);

    let result = workspace
        .read_family_in_basis_context_intent(&family, &branch_context)
        .execute()
        .expect("branch-basis read-family obligation should allow execution");
    let dispatch = result
        .receipt()
        .graph_obligation_dispatch()
        .expect("branch-basis read should retain graph obligation dispatch");

    assert_eq!(dispatch.selection().matched_obligation_count(), 1);
    assert_eq!(
        dispatch.selection().operating_world_digest(),
        ForgeQueryGraphObligationOperatingWorldDescriptor::branch().descriptor_digest()
    );
    assert_eq!(
        dispatch.envelope().unwrap().context().kind(),
        ForgeQueryGraphObligationDispatchContextKind::ReadFamily
    );
}

#[test]
fn live_read_result_helper_preserves_obligation_evidence() {
    let mut result_workspace = workspace_with_live_read_obligation("live-read-result-front");
    let result_view = live_view(&mut result_workspace);
    let result_helper = result_workspace
        .read_live_result(&result_view)
        .expect("live read result helper should execute");

    let mut intent_workspace = workspace_with_live_read_obligation("live-read-result-intent-front");
    let intent_view = live_view(&mut intent_workspace);
    let intent_result = intent_workspace
        .read_live_intent(&intent_view)
        .execute()
        .expect("live read intent should execute");

    assert_eq!(
        result_helper.receipt().graph_obligation_envelope_digest(),
        intent_result.receipt().graph_obligation_envelope_digest()
    );
    assert_eq!(
        result_helper
            .receipt()
            .graph_obligation_dispatch()
            .unwrap()
            .selection()
            .matched_obligation_count(),
        1
    );
}

fn workspace_with_read_obligation(
    name: &str,
    collection: &str,
    support_posture: ForgeQueryGraphObligationSupportPosture,
    operating_world: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryWorkspace {
    let runtime = complete_backend_from_parts_builder()
        .graph_obligation(read_registration(
            collection,
            support_posture,
            operating_world,
        ))
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with read graph obligation");
    ForgeQueryWorkspace::new(name, runtime).expect("workspace should build")
}

fn workspace_with_live_read_obligation(name: &str) -> ForgeQueryWorkspace {
    let runtime = complete_backend_from_parts_builder()
        .graph_obligation(read_registration(
            "tasks.table",
            ForgeQueryGraphObligationSupportPosture::supported(
                ForgeQueryGraphObligationSupportLane::LiveRead,
            ),
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ))
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with live-read graph obligation");
    ForgeQueryWorkspace::new(name, runtime).expect("workspace should build")
}

fn read_registration(
    collection: &str,
    support_posture: ForgeQueryGraphObligationSupportPosture,
    operating_world: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::advisory_obligation(
        ForgeQueryGraphObligationRuleIdentity::new(
            "test.read-obligation-dispatch",
            collection,
            "v1",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::collection(collection).unwrap(),
        operating_world,
    )
    .with_support_posture(support_posture)
}

fn branch_context_for_family(
    family: &ForgeQueryReadFamily,
) -> crate::query_context::AdmittedQueryBasisContext {
    let preflight = runtime_preflight_for_family(family, "read-obligation-branch-basis");
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::branch_head("read-obligation-branch-basis"),
        QueryContextBindingSource::RuntimeBranch(&preflight),
    )
    .expect("branch basis context should bind");
    admit_query_basis_context(binding).expect("branch basis context should admit")
}

fn runtime_preflight_for_family(
    family: &ForgeQueryReadFamily,
    snapshot_token: &str,
) -> crate::facade::ExecutionPreflightBundle {
    let lineage_class = SnapshotLineageClass::CurrentHead;
    let intent = ExecutionBasisIntent::new(
        crate::basis::BasisAuthorityFamily::Runtime,
        lineage_class.clone(),
        false,
    );
    let identity = crate::basis::ResolvedSnapshotIdentity::new(
        crate::basis::BasisAuthorityFamily::Runtime,
        None,
        crate::memory_workspace::admit_external_snapshot_label(snapshot_token).evidence_identity(),
        family.read_graph().schema_basis().clone(),
        lineage_class,
    );
    let basis = resolve_snapshot_basis(intent, identity, BasisResolutionMode::RuntimeDirect)
        .expect("runtime basis should resolve");
    preflight_execution_basis(family.read_graph().execution_plan().clone(), basis)
        .expect("family preflight should build")
}

fn identity_read_family(
    workspace: &mut ForgeQueryWorkspace,
    family_name: &str,
) -> ForgeQueryReadFamily {
    workspace
        .define_read_family(family_name, |read| {
            read.local_detail(
                "user",
                manager_schema(),
                |query| {
                    query.project(
                        AspectFieldSelector::new("identity", "id")
                            .expect("identity projection should build"),
                    )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "id")
                            .expect("identity result-shape field should build"),
                    )
                },
            )
        })
        .expect("read family should define")
}

fn live_view(workspace: &mut ForgeQueryWorkspace) -> ForgeQueryLiveView<ForgeQueryNativeRow> {
    workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("read-obligation-live")
        })
        .expect("live view should declare")
}

fn manager_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "runtime-read-obligation-dispatch",
        [
            SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                SchemaFieldKind::String,
            ),
        ],
        [SchemaRelationView::new(
            crate::authoring::RelationName::new("manager")
                .expect("schema relation literal must be valid"),
            1,
        )],
    )
}
