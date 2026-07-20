use crate::runtime::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture, WorthUiQueryRebindRequiredSurface,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryInspectionLinks {
    binding_identity: WorthUiQueryBindingIdentity,
    posture: WorthUiQueryBindingPosture,
    preservation_receipt: Option<crate::runtime::WorthUiQueryBindingPreservationReceipt>,
    required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
}

impl WorthUiQueryInspectionLinks {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn from_query_posture(
        binding_identity: WorthUiQueryBindingIdentity,
        posture: WorthUiQueryBindingPosture,
        preservation_receipt: Option<crate::runtime::WorthUiQueryBindingPreservationReceipt>,
        required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
    ) -> Self {
        Self {
            binding_identity,
            posture,
            preservation_receipt,
            required_surfaces,
        }
    }

    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.binding_identity
    }

    pub fn posture(&self) -> &WorthUiQueryBindingPosture {
        &self.posture
    }

    pub fn preservation_receipt(
        &self,
    ) -> Option<crate::runtime::WorthUiQueryBindingPreservationReceipt> {
        self.preservation_receipt
    }

    pub fn required_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.required_surfaces
    }
}
