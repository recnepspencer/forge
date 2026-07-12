use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_measurement_basis, admitted_measurement_basis_with_font_seed,
    admitted_measurement_basis_with_generation, denied_measurement_basis, planning_graph_authority,
};
use crate::evidence::{UiEvidenceExpansion, UiEvidenceMaterializedDetail};
use crate::facade::evidence::{
    UiAllocationPlanningCostClass, UiAllocationSolveConvergencePosture,
    UiAllocationSolveRemainderPolicy,
};
use worth_ui_inspection::{
    UiAllocationPlanningQuestion, UiEvidenceBudget, UiEvidenceRichness, UiInspectionQuery,
    UiInspectionScope, UiInspectionTarget,
};

#[test]
fn planning_detail_preserves_neighborhood_propagation_and_special_inputs() {
    let inputs = activation_staging_inputs();
    let (app, runtime, pending) = inputs.into_app_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("allocation-inspection.detail");
    let (snapshot, selected) =
        planning_graph_authority("allocation-inspection.detail", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("detail inspection admits through graph authority"),
    );
    let retained_receipt = runtime.inspect_allocation_planning(&planning);
    let cost = retained_receipt.cost();
    let solve_trace = retained_receipt.solve_trace();
    let receipt = app.inspect(planning_detail_query(
        UiAllocationPlanningQuestion::Neighborhood,
    ));
    let detail = detail_from_receipt(&receipt);
    let constraint_set = planning
        .allocation_constraint_set()
        .expect("admitted planning exposes constraint set");

    assert!(detail.answers(UiAllocationPlanningQuestion::Neighborhood));
    assert!(detail.answers(UiAllocationPlanningQuestion::PropagationEdges));
    assert!(detail.answers(UiAllocationPlanningQuestion::SpecialInputs));
    assert!(detail.answers(UiAllocationPlanningQuestion::DurableResizePosture));
    assert_eq!(detail.neighborhood(), planning.allocation_neighborhood());
    assert_eq!(
        detail.constraint_set_identity_digest(),
        Some(constraint_set.identity().identity_digest())
    );
    assert_eq!(detail.constraint_summary(), Some(constraint_set.summary()));
    assert_eq!(
        detail.propagation_edges(),
        constraint_set.propagation_edges()
    );
    assert_eq!(
        detail.viewport_planning_input(),
        constraint_set.viewport_planning_input()
    );
    assert_eq!(
        detail.scroll_owner_planning_input(),
        constraint_set.scroll_owner_planning_input()
    );
    assert_eq!(
        detail.portal_anchor_planning_input(),
        constraint_set.portal_anchor_planning_input()
    );
    assert_eq!(
        detail.durable_resize_posture(),
        Some(constraint_set.summary().resize_permission_posture())
    );
    assert_eq!(detail.solve_trace(), solve_trace);
    assert_eq!(
        cost.neighborhood_identity_digest(),
        planning
            .allocation_neighborhood()
            .identity()
            .identity_digest()
    );
    assert_eq!(
        cost.nodes_considered(),
        planning.allocation_neighborhood().members().len()
    );
    assert_eq!(
        cost.nodes_admitted(),
        planning.allocation_neighborhood().members().len()
    );
    assert_eq!(
        cost.edges_emitted(),
        constraint_set.propagation_edges().len()
    );
    assert_eq!(cost.special_inputs_loaded(), 0);
    assert_eq!(cost.cost_class(), UiAllocationPlanningCostClass::Local);
    assert_eq!(
        solve_trace.planning_identity_digest(),
        planning.planning_identity_digest()
    );
    assert_eq!(solve_trace.pass_order().len(), cost.propagation_passes());
    assert!(solve_trace.is_deterministic());
    assert_eq!(
        solve_trace.convergence_posture(),
        UiAllocationSolveConvergencePosture::AcyclicDeterministic
    );
    assert_eq!(
        solve_trace.remainder_policy(),
        UiAllocationSolveRemainderPolicy::None
    );
    assert_eq!(
        receipt.authority_generation(),
        Some(retained_receipt.evidence_slice().authority_generation())
    );
    assert_eq!(
        receipt
            .evidence_slice()
            .expect("public planning receipt preserves retained slice")
            .authority_generation(),
        retained_receipt.evidence_slice().authority_generation()
    );
}

