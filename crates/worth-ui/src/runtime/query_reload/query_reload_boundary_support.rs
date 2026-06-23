pub(super) use crate::capability::ViewBindingId;
pub(super) use crate::runtime::{
    WorthUiAdmittedProjectionPlan, WorthUiAdmittedRuntimeChangeEvidence,
    WorthUiHandlePlanGeneration, WorthUiProjectionDependencyDeclaration,
    WorthUiProjectionDependencySet, WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily,
    WorthUiProjectionIdentity, WorthUiProjectionPlanContract, WorthUiQueryBindingComparison,
    WorthUiQueryBindingComparisonCounters, WorthUiQueryBindingComparisonEntry,
    WorthUiQueryBindingComparisonOutcome, WorthUiQueryBindingDriftDenial,
    WorthUiQueryBindingDriftDenialKind, WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryBindingPostureDriftFamily, WorthUiQueryBindingPreservation,
    WorthUiQueryEffectPostureReceipt, WorthUiQueryLiveRebindEntry, WorthUiQueryLiveRebindOutcome,
    WorthUiQueryLiveRebindPlan, WorthUiQueryProjectionFactReceipt,
    WorthUiQueryRuntimeFactLoweringInput, WorthUiQueryRuntimeFactLoweringReceipt,
    WorthUiQueryRuntimeFactLoweringStatus, WorthUiQueryStateSnapshotReceipt,
    WorthUiQuerySupportDenialKind, WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus,
    WorthUiRuntimeChangeActivationPosture, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
    WorthUiRuntimeFactSet, WorthUiRuntimeInstanceWitness, WorthUiViewBindingHandle,
    WorthUiVirtualizedDataFrameTarget, WorthUiVisibleRange,
};

use crate::runtime::projection_contract::plan_contract::private::Sealed;

pub(super) fn query_projection_receipt_with_digest(
    projection_receipt_digest: u64,
) -> WorthUiQueryRuntimeFactLoweringReceipt {
    WorthUiQueryRuntimeFactLoweringReceipt::lower(
        WorthUiQueryRuntimeFactLoweringInput::from_runtime_evidence(
            supported_receipt(),
            preserved_comparison(),
            preserved_live_plan(),
        )
        .with_projection_fact_receipts([WorthUiQueryProjectionFactReceipt::new(
            "query.fact.product_rows",
            projection_receipt_digest,
        )]),
    )
}

pub(super) fn denied_query_projection_receipt_with_digest(
    projection_receipt_digest: u64,
) -> WorthUiQueryRuntimeFactLoweringReceipt {
    WorthUiQueryRuntimeFactLoweringReceipt::lower(
        WorthUiQueryRuntimeFactLoweringInput::from_runtime_evidence(
            supported_receipt(),
            preserved_comparison(),
            denied_live_plan(),
        )
        .with_projection_fact_receipts([WorthUiQueryProjectionFactReceipt::new(
            "query.fact.product_rows",
            projection_receipt_digest,
        )]),
    )
}

pub(super) fn lower_query_change(
    support_receipt: WorthUiQuerySupportReceipt,
    comparison: WorthUiQueryBindingComparison,
    live_rebind_plan: WorthUiQueryLiveRebindPlan,
) -> WorthUiQueryRuntimeFactLoweringReceipt {
    WorthUiQueryRuntimeFactLoweringReceipt::lower(
        WorthUiQueryRuntimeFactLoweringInput::from_runtime_evidence(
            support_receipt,
            comparison,
            live_rebind_plan,
        ),
    )
}

pub(super) fn admitted_query_change(
    receipt: &WorthUiQueryRuntimeFactLoweringReceipt,
) -> WorthUiAdmittedRuntimeChangeEvidence {
    let classified = crate::runtime::WorthUiClassifiedRuntimeChange::from_query_lowering_receipt(
        runtime_witness(),
        receipt,
    );
    WorthUiAdmittedRuntimeChangeEvidence::admit(classified, runtime_witness()).unwrap()
}

pub(super) fn comparison_with_drifts<const N: usize>(
    drifts: [WorthUiQueryBindingPostureDriftFamily; N],
) -> WorthUiQueryBindingComparison {
    comparison(
        WorthUiQueryBindingComparisonOutcome::RebindRequired,
        drifts.to_vec(),
    )
}

pub(super) fn preserved_comparison() -> WorthUiQueryBindingComparison {
    comparison(
        WorthUiQueryBindingComparisonOutcome::PreserveMeaning,
        Vec::new(),
    )
}

fn comparison(
    outcome: WorthUiQueryBindingComparisonOutcome,
    drifts: Vec<WorthUiQueryBindingPostureDriftFamily>,
) -> WorthUiQueryBindingComparison {
    let mut counters = WorthUiQueryBindingComparisonCounters::default();
    counters.record_active_bindings_indexed(1);
    counters.record_candidate_bindings_indexed(1);
    counters.record_entry(outcome, drifts.len());
    WorthUiQueryBindingComparison::new(
        100,
        200,
        vec![WorthUiQueryBindingComparisonEntry::new(
            query_identity(),
            Some(active_posture()),
            Some(candidate_posture()),
            outcome,
            drifts,
        )],
        counters,
    )
}

