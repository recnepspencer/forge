use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, NoProofs, PhaseMarker,
};
use worth_store_authority::StoreCurrentAuthorityWitness;

use crate::{
    DurabilityReplayIdentity, DurabilityReplayKind, DurableCheckpointPublication,
    DurableManifestPublication, PartialPublicationClassification,
    RecoveredOrRejectedPartialPublication,
};

use super::{BlobReplayAdmissionDenial, BlobReplayAdmissionDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobReplaySourceKind {
    Wal,
    Checkpoint,
    Manifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobReplaySourceAdmission {
    kind: BlobReplaySourceKind,
    source_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobResumeReplayReadmission {
    readmitted_checkpoint_source: ReadmittedCheckpointSourceArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BlobResumeReplayReadmissionPhase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BlobResumeReplayReadmissionAuthority;

type ReadmittedCheckpointSourceArtifact = Artifact<
    BlobResumeReplayReadmissionPhase,
    String,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<String>>,
>;

impl PhaseMarker for BlobResumeReplayReadmissionPhase {}

impl AuthorityMarker for BlobResumeReplayReadmissionAuthority {}

impl BlobReplaySourceAdmission {
    pub fn from_replayable_wal_classification(
        classification: &PartialPublicationClassification,
    ) -> Result<Self, BlobReplayAdmissionDenial> {
        if !matches!(
            classification.recovered_or_rejected(),
            RecoveredOrRejectedPartialPublication::ReplayableUnacknowledgedWal { .. }
        ) {
            return Err(BlobReplayAdmissionDenial::new(
                BlobReplayAdmissionDenialKind::MissingWalSource,
                Some(classification.classification_digest().to_owned()),
            ));
        }
        admit_replay_source(
            BlobReplaySourceKind::Wal,
            classification.classification_digest(),
            BlobReplayAdmissionDenialKind::MissingWalSource,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn from_checkpoint_replay_identity(
        identity: &DurabilityReplayIdentity,
    ) -> Result<Self, BlobReplayAdmissionDenial> {
        admit_checkpoint_replay_source(identity)
    }

    pub fn from_durable_checkpoint_publication(
        publication: &DurableCheckpointPublication,
    ) -> Result<Self, BlobReplayAdmissionDenial> {
        admit_checkpoint_replay_source(publication.replay_identity())
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn from_manifest_replay_identity(
        identity: &DurabilityReplayIdentity,
    ) -> Result<Self, BlobReplayAdmissionDenial> {
        admit_manifest_replay_source(identity)
    }

    pub fn from_durable_manifest_publication(
        publication: &DurableManifestPublication,
    ) -> Result<Self, BlobReplayAdmissionDenial> {
        admit_manifest_replay_source(publication.replay_identity())
    }

    pub fn reject_backend_residue(digest: impl Into<String>) -> BlobReplayAdmissionDenial {
        BlobReplayAdmissionDenial::new(
            BlobReplayAdmissionDenialKind::BackendResidueRejected,
            Some(digest.into()),
        )
    }

    pub const fn kind(&self) -> BlobReplaySourceKind {
        self.kind
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }
}

impl BlobResumeReplayReadmission {
    pub fn from_checkpoint_source(
        source: &BlobReplaySourceAdmission,
        current_store_authority: StoreCurrentAuthorityWitness,
    ) -> Result<Self, BlobReplayAdmissionDenial> {
        let current_store_authority_digest = current_store_authority
            .identity()
            .aspect_key()
            .as_str()
            .to_owned();
        if source.kind() != BlobReplaySourceKind::Checkpoint {
            return Err(BlobReplayAdmissionDenial::new(
                BlobReplayAdmissionDenialKind::WrongReplaySourceForResumeSession,
                Some(source.source_digest().to_owned()),
            ));
        }
        if let Err(denial) = crate::verify_store_authority_for_readmission(&current_store_authority)
        {
            return Err(denial);
        }
        let authority =
            AuthorityWitness::from_authority_marker(BlobResumeReplayReadmissionAuthority);
        let readmitted_checkpoint_source =
            Artifact::<BlobResumeReplayReadmissionPhase, _, _, _>::with_current_basis(
                source.source_digest().to_owned(),
                source.source_digest().to_owned(),
                authority,
            )
            .bridge_trust_boundary()
            .readmit_with_authority(
                current_store_authority_digest,
                AuthorityWitness::from_authority_marker(BlobResumeReplayReadmissionAuthority),
            );
        Ok(Self {
            readmitted_checkpoint_source,
        })
    }

    pub fn checkpoint_source_digest(&self) -> &str {
        self.readmitted_checkpoint_source.payload().as_str()
    }

    pub fn current_store_authority_digest(&self) -> &str {
        self.readmitted_checkpoint_source
            .basis()
            .basis()
            .value()
            .as_str()
    }
}

fn admit_checkpoint_replay_source(
    identity: &DurabilityReplayIdentity,
) -> Result<BlobReplaySourceAdmission, BlobReplayAdmissionDenial> {
    admit_durability_replay_source(
        identity,
        DurabilityReplayKind::Checkpoint,
        BlobReplaySourceKind::Checkpoint,
        BlobReplayAdmissionDenialKind::MissingCheckpointSource,
    )
}

fn admit_manifest_replay_source(
    identity: &DurabilityReplayIdentity,
) -> Result<BlobReplaySourceAdmission, BlobReplayAdmissionDenial> {
    admit_durability_replay_source(
        identity,
        DurabilityReplayKind::Manifest,
        BlobReplaySourceKind::Manifest,
        BlobReplayAdmissionDenialKind::MissingManifestSource,
    )
}

fn admit_durability_replay_source(
    identity: &DurabilityReplayIdentity,
    expected_replay_kind: DurabilityReplayKind,
    blob_source_kind: BlobReplaySourceKind,
    denial: BlobReplayAdmissionDenialKind,
) -> Result<BlobReplaySourceAdmission, BlobReplayAdmissionDenial> {
    if identity.kind() != expected_replay_kind || identity.digest().is_empty() {
        return Err(BlobReplayAdmissionDenial::new(
            denial,
            Some(identity.digest().to_owned()),
        ));
    }
    admit_replay_source(blob_source_kind, identity.digest(), denial)
}

fn admit_replay_source(
    kind: BlobReplaySourceKind,
    source_digest: &str,
    denial: BlobReplayAdmissionDenialKind,
) -> Result<BlobReplaySourceAdmission, BlobReplayAdmissionDenial> {
    if source_digest.is_empty() {
        return Err(BlobReplayAdmissionDenial::new(denial, None));
    }
    Ok(BlobReplaySourceAdmission {
        kind,
        source_digest: source_digest.to_owned(),
    })
}
