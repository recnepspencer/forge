#[cfg(test)]
use crate::runtime::planning::execution_plan_input::component_hook::WorthUiComponentLoweringHookAdmission;
#[cfg(test)]
use crate::runtime::planning::execution_plan_input::replacement_input_collection::collect_component_hook_node_inputs;
use crate::runtime::planning::execution_plan_input::replacement_input_collection::{
    collect_plan_node_inputs, plan_lowering_basis, plan_lowering_context,
    record_reconciliation_receipt_inputs,
};
use crate::runtime::planning::execution_plan_input::WorthUiPlanNodeTopologyInputIndex;
use crate::runtime::{
    WorthUiComponentLoweringHook, WorthUiExecutionPlanInput, WorthUiPendingActivation,
    WorthUiPlanLoweringBasis, WorthUiPlanLoweringContext, WorthUiPlanLoweringCounters,
    WorthUiPlanLoweringDenial, WorthUiPlanLoweringDenialReason, WorthUiPlanNodeInput,
    WorthUiRuntimeFrameEpoch,
};

pub(crate) struct WorthUiExecutionPlanInputPreparer;

impl WorthUiExecutionPlanInputPreparer {
    pub(crate) fn prepare_launch(
        artifact: &crate::source::WorthUiArtifact,
        artifact_digest: crate::source::WorthUiArtifactDigest,
        frame_epoch: WorthUiRuntimeFrameEpoch,
        query_binding_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
    ) -> WorthUiExecutionPlanInput {
        let topology_index = WorthUiPlanNodeTopologyInputIndex::from_artifact(artifact);
        let mut counters = WorthUiPlanLoweringCounters::default();
        let mut node_inputs = Vec::new();
        for module_id in artifact.module_ids() {
            let Some(module) = artifact.module(module_id) else {
                continue;
            };
            for node in module.nodes() {
                let topology_input = topology_index
                    .input_for_identity(node.identity_seed().basis())
                    .unwrap_or_default();
                for input in super::lower_launch_ordinary_node(node, topology_input)
                    .expect("launch lowering carries no predecessor succession")
                {
                    node_inputs.push(input);
                    counters.record_staged_node_input();
                }
            }
        }
        let query_evidence =
            crate::runtime::replacement::query_binding::WorthUiQueryBindingEvidenceIndex::from_active_artifact(
                artifact,
            );
        let query_binding_input_count = query_evidence.len();
        for (identity, _posture) in query_evidence.entries() {
            let topology_input = topology_index
                .input_for_identity(identity.view_binding_id())
                .unwrap_or_default();
            let installed_reference = query_binding_plan
                .resolve_definition(identity.query_view_identity(), identity.result_shape());
            node_inputs.push(WorthUiPlanNodeInput::from_launch_query_binding(
                identity,
                installed_reference,
                topology_input,
            ));
            counters.record_query_binding_input();
        }
        let candidate_node_input_count = node_inputs.len();
        WorthUiExecutionPlanInput::new(
            WorthUiPlanLoweringBasis::new(
                None,
                artifact_digest.raw(),
                frame_epoch,
                candidate_node_input_count,
                0,
                query_binding_input_count,
            ),
            WorthUiPlanLoweringContext::launch(),
            node_inputs,
            counters,
        )
    }

    #[cfg(test)]
    pub(crate) fn prepare_launch_with_component_hooks(
        artifact: &crate::source::WorthUiArtifact,
        artifact_digest: crate::source::WorthUiArtifactDigest,
        frame_epoch: WorthUiRuntimeFrameEpoch,
        query_binding_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
        component_hooks: &[WorthUiComponentLoweringHook],
    ) -> WorthUiExecutionPlanInput {
        let launch =
            Self::prepare_launch(artifact, artifact_digest, frame_epoch, query_binding_plan);
        let mut counters = launch.counters();
        let mut node_inputs = launch.node_inputs().to_vec();
        collect_component_hook_node_inputs(component_hooks, &mut counters, &mut node_inputs);
        WorthUiExecutionPlanInput::new(
            WorthUiPlanLoweringBasis::new(
                None,
                artifact_digest.raw(),
                frame_epoch,
                launch.basis().candidate_node_input_count() + counters.component_hook_input_count(),
                0,
                launch.basis().query_binding_input_count(),
            ),
            WorthUiPlanLoweringContext::launch(),
            node_inputs,
            counters,
        )
    }

    pub(crate) fn prepare(
        pending_activation: &WorthUiPendingActivation,
        active_frame_epoch: WorthUiRuntimeFrameEpoch,
        component_hooks: &[WorthUiComponentLoweringHook],
        query_binding_plan: &worth_ui_query_binding::WorthUiQueryBindingPlan,
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

        #[cfg(test)]
        reject_unregistered_component_hooks(
            staged,
            pending_frame_epoch,
            active_frame_epoch,
            component_hooks,
            &mut counters,
        )?;

        let node_inputs =
            collect_plan_node_inputs(staged, component_hooks, query_binding_plan, &mut counters)
                .map_err(|reason| {
                    denial(
                        staged,
                        pending_frame_epoch,
                        active_frame_epoch,
                        reason,
                        counters,
                    )
                })?;
        let query_binding_input_count = staged.query_rebind_plan().live_candidate_binding_count();
        let candidate_node_input_count = staged.node_plan().candidate_structural_node_count()
            + query_binding_input_count
            + counters.component_hook_input_count();
        record_reconciliation_receipt_inputs(staged, &mut counters);

        let basis = plan_lowering_basis(
            staged,
            pending_frame_epoch,
            candidate_node_input_count,
            query_binding_input_count,
        );
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
