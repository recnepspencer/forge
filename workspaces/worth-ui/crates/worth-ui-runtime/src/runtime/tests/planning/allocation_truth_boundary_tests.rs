use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_measurement_basis_with_font_seed, admitted_measurement_basis_with_generation,
    admitted_measurement_basis_with_leaf_intrinsic_seed, admitted_planning_admission,
    denied_measurement_basis, planning_graph_authority,
};
use crate::evidence::allocation::UiAllocationTruthCategory;
use crate::runtime::{
    UiAllocationReceiptCommitDenial, UiAllocationReuseDenial, UiAllocationReuseVerdict,
    WorthUiTransientInteractionState, WorthUiWatcherEvent,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

#[test]
fn allocation_truth_categories_remain_nominal_and_receipt_consumers_require_commit() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (basis, snapshot, selected) =
        admitted_planning_admission("allocation-truth-boundary.categories", "operator:stack");
    let candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, basis, &selected)
            .expect("allocation truth admits through graph authority"),
    );
    assert_eq!(
        candidate.truth_category(),
        UiAllocationTruthCategory::Candidate
    );
    assert_eq!(
        WorthUiWatcherEvent::modified("allocation-truth-boundary.wui").allocation_truth_category(),
        UiAllocationTruthCategory::EphemeralStreamEvent
    );
    assert_eq!(
        WorthUiTransientInteractionState::DragCapture.allocation_truth_category(),
        UiAllocationTruthCategory::LocalProjectedInteractionState
    );
    let preview = runtime.project_allocation_preview(candidate.clone());
    let receipt = runtime
        .commit_allocation_candidate_for_test(candidate.clone())
        .expect("admitted planning commits through the receipt seam");

    assert_eq!(
        preview.truth_category(),
        UiAllocationTruthCategory::PreviewCandidate
    );
    assert_eq!(
        receipt.truth_category(),
        UiAllocationTruthCategory::CommittedReceipt
    );
    assert!(preview.candidate_is_admitted());
    runtime
        .allocate_runtime_handles(&receipt)
        .expect("only committed receipt enters handle allocation");
}

#[test]
fn preview_churn_cannot_change_replayed_candidate_or_commit_outcome() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (basis, snapshot, selected) =
        admitted_planning_admission("allocation-truth-boundary.preview-replay", "operator:stack");
    let first = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, basis.clone(), &selected)
            .expect("preview replay admits through graph authority"),
    );

    for _ in 0..32 {
        let preview = runtime.project_allocation_preview(first.clone());
        assert_eq!(
            preview.truth_category(),
            UiAllocationTruthCategory::PreviewCandidate
        );
    }

    let replayed = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, basis, &selected)
            .expect("replayed preview admits through the same graph authority"),
    );
    let first_receipt = runtime
        .commit_allocation_candidate_for_test(first)
        .expect("admitted planning commits");
    let replayed_receipt = runtime
        .commit_allocation_candidate_for_test(replayed.clone())
        .expect("replayed admitted planning commits");

    assert_eq!(
        replayed.planning_identity_digest(),
        first_receipt
            .committed_allocation()
            .allocation_identity_digest()
    );
    assert_eq!(
        replayed.allocation_neighborhood(),
        first_receipt
            .committed_allocation()
            .allocation_neighborhood()
    );
    assert_eq!(first_receipt.identity(), replayed_receipt.identity());
    assert_eq!(first_receipt.generation(), replayed_receipt.generation());
    assert_eq!(
        replayed_receipt.report().reuse_verdict(),
        &UiAllocationReuseVerdict::FullReuse
    );
}

