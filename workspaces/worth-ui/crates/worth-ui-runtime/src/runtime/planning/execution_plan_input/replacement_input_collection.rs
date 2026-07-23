use crate::runtime::{
    WorthUiComponentLoweringHook, WorthUiPendingActivation, WorthUiPlanLoweringBasis,
    WorthUiPlanLoweringContext, WorthUiPlanLoweringCounters, WorthUiPlanLoweringDenialReason,
    WorthUiPlanNodeInput, WorthUiRuntimeFrameEpoch,
};

pub(super) fn collect_plan_node_inputs(
    staged: &crate::runtime::WorthUiStagedReplacement,
    component_hooks: &[WorthUiComponentLoweringHook],
    query_binding_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
    counters: &mut WorthUiPlanLoweringCounters,
) -> Result<Vec<WorthUiPlanNodeInput>, WorthUiPlanLoweringDenialReason> {
    let mut node_inputs = Vec::new();
    let artifact = staged.admitted_candidate().artifact_bundle().artifact();
    collect_staged_replacement_node_inputs(staged, artifact, counters, &mut node_inputs)?;
    collect_query_binding_node_inputs(
        staged,
        artifact,
        query_binding_plan,
        counters,
        &mut node_inputs,
    );
    collect_component_hook_node_inputs(component_hooks, counters, &mut node_inputs);
    node_inputs.sort_by(|left, right| left.identity_basis().cmp(right.identity_basis()));
    Ok(node_inputs)
}

fn collect_staged_replacement_node_inputs(
    staged: &crate::runtime::WorthUiStagedReplacement,
    artifact: &crate::source::WorthUiArtifact,
    counters: &mut WorthUiPlanLoweringCounters,
    node_inputs: &mut Vec<WorthUiPlanNodeInput>,
) -> Result<(), WorthUiPlanLoweringDenialReason> {
    for classification in staged.node_plan().changed_classifications() {
        if classification.candidate_kind().is_none()
            || classification.candidate_kind()
                == Some(crate::runtime::WorthUiIdentityMatchNodeKind::Binding)
        {
            continue;
        }
        let Some(node) = artifact.node_for_identity_basis(classification.identity_basis()) else {
            continue;
        };
        let topology_input = crate::runtime::WorthUiPlanNodeTopologyInput::from_artifact_node(node);
        let lowered = super::lower_replacement_ordinary_node(
            node,
            topology_input,
            classification.transition(),
            staged.reconciliation_plan(),
        )
        .map_err(WorthUiPlanLoweringDenialReason::from_ordinary_lowering)?;
        for input in lowered {
            node_inputs.push(input);
            counters.record_staged_node_input();
        }
    }
    Ok(())
}

fn collect_query_binding_node_inputs(
    staged: &crate::runtime::WorthUiStagedReplacement,
    artifact: &crate::source::WorthUiArtifact,
    query_binding_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
    counters: &mut WorthUiPlanLoweringCounters,
    node_inputs: &mut Vec<WorthUiPlanNodeInput>,
) {
    for entry in staged.query_rebind_plan().changed_entries() {
        if matches!(
            entry.outcome(),
            crate::runtime::WorthUiQueryLiveRebindOutcome::Retire(_)
                | crate::runtime::WorthUiQueryLiveRebindOutcome::Deny(_)
        ) {
            continue;
        }
        let topology_input = artifact
            .node_for_identity_basis(entry.identity().view_binding_id())
            .map(crate::runtime::WorthUiPlanNodeTopologyInput::from_artifact_node)
            .unwrap_or_default();
        node_inputs.push(WorthUiPlanNodeInput::from_query_rebind_entry(
            entry,
            query_binding_plan.resolve_definition(
                entry.identity().query_view_identity(),
                entry.identity().result_shape(),
            ),
            topology_input,
        ));
        counters.record_query_binding_input();
    }
}

pub(super) fn collect_component_hook_node_inputs(
    component_hooks: &[WorthUiComponentLoweringHook],
    counters: &mut WorthUiPlanLoweringCounters,
    node_inputs: &mut Vec<WorthUiPlanNodeInput>,
) {
    for hook in component_hooks {
        if let Some(family) = hook.admitted_family() {
            node_inputs.push(WorthUiPlanNodeInput::from_component_hook(
                hook,
                family.plan_node_family(),
            ));
            counters.record_component_hook_input();
        }
    }
}

pub(super) fn record_reconciliation_receipt_inputs(
    staged: &crate::runtime::WorthUiStagedReplacement,
    counters: &mut WorthUiPlanLoweringCounters,
) {
    counters.record_reconciliation_receipts(staged.reconciliation_plan().receipts().len());
}

pub(super) fn plan_lowering_basis(
    staged: &crate::runtime::WorthUiStagedReplacement,
    pending_frame_epoch: WorthUiRuntimeFrameEpoch,
    candidate_node_input_count: usize,
    query_binding_input_count: usize,
) -> WorthUiPlanLoweringBasis {
    WorthUiPlanLoweringBasis::new(
        Some(staged.active_artifact_digest()),
        staged.candidate_artifact_digest(),
        pending_frame_epoch,
        candidate_node_input_count,
        staged.reconciliation_plan().receipts().len(),
        query_binding_input_count,
    )
}

pub(super) fn plan_lowering_context(
    pending_activation: &WorthUiPendingActivation,
) -> WorthUiPlanLoweringContext {
    WorthUiPlanLoweringContext::replacement(
        pending_activation.readiness(),
        pending_activation.staging_report().clone(),
    )
}
