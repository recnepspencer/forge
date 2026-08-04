use std::marker::PhantomData;

use worth_store_physical_backend::{BackendDurabilityProfile, BackendDurabilityProfileId};
use worth_store_physical_format::PhysicalReference;

use crate::{
    WalDurabilityObservation, WalDurabilityObservationBasis, WalSegmentGeneration, WalSegmentId,
};

use super::{
    CheckpointId, CheckpointRecoveryCounterSnapshot, CheckpointValidation,
    CheckpointValidationDenial, CheckpointValidationDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointDurabilityRole {
    Manifest,
    Root,
    PageLsnFrontier,
    Locator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointArtifactDurabilityCommitment {
    checkpoint_id: CheckpointId,
    role: CheckpointDurabilityRole,
    root_reference: PhysicalReference,
    digest: String,
}

impl CheckpointArtifactDurabilityCommitment {
    pub fn manifest(validation: &CheckpointValidation) -> Self {
        Self::new(validation, CheckpointDurabilityRole::Manifest)
    }

    pub fn root(validation: &CheckpointValidation) -> Self {
        Self::new(validation, CheckpointDurabilityRole::Root)
    }

    pub fn page_lsn_frontier(validation: &CheckpointValidation) -> Self {
        Self::new(validation, CheckpointDurabilityRole::PageLsnFrontier)
    }

    pub fn locator(validation: &CheckpointValidation) -> Self {
        Self::new(validation, CheckpointDurabilityRole::Locator)
    }

    fn new(validation: &CheckpointValidation, role: CheckpointDurabilityRole) -> Self {
        let checkpoint_id = validation.checkpoint_id().clone();
        let root_reference = validation
            .manifest()
            .root_posture()
            .root_reference()
            .expect("checkpoint validation requires a present root");
        let digest = format!(
            "s4-checkpoint-artifact:{}:{role:?}:{:?}:{:?}",
            checkpoint_id.digest().as_str(),
            validation.manifest().root_posture(),
            validation.locator()
        );
        Self {
            checkpoint_id,
            role,
            root_reference,
            digest,
        }
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn role(&self) -> CheckpointDurabilityRole {
        self.role
    }

    pub const fn root_reference(&self) -> PhysicalReference {
        self.root_reference
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointDurabilityEvidence<P: BackendDurabilityProfile> {
    profile: PhantomData<P>,
    commitment: CheckpointArtifactDurabilityCommitment,
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    basis: WalDurabilityObservationBasis,
}

impl<P: BackendDurabilityProfile> CheckpointDurabilityEvidence<P> {
    fn admit(
        validation: &CheckpointValidation,
        commitment: CheckpointArtifactDurabilityCommitment,
        ack: &WalDurabilityObservation<P>,
    ) -> Result<Self, CheckpointValidationDenial> {
        let counters = validation.counters().with_cutover_decision();
        if commitment.checkpoint_id() != validation.checkpoint_id() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::CutoverDurabilityCheckpointMismatch,
                counters,
            ));
        }
        if ack.profile_id() != P::ID {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::CutoverDurabilityProfileMismatch,
                counters,
            )
            .with_profile_id(ack.profile_id()));
        }
        let redo_lsn = validation.manifest().redo_boundary().lsn();
        if !ack.basis().lsn_range().contains(redo_lsn) {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::CutoverDurabilityRangeMismatch,
                counters,
            )
            .with_lsn_pair(redo_lsn, ack.basis().lsn_range().start()));
        }
        if ack.basis().frame_digest().as_str() != commitment.digest() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::CutoverDurabilityArtifactMismatch,
                counters,
            ));
        }
        Ok(Self {
            profile: PhantomData,
            commitment,
            segment_id: ack.basis().segment_id(),
            generation: ack.basis().generation(),
            basis: ack.basis().clone(),
        })
    }

    pub const fn role(&self) -> CheckpointDurabilityRole {
        self.commitment.role()
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        P::ID
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        self.commitment.checkpoint_id()
    }

    pub const fn root_reference(&self) -> PhysicalReference {
        self.commitment.root_reference()
    }

    fn same_physical_ack(&self, other: &Self) -> bool {
        self.segment_id == other.segment_id
            && self.generation == other.generation
            && self.basis.lsn_range() == other.basis.lsn_range()
            && self.basis.frame_digest() == other.basis.frame_digest()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointDurabilityEvidenceSet<P: BackendDurabilityProfile> {
    profile: PhantomData<P>,
    checkpoint_id: CheckpointId,
    manifest: CheckpointDurabilityEvidence<P>,
    root: CheckpointDurabilityEvidence<P>,
    page_lsn_frontier: CheckpointDurabilityEvidence<P>,
    locator: CheckpointDurabilityEvidence<P>,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl<P: BackendDurabilityProfile> CheckpointDurabilityEvidenceSet<P> {
    pub fn admit(
        validation: &CheckpointValidation,
        manifest_ack: &WalDurabilityObservation<P>,
        root_ack: &WalDurabilityObservation<P>,
        page_lsn_frontier_ack: &WalDurabilityObservation<P>,
        locator_ack: &WalDurabilityObservation<P>,
    ) -> Result<Self, CheckpointValidationDenial> {
        let manifest = CheckpointDurabilityEvidence::admit(
            validation,
            CheckpointArtifactDurabilityCommitment::manifest(validation),
            manifest_ack,
        )?;
        let root = CheckpointDurabilityEvidence::admit(
            validation,
            CheckpointArtifactDurabilityCommitment::root(validation),
            root_ack,
        )?;
        let page_lsn_frontier = CheckpointDurabilityEvidence::admit(
            validation,
            CheckpointArtifactDurabilityCommitment::page_lsn_frontier(validation),
            page_lsn_frontier_ack,
        )?;
        let locator = CheckpointDurabilityEvidence::admit(
            validation,
            CheckpointArtifactDurabilityCommitment::locator(validation),
            locator_ack,
        )?;
        let counters = validation.counters().with_cutover_decision();
        let proofs = [&manifest, &root, &page_lsn_frontier, &locator];
        for left in 0..proofs.len() {
            for right in (left + 1)..proofs.len() {
                if proofs[left].same_physical_ack(proofs[right]) {
                    return Err(CheckpointValidationDenial::new(
                        CheckpointValidationDenialKind::CutoverDurabilityRoleReuse,
                        counters,
                    ));
                }
            }
        }
        Ok(Self {
            profile: PhantomData,
            checkpoint_id: validation.checkpoint_id().clone(),
            manifest,
            root,
            page_lsn_frontier,
            locator,
            counters,
        })
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        P::ID
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }

    pub const fn manifest(&self) -> &CheckpointDurabilityEvidence<P> {
        &self.manifest
    }

    pub const fn root(&self) -> &CheckpointDurabilityEvidence<P> {
        &self.root
    }

    pub const fn page_lsn_frontier(&self) -> &CheckpointDurabilityEvidence<P> {
        &self.page_lsn_frontier
    }

    pub const fn locator(&self) -> &CheckpointDurabilityEvidence<P> {
        &self.locator
    }
}
