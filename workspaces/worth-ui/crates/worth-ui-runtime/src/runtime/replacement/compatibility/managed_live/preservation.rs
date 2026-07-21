use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingUiRequirements,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingPreservationReceipt {
    binding_identity: u64,
    ui_requirements_identity: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingPreservation {
    identity: WorthUiQueryBindingIdentity,
    preserved_ui_requirements: WorthUiQueryBindingUiRequirements,
    preservation_receipt: WorthUiQueryBindingPreservationReceipt,
}

impl WorthUiQueryBindingPreservation {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        preserved_ui_requirements: WorthUiQueryBindingUiRequirements,
    ) -> Self {
        let preservation_receipt = WorthUiQueryBindingPreservationReceipt {
            binding_identity: identity.canonical_identity(),
            ui_requirements_identity: preserved_ui_requirements.canonical_identity(),
        };
        Self {
            identity,
            preserved_ui_requirements,
            preservation_receipt,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn preserved_ui_requirements(&self) -> &WorthUiQueryBindingUiRequirements {
        &self.preserved_ui_requirements
    }

    pub fn preservation_receipt(&self) -> WorthUiQueryBindingPreservationReceipt {
        self.preservation_receipt
    }
}

impl WorthUiQueryBindingPreservationReceipt {
    pub fn binding_identity(self) -> u64 {
        self.binding_identity
    }

    pub fn ui_requirements_identity(self) -> u64 {
        self.ui_requirements_identity
    }

    pub fn canonical_identity(self) -> u64 {
        self.binding_identity.rotate_left(17) ^ self.ui_requirements_identity.rotate_left(41)
    }
}
