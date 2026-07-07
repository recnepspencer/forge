use crate::{
    ManifestDiscoveryCounterSnapshot, PhysicalReference, PhysicalReferenceValidationDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestDiscoveryDenialKind {
    BackendResidueDiscoverySource,
    MissingSegmentManifestMembership,
    MissingPageSlotManifestMembership,
    MissingExtentManifestMembership,
    MissingFreeSpaceManifestMembership,
    ReferenceValidationDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestDiscoveryDenial {
    kind: ManifestDiscoveryDenialKind,
    reference: Option<PhysicalReference>,
    counters: ManifestDiscoveryCounterSnapshot,
    reference_denial: Option<PhysicalReferenceValidationDenial>,
}

impl ManifestDiscoveryDenial {
    pub(crate) const fn new(
        kind: ManifestDiscoveryDenialKind,
        reference: Option<PhysicalReference>,
        counters: ManifestDiscoveryCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            reference,
            counters,
            reference_denial: None,
        }
    }

    pub(crate) const fn reference_validation_denied(
        denial: PhysicalReferenceValidationDenial,
        counters: ManifestDiscoveryCounterSnapshot,
    ) -> Self {
        Self {
            kind: ManifestDiscoveryDenialKind::ReferenceValidationDenied,
            reference: Some(denial.reference()),
            counters,
            reference_denial: Some(denial),
        }
    }

    pub const fn kind(self) -> ManifestDiscoveryDenialKind {
        self.kind
    }

    pub const fn reference(self) -> Option<PhysicalReference> {
        self.reference
    }

    pub const fn counters(self) -> ManifestDiscoveryCounterSnapshot {
        self.counters
    }

    pub const fn reference_denial(self) -> Option<PhysicalReferenceValidationDenial> {
        self.reference_denial
    }
}
