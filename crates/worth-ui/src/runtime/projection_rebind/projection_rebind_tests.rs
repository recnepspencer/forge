use super::projection_rebind_test_support::*;
use crate::capability::{AppearanceTokenId, CommandId, CommandProjectionId, DensityTokenId};
use crate::runtime::{
    WorthUiDropdownAppearanceRequest, WorthUiDropdownProjectionRequest,
    WorthUiHeaderFrameRebindStatus, WorthUiPageHostRebindStatus, WorthUiPageHostRequest,
    WorthUiProjectionFrameReplayCertification, WorthUiProjectionRebindBatchAggregationDenial,
    WorthUiProjectionRebindBatchReceipt, WorthUiProjectionRebindPlanDenial,
    WorthUiProjectionRebindStatus, WorthUiReloadProjectionBreadthCertification,
};

#[test]
fn ready_evidence_is_denied_before_projection_rebuild_can_run() {
    let app = projection_rebind_app("Save");
    let runtime = runtime_for_source_authored_page_host(&app);
    let page_plan = page_host_plan(&runtime);
    let admitted_page = runtime.admit_projection_plan(page_plan).unwrap();
    let admitted_change = runtime
        .admit_validation_runtime_change(&validation_ready(&runtime))
        .unwrap();

    let denial = runtime
        .prepare_projection_rebind(&admitted_change, admitted_page)
        .expect_err("ready evidence must not enter rebuild planning");

    assert_eq!(
        denial,
        WorthUiProjectionRebindPlanDenial::ReloadNotActivated
    );
}

#[test]
fn denied_and_equivalent_evidence_preserve_projection_without_rebuild() {
    let app = projection_rebind_app("Save");
    let runtime = runtime_for_source_authored_page_host(&app);
    let header_plan = header_frame_plan(&app);
    let denied = runtime
        .admit_capability_runtime_change(&capability_denied(&runtime))
        .unwrap();
    let equivalent = runtime
        .admit_capability_runtime_change(&capability_equivalent(&runtime))
        .unwrap();

    let denied_batch = match prepare_projection_rebind_plan(&runtime, header_plan.clone(), &denied)
    {
        crate::runtime::WorthUiProjectionRebindPlan::Preserve(preserved) => {
            preserved.complete_preserved().1
        }
        crate::runtime::WorthUiProjectionRebindPlan::Rebuild(_) => {
            panic!("denied evidence must stay preserved")
        }
    };
    let equivalent_batch = match prepare_projection_rebind_plan(&runtime, header_plan, &equivalent)
    {
        crate::runtime::WorthUiProjectionRebindPlan::Preserve(preserved) => {
            preserved.complete_preserved().1
        }
        crate::runtime::WorthUiProjectionRebindPlan::Rebuild(_) => {
            panic!("equivalent evidence must stay preserved")
        }
    };

    assert_eq!(
        denied_batch.rows()[0].status(),
        WorthUiProjectionRebindStatus::PreservedDeniedReload
    );
    assert_eq!(denied_batch.counters().rebuild_attempt_count(), 0);
    assert_eq!(
        equivalent_batch.rows()[0].status(),
        WorthUiProjectionRebindStatus::PreservedEquivalentReload
    );
    assert_eq!(equivalent_batch.counters().preserved_frame_count(), 1);
}

#[test]
fn activated_header_fact_rebuilds_header_and_preserves_page_host_by_intersection() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let header_plan = header_frame_plan(&app);
    let page_plan = page_host_plan(&runtime);
    let evidence = capability_activated(&mut runtime, "theme.header.text", "#ffffff");
    let admitted = runtime.admit_capability_runtime_change(&evidence).unwrap();

    let header_batch = rebuilt_header_batch(&mut runtime, header_plan, &evidence);
    let page_batch = match prepare_projection_rebind_plan(&runtime, page_plan, &admitted) {
        crate::runtime::WorthUiProjectionRebindPlan::Preserve(preserved) => {
            preserved.complete_preserved().1
        }
        crate::runtime::WorthUiProjectionRebindPlan::Rebuild(_) => {
            panic!("page host must stay preserved for header-only fact change")
        }
    };

    assert_eq!(header_batch.counters().dependency_intersection_count(), 1);
    assert_eq!(header_batch.counters().rebuild_attempt_count(), 1);
    assert_eq!(page_batch.counters().dependency_intersection_count(), 0);
    assert_eq!(
        page_batch.rows()[0].status(),
        WorthUiProjectionRebindStatus::EquivalentAfterActivation
    );
}

