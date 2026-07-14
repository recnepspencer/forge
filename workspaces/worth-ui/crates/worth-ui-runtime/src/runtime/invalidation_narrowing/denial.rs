#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInvalidationNarrowingDenial {
    CardinalityExhausted,
    OrdinalExhausted,
    SourceCardinalityMismatch {
        invalidations: u16,
        sources: u16,
    },
    SourceFamilyMismatch {
        ordinal: u16,
    },
    QuerySettlementFamilyMissing {
        ordinal: u16,
    },
    GraphTargetNotAdmitted {
        ordinal: u16,
    },
    HostMeasurementTargetNotAdmitted {
        ordinal: u16,
    },
    QueryTargetNotAdmitted {
        ordinal: u16,
    },
    DurableResizeTargetNotAdmitted {
        ordinal: u16,
    },
    HostEvidenceGenerationMismatch {
        ordinal: u16,
    },
    HostNormalizationAuthorityMismatch {
        ordinal: u16,
    },
    PortalAnchorNotAdmitted {
        ordinal: u16,
    },
    PortalAnchorObservationInvalid {
        ordinal: u16,
    },
    PortalAnchorEvidenceStale {
        ordinal: u16,
    },
    PortalAnchorSuccessorBasisDenied {
        ordinal: u16,
    },
    ScrollOwnershipNotAdmitted {
        ordinal: u16,
    },
    ContradictoryScrollOwnership {
        ordinal: u16,
    },
    ViewportTargetBudgetExceeded {
        ordinal: u16,
        attempted: u16,
        maximum: u16,
    },
    DragResizeTargetBudgetExceeded {
        ordinal: u16,
        attempted: u16,
        maximum: u16,
    },
    QueryBasisMismatch {
        ordinal: u16,
    },
    QueryContractMismatch {
        ordinal: u16,
    },
    QuerySourceGenerationMismatch {
        ordinal: u16,
    },
    QuerySourceOrderMismatch {
        ordinal: u16,
    },
    QueryConsumptionReceiptMismatch {
        ordinal: u16,
    },
    QueryExtentUnordered {
        ordinal: u16,
    },
    AuthorityCounterExhausted {
        ordinal: u16,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiAllocationInvalidationNarrowingRejection {
    frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    denial: UiAllocationInvalidationNarrowingDenial,
    counters: super::UiAllocationInvalidationNarrowingCounters,
}

impl UiAllocationInvalidationNarrowingRejection {
    pub(super) fn new(
        frame_epoch: crate::runtime::UiAllocationFrameEpoch,
        denial: UiAllocationInvalidationNarrowingDenial,
        counters: super::UiAllocationInvalidationNarrowingCounters,
    ) -> Self {
        Self {
            frame_epoch,
            denial,
            counters,
        }
    }
    pub fn frame_epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.frame_epoch
    }
    pub fn denial(&self) -> UiAllocationInvalidationNarrowingDenial {
        self.denial
    }
    pub fn counters(&self) -> super::UiAllocationInvalidationNarrowingCounters {
        self.counters
    }
}

pub(super) fn map_host_lookup_denial(
    denial: super::authority::UiInvalidationAuthorityLookupDenial,
    ordinal: u16,
) -> UiAllocationInvalidationNarrowingDenial {
    match denial {
        super::authority::UiInvalidationAuthorityLookupDenial::HostEvidenceGenerationMismatch =>
            UiAllocationInvalidationNarrowingDenial::HostEvidenceGenerationMismatch { ordinal },
        super::authority::UiInvalidationAuthorityLookupDenial::HostNormalizationAuthorityMismatch =>
            UiAllocationInvalidationNarrowingDenial::HostNormalizationAuthorityMismatch { ordinal },
        super::authority::UiInvalidationAuthorityLookupDenial::AuthorityCounterExhausted =>
            UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal },
    }
}
