mod entries;
mod observation_entries;

use worth_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisEntry,
};
use worth_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalSingleSequenceDigestAlgorithmSlot,
};
use worth_proof::TransitionOutcome;

use super::{ExecutedTranscriptParts, TranscriptReplayDenial};
use entries::{replay_basis_entries, transcript_entries, transcript_version, TRANSCRIPT_DOMAIN};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationTranscriptIdentity {
    digest: CanonicalDerivedDigest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptReplayEvidenceIdentity {
    digest: CanonicalDerivedDigest,
}

pub type SimulationRunIdentity = PhysicalSimulationTranscriptIdentity;

impl PhysicalSimulationTranscriptIdentity {
    pub(crate) fn from_parts(
        parts: &ExecutedTranscriptParts,
    ) -> Result<Self, TranscriptReplayDenial> {
        Ok(Self {
            digest: derive_transcript_digest(transcript_entries(parts))?,
        })
    }

    pub fn digest_bytes(&self) -> &[u8; 32] {
        self.digest.value().bytes()
    }

    pub fn canonical_basis_entry_count(&self) -> u32 {
        self.digest.metadata().entry_count()
    }
}

impl TranscriptReplayEvidenceIdentity {
    pub(crate) fn from_parts(
        parts: &ExecutedTranscriptParts,
    ) -> Result<Self, TranscriptReplayDenial> {
        Ok(Self {
            digest: derive_transcript_digest(replay_basis_entries(parts))?,
        })
    }

    pub fn digest_bytes(&self) -> &[u8; 32] {
        self.digest.value().bytes()
    }

    pub fn canonical_basis_entry_count(&self) -> u32 {
        self.digest.metadata().entry_count()
    }
}

fn derive_transcript_digest(
    entries: Vec<CanonicalBasisEntry>,
) -> Result<CanonicalDerivedDigest, TranscriptReplayDenial> {
    let ready_basis =
        match prepare_canonical_basis_sequence(transcript_version(), TRANSCRIPT_DOMAIN, entries) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(_) => {
                return Err(TranscriptReplayDenial::CopiedTranscriptFieldsDenied);
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
    let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
        CanonicalDigestAlgorithmId::test_stable_fixture(),
        ready_basis.payload().domain(),
        ready_basis.payload().version().clone(),
    );
    match admit_canonical_sequence_digest_derivation(ready_basis, slot) {
        TransitionOutcome::Success(ready) => Ok(derive_canonical_digest(ready)),
        TransitionOutcome::Denied(_) => Err(TranscriptReplayDenial::CopiedTranscriptFieldsDenied),
        TransitionOutcome::Deferred(value) => match value {},
        TransitionOutcome::Stale(value) => match value {},
        TransitionOutcome::RebindRequired(value) => match value {},
        TransitionOutcome::Failed(value) => match value {},
    }
}