#[test]
fn batch_receipt_reports_multi_projection_breadth_not_total_runtime_breadth() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let evidence = capability_activated(&mut runtime, "theme.header.text", "#ffffff");
    let admitted = runtime.admit_capability_runtime_change(&evidence).unwrap();

    let header_batch = rebuilt_header_batch(&mut runtime, header_frame_plan(&app), &evidence);
    let page_batch =
        match prepare_projection_rebind_plan(&runtime, page_host_plan(&runtime), &admitted) {
            crate::runtime::WorthUiProjectionRebindPlan::Preserve(preserved) => {
                preserved.complete_preserved().1
            }
            crate::runtime::WorthUiProjectionRebindPlan::Rebuild(_) => {
                panic!("page host must stay preserved for header-only fact change")
            }
        };
    let aggregate =
        crate::runtime::WorthUiProjectionRebindBatchReceipt::aggregate([header_batch, page_batch])
            .expect("same evidence and runtime can aggregate projection rows");

    assert_eq!(aggregate.rows().len(), 4);
    assert_eq!(aggregate.counters().inspected_projection_count(), 4);
    assert_eq!(aggregate.counters().dependency_intersection_count(), 1);
    assert_eq!(aggregate.counters().rebuild_attempt_count(), 1);
    assert_eq!(aggregate.counters().preserved_frame_count(), 3);
    assert_eq!(aggregate.counters().rebuilt_frame_count(), 1);
}

#[test]
fn projection_frame_replay_certifies_real_rebind_frame_convergence() {
    let app = projection_rebind_app("Save");
    let mut original_runtime = runtime_for_source_authored_page_host(&app);
    let mut replayed_runtime = runtime_for_source_authored_page_host(&app);
    let original_evidence =
        capability_activated(&mut original_runtime, "theme.header.text", "#ffffff");
    let replayed_evidence =
        capability_activated(&mut replayed_runtime, "theme.header.text", "#ffffff");
    let original_change = original_runtime
        .admit_capability_runtime_change(&original_evidence)
        .expect("original change admits");
    let replayed_change = replayed_runtime
        .admit_capability_runtime_change(&replayed_evidence)
        .expect("replayed change admits");
    let original_batch = aggregate_header_and_page_rebinds(
        &app,
        &mut original_runtime,
        &original_evidence,
        &original_change,
    );
    let replayed_batch = aggregate_header_and_page_rebinds(
        &app,
        &mut replayed_runtime,
        &replayed_evidence,
        &replayed_change,
    );
    let original_breadth =
        WorthUiReloadProjectionBreadthCertification::certify(&original_change, &original_batch)
            .expect("original projection breadth certifies");
    let replayed_breadth =
        WorthUiReloadProjectionBreadthCertification::certify(&replayed_change, &replayed_batch)
            .expect("replayed projection breadth certifies");

    let replay = WorthUiProjectionFrameReplayCertification::certify(
        &original_breadth,
        &original_batch,
        &replayed_breadth,
        &replayed_batch,
    )
    .expect("projection frame rows converge under replay");

    assert_eq!(replay.projection_frame_count(), 4);
    assert_ne!(replay.projection_frame_replay_digest().raw(), 0);
}

