use worth_store_physical_backend::BackendDurabilityProfile;
use worth_store_physical_format::PhysicalRootReference;

use super::{
    CheckpointId, CheckpointManifest, CheckpointRecoveryCounterSnapshot, CheckpointRedoBoundary,
    CheckpointValidationDenial, CheckpointValidationDenialKind,
};
use crate::DurableAckReceipt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointCandidateDiscoverySource {
    DirectoryListing,
    BackendResidue,
    OrphanedManifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointCandidate {
    manifest: CheckpointManifest,
    discovery_source: CheckpointCandidateDiscoverySource,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl CheckpointCandidate {
    pub fn from_manifest(
        manifest: CheckpointManifest,
        discovery_source: CheckpointCandidateDiscoverySource,
    ) -> Self {
        Self {
            manifest,
            discovery_source,
            counters: CheckpointRecoveryCounterSnapshot::new().with_candidate(),
        }
    }

    pub fn manifest(&self) -> &CheckpointManifest {
        &self.manifest
    }

    pub const fn discovery_source(&self) -> CheckpointCandidateDiscoverySource {
        self.discovery_source
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableRootSelector {
    checkpoint_id: CheckpointId,
    root_reference: PhysicalRootReference,
}

impl DurableRootSelector {
    pub(crate) fn new(checkpoint_id: CheckpointId, root_reference: PhysicalRootReference) -> Self {
        Self {
            checkpoint_id,
            root_reference,
        }
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn root_reference(&self) -> PhysicalRootReference {
        self.root_reference
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuperblockRingCheckpointPointer {
    checkpoint_id: CheckpointId,
    ring_slot: u8,
}

impl SuperblockRingCheckpointPointer {
    pub(crate) fn new(checkpoint_id: CheckpointId, ring_slot: u8) -> Self {
        Self {
            checkpoint_id,
            ring_slot,
        }
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn ring_slot(&self) -> u8 {
        self.ring_slot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckpointLocator {
    DurableRootSelector(DurableRootSelector),
    SuperblockRingCheckpointPointer(SuperblockRingCheckpointPointer),
    ManifestPointer(CheckpointId),
}

impl CheckpointLocator {
    pub fn checkpoint_id(&self) -> &CheckpointId {
        match self {
            Self::DurableRootSelector(selector) => selector.checkpoint_id(),
            Self::SuperblockRingCheckpointPointer(pointer) => pointer.checkpoint_id(),
            Self::ManifestPointer(checkpoint_id) => checkpoint_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointLocatorArtifactCommitment {
    locator: CheckpointLocator,
    redo_boundary: CheckpointRedoBoundary,
    digest: String,
}

impl CheckpointLocatorArtifactCommitment {
    pub fn durable_root_selector(manifest: &CheckpointManifest) -> Self {
        let root_reference = manifest
            .root_posture()
            .root_reference()
            .expect("checkpoint manifest validation already requires root");
        let locator = CheckpointLocator::DurableRootSelector(DurableRootSelector::new(
            manifest.checkpoint_id().clone(),
            root_reference,
        ));
        Self::new(manifest, locator)
    }

    pub fn superblock_ring_pointer(manifest: &CheckpointManifest, ring_slot: u8) -> Self {
        let locator = CheckpointLocator::SuperblockRingCheckpointPointer(
            SuperblockRingCheckpointPointer::new(manifest.checkpoint_id().clone(), ring_slot),
        );
        Self::new(manifest, locator)
    }

    pub fn manifest_pointer(manifest: &CheckpointManifest) -> Self {
        let locator = CheckpointLocator::ManifestPointer(manifest.checkpoint_id().clone());
        Self::new(manifest, locator)
    }

    fn new(manifest: &CheckpointManifest, locator: CheckpointLocator) -> Self {
        let digest = format!(
            "s4-checkpoint-locator:{}:{locator:?}",
            manifest.checkpoint_id().digest().as_str()
        );
        Self {
            locator,
            redo_boundary: manifest.redo_boundary(),
            digest,
        }
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        self.locator.checkpoint_id()
    }

    pub const fn redo_boundary(&self) -> CheckpointRedoBoundary {
        self.redo_boundary
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreOwnedCheckpointLocator {
    locator: CheckpointLocator,
}

impl StoreOwnedCheckpointLocator {
    pub fn admit<P: BackendDurabilityProfile>(
        commitment: CheckpointLocatorArtifactCommitment,
        ack: &DurableAckReceipt<P>,
    ) -> Result<Self, CheckpointValidationDenial> {
        if !ack
            .ack_basis()
            .lsn_range()
            .contains(commitment.redo_boundary().lsn())
        {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::CutoverDurabilityRangeMismatch,
                CheckpointRecoveryCounterSnapshot::new().with_locator_check(),
            )
            .with_lsn_pair(
                commitment.redo_boundary().lsn(),
                ack.ack_basis().lsn_range().start(),
            ));
        }
        if ack.ack_basis().frame_digest().as_str() != commitment.digest() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::CutoverDurabilityArtifactMismatch,
                CheckpointRecoveryCounterSnapshot::new().with_locator_check(),
            ));
        }
        Ok(Self {
            locator: commitment.locator,
        })
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        self.locator.checkpoint_id()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointSelectorEvidence {
    locator: CheckpointLocator,
}

impl CheckpointSelectorEvidence {
    pub fn from_store_owned_locator(locator: StoreOwnedCheckpointLocator) -> Self {
        Self {
            locator: locator.locator,
        }
    }

    pub fn bind_candidate(
        self,
        candidate: CheckpointCandidate,
    ) -> Result<LocatedCheckpointCandidate, CheckpointValidationDenial> {
        let counters = candidate.counters().with_locator_check();
        if self.locator.checkpoint_id() != candidate.manifest().checkpoint_id() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::LocatorCheckpointMismatch,
                counters,
            ));
        }
        Ok(LocatedCheckpointCandidate {
            candidate,
            locator: self.locator,
            counters,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatedCheckpointCandidate {
    candidate: CheckpointCandidate,
    locator: CheckpointLocator,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl LocatedCheckpointCandidate {
    #[cfg(feature = "certification-test-authority")]
    pub fn from_manifest_for_certification_test(manifest: CheckpointManifest) -> Self {
        let candidate = CheckpointCandidate::from_manifest(
            manifest,
            CheckpointCandidateDiscoverySource::DirectoryListing,
        );
        let locator =
            CheckpointLocator::ManifestPointer(candidate.manifest().checkpoint_id().clone());
        let counters = candidate.counters().with_locator_check();
        Self {
            candidate,
            locator,
            counters,
        }
    }

    pub fn candidate(&self) -> &CheckpointCandidate {
        &self.candidate
    }

    pub fn locator(&self) -> &CheckpointLocator {
        &self.locator
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}
