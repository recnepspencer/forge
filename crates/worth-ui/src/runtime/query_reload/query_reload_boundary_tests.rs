use super::query_reload_boundary_support::*;

#[test]
fn query_posture_drift_lowers_to_exact_runtime_fact_families() {
    let receipt = lower_query_change(
        WorthUiQuerySupportReceipt::with_runtime_hook_count_for_test(
            WorthUiQuerySupportStatus::Supported,
            3,
            900,
        ),
        comparison_with_drifts([
            WorthUiQueryBindingPostureDriftFamily::LiveCompatibility,
            WorthUiQueryBindingPostureDriftFamily::AsyncResultState,
            WorthUiQueryBindingPostureDriftFamily::Recovery,
            WorthUiQueryBindingPostureDriftFamily::Inspection,
            WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption,
        ]),
        preserved_live_plan(),
    );

    assert_eq!(
        receipt.status(),
        WorthUiQueryRuntimeFactLoweringStatus::AdmittedChanged
    );
    assert_exact_facts(
        receipt.changed_facts().changed_facts().facts(),
        [
            WorthUiRuntimeFactId::query_binding(&binding_id()),
            WorthUiRuntimeFactId::query_live_view(&binding_id()),
            WorthUiRuntimeFactId::query_result_posture(&binding_id()),
            WorthUiRuntimeFactId::query_recovery_posture(binding_id().as_str()),
            WorthUiRuntimeFactId::query_inspection_target(binding_id().as_str()),
            WorthUiRuntimeFactId::query_projection_fact(binding_id().as_str()),
        ],
    );
    assert_eq!(receipt.counters().bindings_compared(), 1);
    assert_eq!(receipt.counters().changed_fact_count(), 6);
}

#[test]
fn mixed_query_comparison_lowers_only_changed_entries_without_collapsing_to_global_query_fact() {
    let receipt = lower_query_change(
        supported_receipt(),
        mixed_preserved_and_changed_comparison(),
        preserved_live_plan(),
    );

    assert_eq!(
        receipt.status(),
        WorthUiQueryRuntimeFactLoweringStatus::AdmittedChanged
    );
    assert_exact_facts(
        receipt.changed_facts().changed_facts().facts(),
        [
            WorthUiRuntimeFactId::query_binding(&changed_binding_id()),
            WorthUiRuntimeFactId::query_live_view(&changed_binding_id()),
            WorthUiRuntimeFactId::query_result_posture(&changed_binding_id()),
        ],
    );
    assert_eq!(receipt.counters().bindings_compared(), 2);
    assert_eq!(receipt.counters().changed_fact_count(), 3);
}

#[test]
fn query_projection_state_and_effect_receipts_are_first_class_facts() {
    let receipt = WorthUiQueryRuntimeFactLoweringReceipt::lower(
        WorthUiQueryRuntimeFactLoweringInput::from_runtime_evidence(
            supported_receipt(),
            preserved_comparison(),
            preserved_live_plan(),
        )
        .with_projection_fact_receipts([WorthUiQueryProjectionFactReceipt::new(
            "query.fact.product_rows",
            701,
        )])
        .with_state_snapshot_receipts([WorthUiQueryStateSnapshotReceipt::new(
            "query.state.product_filter",
            702,
        )])
        .with_effect_posture_receipts([WorthUiQueryEffectPostureReceipt::new(
            "query.effect.save_product",
            703,
        )]),
    );

    assert_exact_facts(
        receipt.changed_facts().changed_facts().facts(),
        [
            WorthUiRuntimeFactId::query_projection_fact("query.fact.product_rows"),
            WorthUiRuntimeFactId::query_state_snapshot("query.state.product_filter"),
            WorthUiRuntimeFactId::query_effect_posture("query.effect.save_product"),
        ],
    );
    assert_eq!(receipt.counters().consumed_projection_fact_count(), 1);
    assert_eq!(receipt.counters().consumed_state_snapshot_count(), 1);
    assert_eq!(receipt.counters().consumed_effect_posture_count(), 1);
}

#[test]
fn unsupported_query_support_denies_before_changed_facts_or_rebind() {
    let receipt = lower_query_change(
        WorthUiQuerySupportReceipt::with_runtime_hook_count_for_test(
            WorthUiQuerySupportStatus::Unsupported,
            4,
            901,
        ),
        comparison_with_drifts([WorthUiQueryBindingPostureDriftFamily::LiveCompatibility]),
        preserved_live_plan(),
    );

    assert_eq!(
        receipt.status(),
        WorthUiQueryRuntimeFactLoweringStatus::Denied
    );
    assert!(receipt.changed_facts().changed_facts().is_empty());
    assert_eq!(
        receipt.support_denials()[0].kind(),
        WorthUiQuerySupportDenialKind::Unsupported
    );
    assert_eq!(receipt.support_denials()[0].runtime_hook_count(), 4);
}