#[test]
fn receipt_identity_is_per_neighborhood_and_reuse_is_runtime_owned() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (basis, snapshot, selected) =
        admitted_planning_admission("allocation-receipt.identity", "operator:stack");
    let first = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, basis.clone(), &selected)
            .expect("receipt identity admits through graph authority"),
    );
    let neighborhood = first.allocation_neighborhood().clone();
    let receipt = runtime
        .commit_allocation_candidate_for_test(first.clone())
        .expect("admitted candidate commits");
    let replayed = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, basis, &selected)
            .expect("receipt replay admits through the same graph authority"),
    );

    assert_eq!(
        receipt.identity().neighborhood_scope(),
        &crate::evidence::UiAllocationNeighborhoodScope::from_neighborhood(&neighborhood)
    );
    assert_eq!(receipt.transaction().ordered_neighborhoods().len(), 1);
    assert_eq!(
        receipt.transaction().primary_neighborhood(),
        neighborhood.identity()
    );
    let reused = runtime
        .commit_allocation_candidate_for_test(replayed)
        .expect("matching prior receipt must commit as full reuse");
    assert_eq!(
        reused.report().reuse_verdict(),
        &UiAllocationReuseVerdict::FullReuse
    );
}

#[test]
fn receipt_report_projects_the_committed_identity_generation_and_reuse_basis() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (basis, snapshot, selected) =
        admitted_planning_admission("allocation-receipt.report-projection", "operator:stack");
    let candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, basis, &selected)
            .expect("report projection admits through graph authority"),
    );
    let receipt = runtime
        .commit_allocation_candidate_for_test(candidate)
        .expect("admitted candidate commits");

    let report = receipt.report();
    assert_eq!(report.receipt_identity(), receipt.identity());
    assert_eq!(report.receipt_generation(), receipt.generation());
    assert_eq!(
        receipt.equivalence_basis().coordinate_ownership(),
        receipt.identity().coordinate_ownership()
    );
    assert_eq!(report.reuse_verdict(), &UiAllocationReuseVerdict::NewCommit);
}

#[test]
fn receipt_and_denial_inspection_are_projected_from_commit_lineage() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (basis, snapshot, selected) =
        admitted_planning_admission("allocation-receipt.inspection", "operator:stack");
    let candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, basis, &selected)
            .expect("receipt inspection admits through graph authority"),
    );
    let receipt = runtime
        .commit_allocation_candidate_for_test(candidate)
        .expect("admitted planning commits");

    let inspection = receipt.inspection_receipt();
    assert_eq!(inspection.report(), receipt.report());
    assert_eq!(inspection.transaction(), receipt.transaction());

    let denied_basis = denied_measurement_basis("allocation-receipt.inspection.denied");
    let (denied_snapshot, denied_selected) =
        planning_graph_authority("allocation-receipt.inspection.denied", "operator:stack");
    let denied_candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &denied_snapshot, denied_basis, &denied_selected)
            .expect("denied planning still requires graph admission"),
    );
    let denial = runtime
        .commit_allocation_candidate_for_test(denied_candidate)
        .expect_err("denied candidate exposes immutable denial lineage");
    let UiAllocationReceiptCommitDenial::CandidatePlanningDenied(report) = denial else {
        panic!("expected candidate-planning denial");
    };
    assert_eq!(report.inspection_receipt().denial(), report.as_ref());
}

#[test]
fn stale_generation_is_denied_in_the_same_admitted_neighborhood_slot() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let first_basis = admitted_measurement_basis_with_generation(
        "allocation-receipt.stale-generation",
        UiEvidenceAuthorityGeneration::new(17),
    );
    let (snapshot, selected) =
        planning_graph_authority("allocation-receipt.stale-generation", "operator:stack");
    let first_candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, first_basis, &selected)
            .expect("first generation admits through graph authority"),
    );
    runtime
        .commit_allocation_candidate_for_test(first_candidate)
        .expect("first admitted generation commits");

    let stale_basis = admitted_measurement_basis_with_generation(
        "allocation-receipt.stale-generation",
        UiEvidenceAuthorityGeneration::new(18),
    );
    let stale_candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, stale_basis, &selected)
            .expect("stale generation admits through graph authority before receipt denial"),
    );
    let denial = runtime
        .commit_allocation_candidate_for_test(stale_candidate)
        .expect_err("generation replacement must meet and deny against its committed scope");

    assert!(matches!(
        denial,
        UiAllocationReceiptCommitDenial::ReuseDenied(report)
            if report.denial() == Some(UiAllocationReuseDenial::GenerationMismatch)
    ));
}

