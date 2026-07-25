use super::{
    UiHostObservationFamily, UiHostObservationPayload, UiHostObservationSequence,
    UiHostObservationTimeBasis,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostObservationMountedBasis {
    instance: crate::UiMountedInstanceIdentity,
    node_receipt: crate::UiMountedNodeReceiptIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiHostObservationReport {
    sequence: UiHostObservationSequence,
    time_basis: UiHostObservationTimeBasis,
    payload: UiHostObservationPayload,
    mounted_basis: Option<UiHostObservationMountedBasis>,
}

impl UiHostObservationMountedBasis {
    pub const fn new(
        instance: crate::UiMountedInstanceIdentity,
        node_receipt: crate::UiMountedNodeReceiptIdentity,
    ) -> Self {
        Self {
            instance,
            node_receipt,
        }
    }

    pub const fn instance(self) -> crate::UiMountedInstanceIdentity {
        self.instance
    }

    pub const fn node_receipt(self) -> crate::UiMountedNodeReceiptIdentity {
        self.node_receipt
    }
}

impl UiHostObservationReport {
    pub fn new(
        sequence: UiHostObservationSequence,
        time_basis: UiHostObservationTimeBasis,
        payload: UiHostObservationPayload,
    ) -> Self {
        Self {
            sequence,
            time_basis,
            payload,
            mounted_basis: None,
        }
    }

    pub fn with_mounted_basis(mut self, basis: UiHostObservationMountedBasis) -> Self {
        self.mounted_basis = Some(basis);
        self
    }

    pub const fn sequence(&self) -> UiHostObservationSequence {
        self.sequence
    }

    pub const fn time_basis(&self) -> UiHostObservationTimeBasis {
        self.time_basis
    }

    pub fn payload(&self) -> &UiHostObservationPayload {
        &self.payload
    }

    pub const fn family(&self) -> UiHostObservationFamily {
        self.payload.family()
    }

    pub const fn mounted_basis(&self) -> Option<UiHostObservationMountedBasis> {
        self.mounted_basis
    }

    pub fn encoded_len(&self) -> usize {
        24 + self.payload.encoded_len() + usize::from(self.mounted_basis.is_some()) * 16
    }
}
