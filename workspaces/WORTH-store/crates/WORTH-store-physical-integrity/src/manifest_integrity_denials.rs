use crate::ManifestIntegrityCounters;
use worth_store_physical_format::{PhysicalGenerationOwner, RootManifestIntegrityPosture};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestIntegrityDenialKind {
    MissingRootPage,
    DamagedRoot,
    TornRootPointer,
    MultipleValidRoots,
    RootGenerationMismatch,
    ResidueRootRejected,
    RecoveryBlockingRootDamage,
    StaleManifestGeneration,
    WrongSegmentId,
    MismatchedExtentId,
    DamagedAllocationMap,
    BackendResidueFallback,
    SourcePrecedenceViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestIntegrityDenial {
    kind: ManifestIntegrityDenialKind,
    posture: RootManifestIntegrityPosture,
    locality: Option<PhysicalGenerationOwner>,
    counters: ManifestIntegrityCounters,
}

impl ManifestIntegrityDenial {
    pub(crate) const fn new(
        kind: ManifestIntegrityDenialKind,
        posture: RootManifestIntegrityPosture,
        counters: ManifestIntegrityCounters,
    ) -> Self {
        Self {
            kind,
            posture,
            locality: None,
            counters,
        }
    }

    pub(crate) const fn with_locality(mut self, locality: PhysicalGenerationOwner) -> Self {
        self.locality = Some(locality);
        self
    }

    pub const fn kind(&self) -> ManifestIntegrityDenialKind {
        self.kind
    }

    pub const fn posture(&self) -> RootManifestIntegrityPosture {
        self.posture
    }

    pub const fn locality(&self) -> Option<PhysicalGenerationOwner> {
        self.locality
    }

    pub const fn counters(&self) -> ManifestIntegrityCounters {
        self.counters
    }
}

pub type ManifestReferenceMismatchDenial = ManifestIntegrityDenial;
pub type ManifestGenerationMismatchDenial = ManifestIntegrityDenial;
