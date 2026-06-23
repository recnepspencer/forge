use crate::runtime::{
    WorthUiHandlePlanGeneration, WorthUiQueryBindingComparison,
    WorthUiQueryBindingComparisonCounters, WorthUiQueryBindingComparisonEntry,
    WorthUiQueryBindingComparisonOutcome, WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryBindingPostureDriftFamily, WorthUiQueryBindingPreservation,
    WorthUiQueryBindingRebind, WorthUiQueryBindingRebindReason, WorthUiQueryLiveRebindEntry,
    WorthUiQueryLiveRebindOutcome, WorthUiQueryLiveRebindPlan,
    WorthUiQueryRuntimeFactLoweringInput, WorthUiQueryRuntimeFactLoweringReceipt,
    WorthUiQuerySemanticSliceProjection, WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus,
    WorthUiSemanticSliceId, WorthUiSemanticSliceInventory, WorthUiViewBindingHandle,
    WorthUiVirtualizedDataFrameTarget, WorthUiVisibleRange,
};

#[test]
fn query_semantic_projection_preserves_gap_posture_rows_without_dropping_exact_rows() {
    let comparison = query_binding_comparison();
    let live_rebind_plan = query_live_rebind_plan();
    let lowering_receipt = WorthUiQueryRuntimeFactLoweringReceipt::lower(
        WorthUiQueryRuntimeFactLoweringInput::from_runtime_evidence(
            WorthUiQuerySupportReceipt::with_runtime_hook_count_for_test(
                WorthUiQuerySupportStatus::Supported,
                2,
                99,
            ),
            comparison.clone(),
            live_rebind_plan.clone(),
        )
        .with_projection_fact_receipts([crate::runtime::WorthUiQueryProjectionFactReceipt::new(
            "query.fact.products",
            44,
        )])
        .with_state_snapshot_receipts([crate::runtime::WorthUiQueryStateSnapshotReceipt::new(
            "query.state.products",
            45,
        )])
        .with_effect_posture_receipts([crate::runtime::WorthUiQueryEffectPostureReceipt::new(
            "query.effect.products",
            46,
        )])
        .with_virtualized_frame_targets([WorthUiVirtualizedDataFrameTarget::view_binding(
            WorthUiViewBindingHandle::new(2, WorthUiHandlePlanGeneration::new(9)),
            WorthUiVisibleRange::rows(0, 20).unwrap(),
        )]),
    );

    let projection = WorthUiQuerySemanticSliceProjection::project(
        &WorthUiSemanticSliceInventory::current(),
        &comparison,
        &live_rebind_plan,
        &lowering_receipt,
    );

    assert!(projection
        .slices()
        .contains_slice_id(WorthUiSemanticSliceId::QueryBindingIdentity));
    assert!(projection
        .slices()
        .contains_slice_id(WorthUiSemanticSliceId::QueryLiveViewBinding));
    assert!(projection
        .slices()
        .contains_slice_id(WorthUiSemanticSliceId::QueryBindingPreservationPosture));
    assert!(projection
        .slices()
        .contains_slice_id(WorthUiSemanticSliceId::QueryBindingRebindPosture));
    assert!(projection
        .slices()
        .contains_slice_id(WorthUiSemanticSliceId::QueryProjectionFact));
    assert!(projection
        .slices()
        .contains_slice_id(WorthUiSemanticSliceId::QueryStateSnapshot));
    assert!(projection
        .slices()
        .contains_slice_id(WorthUiSemanticSliceId::QueryEffectPosture));
    assert!(projection
        .slices()
        .contains_slice_id(WorthUiSemanticSliceId::VirtualizedDataFrameTarget));
}

fn query_binding_comparison() -> WorthUiQueryBindingComparison {
    let mut counters = WorthUiQueryBindingComparisonCounters::default();
    counters.record_active_bindings_indexed(2);
    counters.record_candidate_bindings_indexed(2);
    counters.record_entry(WorthUiQueryBindingComparisonOutcome::PreserveMeaning, 0);
    counters.record_entry(WorthUiQueryBindingComparisonOutcome::RebindRequired, 1);
    WorthUiQueryBindingComparison::new(
        11,
        22,
        vec![
            WorthUiQueryBindingComparisonEntry::new(
                query_identity("validation.query.products"),
                Some(query_posture("active")),
                Some(query_posture("candidate")),
                WorthUiQueryBindingComparisonOutcome::PreserveMeaning,
                Vec::new(),
            ),
            WorthUiQueryBindingComparisonEntry::new(
                query_identity("validation.query.products.changed"),
                Some(query_posture("active")),
                Some(query_posture("candidate.changed")),
                WorthUiQueryBindingComparisonOutcome::RebindRequired,
                vec![WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption],
            ),
        ],
        counters,
    )
}

fn query_live_rebind_plan() -> WorthUiQueryLiveRebindPlan {
    WorthUiQueryLiveRebindPlan::new(
        11,
        22,
        vec![
            WorthUiQueryLiveRebindEntry::new(
                query_identity("validation.query.products"),
                WorthUiQueryLiveRebindOutcome::Preserve(WorthUiQueryBindingPreservation::new(
                    query_identity("validation.query.products"),
                    query_posture("candidate"),
                )),
            ),
            WorthUiQueryLiveRebindEntry::new(
                query_identity("validation.query.products.changed"),
                WorthUiQueryLiveRebindOutcome::Rebind(WorthUiQueryBindingRebind::new(
                    query_identity("validation.query.products.changed"),
                    query_posture("candidate.changed"),
                    WorthUiQueryBindingRebindReason::QueryOwnedPostureDrift,
                    vec![WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption],
                )),
            ),
        ],
    )
}

fn query_identity(view_binding_id: &str) -> WorthUiQueryBindingIdentity {
    WorthUiQueryBindingIdentity::new(
        &crate::facade::ViewBindingId::new(view_binding_id).unwrap(),
        "query-capability".to_owned(),
        "profile".to_owned(),
        "table".to_owned(),
    )
}

fn query_posture(label: &str) -> WorthUiQueryBindingPosture {
    WorthUiQueryBindingPosture::new(
        WorthUiQuerySupportStatus::Supported,
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
