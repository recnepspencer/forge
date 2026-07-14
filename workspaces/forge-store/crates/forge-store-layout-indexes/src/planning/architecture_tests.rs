#[test]
fn planning_has_one_classifier_and_issuance_cannot_reclassify_payloads() {
    let decision = include_str!("decision.rs");
    let issuance = include_str!("selection_issuance.rs");
    let receipt = include_str!("selection_receipt.rs");

    assert_eq!(
        decision.matches("fn classify_indexed_operation(").count(),
        1,
        "planning must have exactly one operation classifier"
    );
    assert_eq!(
        decision.matches("PlanningAlternativeSet::derive(").count(),
        1,
        "candidate derivation must occur exactly once"
    );
    assert_eq!(
        receipt
            .matches("issue_selection_outcome(decide_access_plan(")
            .count(),
        1,
        "temporary test entrypoints must delegate to the sole production decision-to-issuance chain"
    );
    for forbidden in [
        "selected_family()",
        "access_shape()",
        "is_btree_",
        "is_lsm_",
        "SelectionRoute",
    ] {
        assert!(
            !issuance.contains(forbidden),
            "selection issuance must not reinterpret an issued decision through {forbidden}"
        );
    }
}

#[test]
fn raw_access_shape_stops_at_request_admission() {
    for (name, source) in [
        ("candidates", include_str!("candidates/set.rs")),
        ("cost derivation", include_str!("cost/derivation.rs")),
        ("cost estimate", include_str!("cost/estimate.rs")),
        ("decision", include_str!("decision.rs")),
        ("selected plan", include_str!("selected_plan.rs")),
    ] {
        assert!(
            !source.contains("AccessShapeContract"),
            "{name} must consume admitted operation intent rather than raw shape declarations"
        );
        assert!(
            !source.contains(".access_shape()"),
            "{name} must not recover the displaced raw shape lane"
        );
    }

    let execution = [
        include_str!("../access/execution/btree_lookup/operation.rs"),
        include_str!("../access/execution/view.rs"),
        include_str!("../access/execution/degraded_scan/executed.rs"),
        include_str!("../access/execution/degraded_scan/rebind.rs"),
    ];
    assert!(
        execution
            .iter()
            .all(|source| !source.contains(".access_shape()")),
        "execution must consume admitted intent and materialization independently"
    );

    let alternatives = include_str!("candidates/set.rs");
    for forbidden in [".lifecycle()", ".witness()", "LayoutAdmissionRequest::new("] {
        assert!(
            !alternatives.contains(forbidden),
            "candidate strategy admission must retain admitted family/domain authority instead of projecting through {forbidden}"
        );
    }
}

#[test]
fn selected_operations_retain_request_bound_strategy_admission() {
    let selected = include_str!("selected_plan.rs");
    let identity = include_str!("plan_identity.rs");
    let materialization = include_str!("../materialization/mod.rs");
    let coverage = include_str!("../materialization/coverage.rs");

    assert!(selected.contains("strategy_admission: Option<LayoutStrategyRegistrySnapshot>"));
    assert!(identity.contains("strategy_admission: Option<LayoutStrategyRegistrySnapshot>"));
    for removed in [
        "RangeCompletenessWitness",
        "PrefixCompletenessWitness",
        "MaterializationCompleteness",
    ] {
        assert!(!materialization.contains(removed));
        assert!(!coverage.contains(removed));
    }
}

#[test]
fn selected_operation_is_classified_once_at_candidate_admission() {
    let candidate_operation = include_str!("candidates/operation.rs");
    let decision = include_str!("decision.rs");
    let btree_execution = include_str!("../strategy/btree/execution/lookup/operation.rs");

    assert!(!candidate_operation.contains("_ =>"));
    assert!(!decision.contains("plan.selected_family()"));
    assert!(!decision.contains("plan.intent().shape()"));
    assert!(!btree_execution.contains("intent().shape()"));
    assert!(decision.contains("match plan"));
    assert!(decision.contains(".selected_operation()"));
}

