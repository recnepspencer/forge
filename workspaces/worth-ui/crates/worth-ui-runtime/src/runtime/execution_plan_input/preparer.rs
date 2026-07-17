#[cfg(test)]
use crate::runtime::execution_plan_input::component_hook::WorthUiComponentLoweringHookAdmission;
use crate::runtime::execution_plan_input::WorthUiPlanNodeTopologyInputIndex;
use crate::runtime::{
    WorthUiComponentLoweringHook, WorthUiExecutionPlanInput, WorthUiPendingActivation,
    WorthUiPlanLoweringBasis, WorthUiPlanLoweringContext, WorthUiPlanLoweringCounters,
    WorthUiPlanLoweringDenial, WorthUiPlanLoweringDenialReason, WorthUiPlanNodeInput,
    WorthUiRuntimeFrameEpoch,
};

pub(crate) struct WorthUiExecutionPlanInputPreparer;

impl WorthUiExecutionPlanInputPreparer {
    pub(crate) fn prepare(
        pending_activation: &WorthUiPendingActivation,
        active_frame_epoch: WorthUiRuntimeFrameEpoch,
        component_hooks: &[WorthUiComponentLoweringHook],
    ) -> Result<WorthUiExecutionPlanInput, WorthUiPlanLoweringDenial> {
        let pending_frame_epoch = pending_activation.frame_epoch();
        let staged = pending_activation.staged_replacement();
        let mut counters = WorthUiPlanLoweringCounters::default();
        counters.record_epoch_verification();

        reject_stale_pending_activation(staged, pending_frame_epoch, active_frame_epoch, counters)?;

        counters.record_readiness_verification();
        reject_missing_activation_readiness(
            pending_activation,
            staged,
            pending_frame_epoch,
            active_frame_epoch,
            counters,
        )?;

        reject_mismatched_pending_input(staged, pending_frame_epoch, active_frame_epoch, counters)?;
        #[cfg(test)]
        reject_unregistered_component_hooks(
            staged,
            pending_frame_epoch,
            active_frame_epoch,
            component_hooks,
            &mut counters,
        )?;

        let node_inputs = collect_plan_node_inputs(staged, component_hooks, &mut counters);
        record_reconciliation_receipt_inputs(staged, &mut counters);

        let basis = plan_lowering_basis(staged, pending_frame_epoch);
        let context = plan_lowering_context(pending_activation);

        Ok(WorthUiExecutionPlanInput::new(
            basis,
            context,
            node_inputs,
            counters,
        ))
    }
}

fn reject_stale_pending_activation(
    staged: &crate::runtime::WorthUiStagedReplacement,
    pending_frame_epoch: WorthUiRuntimeFrameEpoch,
    active_frame_epoch: WorthUiRuntimeFrameEpoch,
    counters: WorthUiPlanLoweringCounters,
) -> Result<(), WorthUiPlanLoweringDenial> {
    if pending_frame_epoch == active_frame_epoch {
        return Ok(());
    }

    Err(denial(
        staged,
        pending_frame_epoch,
        active_frame_epoch,
        WorthUiPlanLoweringDenialReason::StalePendingActivation,
        counters,
    ))
}

fn reject_missing_activation_readiness(
    pending_activation: &WorthUiPendingActivation,
    staged: &crate::runtime::WorthUiStagedReplacement,
    pending_frame_epoch: WorthUiRuntimeFrameEpoch,
    active_frame_epoch: WorthUiRuntimeFrameEpoch,
    counters: WorthUiPlanLoweringCounters,
) -> Result<(), WorthUiPlanLoweringDenial> {
    if pending_activation
        .readiness()
        .is_ready_for_execution_plan_input()
    {
        return Ok(());
    }

    Err(denial(
        staged,
        pending_frame_epoch,
        active_frame_epoch,
        WorthUiPlanLoweringDenialReason::MissingActivationReadiness,
        counters,
    ))
}

fn reject_mismatched_pending_input(
    staged: &crate::runtime::WorthUiStagedReplacement,
    pending_frame_epoch: WorthUiRuntimeFrameEpoch,
    active_frame_epoch: WorthUiRuntimeFrameEpoch,
    counters: WorthUiPlanLoweringCounters,
) -> Result<(), WorthUiPlanLoweringDenial> {
    if pending_input_matches_staged_artifacts(staged) {
        return Ok(());
    }

    Err(denial(
        staged,
        pending_frame_epoch,
        active_frame_epoch,
        WorthUiPlanLoweringDenialReason::ExecutionPlanLoweringInputMismatch,
        counters,
    ))
}