#[test]
fn batch_receipt_rejects_cross_runtime_or_cross_evidence_aggregation() {
    let app = projection_rebind_app("Save");
    let mut runtime_left = runtime_for_source_authored_page_host(&app);
    let mut runtime_right = runtime_for_source_authored_page_host(&app);
    let left_evidence = capability_activated(&mut runtime_left, "theme.header.text", "#ffffff");
    let right_evidence = capability_activated(&mut runtime_right, "theme.header.text", "#ffffff");

    let left_batch =
        rebuilt_header_batch(&mut runtime_left, header_frame_plan(&app), &left_evidence);
    let right_batch =
        rebuilt_header_batch(&mut runtime_right, header_frame_plan(&app), &right_evidence);
    let cross_runtime_denial =
        crate::runtime::WorthUiProjectionRebindBatchReceipt::aggregate([left_batch, right_batch])
            .expect_err("cross-runtime receipts must not aggregate");

    assert_eq!(
        cross_runtime_denial,
        WorthUiProjectionRebindBatchAggregationDenial::RuntimeEvidenceMismatch
    );

    let first_evidence = capability_activated(&mut runtime_left, "theme.header.text", "#fefefe");
    let second_evidence = capability_activated(&mut runtime_left, "theme.header.menu", "#222222");
    let first_batch =
        rebuilt_header_batch(&mut runtime_left, header_frame_plan(&app), &first_evidence);
    let second_batch =
        rebuilt_header_batch(&mut runtime_left, header_frame_plan(&app), &second_evidence);
    let cross_evidence_denial =
        crate::runtime::WorthUiProjectionRebindBatchReceipt::aggregate([first_batch, second_batch])
            .expect_err("same-runtime receipts from different changes must not aggregate");

    assert_eq!(
        cross_evidence_denial,
        WorthUiProjectionRebindBatchAggregationDenial::RuntimeEvidenceMismatch
    );
}

fn aggregate_header_and_page_rebinds(
    app: &crate::facade::WorthUiApp,
    runtime: &mut crate::runtime::WorthUiRuntimeHost,
    evidence: &crate::runtime::WorthUiCapabilityReloadEvidence,
    change: &crate::runtime::WorthUiAdmittedRuntimeChangeEvidence,
) -> WorthUiProjectionRebindBatchReceipt {
    let header_batch = rebuilt_header_batch(runtime, header_frame_plan(app), evidence);
    let page_batch = match prepare_projection_rebind_plan(runtime, page_host_plan(runtime), change)
    {
        crate::runtime::WorthUiProjectionRebindPlan::Preserve(preserved) => {
            preserved.complete_preserved().1
        }
        crate::runtime::WorthUiProjectionRebindPlan::Rebuild(_) => {
            panic!("page host must stay preserved when changed facts do not intersect")
        }
    };
    WorthUiProjectionRebindBatchReceipt::aggregate([header_batch, page_batch])
        .expect("real projection rebind rows aggregate")
}

#[test]
fn empty_batch_aggregation_reports_structured_denial() {
    let denial = crate::runtime::WorthUiProjectionRebindBatchReceipt::aggregate([])
        .expect_err("empty aggregation must report why no batch receipt could be produced");

    assert_eq!(
        denial,
        WorthUiProjectionRebindBatchAggregationDenial::EmptyBatch
    );
}

#[test]
fn source_artifact_fact_rebuilds_page_host_through_family_adapter() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let page_plan = page_host_plan(&runtime);
    let evidence = validation_activated(&mut runtime);

    let (_, receipt) = runtime
        .rebind_page_host_after_reload(
            &page_plan,
            WorthUiPageHostRequest::new("ProductsPage"),
            &evidence,
        )
        .expect("activated source fact intersects page host dependencies");

    assert_eq!(
        receipt.status(),
        WorthUiPageHostRebindStatus::ReboundAfterActivation
    );
    assert_eq!(receipt.projection_rebuild_count(), 1);
    assert_eq!(receipt.projection_rebind_batch().rows().len(), 1);
}

#[test]
fn capability_reload_family_adapter_preserves_page_host_when_changed_facts_do_not_intersect() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let page_plan = page_host_plan(&runtime);
    let evidence = capability_activated(&mut runtime, "theme.header.text", "#ffffff");

    let (_, receipt) = runtime
        .rebind_page_host_after_capability_reload(
            &page_plan,
            WorthUiPageHostRequest::new("ProductsPage"),
            &evidence,
        )
        .expect("header-only capability change should preserve page-host projection");

    assert_eq!(
        receipt.status(),
        WorthUiPageHostRebindStatus::EquivalentAfterActivation
    );
    assert_eq!(
        receipt
            .projection_rebind_batch()
            .counters()
            .dependency_intersection_count(),
        0
    );
    assert_eq!(receipt.projection_rebuild_count(), 0);
}

