use super::UiRebindReceipt;

pub(super) fn project_rebind_decision(
    receipt: &UiRebindReceipt,
) -> worth_ui_inspection::UiRebindDecisionRecord {
    let plan = receipt.plan();
    let classification = plan.basis().classification();
    let (changed_fact_count, affected_aspect_count, consumer_count) = plan
        .scope()
        .map(|scope| {
            (
                scope.facts().len(),
                scope.affected_aspects().len(),
                scope.consumers().len(),
            )
        })
        .unwrap_or((0, 0, 0));
    let cost = plan.cost();
    worth_ui_inspection::UiRebindDecisionRecord::from_runtime_projection(
        worth_ui_inspection::UiRebindDecisionRecordInput {
            key: receipt.decision_key(),
            source_basis: classification.source_basis(),
            observation_count: classification.observation_count(),
            changed_fact_count,
            affected_aspect_count,
            consumer_count,
            disposition: receipt.inspection_disposition(),
            cost: [
                cost.selected_decisions(),
                cost.graph_and_mounted_entries(),
                cost.measurement_and_allocation_entries(),
                cost.binding_transitions(),
                cost.effects(),
            ],
        },
    )
}

pub(super) fn project_rebind_decision_index(
    receipt: &UiRebindReceipt,
) -> worth_ui_inspection::UiRebindDecisionIndex {
    worth_ui_inspection::UiRebindDecisionIndex::from_runtime_projection(
        receipt.plan().budget().terminal_decision_records,
        vec![project_rebind_decision(receipt)],
    )
    .expect("an admitted plan reserves one terminal decision record")
}