#[test]
fn retained_planning_expansion_converges_on_registered_detail() {
    let inputs = activation_staging_inputs();
    let (app, runtime, pending) = inputs.into_app_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("allocation-inspection.retained");
    let (snapshot, selected) =
        planning_graph_authority("allocation-inspection.retained", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("retained inspection admits through graph authority"),
    );
    runtime.inspect_allocation_planning(&planning);
    let receipt = app.inspect(planning_detail_query(
        UiAllocationPlanningQuestion::PropagationEdges,
    ));
    let evidence_ref = receipt
        .evidence_slice()
        .expect("planning receipt exposes slice")
        .refs()[0];
    let expansion =
        app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::materialized_detail());

    assert_eq!(
        expansion.outcome(),
        worth_ui_inspection::UiEvidenceExpansionOutcome::Available
    );
    assert_eq!(
        detail_from_expansion(&expansion),
        detail_from_receipt(&receipt)
    );
}

#[test]
fn summary_expansion_stays_refs_first_until_detail_is_requested() {
    let inputs = activation_staging_inputs();
    let (app, runtime, pending) = inputs.into_app_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("allocation-inspection.summary");
    let (snapshot, selected) =
        planning_graph_authority("allocation-inspection.summary", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("summary inspection admits through graph authority"),
    );
    runtime.inspect_allocation_planning(&planning);
    let receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::ProductRoot,
            UiInspectionScope::planning(),
        )
        .with_richness(UiEvidenceRichness::materialized_detail())
        .with_budget(UiEvidenceBudget::narrow())
        .with_allocation_planning_question(UiAllocationPlanningQuestion::DurableResizePosture),
    );
    let evidence_slice = receipt
        .evidence_slice()
        .expect("summary planning receipt exposes slice");
    let evidence_ref = evidence_slice.refs()[0];
    let expansion = app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());

    assert_eq!(receipt.query().scope(), UiInspectionScope::Planning);
    assert_eq!(
        receipt.query().allocation_planning_question(),
        Some(UiAllocationPlanningQuestion::DurableResizePosture)
    );
    assert_eq!(
        expansion.outcome(),
        worth_ui_inspection::UiEvidenceExpansionOutcome::Available
    );
    assert_eq!(expansion.evidence_ref(), evidence_ref);
    assert!(expansion.materialized_detail().is_none());
    assert!(evidence_slice.materialized_detail().is_none());
}

#[test]
fn planning_requires_explicit_question_before_materializing_detail() {
    let inputs = activation_staging_inputs();
    let (app, runtime, pending) = inputs.into_app_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("allocation-inspection.question");
    let (snapshot, selected) =
        planning_graph_authority("allocation-inspection.question", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("question inspection admits through graph authority"),
    );
    runtime.inspect_allocation_planning(&planning);

    let receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::ProductRoot,
            UiInspectionScope::planning(),
        )
        .with_richness(UiEvidenceRichness::materialized_detail())
        .with_budget(UiEvidenceBudget::ordinary()),
    );

    let evidence_slice = receipt
        .evidence_slice()
        .expect("planning receipt exposes retained slice");
    assert!(evidence_slice.materialized_detail().is_none());
    assert_eq!(
        evidence_slice.omission(),
        Some(worth_ui_inspection::UiEvidenceSliceOmission::ByScope {
            scope: UiInspectionScope::Planning
        })
    );
}

#[test]
fn explicit_planning_question_stays_refs_first_when_multiple_receipts_are_retained() {
    let inputs = activation_staging_inputs();
    let (app, runtime, pending) = inputs.into_app_runtime_and_pending();
    let first_basis =
        admitted_measurement_basis_with_font_seed("allocation-inspection.multiple.first", 101);
    let second_basis =
        admitted_measurement_basis_with_font_seed("allocation-inspection.multiple.second", 202);
    let (first_snapshot, first_selected) =
        planning_graph_authority("allocation-inspection.multiple.first", "operator:stack");
    let (second_snapshot, second_selected) =
        planning_graph_authority("allocation-inspection.multiple.second", "operator:stack");
    let first = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &first_snapshot, first_basis, &first_selected)
            .expect("first multiple inspection admits through graph authority"),
    );
    let second = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &second_snapshot, second_basis, &second_selected)
            .expect("second multiple inspection admits through graph authority"),
    );
    runtime.inspect_allocation_planning(&first);
    runtime.inspect_allocation_planning(&second);

    let receipt = app.inspect(planning_detail_query(
        UiAllocationPlanningQuestion::PropagationEdges,
    ));
    let evidence_slice = receipt
        .evidence_slice()
        .expect("planning receipt exposes retained slice");

    assert_eq!(evidence_slice.refs().len(), 2);
    assert!(evidence_slice.materialized_detail().is_none());
    assert_eq!(
        evidence_slice.omission(),
        Some(worth_ui_inspection::UiEvidenceSliceOmission::ByScope {
            scope: UiInspectionScope::Planning
        })
    );
}

