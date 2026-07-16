#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAdmittedScrollInvalidationBinding {
    contract: crate::runtime::UiAdmittedScrollOwnedContract,
    target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
    cause: crate::evidence::UiScrollOwnedExtentCause,
    authority_probes: u16,
    receipt_key: Option<crate::runtime::UiScrollReceiptActivationKey>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiScrollInvalidationBindingDenial {
    ContradictorySource,
    MissingReceiptBinding,
    DuplicateReceiptBinding,
    ReceiptContextMismatch,
    MissingSourceBinding,
    MissingGraphTarget,
    SourceCauseMismatch,
    GraphGenerationMismatch,
    NeighborhoodIdentityMismatch,
    ConflictingContractTarget,
    ConflictingOwnerCapability,
    AuthorityCounterExhausted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiScrollOwnerAcquisitionDenial {
    OwnerNotActive,
    AmbiguousOwner,
    ReceiptNotActive,
    ReceiptGenerationMismatch,
    ReceiptEquivalenceMismatch,
    SourceNotAdmitted,
    ContradictorySource,
    AuthorityCounterExhausted,
}

impl UiAdmittedScrollInvalidationBinding {
    pub(super) fn seal(
        contract: crate::runtime::UiAdmittedScrollOwnedContract,
        target: crate::graph::UiAdmittedAllocationInvalidationTargetSet,
        cause: crate::evidence::UiScrollOwnedExtentCause,
        authority_probes: u16,
        receipt: Option<&crate::runtime::UiAllocationReceipt>,
    ) -> Result<Self, UiScrollInvalidationBindingDenial> {
        let source_matches = matches!(
            (contract.source(), cause),
            (
                crate::runtime::UiAdmittedScrollExtentSource::HostViewport { .. },
                crate::evidence::UiScrollOwnedExtentCause::HostContainerViewport
            ) | (
                crate::runtime::UiAdmittedScrollExtentSource::QueryContent(_),
                crate::evidence::UiScrollOwnedExtentCause::QueryContentExtent
            )
        );
        if !source_matches {
            return Err(UiScrollInvalidationBindingDenial::SourceCauseMismatch);
        }
        if contract.graph_generation() != target.primary().graph_generation() {
            return Err(UiScrollInvalidationBindingDenial::GraphGenerationMismatch);
        }
        if contract.neighborhood_identity() != target.primary().neighborhood_identity() {
            return Err(UiScrollInvalidationBindingDenial::NeighborhoodIdentityMismatch);
        }
        Ok(Self {
            receipt_key: receipt.map(|receipt| {
                crate::runtime::UiScrollReceiptActivationKey::from_receipt_and_source(
                    receipt,
                    contract.source().clone(),
                )
            }),
            contract,
            target,
            cause,
            authority_probes,
        })
    }

    pub(crate) fn contract(&self) -> &crate::runtime::UiAdmittedScrollOwnedContract {
        &self.contract
    }
    pub(crate) fn target(&self) -> &crate::graph::UiAdmittedAllocationInvalidationTargetSet {
        &self.target
    }
    pub(crate) fn cause(&self) -> crate::evidence::UiScrollOwnedExtentCause {
        self.cause
    }
    pub(crate) fn authority_probes(&self) -> u16 {
        self.authority_probes
    }
    pub(crate) fn receipt_key(&self) -> Option<&crate::runtime::UiScrollReceiptActivationKey> {
        self.receipt_key.as_ref()
    }
}
