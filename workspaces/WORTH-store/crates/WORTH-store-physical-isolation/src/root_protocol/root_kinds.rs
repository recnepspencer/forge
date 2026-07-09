use crate::{
    CurrentGenerationPhysicalReference, ExtentPublicationEpochBasis,
    FutureChunkPublicationEpochBasis, GenerationCountedReferenceDenial, ManifestEpoch,
    PagePublicationEpochBasis, PhysicalOrderingContract, PhysicalOrderingContractDenial,
    PhysicalOrderingSite, RootEpoch, SegmentPublicationEpochBasis,
};
use worth_store_recovery_physics::CheckpointId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentPhysicalRoot {
    epoch: RootEpoch,
    manifest_epoch: ManifestEpoch,
    ordering: PhysicalOrderingContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointPublicationRoot {
    epoch: RootEpoch,
    ordering: PhysicalOrderingContract,
    checkpoint_identity: CheckpointPublicationIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryRoot {
    epoch: RootEpoch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestLocatorRoot {
    epoch: RootEpoch,
}

#[derive(Debug, Clone, Copy)]
pub struct CurrentPhysicalRootBasis {
    root_epoch: RootEpoch,
    manifest_epoch: ManifestEpoch,
}

#[derive(Debug, Clone, Copy)]
pub struct CheckpointPublicationRootBasis {
    epoch: RootEpoch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointPublicationIdentity {
    digest: String,
}

#[derive(Debug, Clone, Copy)]
pub struct RecoveryRootBasis {
    epoch: RootEpoch,
}

#[derive(Debug, Clone, Copy)]
pub struct ManifestLocatorRootBasis {
    epoch: RootEpoch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootKindMismatchDenial {
    CheckpointPublicationRootCannotAdmitCurrentReadPlan,
    RecoveryRootRequiresEntryReadmission,
    ManifestLocatorRootCannotAdmitCurrentReadPlan,
}

impl CurrentPhysicalRoot {
    pub fn from_s5_entry(
        basis: CurrentPhysicalRootBasis,
        ordering: PhysicalOrderingContract,
    ) -> Result<Self, PhysicalOrderingContractDenial> {
        let ordering = ordering.require_site(PhysicalOrderingSite::RootSwap)?;
        Ok(Self {
            epoch: basis.root_epoch,
            manifest_epoch: basis.manifest_epoch,
            ordering,
        })
    }

    pub const fn epoch(self) -> RootEpoch {
        self.epoch
    }

    pub const fn manifest_epoch(self) -> ManifestEpoch {
        self.manifest_epoch
    }

    pub const fn ordering(self) -> PhysicalOrderingContract {
        self.ordering
    }

    pub const fn scope(self) -> u64 {
        self.epoch.get()
    }

    pub fn admit_segment_publication_epoch(
        self,
        reference: CurrentGenerationPhysicalReference,
    ) -> Result<SegmentPublicationEpochBasis, GenerationCountedReferenceDenial> {
        SegmentPublicationEpochBasis::admit(self.scope(), reference)
    }

    pub fn admit_extent_publication_epoch(
        self,
        reference: CurrentGenerationPhysicalReference,
    ) -> Result<ExtentPublicationEpochBasis, GenerationCountedReferenceDenial> {
        ExtentPublicationEpochBasis::admit(self.scope(), reference)
    }

    pub fn admit_page_publication_epoch(
        self,
        reference: CurrentGenerationPhysicalReference,
    ) -> Result<PagePublicationEpochBasis, GenerationCountedReferenceDenial> {
        PagePublicationEpochBasis::admit(self.scope(), reference)
    }

    pub const fn future_chunk_publication_epoch_placeholder(
        self,
    ) -> FutureChunkPublicationEpochBasis {
        FutureChunkPublicationEpochBasis::s7_placeholder(self.scope())
    }
}

impl CheckpointPublicationRoot {
    pub fn from_checkpoint_publication(
        basis: CheckpointPublicationRootBasis,
        ordering: PhysicalOrderingContract,
        checkpoint_identity: CheckpointPublicationIdentity,
    ) -> Result<Self, PhysicalOrderingContractDenial> {
        let ordering = ordering.require_site(PhysicalOrderingSite::RootSwap)?;
        Ok(Self {
            epoch: basis.epoch,
            ordering,
            checkpoint_identity,
        })
    }

    pub const fn epoch(&self) -> RootEpoch {
        self.epoch
    }

    pub const fn ordering(&self) -> PhysicalOrderingContract {
        self.ordering
    }

    pub const fn checkpoint_identity(&self) -> &CheckpointPublicationIdentity {
        &self.checkpoint_identity
    }
}

impl CheckpointPublicationIdentity {
    pub fn from_checkpoint_id(checkpoint_id: &CheckpointId) -> Self {
        Self {
            digest: checkpoint_id.digest().as_str().to_owned(),
        }
    }

    pub fn matches_checkpoint_id(&self, checkpoint_id: &CheckpointId) -> bool {
        self.digest == checkpoint_id.digest().as_str()
    }
}

impl RecoveryRoot {
    pub const fn from_recovery_basis(basis: RecoveryRootBasis) -> Self {
        Self { epoch: basis.epoch }
    }

    pub const fn epoch(self) -> RootEpoch {
        self.epoch
    }
}

impl ManifestLocatorRoot {
    pub const fn from_manifest_locator_basis(basis: ManifestLocatorRootBasis) -> Self {
        Self { epoch: basis.epoch }
    }

    pub const fn epoch(self) -> RootEpoch {
        self.epoch
    }
}

impl CurrentPhysicalRootBasis {
    pub(crate) const fn new(root_epoch: RootEpoch, manifest_epoch: ManifestEpoch) -> Self {
        Self {
            root_epoch,
            manifest_epoch,
        }
    }

    pub const fn root_epoch(self) -> RootEpoch {
        self.root_epoch
    }

    pub const fn manifest_epoch(self) -> ManifestEpoch {
        self.manifest_epoch
    }
}

impl CheckpointPublicationRootBasis {
    pub(crate) const fn new(epoch: RootEpoch) -> Self {
        Self { epoch }
    }

    pub const fn epoch(self) -> RootEpoch {
        self.epoch
    }
}

impl RecoveryRootBasis {
    pub(crate) const fn new(epoch: RootEpoch) -> Self {
        Self { epoch }
    }

    pub const fn epoch(self) -> RootEpoch {
        self.epoch
    }
}

impl ManifestLocatorRootBasis {
    pub(crate) const fn new(epoch: RootEpoch) -> Self {
        Self { epoch }
    }

    pub const fn epoch(self) -> RootEpoch {
        self.epoch
    }
}
