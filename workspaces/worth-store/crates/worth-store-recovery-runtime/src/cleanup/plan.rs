use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use worth_store_physical_format::{PhysicalCheckpointIdentity, RecordArtifactFile};
use worth_store_recovery_physics::{
    PhysicalRecoveryResidueKind, PhysicalSourceSelection, WalLsnRange, WalSegmentArtifactIdentity,
};

use crate::entry::PhysicalRecoveryLimitDeclaration;
use crate::handoff::RecoveryOperationFateSet;
use crate::progression::{RecoveryBaseImagePlan, RecoveryPublicationExpectation};

use super::{
    RecoveryCleanupDeferralReason, RecoveryCleanupDisposition, RecoveryCleanupDispositionKind,
    RecoveryCleanupEligibility, RecoveryCleanupTarget,
};

#[cfg(test)]
mod tests;

pub(crate) struct RecoveryCleanupPlan {
    identity: [u8; 32],
    published_generation: u64,
    checkpoint: PhysicalCheckpointIdentity,
    candidates: Vec<RecoveryCleanupEligibility>,
    dispositions: Vec<RecoveryCleanupDisposition>,
}

pub(crate) struct RecoveryCleanupPlanBasis<'a> {
    pub(crate) selection: &'a PhysicalSourceSelection,
    pub(crate) base: &'a RecoveryBaseImagePlan,
    pub(crate) publication: &'a RecoveryPublicationExpectation,
    pub(crate) fates: &'a RecoveryOperationFateSet,
    pub(crate) limits: PhysicalRecoveryLimitDeclaration,
}

pub(crate) fn build_plan(basis: RecoveryCleanupPlanBasis<'_>) -> RecoveryCleanupPlan {
    let RecoveryCleanupPlanBasis {
        selection,
        base,
        publication,
        fates,
        limits,
    } = basis;
    let checkpoint = publication.checkpoint_identity();
    let mut dispositions = retained_dispositions(selection, base, checkpoint);
    dispositions.extend(consumed_publication_candidates(publication));
    let covered_wal = admit_checkpoint_covered_wal(selection, fates, limits);
    let candidates = covered_wal.candidates;
    dispositions.extend(covered_wal.dispositions);
    dispositions.extend(selection.residue().iter().map(|residue| {
        RecoveryCleanupDisposition::new(
            RecoveryCleanupTarget::Residue {
                name: residue.name().into(),
                kind: residue.kind(),
            },
            RecoveryCleanupDispositionKind::QuarantinedOrUnsupported,
            None,
            residue.observed_bytes(),
        )
    }));
    dispositions.sort_by(|left, right| left.target().cmp(right.target()));
    let identity = plan_identity(publication, checkpoint, &candidates, &dispositions);
    RecoveryCleanupPlan {
        identity,
        published_generation: publication.recovered_root().generation(),
        checkpoint,
        candidates,
        dispositions,
    }
}

struct CheckpointCoveredWalAdmission {
    candidates: Vec<RecoveryCleanupEligibility>,
    dispositions: Vec<RecoveryCleanupDisposition>,
}

fn admit_checkpoint_covered_wal(
    selection: &PhysicalSourceSelection,
    fates: &RecoveryOperationFateSet,
    limits: PhysicalRecoveryLimitDeclaration,
) -> CheckpointCoveredWalAdmission {
    let mut admission = CheckpointCoveredWalAdmission {
        candidates: Vec::new(),
        dispositions: Vec::new(),
    };
    let mut candidate_bytes = 0_u64;
    for covered in selection.wal_tail().checkpoint_covered() {
        let kind = checkpoint_covered_disposition(CheckpointCoveredWalDecision {
            cleanup_safe: covered.cleanup_safe(),
            unresolved: fates.indeterminate() != 0,
            next_count: admission.candidates.len() as u64 + 1,
            next_bytes: candidate_bytes.checked_add(covered.byte_count()),
            limits,
        });
        if kind == RecoveryCleanupDispositionKind::Eligible {
            candidate_bytes = candidate_bytes
                .checked_add(covered.byte_count())
                .expect("bounded cleanup byte sum");
            admission.candidates.push(RecoveryCleanupEligibility::new(
                covered.identity(),
                covered.lsn_range(),
                covered.byte_count(),
            ));
        }
        admission.dispositions.push(RecoveryCleanupDisposition::new(
            RecoveryCleanupTarget::Wal(covered.identity()),
            kind,
            Some(covered.lsn_range()),
            covered.byte_count(),
        ));
    }
    admission
}

struct CheckpointCoveredWalDecision {
    cleanup_safe: bool,
    unresolved: bool,
    next_count: u64,
    next_bytes: Option<u64>,
    limits: PhysicalRecoveryLimitDeclaration,
}

