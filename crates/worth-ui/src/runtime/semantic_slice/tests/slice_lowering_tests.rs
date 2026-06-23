use crate::runtime::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiClassifiedRuntimeChange,
    WorthUiRuntimeChangeActivationPosture, WorthUiRuntimeInstanceWitness,
    WorthUiSemanticChangedSliceSet, WorthUiSemanticSliceId, WorthUiSemanticSliceInventory,
};

#[test]
fn semantic_slice_lowering_projects_runtime_change_rows_into_semantic_query_slices() {
    let row = crate::runtime::WorthUiRuntimeChangeFamilyRow::from_query_lowering_receipt(
        WorthUiRuntimeInstanceWitness::from_raw(17),
        &query_lowering_receipt(),
    );
    let classified = WorthUiClassifiedRuntimeChange::from_rows(vec![row])
        .expect("runtime change row should classify coherently");
    assert_eq!(
        classified.posture(),
        WorthUiRuntimeChangeActivationPosture::Activated
    );
    let admitted = WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(17),
    )
    .expect("classified change should admit");

    let lowered = WorthUiSemanticChangedSliceSet::lower_runtime_change(
        &WorthUiSemanticSliceInventory::current(),
        &admitted,
    );

    assert!(lowered.contains_slice_id(WorthUiSemanticSliceId::QueryProjectionFact));
    assert!(lowered.contains_slice_id(WorthUiSemanticSliceId::QueryStateSnapshot));
}

fn query_lowering_receipt() -> crate::runtime::WorthUiQueryRuntimeFactLoweringReceipt {
    crate::runtime::WorthUiQueryRuntimeFactLoweringReceipt::lower(
        crate::runtime::WorthUiQueryRuntimeFactLoweringInput::from_runtime_evidence(
            crate::runtime::WorthUiQuerySupportReceipt::with_runtime_hook_count_for_test(
                crate::runtime::WorthUiQuerySupportStatus::Supported,
                1,
                77,
            ),
            query_binding_comparison(),
            empty_live_rebind_plan(),
        )
        .with_projection_fact_receipts([crate::runtime::WorthUiQueryProjectionFactReceipt::new(
            "query.fact.synthetic",
            10,
        )])
        .with_state_snapshot_receipts([
            crate::runtime::WorthUiQueryStateSnapshotReceipt::new("query.state.synthetic", 20),
        ]),
    )
}

fn query_binding_comparison() -> crate::runtime::WorthUiQueryBindingComparison {
    let mut counters = crate::runtime::WorthUiQueryBindingComparisonCounters::default();
    counters.record_active_bindings_indexed(1);
    counters.record_candidate_bindings_indexed(1);
    counters.record_entry(
        crate::runtime::WorthUiQueryBindingComparisonOutcome::RebindRequired,
        1,
    );
    crate::runtime::WorthUiQueryBindingComparison::new(
        1,
        2,
        vec![crate::runtime::WorthUiQueryBindingComparisonEntry::new(
            query_identity(),
            Some(query_posture("active")),
            Some(query_posture("candidate")),
            crate::runtime::WorthUiQueryBindingComparisonOutcome::RebindRequired,
            vec![crate::runtime::WorthUiQueryBindingPostureDriftFamily::AsyncResultState],
        )],
        counters,
    )
}

fn empty_live_rebind_plan() -> crate::runtime::WorthUiQueryLiveRebindPlan {
    crate::runtime::WorthUiQueryLiveRebindPlan::new(1, 2, Vec::new())
}

fn query_identity() -> crate::runtime::WorthUiQueryBindingIdentity {
    crate::runtime::WorthUiQueryBindingIdentity::new(
        &crate::facade::ViewBindingId::new("validation.query.products").unwrap(),
        "query-capability".to_owned(),
        "profile".to_owned(),
        "table".to_owned(),
    )
}

fn query_posture(label: &str) -> crate::runtime::WorthUiQueryBindingPosture {
    crate::runtime::WorthUiQueryBindingPosture::new(
        crate::runtime::WorthUiQuerySupportStatus::Supported,
        format!("{label}:support"),
        format!("{label}:basis"),
        format!("{label}:live"),
        format!("{label}:async"),
        format!("{label}:recovery"),
        format!("{label}:inspection"),
        format!("{label}:projection"),
        format!("{label}:denial"),
    )
}
