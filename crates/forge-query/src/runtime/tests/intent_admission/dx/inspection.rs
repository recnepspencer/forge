use super::*;

fn derived_summary_view(
    workspace: &mut ForgeQueryWorkspace,
    view_name: &str,
) -> ForgeQueryDerivedViewHandle<Value> {
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("intent-admission-derived-inspection")
        })
        .expect("live view should declare");
    workspace
        .computed_view::<Value>(
            ForgeQueryDerivedView::new(view_name, ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("derived view should declare")
}

#[test]
fn materialize_intent_common_path_helper_executes_through_canonical_handoff() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("derived-materialize-common", runtime)
        .expect("workspace should build");
    let derived = derived_summary_view(&mut workspace, "computed.materialize.common");
    workspace
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("derived-common-1")),
                ("title.value", json!("Derived common")),
            ],
        ))
        .expect("write should materialize derived output");

    let result = workspace
        .materialize_intent(&derived)
        .execute()
        .expect("materialize common path should execute");

    assert_eq!(result.rows().len(), 1);
    assert_eq!(
        result
            .receipt()
            .consumer_inspection()
            .expect("consumer lane should exist")
            .covered_entrypoint(),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization)
    );
}

#[test]
fn inspect_derived_intent_advanced_path_helper_exposes_request_eligibility_decision_and_handoff() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("derived-inspect-advanced", runtime)
        .expect("workspace should build");
    let derived = derived_summary_view(&mut workspace, "computed.inspect.advanced");

    let review = workspace
        .inspect_derived_intent(&derived)
        .review()
        .expect("derived inspection review should succeed");

    assert_eq!(
        review.request().family(),
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
    );
    assert_eq!(
        review.request().entrypoint(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection
    );
    assert_eq!(
        review.eligibility().routing_support_posture(),
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute
        )
    );
    assert!(review.admitted_handoff().is_some());
    assert_eq!(
        review.consumer_inspection().covered_entrypoint(),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection)
    );
}

#[test]
fn inspect_intent_common_path_helper_executes_through_canonical_handoff() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("generic-inspect-common", runtime)
        .expect("workspace should build");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("intent-admission-generic-inspection-common")
        })
        .expect("live view should declare");

    let result = workspace
        .inspect_intent(&live)
        .execute()
        .expect("generic inspection common path should execute");

    assert!(matches!(
        result.inspection(),
        ForgeQueryInspection::LiveView(_)
    ));
    assert_eq!(
        result
            .receipt()
            .consumer_inspection()
            .expect("consumer lane should exist")
            .covered_entrypoint(),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection)
    );
}

#[test]
fn inspect_intent_advanced_path_helper_exposes_request_eligibility_decision_and_handoff() {
    let runtime = read_runtime();
    let mut workspace = ForgeQueryWorkspace::new("generic-inspect-advanced", runtime)
        .expect("workspace should build");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("intent-admission-generic-inspection-advanced")
        })
        .expect("live view should declare");

    let review = workspace
        .inspect_intent(&live)
        .review()
        .expect("generic inspection review should succeed");

    assert_eq!(
        review.request().family(),
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
    );
    assert_eq!(
        review.request().entrypoint(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection
    );
    assert_eq!(
        review.eligibility().routing_support_posture(),
        ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
            ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute
        )
    );
    assert!(review.admitted_handoff().is_some());
    assert_eq!(
        review.consumer_inspection().covered_entrypoint(),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection)
    );
}
