use super::WorthUiPlanNodeTopologyInputIndex;
use crate::runtime::{
    WorthUiExecutionPlanInput, WorthUiPendingActivation, WorthUiPlanLoweringBasis,
    WorthUiPlanLoweringContext, WorthUiPlanNodeInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiExecutionPlanInputWitness {
    basis: WorthUiPlanLoweringBasis,
    context: WorthUiPlanLoweringContext,
    node_inputs: Vec<WorthUiPlanNodeInput>,
}

impl WorthUiExecutionPlanInputWitness {
    pub(crate) fn from_execution_plan_input(plan_input: &WorthUiExecutionPlanInput) -> Self {
        Self {
            basis: plan_input.basis().clone(),
            context: plan_input.context().clone(),
            node_inputs: plan_input.node_inputs().to_vec(),
        }
    }

    pub(crate) fn from_pending_activation(pending_activation: &WorthUiPendingActivation) -> Self {
        let staged = pending_activation.staged_replacement();
        let topology_index = WorthUiPlanNodeTopologyInputIndex::from_artifact(
            staged.admitted_candidate().artifact_bundle().artifact(),
        );
        let mut node_inputs = Vec::new();

        for classification in staged.node_plan().classifications() {
            let topology_input = topology_index
                .input_for_identity(classification.identity_basis())
                .unwrap_or_default();
            node_inputs.push(WorthUiPlanNodeInput::from_replacement_classification(
                classification,
                topology_input,
            ));
        }

        for entry in staged.query_rebind_plan().entries() {
            let topology_input = topology_index
                .input_for_identity(entry.identity().view_binding_id())
                .unwrap_or_default();
            node_inputs.push(WorthUiPlanNodeInput::from_query_rebind_entry(
                entry,
                topology_input,
            ));
        }

        node_inputs.sort_by(|left, right| {
            left.family()
                .cmp(&right.family())
                .then_with(|| left.identity_basis().cmp(right.identity_basis()))
        });

        Self {
            basis: WorthUiPlanLoweringBasis::new(
                staged.active_artifact_digest(),
                staged.candidate_artifact_digest(),
                pending_activation.frame_epoch(),
                staged.node_plan().classifications().len(),
                staged.reconciliation_plan().receipts().len(),
                staged.query_rebind_plan().entries().len(),
            ),
            context: WorthUiPlanLoweringContext::new(
                pending_activation.readiness(),
                pending_activation.staging_report().clone(),
            ),
            node_inputs,
        }
    }

    pub(crate) fn matches_execution_plan_input(
        &self,
        lowered_input: &WorthUiExecutionPlanInput,
    ) -> bool {
        self == &Self::from_execution_plan_input(lowered_input)
    }

    pub(crate) fn basis(&self) -> &WorthUiPlanLoweringBasis {
        &self.basis
    }

    pub(crate) fn digest(&self) -> u64 {
        use crate::declaration::stable_text_digest;

        let mut digest = stable_text_digest("worth-ui.runtime.execution-plan-input-witness")
            ^ self.basis.active_artifact_digest().rotate_left(7)
            ^ self.basis.candidate_artifact_digest().rotate_left(11)
            ^ self.basis.frame_epoch().as_u64().rotate_left(13)
            ^ (self.basis.staged_node_classification_count() as u64).rotate_left(17)
            ^ (self.basis.staged_reconciliation_receipt_count() as u64).rotate_left(19)
            ^ (self.basis.staged_query_rebind_entry_count() as u64).rotate_left(23)
            ^ u64::from(self.context.readiness().is_ready_for_execution_plan_input())
                .rotate_left(29)
            ^ self
                .context
                .staging_report()
                .active_artifact_digest()
                .rotate_left(31)
            ^ self
                .context
                .staging_report()
                .candidate_artifact_digest()
                .rotate_left(37)
            ^ u64::from(
                self.context
                    .staging_report()
                    .readiness()
                    .is_ready_for_execution_plan_input(),
            )
            .rotate_left(41);

        for (index, node_input) in self.node_inputs.iter().enumerate() {
            digest ^= (index as u64).rotate_left(3);
            digest ^= (node_input.family() as u64).rotate_left(5);
            digest ^= stable_text_digest(node_input.identity_basis()).rotate_left(7);
            digest ^= node_input
                .authored_provenance_digest()
                .unwrap_or_default()
                .rotate_left(11);
            digest ^= u64::from(node_input.transition().is_some()).rotate_left(13);
            digest ^= u64::from(node_input.egui_boundary_input().is_some()).rotate_left(17);
            if let Some(identity) = node_input.query_binding_identity() {
                digest ^= stable_text_digest(identity.view_binding_id()).rotate_left(19);
                digest ^= stable_text_digest(identity.query_capability_digest()).rotate_left(23);
                digest ^=
                    stable_text_digest(identity.query_composition_profile_digest()).rotate_left(29);
                digest ^= stable_text_digest(identity.result_shape_digest()).rotate_left(31);
            }
            if let Some(posture) = node_input.query_binding_posture() {
                digest ^= (posture.query_support_status() as u64).rotate_left(37);
                digest ^= stable_text_digest(posture.support_admission_digest()).rotate_left(41);
                digest ^= stable_text_digest(posture.basis_capability_digest()).rotate_left(43);
                digest ^= stable_text_digest(posture.live_compatibility_digest()).rotate_left(47);
                digest ^= stable_text_digest(posture.async_result_state_digest()).rotate_left(53);
                digest ^= stable_text_digest(posture.recovery_digest()).rotate_left(59);
                digest ^= stable_text_digest(posture.inspection_digest()).rotate_left(61);
                digest ^=
                    stable_text_digest(posture.projection_consumption_digest()).rotate_left(2);
                digest ^= stable_text_digest(posture.denial_presentation_digest()).rotate_left(9);
            }
            for surface in node_input.query_required_surfaces() {
                digest ^= (*surface as u64).rotate_left(27);
            }
            if let Some(receipt) = node_input.query_preservation_receipt() {
                digest ^= stable_text_digest(receipt).rotate_left(33);
            }
            if let Some(boundary) = node_input.egui_boundary_input() {
                digest ^= (boundary as u64).rotate_left(39);
            }
            let topology_input = node_input.topology_input();
            digest ^= u64::from(topology_input.structure_declared()).rotate_left(45);
            digest ^= (topology_input.root_region_count() as u64).rotate_left(49);
            digest ^= (topology_input.region_count() as u64).rotate_left(51);
            digest ^= (topology_input.mount_count() as u64).rotate_left(55);
            digest ^= (topology_input.max_region_depth() as u64).rotate_left(57);
        }

        digest
    }
}
