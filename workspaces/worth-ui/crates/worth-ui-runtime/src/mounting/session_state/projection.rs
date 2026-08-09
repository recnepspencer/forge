use super::WorthUiMountedSessionState;

impl WorthUiMountedSessionState {
    pub(crate) fn native_filled_rect_attribution(
        &self,
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Option<(
        worth_ui_host_contract::UiSemanticSurfaceIdentity,
        worth_ui_host_contract::UiMountedInstanceIdentity,
        worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    )> {
        let view = self.identity.current_projection()?.view_for(binding).ok()?;
        if view.frame() != frame {
            return None;
        }
        let [mechanic] = view.filled_rects().rows() else {
            return None;
        };
        Some((
            view.surface(),
            mechanic.mounted_instance(),
            mechanic.node_receipt(),
        ))
    }

    pub(crate) fn classify_frame_reuse(
        &self,
        contract: crate::mounting::UiMountedFrameReuseContract,
    ) -> crate::mounting::UiMountedFrameReuse {
        self.identity.classify_reuse(contract)
    }

    pub(crate) fn current_allocation_truth_revision(&self) -> Option<u64> {
        self.identity.current_allocation_truth_revision()
    }

    pub(crate) fn seal_frame_reuse_contract(
        &self,
        basis: crate::mounting::UiMountedFrameReuseExternalBasis,
    ) -> crate::mounting::UiMountedFrameReuseContract {
        self.identity.seal_reuse_contract(basis)
    }

    pub(crate) fn begin_frame_assembly(
        &self,
        input: crate::mounting::UiMountedFrameAssemblyInput<'_, '_>,
    ) -> Result<
        crate::mounting::UiMountedFrameAssembler<'_>,
        crate::mounting::UiMountedFramePreparationDenial,
    > {
        crate::mounting::UiMountedFrameAssembler::begin(&self.identity, input)
    }

    pub(crate) fn current_projection_input(
        &self,
        slot: worth_ui_query_binding::UiProjectionInputSlot,
    ) -> Option<worth_ui_query_binding::UiProjectionInputFactReference> {
        self.retention.current_projection_input(slot)
    }
}
