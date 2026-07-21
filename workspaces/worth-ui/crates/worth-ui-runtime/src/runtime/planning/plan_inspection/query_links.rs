use crate::runtime::{WorthUiQueryBindingIdentity, WorthUiQuerySettledFactLink};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryInspectionLinks {
    binding_identity: WorthUiQueryBindingIdentity,
    settled_fact_link: WorthUiQuerySettledFactLink,
    preservation_receipt: Option<crate::runtime::WorthUiQueryBindingPreservationReceipt>,
}

impl WorthUiQueryInspectionLinks {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn from_settled_fact_link(
        binding_identity: WorthUiQueryBindingIdentity,
        settled_fact_link: WorthUiQuerySettledFactLink,
        preservation_receipt: Option<crate::runtime::WorthUiQueryBindingPreservationReceipt>,
    ) -> Self {
        Self {
            binding_identity,
            settled_fact_link,
            preservation_receipt,
        }
    }

    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.binding_identity
    }

    pub fn settled_fact_link(&self) -> &WorthUiQuerySettledFactLink {
        &self.settled_fact_link
    }

    pub fn preservation_receipt(
        &self,
    ) -> Option<crate::runtime::WorthUiQueryBindingPreservationReceipt> {
        self.preservation_receipt
    }
}
