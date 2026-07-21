use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingPreservationReceipt {
    binding_identity: u64,
    posture_identity: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingPreservation {
    identity: WorthUiQueryBindingIdentity,
    preserved_posture: WorthUiQueryBindingPosture,
    preservation_receipt: WorthUiQueryBindingPreservationReceipt,
}

impl WorthUiQueryBindingPreservation {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        preserved_posture: WorthUiQueryBindingPosture,
    ) -> Self {
        let preservation_receipt = WorthUiQueryBindingPreservationReceipt {
            binding_identity: identity.canonical_identity(),
            posture_identity: preserved_posture.canonical_identity(),
        };
        Self {
            identity,
            preserved_posture,
            preservation_receipt,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn preserved_posture(&self) -> &WorthUiQueryBindingPosture {
        &self.preserved_posture
    }

    pub fn preservation_receipt(&self) -> WorthUiQueryBindingPreservationReceipt {
        self.preservation_receipt
    }
}

impl WorthUiQueryBindingPreservationReceipt {
    pub fn binding_identity(self) -> u64 {
        self.binding_identity
    }

    pub fn posture_identity(self) -> u64 {
        self.posture_identity
    }

    pub fn canonical_identity(self) -> u64 {
        self.binding_identity.rotate_left(17) ^ self.posture_identity.rotate_left(41)
    }
}