#[test]
fn mixed_planning_authorities_do_not_collapse_into_one_public_slice() {
    let inputs = activation_staging_inputs();
    let (app, runtime, pending) = inputs.into_app_runtime_and_pending();
    let first_basis = admitted_measurement_basis("allocation-inspection.authority.first");
    let second_basis = admitted_measurement_basis_with_generation(
        "allocation-inspection.authority.second",
        worth_ui_inspection::UiEvidenceAuthorityGeneration::new(19),
    );
    let (first_snapshot, first_selected) =
        planning_graph_authority("allocation-inspection.authority.first", "operator:stack");
    let (second_snapshot, second_selected) =
        planning_graph_authority("allocation-inspection.authority.second", "operator:stack");
    let first = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &first_snapshot, first_basis, &first_selected)
            .expect("first authority inspection admits through graph authority"),
    );
    let second = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &second_snapshot, second_basis, &second_selected)
            .expect("second authority inspection admits through graph authority"),
    );
    let first_receipt = runtime.inspect_allocation_planning(&first);
    let second_receipt = runtime.inspect_allocation_planning(&second);

    assert_ne!(
        first_receipt.evidence_slice().authority_generation(),
        second_receipt.evidence_slice().authority_generation()
    );

    let public = app.inspect(planning_detail_query(
        UiAllocationPlanningQuestion::Neighborhood,
    ));

    assert!(public.evidence_slice().is_none());
    assert!(public.authority_generation().is_none());
}

#[test]
fn denied_planning_detail_keeps_neighborhood_without_claiming_admitted_special_inputs() {
    let inputs = activation_staging_inputs();
    let (app, runtime, pending) = inputs.into_app_runtime_and_pending();
    let measurement_basis = denied_measurement_basis("allocation-inspection.denied");
    let (snapshot, selected) =
        planning_graph_authority("allocation-inspection.denied", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("denied inspection admits through graph authority"),
    );
    runtime.inspect_allocation_planning(&planning);
    let receipt = app.inspect(planning_detail_query(
        UiAllocationPlanningQuestion::SpecialInputs,
    ));
    let detail = detail_from_receipt(&receipt);
    let cost = runtime.inspect_allocation_planning(&planning).cost();
    let constraint_set = planning.allocation_constraint_set();

    assert!(!planning.is_admitted());
    assert_eq!(detail.neighborhood(), planning.allocation_neighborhood());
    assert_eq!(
        detail.constraint_set_identity_digest(),
        constraint_set
            .map(|retained_constraint_set| retained_constraint_set.identity().identity_digest())
    );
    assert_eq!(
        detail.propagation_edges(),
        constraint_set
            .map(|retained_constraint_set| retained_constraint_set.propagation_edges())
            .unwrap_or(&[])
    );
    assert_eq!(
        detail.answers(UiAllocationPlanningQuestion::SpecialInputs),
        constraint_set.is_some()
    );
    assert_eq!(
        detail.answers(UiAllocationPlanningQuestion::DurableResizePosture),
        constraint_set.is_some()
    );
    assert_eq!(cost.nodes_admitted(), 0);
    assert!(cost.denied_broadening_reason().is_some());
    assert_eq!(
        cost.cost_class(),
        UiAllocationPlanningCostClass::DeniedUnbounded
    );
    assert!(detail.denial().is_some());
}

fn detail_from_receipt(
    receipt: &crate::facade::inspection_bridge::UiInspectionReceipt,
) -> &crate::evidence::UiAllocationPlanningEvidenceDetail {
    match receipt
        .evidence_slice()
        .expect("planning inspection receipt exposes retained evidence")
        .materialized_detail()
    {
        Some(UiEvidenceMaterializedDetail::AllocationPlanning(detail)) => detail,
        other => panic!("expected allocation-planning detail, got {other:?}"),
    }
}

fn detail_from_expansion(
    expansion: &UiEvidenceExpansion,
) -> &crate::evidence::UiAllocationPlanningEvidenceDetail {
    match expansion.materialized_detail() {
        Some(UiEvidenceMaterializedDetail::AllocationPlanning(detail)) => detail,
        other => panic!("expected allocation-planning expansion detail, got {other:?}"),
    }
}

fn planning_detail_query(question: UiAllocationPlanningQuestion) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::ProductRoot,
        UiInspectionScope::planning(),
    )
    .with_richness(UiEvidenceRichness::materialized_detail())
    .with_budget(UiEvidenceBudget::ordinary())
    .with_allocation_planning_question(question)
}