fn checkpoint_covered_disposition(
    decision: CheckpointCoveredWalDecision,
) -> RecoveryCleanupDispositionKind {
    if !decision.cleanup_safe {
        RecoveryCleanupDispositionKind::QuarantinedOrUnsupported
    } else if decision.unresolved {
        RecoveryCleanupDispositionKind::Deferred(
            RecoveryCleanupDeferralReason::UnresolvedOperationFate,
        )
    } else if decision.next_count > decision.limits.cleanup_candidates {
        RecoveryCleanupDispositionKind::Deferred(RecoveryCleanupDeferralReason::CandidateLimit)
    } else if decision
        .next_bytes
        .is_none_or(|bytes| bytes > decision.limits.cleanup_bytes)
    {
        RecoveryCleanupDispositionKind::Deferred(RecoveryCleanupDeferralReason::ByteLimit)
    } else {
        RecoveryCleanupDispositionKind::Eligible
    }
}

fn retained_dispositions(
    selection: &PhysicalSourceSelection,
    base: &RecoveryBaseImagePlan,
    checkpoint: PhysicalCheckpointIdentity,
) -> Vec<RecoveryCleanupDisposition> {
    let mut records = BTreeMap::new();
    records.insert(
        RecordArtifactFile::CurrentRootSelector,
        RecoveryCleanupDispositionKind::Current,
    );
    records.insert(
        RecordArtifactFile::RootManifest {
            generation: base.destination_generation(),
        },
        RecoveryCleanupDispositionKind::Current,
    );
    for artifact in base.source_artifacts() {
        records
            .entry(*artifact)
            .or_insert(RecoveryCleanupDispositionKind::Retained);
    }
    let mut dispositions = records
        .into_iter()
        .map(|(artifact, kind)| {
            RecoveryCleanupDisposition::new(RecoveryCleanupTarget::Record(artifact), kind, None, 0)
        })
        .collect::<Vec<_>>();
    dispositions.push(RecoveryCleanupDisposition::new(
        RecoveryCleanupTarget::Checkpoint(checkpoint),
        RecoveryCleanupDispositionKind::Retained,
        None,
        selection
            .checkpoint()
            .map_or(0, |checkpoint| checkpoint.checkpoint().encoded_bytes()),
    ));
    dispositions.extend(selection.wal_tail().segments().iter().map(|segment| {
        RecoveryCleanupDisposition::new(
            RecoveryCleanupTarget::Wal(segment.identity()),
            RecoveryCleanupDispositionKind::Retained,
            Some(segment.inspection().lsn_range()),
            segment.inspection().byte_count(),
        )
    }));
    dispositions
}

fn consumed_publication_candidates(
    publication: &RecoveryPublicationExpectation,
) -> Vec<RecoveryCleanupDisposition> {
    let root_protocol = publication.root_protocol();
    [
        root_protocol.previous_candidate(),
        root_protocol.current_candidate(),
        root_protocol.catalog_candidate(),
    ]
    .into_iter()
    .map(|artifact| {
        RecoveryCleanupDisposition::new(
            RecoveryCleanupTarget::Record(artifact),
            RecoveryCleanupDispositionKind::SafelyRemoved,
            None,
            0,
        )
    })
    .collect()
}

fn plan_identity(
    publication: &RecoveryPublicationExpectation,
    checkpoint: PhysicalCheckpointIdentity,
    candidates: &[RecoveryCleanupEligibility],
    dispositions: &[RecoveryCleanupDisposition],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.recovery.cleanup-plan.v1");
    digest.update(publication.plan_identity());
    digest.update(publication.recovered_root().generation().to_le_bytes());
    digest.update(checkpoint.store_identity().bytes());
    digest.update(checkpoint.sequence().get().to_le_bytes());
    digest.update((candidates.len() as u64).to_le_bytes());
    for candidate in candidates {
        hash_wal(
            &mut digest,
            candidate.artifact(),
            candidate.range(),
            candidate.byte_count(),
        );
    }
    digest.update((dispositions.len() as u64).to_le_bytes());
    for disposition in dispositions {
        hash_disposition(&mut digest, disposition);
    }
    digest.finalize().into()
}