#[test]
fn changed_leaf_intrinsic_evidence_is_recompute_pending_not_a_receipt_replacement() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let first_basis = admitted_measurement_basis_with_leaf_intrinsic_seed(
        "allocation-receipt.leaf-remeasure",
        101,
    );
    let (snapshot, selected) =
        planning_graph_authority("allocation-receipt.leaf-remeasure", "operator:stack");
    let first_candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, first_basis, &selected)
            .expect("first leaf basis admits through graph authority"),
    );
    let receipt = runtime
        .commit_allocation_candidate_for_test(first_candidate)
        .expect("first committed receipt owns preserved structure");

    let changed_basis = admitted_measurement_basis_with_leaf_intrinsic_seed(
        "allocation-receipt.leaf-remeasure",
        202,
    );
    let changed_candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, changed_basis, &selected)
            .expect("changed leaf basis admits through graph authority"),
    );
    let pending_outcome = runtime.commit_allocation_candidate_for_test(changed_candidate);

    let crate::runtime::UiAllocationReceiptCommitOutcome::RecomputePending(report) =
        pending_outcome
    else {
        panic!("expected the sole admitted partial-reuse posture");
    };
    let UiAllocationReuseVerdict::StructureReuseLeafRemeasure(witness) = report.reuse_verdict()
    else {
        panic!("expected structure reuse with leaf remeasure");
    };
    assert_eq!(
        witness.preserved_structure_scope(),
        receipt.identity().neighborhood_scope()
    );
    assert!(!witness.leaf_graph_node_identities().is_empty());
    assert!(witness
        .preserved_structure_graph_node_identities()
        .iter()
        .all(|identity| !witness.leaf_graph_node_identities().contains(identity)));
    assert_eq!(
        report.freshness(),
        crate::runtime::UiAllocationReceiptFreshnessPosture::RecomputePending
    );
    assert_eq!(
        receipt.identity().neighborhood_scope(),
        report.receipt_identity().neighborhood_scope()
    );
}

#[test]
fn non_leaf_measurement_change_cannot_launder_into_partial_reuse() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let first_basis =
        admitted_measurement_basis_with_font_seed("allocation-receipt.non-leaf-change", 101);
    let (snapshot, selected) =
        planning_graph_authority("allocation-receipt.non-leaf-change", "operator:stack");
    let first_candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, first_basis, &selected)
            .expect("first non-leaf basis admits through graph authority"),
    );
    runtime
        .commit_allocation_candidate_for_test(first_candidate)
        .expect("first receipt commits");

    let changed_basis =
        admitted_measurement_basis_with_font_seed("allocation-receipt.non-leaf-change", 202);
    let changed_candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, changed_basis, &selected)
            .expect("changed non-leaf basis admits through graph authority"),
    );
    let denial = runtime
        .commit_allocation_candidate_for_test(changed_candidate)
        .expect_err("non-leaf evidence change must deny partial reuse");

    assert!(matches!(
        denial,
        UiAllocationReceiptCommitDenial::ReuseDenied(report)
            if report.denial() == Some(UiAllocationReuseDenial::UnsupportedPartialReuse)
    ));
}

#[test]
fn denied_candidate_planning_cannot_mint_a_committed_receipt() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let basis = denied_measurement_basis("allocation-truth-boundary.denied");
    let (snapshot, selected) =
        planning_graph_authority("allocation-truth-boundary.denied", "operator:stack");
    let candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, basis, &selected)
            .expect("denied candidate still requires graph admission"),
    );

    assert!(!candidate.is_admitted());
    let denial = runtime
        .commit_allocation_candidate_for_test(candidate)
        .expect_err("denied planning cannot mint a receipt");
    assert!(matches!(
        denial,
        UiAllocationReceiptCommitDenial::CandidatePlanningDenied(report)
            if report.denial().is_none()
    ));
}

#[test]
fn durable_semantic_state_is_emitted_from_reconciliation_not_a_marker_or_receipt() {
    let inputs = activation_staging_inputs();
    let durable_state = inputs
        .reconciliation_plan
        .allocation_durable_semantic_state();

    assert_eq!(
        durable_state.truth_category(),
        UiAllocationTruthCategory::DurableSemanticState
    );
    assert_eq!(
        durable_state.reconciliation().receipts(),
        inputs.reconciliation_plan.receipts()
    );
}