#[test]
fn live_rebind_denial_blocks_query_change_admission() {
    let receipt = WorthUiQueryRuntimeFactLoweringReceipt::lower(
        WorthUiQueryRuntimeFactLoweringInput::from_runtime_evidence(
            supported_receipt(),
            preserved_comparison(),
            denied_live_plan(),
        )
        .with_projection_fact_receipts([WorthUiQueryProjectionFactReceipt::new(
            "query.fact.should_not_be_consumed",
            1001,
        )]),
    );

    assert_eq!(
        receipt.status(),
        WorthUiQueryRuntimeFactLoweringStatus::Denied
    );
    assert_eq!(
        receipt.support_denials()[0].kind(),
        WorthUiQuerySupportDenialKind::LiveRebindDenied
    );
    assert_eq!(receipt.support_denials()[0].denied_binding_count(), 1);
    assert!(receipt.changed_facts().changed_facts().is_empty());
    assert_eq!(receipt.counters().consumed_projection_fact_count(), 0);
}

#[test]
fn query_lowering_digest_changes_when_consumed_query_proof_digest_changes() {
    let first = query_projection_receipt_with_digest(701);
    let second = query_projection_receipt_with_digest(702);

    assert_ne!(first.receipt_digest(), second.receipt_digest());
}

#[test]
fn denied_query_lowering_digest_ignores_unconsumed_projection_proof_digest() {
    let first = denied_query_projection_receipt_with_digest(701);
    let second = denied_query_projection_receipt_with_digest(702);

    assert_eq!(first.receipt_digest(), second.receipt_digest());
    assert_eq!(first.counters().consumed_projection_fact_count(), 0);
    assert_eq!(second.counters().consumed_projection_fact_count(), 0);
}

#[test]
fn query_lowering_receipt_enters_common_runtime_change_evidence() {
    let receipt = lower_query_change(
        supported_receipt(),
        comparison_with_drifts([WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption]),
        preserved_live_plan(),
    );
    let admitted = admitted_query_change(&receipt);

    assert_eq!(
        admitted.posture(),
        WorthUiRuntimeChangeActivationPosture::Activated
    );
    assert_eq!(admitted.counters().changed_fact_count(), 2);
    assert_eq!(admitted.family_rows()[0].changed_facts().len(), 2);
}

#[test]
fn query_projection_dependency_rebuilds_only_on_intersecting_query_fact() {
    let receipt = lower_query_change(
        supported_receipt(),
        comparison_with_drifts([WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption]),
        preserved_live_plan(),
    );
    let admitted_change = admitted_query_change(&receipt);
    let intersecting = WorthUiAdmittedProjectionPlan::admit(
        query_projection_plan(WorthUiRuntimeFactId::query_projection_fact(
            binding_id().as_str(),
        )),
        runtime_witness(),
    )
    .unwrap();
    let unrelated = WorthUiAdmittedProjectionPlan::admit(
        query_projection_plan(WorthUiRuntimeFactId::query_result_posture(&binding_id())),
        runtime_witness(),
    )
    .unwrap();

    assert!(intersecting
        .dependencies()
        .intersects_changed_facts(admitted_change.family_rows()[0].changed_facts()));
    assert!(!unrelated
        .dependencies()
        .intersects_changed_facts(admitted_change.family_rows()[0].changed_facts()));
}

#[test]
fn virtualized_frame_target_lowers_without_materializing_collection_width() {
    let receipt = WorthUiQueryRuntimeFactLoweringReceipt::lower(
        WorthUiQueryRuntimeFactLoweringInput::from_runtime_evidence(
            supported_receipt(),
            preserved_comparison(),
            preserved_live_plan(),
        )
        .with_virtualized_frame_targets([WorthUiVirtualizedDataFrameTarget::view_binding(
            WorthUiViewBindingHandle::new(2, WorthUiHandlePlanGeneration::new(9)),
            WorthUiVisibleRange::grid(25, 40, 3, 8).unwrap(),
        )]),
    );

    assert_eq!(receipt.counters().virtualized_frame_target_count(), 1);
    assert_eq!(receipt.counters().changed_fact_count(), 1);
    assert!(receipt
        .changed_facts()
        .changed_facts()
        .facts()
        .contains_family(WorthUiRuntimeFactFamily::VirtualizedDataFrame));
}