#[test]
fn header_family_adapter_uses_shared_coordinator_counters() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let header_plan = header_frame_plan(&app);
    let evidence = capability_activated(&mut runtime, "theme.header.text", "#ffffff");

    let (_, receipt) = runtime
        .rebind_header_frame_after_capability_reload(
            &header_plan,
            header_rebind_request(),
            &evidence,
        )
        .expect("header rebind runs through coordinator");

    assert_eq!(
        receipt.status(),
        WorthUiHeaderFrameRebindStatus::ReboundAfterActivation
    );
    assert_eq!(receipt.projection_rebuild_count(), 1);
    assert_eq!(receipt.projection_rebind_batch().rows().len(), 3);
}

#[test]
fn foreign_runtime_projection_cannot_join_rebind_plan() {
    let app = projection_rebind_app("Save");
    let mut runtime_left = runtime_for_source_authored_page_host(&app);
    let runtime_right = runtime_for_source_authored_page_host(&app);
    let admitted_foreign = runtime_right
        .admit_projection_plan(header_frame_plan(&app))
        .unwrap();
    let activated = capability_activated(&mut runtime_left, "theme.header.text", "#ffffff");
    let evidence = runtime_left
        .admit_capability_runtime_change(&activated)
        .unwrap();

    let denial = runtime_left
        .prepare_projection_rebind(&evidence, admitted_foreign)
        .expect_err("foreign admitted projection cannot enter local rebind");

    assert_eq!(
        denial,
        WorthUiProjectionRebindPlanDenial::RuntimeEvidenceMismatch
    );
}

#[test]
fn dropdown_rebind_uses_real_runtime_selection_authority() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let projection_id = CommandProjectionId::new("workspace.header.file").unwrap();
    let command_id = CommandId::new("workspace.command.save").unwrap();
    runtime
        .select_dropdown_command(&projection_id, &command_id)
        .expect("runtime-owned dropdown interaction should seed selection");
    let current_plan = header_frame_plan(&app).menu_plan().dropdown_plans()[0].clone();
    let request = WorthUiDropdownProjectionRequest::for_command_projection(
        projection_id.clone(),
        crate::capability::ComponentId::new("validation.component.sample").unwrap(),
        crate::capability::ComponentId::new("validation.component.sample").unwrap(),
        WorthUiDropdownAppearanceRequest::new(
            AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
            DensityTokenId::new("density.header.row_padding").unwrap(),
            DensityTokenId::new("density.header.control_spacing").unwrap(),
        ),
    );
    let evidence = command_projection_activated(&mut runtime, "workspace.header.file = multi");

    let (rebound, receipt) = runtime
        .rebind_dropdown_projection_after_capability_reload(&current_plan, request, &evidence)
        .expect("dropdown-only rebind should use real boundary");

    assert_eq!(
        rebound
            .execute_frame()
            .selection_state()
            .selected_command_ids(),
        vec!["workspace.command.save".to_owned()]
    );
    assert_eq!(receipt.rows().len(), 1);
}

#[test]
fn header_rebind_after_interaction_uses_changed_fact_intersection_without_surface_shortcuts() {
    let app = projection_rebind_app("Save");
    let mut runtime = runtime_for_source_authored_page_host(&app);
    let projection_id = CommandProjectionId::new("workspace.header.file").unwrap();
    let command_id = CommandId::new("workspace.command.save").unwrap();
    let current_plan = header_frame_plan(&app);
    let interaction = runtime
        .select_dropdown_command(&projection_id, &command_id)
        .expect("runtime-owned dropdown interaction should be recorded");
    let admitted = runtime
        .admit_dropdown_selection_runtime_change(&interaction)
        .expect("interaction change should admit through runtime change evidence");

    let (rebound, receipt) = runtime
        .rebind_header_frame_after_runtime_change(&current_plan, header_rebind_request(), &admitted)
        .expect("header rebind should flow through generic runtime-change evidence");

    assert_eq!(
        rebound.menu_plan().dropdown_plans()[0]
            .execute_frame()
            .selection_state()
            .selected_command_ids(),
        vec!["workspace.command.save".to_owned()]
    );
    assert_eq!(receipt.projection_rebuild_count(), 1);
    assert_eq!(
        receipt
            .projection_rebind_batch()
            .counters()
            .dependency_intersection_count(),
        1
    );
}
