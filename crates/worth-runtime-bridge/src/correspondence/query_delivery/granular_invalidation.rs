use super::{BridgeDeliveredTruthChange, BridgePerformedSignalInvalidation};
use crate::correspondence::BridgeCorrespondenceDeliveryReceipt;

pub struct BridgeGranularInvalidationDelivery {
    correspondence: BridgeCorrespondenceDeliveryReceipt,
    performed_signal: Option<BridgePerformedSignalInvalidation>,
}

impl BridgeGranularInvalidationDelivery {
    pub fn direct(truth: &BridgeCorrespondenceDeliveryReceipt) -> Self {
        Self {
            correspondence: truth.clone(),
            performed_signal: None,
        }
    }

    pub(crate) fn with_performed_signal(
        mut self,
        performed: BridgePerformedSignalInvalidation,
    ) -> Self {
        debug_assert!(performed.retains_truth(self.correspondence.change_set()));
        self.performed_signal = Some(performed);
        self
    }

    pub const fn truth(&self) -> &BridgeDeliveredTruthChange {
        self.correspondence.truth_change()
    }

    pub const fn correspondence_receipt(&self) -> &BridgeCorrespondenceDeliveryReceipt {
        &self.correspondence
    }

    pub const fn performed_signal(&self) -> Option<&BridgePerformedSignalInvalidation> {
        self.performed_signal.as_ref()
    }
}