fn hash_disposition(digest: &mut Sha256, disposition: &RecoveryCleanupDisposition) {
    match disposition.target() {
        RecoveryCleanupTarget::Record(artifact) => {
            digest.update([0]);
            hash_bytes(digest, artifact.file_name().as_bytes());
        }
        RecoveryCleanupTarget::Checkpoint(checkpoint) => {
            digest.update([1]);
            digest.update(checkpoint.store_identity().bytes());
            digest.update(checkpoint.sequence().get().to_le_bytes());
        }
        RecoveryCleanupTarget::Wal(artifact) => {
            digest.update([2]);
            digest.update(artifact.segment().get().to_le_bytes());
            digest.update(artifact.generation().get().to_le_bytes());
        }
        RecoveryCleanupTarget::Residue { name, kind } => {
            digest.update([3, residue_kind(*kind)]);
            hash_bytes(digest, name.as_bytes());
        }
    }
    digest.update([disposition_kind(disposition.kind())]);
    if let RecoveryCleanupDispositionKind::Deferred(reason) = disposition.kind() {
        digest.update([deferral_reason(reason)]);
    }
    match disposition.wal_range() {
        Some(range) => {
            digest.update([1]);
            digest.update(range.start().get().to_le_bytes());
            digest.update(range.end_exclusive().get().to_le_bytes());
        }
        None => digest.update([0]),
    }
    digest.update(disposition.byte_count().to_le_bytes());
}

fn hash_bytes(digest: &mut Sha256, bytes: &[u8]) {
    digest.update((bytes.len() as u64).to_le_bytes());
    digest.update(bytes);
}

const fn disposition_kind(kind: RecoveryCleanupDispositionKind) -> u8 {
    match kind {
        RecoveryCleanupDispositionKind::Current => 0,
        RecoveryCleanupDispositionKind::Retained => 1,
        RecoveryCleanupDispositionKind::Eligible => 2,
        RecoveryCleanupDispositionKind::Deferred(_) => 3,
        RecoveryCleanupDispositionKind::QuarantinedOrUnsupported => 4,
        RecoveryCleanupDispositionKind::SafelyRemoved => 5,
    }
}

const fn deferral_reason(reason: RecoveryCleanupDeferralReason) -> u8 {
    match reason {
        RecoveryCleanupDeferralReason::CandidateLimit => 0,
        RecoveryCleanupDeferralReason::ByteLimit => 1,
        RecoveryCleanupDeferralReason::UnresolvedOperationFate => 2,
        RecoveryCleanupDeferralReason::FreshnessUnavailable => 3,
        RecoveryCleanupDeferralReason::PublishedGenerationChanged => 4,
        RecoveryCleanupDeferralReason::EligibilityChanged => 5,
        RecoveryCleanupDeferralReason::DeniedBeforeEffect => 6,
        RecoveryCleanupDeferralReason::IndeterminateEffect => 7,
    }
}

const fn residue_kind(kind: PhysicalRecoveryResidueKind) -> u8 {
    match kind {
        PhysicalRecoveryResidueKind::NonCanonicalWalArtifact => 0,
        PhysicalRecoveryResidueKind::NonRegularWalEntry => 1,
        PhysicalRecoveryResidueKind::TrailingEmptyWalSegment => 2,
        PhysicalRecoveryResidueKind::InterruptedWalSegmentStart => 3,
        PhysicalRecoveryResidueKind::UnreferencedCompactionProduct => 4,
    }
}

fn hash_wal(
    digest: &mut Sha256,
    artifact: WalSegmentArtifactIdentity,
    range: WalLsnRange,
    bytes: u64,
) {
    digest.update(artifact.segment().get().to_le_bytes());
    digest.update(artifact.generation().get().to_le_bytes());
    digest.update(range.start().get().to_le_bytes());
    digest.update(range.end_exclusive().get().to_le_bytes());
    digest.update(bytes.to_le_bytes());
}

impl RecoveryCleanupPlan {
    pub(crate) const fn identity(&self) -> [u8; 32] {
        self.identity
    }
    pub(crate) const fn published_generation(&self) -> u64 {
        self.published_generation
    }
    pub(crate) const fn checkpoint(&self) -> PhysicalCheckpointIdentity {
        self.checkpoint
    }
    pub(crate) fn candidates(&self) -> &[RecoveryCleanupEligibility] {
        &self.candidates
    }
    pub(crate) fn dispositions(&self) -> &[RecoveryCleanupDisposition] {
        &self.dispositions
    }
    pub(crate) fn transition_candidate(
        &mut self,
        artifact: WalSegmentArtifactIdentity,
        kind: RecoveryCleanupDispositionKind,
    ) -> bool {
        let target = RecoveryCleanupTarget::Wal(artifact);
        self.dispositions
            .iter_mut()
            .find(|disposition| disposition.target() == &target)
            .is_some_and(|disposition| disposition.transition_eligible(kind))
    }

    pub(crate) fn defer_remaining(&mut self, reason: RecoveryCleanupDeferralReason) {
        for disposition in &mut self.dispositions {
            disposition.transition_eligible(RecoveryCleanupDispositionKind::Deferred(reason));
        }
    }

    pub(crate) fn into_dispositions(self) -> Box<[RecoveryCleanupDisposition]> {
        self.dispositions.into_boxed_slice()
    }

    pub(crate) fn take_eligibilities(&mut self) -> Vec<RecoveryCleanupEligibility> {
        std::mem::take(&mut self.candidates)
    }
}
