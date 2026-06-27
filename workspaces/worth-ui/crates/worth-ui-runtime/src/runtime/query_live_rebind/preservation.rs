use crate::runtime::query_binding::{WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingPreservation {
    identity: WorthUiQueryBindingIdentity,
    preserved_posture: WorthUiQueryBindingPosture,
    preservation_receipt: String,
}

impl WorthUiQueryBindingPreservation {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        preserved_posture: WorthUiQueryBindingPosture,
    ) -> Self {
        let preservation_receipt = format!(
            "query-live-preserve:{}:{}",
            identity.view_binding_id(),
            preserved_posture.live_compatibility_digest()
        );
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

    pub fn preservation_receipt(&self) -> &str {
        &self.preservation_receipt
    }
}
