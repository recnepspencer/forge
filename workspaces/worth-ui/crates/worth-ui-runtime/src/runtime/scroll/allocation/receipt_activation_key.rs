#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiScrollReceiptActivationKey {
    source: super::UiAdmittedScrollExtentSource,
    receipt_identity: crate::runtime::UiAllocationReceiptIdentity,
    generation: crate::runtime::UiAllocationReceiptGeneration,
    equivalence_basis: crate::runtime::UiAllocationReceiptEquivalenceBasis,
    virtualization: super::UiScrollVirtualizationPosture,
    offset_allocation: super::UiScrollOffsetAllocationPosture,
}

impl std::hash::Hash for UiScrollReceiptActivationKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.identity_digest());
    }
}

impl UiScrollReceiptActivationKey {
    pub(crate) fn from_receipt_and_source(
        receipt: &crate::runtime::UiAllocationReceipt,
        source: super::UiAdmittedScrollExtentSource,
    ) -> Self {
        Self {
            source,
            receipt_identity: receipt.identity().clone(),
            generation: receipt.generation(),
            equivalence_basis: receipt.equivalence_basis().clone(),
            virtualization: super::UiScrollVirtualizationPosture::NonVirtualized,
            offset_allocation: super::UiScrollOffsetAllocationPosture::ProjectedInteractionOnly,
        }
    }

    pub fn planning_evidence_digest(&self) -> u64 {
        self.generation.planning_evidence_digest()
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.source.identity_digest()
            ^ self.receipt_identity.identity_digest().rotate_left(7)
            ^ self.generation.identity_digest().rotate_left(19)
            ^ self.equivalence_basis.identity_digest().rotate_left(31)
    }

    pub(crate) fn receipt_identity(&self) -> &crate::runtime::UiAllocationReceiptIdentity {
        &self.receipt_identity
    }

    pub(crate) fn source(&self) -> &super::UiAdmittedScrollExtentSource {
        &self.source
    }

    pub(crate) fn mismatch_denial(
        &self,
        requested: &Self,
    ) -> crate::runtime::UiScrollOwnerAcquisitionDenial {
        if self.source != requested.source {
            crate::runtime::UiScrollOwnerAcquisitionDenial::ContradictorySource
        } else if self.receipt_identity != requested.receipt_identity {
            crate::runtime::UiScrollOwnerAcquisitionDenial::ReceiptNotActive
        } else if self.generation != requested.generation {
            crate::runtime::UiScrollOwnerAcquisitionDenial::ReceiptGenerationMismatch
        } else {
            crate::runtime::UiScrollOwnerAcquisitionDenial::ReceiptEquivalenceMismatch
        }
    }
}