pub(super) fn mixed_preserved_and_changed_comparison() -> WorthUiQueryBindingComparison {
    let mut counters = WorthUiQueryBindingComparisonCounters::default();
    counters.record_active_bindings_indexed(2);
    counters.record_candidate_bindings_indexed(2);
    counters.record_entry(WorthUiQueryBindingComparisonOutcome::PreserveMeaning, 0);
    counters.record_entry(WorthUiQueryBindingComparisonOutcome::RebindRequired, 2);
    WorthUiQueryBindingComparison::new(
        100,
        200,
        vec![
            WorthUiQueryBindingComparisonEntry::new(
                query_identity_for("validation.query.preserved"),
                Some(active_posture()),
                Some(candidate_posture()),
                WorthUiQueryBindingComparisonOutcome::PreserveMeaning,
                Vec::new(),
            ),
            WorthUiQueryBindingComparisonEntry::new(
                query_identity_for("validation.query.changed"),
                Some(active_posture()),
                Some(candidate_posture()),
                WorthUiQueryBindingComparisonOutcome::RebindRequired,
                vec![
                    WorthUiQueryBindingPostureDriftFamily::LiveCompatibility,
                    WorthUiQueryBindingPostureDriftFamily::AsyncResultState,
                ],
            ),
        ],
        counters,
    )
}

pub(super) fn preserved_live_plan() -> WorthUiQueryLiveRebindPlan {
    WorthUiQueryLiveRebindPlan::new(
        100,
        200,
        vec![WorthUiQueryLiveRebindEntry::new(
            query_identity(),
            WorthUiQueryLiveRebindOutcome::Preserve(WorthUiQueryBindingPreservation::new(
                query_identity(),
                candidate_posture(),
            )),
        )],
    )
}

pub(super) fn denied_live_plan() -> WorthUiQueryLiveRebindPlan {
    WorthUiQueryLiveRebindPlan::new(
        100,
        200,
        vec![WorthUiQueryLiveRebindEntry::new(
            query_identity(),
            WorthUiQueryLiveRebindOutcome::Deny(WorthUiQueryBindingDriftDenial::new(
                query_identity(),
                Some(active_posture()),
                Some(candidate_posture()),
                vec![WorthUiQueryBindingPostureDriftFamily::DenialPresentation],
                WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery,
            )),
        )],
    )
}

fn active_posture() -> WorthUiQueryBindingPosture {
    query_posture("active")
}

fn candidate_posture() -> WorthUiQueryBindingPosture {
    query_posture("candidate")
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
        "shared:denial".to_owned(),
    )
}

pub(super) fn query_identity() -> WorthUiQueryBindingIdentity {
    WorthUiQueryBindingIdentity::new(
        &binding_id(),
        "query-capability".to_owned(),
        "profile".to_owned(),
        "table".to_owned(),
    )
}

pub(super) fn binding_id() -> ViewBindingId {
    ViewBindingId::new("validation.query.products").unwrap()
}

pub(super) fn changed_binding_id() -> ViewBindingId {
    ViewBindingId::new("validation.query.changed").unwrap()
}

pub(super) fn supported_receipt() -> WorthUiQuerySupportReceipt {
    WorthUiQuerySupportReceipt::with_runtime_hook_count_for_test(
        WorthUiQuerySupportStatus::Supported,
        3,
        900,
    )
}

fn query_identity_for(raw_view_binding_id: &str) -> WorthUiQueryBindingIdentity {
    WorthUiQueryBindingIdentity::new(
        &ViewBindingId::new(raw_view_binding_id).unwrap(),
        "query-capability".to_owned(),
        "profile".to_owned(),
        "table".to_owned(),
    )
}

pub(super) fn runtime_witness() -> WorthUiRuntimeInstanceWitness {
    WorthUiRuntimeInstanceWitness::from_raw(7)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct QueryProjectionPlan {
    dependency: WorthUiRuntimeFactId,
}

impl Sealed for QueryProjectionPlan {}

impl WorthUiProjectionPlanContract for QueryProjectionPlan {
    fn projection_identity(&self) -> WorthUiProjectionIdentity {
        WorthUiProjectionIdentity::runtime("query.products.projection")
    }

    fn projection_family(&self) -> WorthUiProjectionFamily {
        WorthUiProjectionFamily::QueryProjectionConsumption
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        WorthUiProjectionDependencyDeclaration::from_set(
            WorthUiProjectionDependencySet::empty().depends_on(self.dependency.clone()),
        )
    }

    fn projection_equivalence_digest(&self) -> u64 {
        33
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::ProjectionDigest
    }
}

pub(super) fn query_projection_plan(dependency: WorthUiRuntimeFactId) -> QueryProjectionPlan {
    QueryProjectionPlan { dependency }
}

pub(super) fn assert_exact_facts<const N: usize>(
    facts: &WorthUiRuntimeFactSet,
    expected: [WorthUiRuntimeFactId; N],
) {
    assert_eq!(facts.len(), expected.len());
    for fact in expected {
        assert!(facts.contains_exact(&fact), "missing fact: {fact:?}");
    }
}