#[cfg(test)]
fn reject_unregistered_component_hooks(
    staged: &crate::runtime::WorthUiStagedReplacement,
    pending_frame_epoch: WorthUiRuntimeFrameEpoch,
    active_frame_epoch: WorthUiRuntimeFrameEpoch,
    component_hooks: &[WorthUiComponentLoweringHook],
    counters: &mut WorthUiPlanLoweringCounters,
) -> Result<(), WorthUiPlanLoweringDenial> {
    for hook in component_hooks {
        if matches!(
            hook.admission(),
            WorthUiComponentLoweringHookAdmission::Unregistered
        ) {
            counters.record_rejected_component_hook();
            return Err(denial(
                staged,
                pending_frame_epoch,
                active_frame_epoch,
                WorthUiPlanLoweringDenialReason::UnregisteredPlanNodeFamily,
                *counters,
            ));
        }
    }

    Ok(())
}

fn collect_plan_node_inputs(
    staged: &crate::runtime::WorthUiStagedReplacement,
    component_hooks: &[WorthUiComponentLoweringHook],
    counters: &mut WorthUiPlanLoweringCounters,
) -> Vec<WorthUiPlanNodeInput> {
    let mut node_inputs = Vec::new();
    let topology_index = WorthUiPlanNodeTopologyInputIndex::from_artifact(
        staged.admitted_candidate().artifact_bundle().artifact(),
    );
    collect_staged_replacement_node_inputs(staged, &topology_index, counters, &mut node_inputs);
    collect_query_binding_node_inputs(staged, &topology_index, counters, &mut node_inputs);
    collect_component_hook_node_inputs(component_hooks, counters, &mut node_inputs);
    node_inputs
}

fn collect_staged_replacement_node_inputs(
    staged: &crate::runtime::WorthUiStagedReplacement,
    topology_index: &WorthUiPlanNodeTopologyInputIndex,
    counters: &mut WorthUiPlanLoweringCounters,
    node_inputs: &mut Vec<WorthUiPlanNodeInput>,
) {
    for classification in staged.node_plan().classifications() {
        let topology_input = topology_index
            .input_for_identity(classification.identity_basis())
            .unwrap_or_default();
        node_inputs.push(WorthUiPlanNodeInput::from_replacement_classification(
            classification,
            topology_input,
        ));
        counters.record_staged_node_input();
    }
}

fn collect_query_binding_node_inputs(
    staged: &crate::runtime::WorthUiStagedReplacement,
    topology_index: &WorthUiPlanNodeTopologyInputIndex,
    counters: &mut WorthUiPlanLoweringCounters,
    node_inputs: &mut Vec<WorthUiPlanNodeInput>,
) {
    for entry in staged.query_rebind_plan().entries() {
        let topology_input = topology_index
            .input_for_identity(entry.identity().view_binding_id())
            .unwrap_or_default();
        node_inputs.push(WorthUiPlanNodeInput::from_query_rebind_entry(
            entry,
            topology_input,
        ));
        counters.record_query_binding_input();
    }
}

fn collect_component_hook_node_inputs(
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

fn record_reconciliation_receipt_inputs(
    staged: &crate::runtime::WorthUiStagedReplacement,
    counters: &mut WorthUiPlanLoweringCounters,
) {
    counters.record_reconciliation_receipts(staged.reconciliation_plan().receipts().len());
}

fn plan_lowering_basis(
    staged: &crate::runtime::WorthUiStagedReplacement,
    pending_frame_epoch: WorthUiRuntimeFrameEpoch,
) -> WorthUiPlanLoweringBasis {
    WorthUiPlanLoweringBasis::new(
        staged.active_artifact_digest(),
        staged.candidate_artifact_digest(),
        pending_frame_epoch,
        staged.node_plan().classifications().len(),
        staged.reconciliation_plan().receipts().len(),
        staged.query_rebind_plan().entries().len(),
    )
}

fn plan_lowering_context(
    pending_activation: &WorthUiPendingActivation,
) -> WorthUiPlanLoweringContext {
    WorthUiPlanLoweringContext::new(
        pending_activation.readiness(),
        pending_activation.staging_report().clone(),
    )
}

fn pending_input_matches_staged_artifacts(
    staged: &crate::runtime::WorthUiStagedReplacement,
) -> bool {
    let input = staged.pending_execution_plan_lowering_input();
    input.active_artifact_digest() == staged.active_artifact_digest()
        && input.candidate_artifact_digest() == staged.candidate_artifact_digest()
        && input.node_classification_count() == staged.node_plan().classifications().len()
        && input.reconciliation_receipt_count() == staged.reconciliation_plan().receipts().len()
        && input.query_rebind_entry_count() == staged.query_rebind_plan().entries().len()
}

fn denial(
    staged: &crate::runtime::WorthUiStagedReplacement,
    pending_frame_epoch: WorthUiRuntimeFrameEpoch,
    active_frame_epoch: WorthUiRuntimeFrameEpoch,
    reason: WorthUiPlanLoweringDenialReason,
    counters: WorthUiPlanLoweringCounters,
) -> WorthUiPlanLoweringDenial {
    WorthUiPlanLoweringDenial::new(
        staged.active_artifact_digest(),
        staged.candidate_artifact_digest(),
        pending_frame_epoch,
        active_frame_epoch,
        reason,
        counters,
    )
}
