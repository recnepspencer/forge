#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedVisualOverlayProjectionInput {
    pub(crate) overlay_identity: u64,
    pub(crate) base_snapshot: u64,
    pub(crate) base_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    pub(crate) target_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    pub(crate) surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    pub(crate) binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    pub(crate) coordinate_basis: worth_ui_host_contract::UiMountedClientCoordinateBasis,
    pub(crate) target_region: worth_ui_host_contract::UiMountedClientPhysicalRect,
}

impl UiMountedVisualOverlayProjectionInput {
    pub(crate) const fn target_instance(self) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.target_receipt.mounted_instance()
    }

    pub(crate) fn mechanic_for(
        self,
        successor_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Option<worth_ui_host_contract::UiMountedIdentityOverlayMechanic> {
        if self.surface != surface || self.binding != binding {
            return None;
        }
        worth_ui_host_contract::UiMountedIdentityOverlayMechanic::from_runtime_mounting(
            worth_ui_host_contract::UiMountedIdentityOverlayMechanicInput {
                overlay_identity: self.overlay_identity,
                base_snapshot: self.base_snapshot,
                base_frame: self.base_frame,
                target_receipt: self.target_receipt,
                successor_frame,
                surface,
                binding,
                coordinate_basis: self.coordinate_basis,
                target_region: self.target_region,
            },
        )
    }
}