#[test]
fn planning_does_not_advertise_an_unexecutable_ranking_policy() {
    let planning_root = include_str!("../planning.rs");
    let decision = include_str!("decision.rs");
    let basis = include_str!("selection_basis.rs");
    let candidates = include_str!("candidates/set.rs");

    assert!(!planning_root.contains("selection_policy"));
    assert!(!decision.contains(".rank("));
    assert!(!basis.contains("PreferBTree"));
    assert!(!basis.contains("PreferLsm"));
    assert!(candidates.contains("selected: Option<PlanningAlternative>"));
    assert!(!candidates.contains("primary: Option<PlanningAlternative>"));
    assert!(!candidates.contains("secondary: Option<PlanningAlternative>"));
    assert!(!decision.contains("OverlappingEligibleStrategyAuthority"));
}

#[test]
fn cost_derivation_consumes_owner_classified_operations_not_family_shape_projections() {
    let derivation = include_str!("cost/derivation.rs");

    assert!(derivation.contains("EligibleStrategyOperation"));
    assert!(!derivation.contains("LayoutStrategyFamily"));
    assert!(!derivation.contains("aggregate_profile"));
    assert!(!derivation.contains("intent.shape()"));
    assert!(derivation.contains("envelope.lookup()"));
    assert!(derivation.contains("envelope.publication()"));
    assert!(derivation.contains("envelope.recovery()"));
}

#[test]
fn selected_plan_owner_alone_pairs_cost_with_budget_admission() {
    let decision = include_str!("decision.rs");
    let selected = include_str!("selected_plan.rs");

    assert!(!decision.contains("pre_execution_budget_admission"));
    assert!(!decision.contains("PreExecutionBudgetAdmissionReceipt"));
    assert!(!decision.contains("from_budget_admission"));
    assert!(selected.contains("fn admit_selected_plan_budget("));
    assert!(selected.contains("receipt.request(), request"));
    assert_eq!(selected.matches("from_budget_admission(").count(), 2);
}

#[test]
fn plan_identity_is_native_complete_equivalence_not_digest_authority() {
    let identity = include_str!("plan_identity.rs");
    let selected = include_str!("selected_plan.rs");

    for required in [
        "request_identity: AdmittedPhysicalAccessIdentity",
        "materialization: Option<AdmittedLayoutMaterialization>",
        "strategy_admission: Option<LayoutStrategyRegistrySnapshot>",
        "cost_estimate: AccessPlanCostEstimate",
        "budget_request: PreExecutionBudgetRequest",
        "budget_envelope: PreExecutionBudgetEnvelope",
    ] {
        assert!(
            identity.contains(required),
            "plan identity must retain {required}"
        );
    }
    assert!(!identity.contains("digest"));
    assert!(!identity.contains("hash"));
    assert!(selected.contains("budget_receipt.request()"));
    assert!(selected.contains("budget_receipt.admitted_envelope()"));
}

#[test]
fn selected_read_authority_remains_a_compact_handle_to_complete_native_identity() {
    use std::mem::size_of;

    assert_eq!(size_of::<super::AccessPlanIdentity>(), size_of::<usize>());
    for (name, size) in [
        ("B-tree lookup", size_of::<super::SelectedBTreeLookup>()),
        ("LSM lookup", size_of::<super::SelectedLsmLookup>()),
        (
            "degraded exact scan",
            size_of::<super::SelectedDegradedExactScan>(),
        ),
    ] {
        assert!(
            size <= 256,
            "{name} selected authority embedded {size} bytes instead of retaining a compact identity handle"
        );
    }
}

#[test]
fn operation_selection_requires_the_exact_unforgeable_decision_grant() {
    let decision = include_str!("decision.rs");
    let selected = include_str!("selected_plan.rs");
    let issuance = include_str!("selection_issuance.rs");

    assert!(!selected.contains("from_unclassified"));
    assert!(selected.contains("fn from_decision("));
    for grant in [
        "BTreeLookupSelectionGrant",
        "BTreeReplaySelectionGrant",
        "LsmLookupSelectionGrant",
        "LsmPublicationSelectionGrant",
        "LsmReplaySelectionGrant",
        "LsmCompactionSelectionGrant",
        "DegradedScanSelectionGrant",
    ] {
        assert!(decision.contains(&format!("define_selection_grant!({grant})")));
        assert!(selected.contains(grant));
        assert!(issuance.contains("from_decision"));
    }
    assert!(decision.contains("_issued_by_decision_owner: ()"));
    assert!(!issuance.contains("selected_family()"));
    assert!(!issuance.contains("intent()"));
}
