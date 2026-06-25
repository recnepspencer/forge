use crate::runtime::{
    WorthUiClassifiedRuntimeChange, WorthUiLiveViewStateBindingGraphPosture,
    WorthUiRuntimeChangeAdmissionDenial, WorthUiRuntimeInstanceWitness,
};

use super::admission::{
    admit_live_view_declaration, live_view_graph_execution, target_binding_stale_denial,
};
use super::{
    WorthUiLiveViewAdmissionReport, WorthUiLiveViewDeclaration,
    WorthUiLiveViewDeclarationRebindReceipt, WorthUiLiveViewDeclarationReceipt,
    WorthUiLiveViewEditReceipt, WorthUiLiveViewStateEditDenial, WorthUiLiveViewStateEditIntent,
};

impl crate::runtime::WorthUiRuntimeHost {
    pub fn admit_live_view_declaration(
        &self,
        declaration: WorthUiLiveViewDeclaration,
    ) -> Result<WorthUiLiveViewDeclarationReceipt, WorthUiLiveViewAdmissionReport> {
        admit_live_view_declaration(self, declaration)
    }

    pub fn apply_live_view_state_edit(
        &mut self,
        intent: WorthUiLiveViewStateEditIntent,
    ) -> Result<WorthUiLiveViewEditReceipt, WorthUiLiveViewStateEditDenial> {
        let (binding, value) = intent.into_parts();
        if let Some(denial) = target_binding_stale_denial(self, binding.target_binding()) {
            return Err(stale_edit_denial(binding.binding_id(), denial));
        }
        if binding.access() != super::WorthUiLiveViewStateAccess::ReadWrite {
            return Err(WorthUiLiveViewStateEditDenial::ReadOnlyBinding {
                binding_id: binding.binding_id().to_owned(),
            });
        }
        if value.value_kind() != *binding.value_kind() {
            return Err(WorthUiLiveViewStateEditDenial::ValueKindMismatch {
                binding_id: binding.binding_id().to_owned(),
                expected: binding.value_kind().clone(),
                actual: value.value_kind(),
            });
        }
        let graph_execution = live_view_graph_execution(
            self.graph_authority(),
            binding.live_view_id(),
            binding.target_binding(),
            std::slice::from_ref(&binding),
            WorthUiLiveViewStateBindingGraphPosture::Admitted,
        );
        let previous = self
            .active_state_for_swap_mut()
            .live_view_state_store_mut()
            .record(binding.state_fact().clone(), value.clone());
        Ok(WorthUiLiveViewEditReceipt::new(
            binding,
            previous,
            value,
            graph_execution,
        ))
    }

    pub fn compare_live_view_declaration_rebind(
        &self,
        prior: &WorthUiLiveViewDeclarationReceipt,
        next: &WorthUiLiveViewDeclarationReceipt,
    ) -> WorthUiLiveViewDeclarationRebindReceipt {
        WorthUiLiveViewDeclarationRebindReceipt::from_admitted_declarations(prior, next)
    }

    pub fn live_view_state_value(
        &self,
        binding: &super::WorthUiLiveViewStateBindingReceipt,
    ) -> Option<&super::WorthUiLiveViewStateValue> {
        self.active_state_for_read()
            .live_view_state_store()
            .get(binding.state_fact())
    }

    pub fn admit_live_view_state_runtime_change(
        &self,
        receipt: &WorthUiLiveViewEditReceipt,
    ) -> Result<
        crate::runtime::WorthUiAdmittedRuntimeChangeEvidence,
        WorthUiRuntimeChangeAdmissionDenial,
    > {
        crate::runtime::WorthUiAdmittedRuntimeChangeEvidence::admit(
            WorthUiClassifiedRuntimeChange::from_live_view_state_edit(
                WorthUiRuntimeInstanceWitness::from_raw(self.instance_id().raw()),
                receipt,
            ),
            WorthUiRuntimeInstanceWitness::from_raw(self.instance_id().raw()),
        )
    }
}

fn stale_edit_denial(
    binding_id: &str,
    denial: super::WorthUiLiveViewDenial,
) -> WorthUiLiveViewStateEditDenial {
    match denial {
        super::WorthUiLiveViewDenial::StaleTargetBinding {
            slot_name,
            surface_id,
            expected_component_id,
            actual_component_id,
        } => WorthUiLiveViewStateEditDenial::StaleTargetBinding {
            binding_id: binding_id.to_owned(),
            slot_name,
            surface_id,
            expected_component_id,
            actual_component_id,
        },
        _ => unreachable!("target_binding_stale_denial only returns stale target denials"),
    }
}
