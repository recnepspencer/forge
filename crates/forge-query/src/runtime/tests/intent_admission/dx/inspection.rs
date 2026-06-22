use super::*;

fn derived_summary_view(
    workspace: &mut ForgeQueryWorkspace,
    view_name: &str,
) -> ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> {
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("intent-admission-derived-inspection")
        })
        .expect("live view should declare");
    workspace
        .computed_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new(view_name, test_aspect_touches(["title"]))
                .depends_on_live(&live)
                .produces(test_aspect_touches(["title.summary"])),
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
                ("identity.id", test_string_aspect_value("derived-common-1")),
                ("title.value", test_string_aspect_value("Derived common")),
            ],
        ))
        .expect("write should materialize derived output");

    let result = workspace
        .materialize_intent(&derived)
        .execute()
        .expect("materialize common path should execute");

    assert_eq!(result.row_count(), 1);
    let value_path = retained_test_field_path("value").expect("value path should admit");
    let retained_value = result
        .single_retained_row()
        .expect("single materialized row should be retained")
        .field_value_at(&value_path)
        .expect("materialized row should retain value field");
    assert!(
        matches!(retained_value, AspectValue::String(_)),
        "materialized retained value should be a native string scalar, got {retained_value:?}"
    );
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
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
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
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
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
