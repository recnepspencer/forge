use worth_store_physical_backend::BackendDurabilityProfileId;
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReferenceAuthority,
    PhysicalRootReference, PhysicalSegmentId,
};

use crate::{
    CheckpointBaseAdmission, CheckpointId, CheckpointRecoveryCounterSnapshot, LogSequenceNumber,
    PageLsn, PageRedoDigestState, PageRedoEligibility, RecoveryCandidateDiscoveryTrace,
    RedoApplicationCursor, RedoApplicationPageFact, RedoPlanningDenial, WalLsnRange,
};

use super::decoded_recovery_record_set::DecodedRecoveryRecords;
use super::physical_record_grammar::{
    CheckpointManifestRecord, CheckpointPageImageRecord, PersistedPhysicalRecord,
};
use super::{
    OfflineRecoveryVerificationReport, OfflineRecoveryVerifierConclusion,
    PersistedRecoveryArtifactDigest, PersistedRecoveryArtifacts, RecoveryProfileId,
};

const REOPENED_ARTIFACT_SEGMENT_ID: u64 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReopenedRecoveryArtifactAdmission {
    report: OfflineRecoveryVerificationReport,
    artifact_digest: PersistedRecoveryArtifactDigest,
    recovery_profile: RecoveryProfileId,
    checkpoint_base: CheckpointBaseAdmission,
    replay_cursor: RedoApplicationCursor,
    inspected_records: usize,
    inspected_bytes: usize,
}

impl ReopenedRecoveryArtifactAdmission {
    pub fn admit(
        report: OfflineRecoveryVerificationReport,
        artifacts: &PersistedRecoveryArtifacts,
    ) -> Result<Self, ReopenedRecoveryArtifactAdmissionDenial> {
        require_verified_report(&report)?;
        let artifact_digest = PersistedRecoveryArtifactDigest::from_artifacts(artifacts);
        require_matching_digest(&report, &artifact_digest)?;
        require_offline_only_report(&report)?;
        let decoded = DecodedRecoveryRecords::from_artifacts(artifacts);
        let checkpoint_base = reopened_checkpoint_base(&decoded)?;
        let replay_cursor = replay_cursor_from_reopened_artifacts(artifacts)?;
        Ok(Self {
            recovery_profile: report.recovery_profile().clone(),
            inspected_records: report.inspected_records(),
            inspected_bytes: report.inspected_bytes(),
            report,
            artifact_digest,
            checkpoint_base,
            replay_cursor,
        })
    }

    pub const fn report(&self) -> &OfflineRecoveryVerificationReport {
        &self.report
    }

    pub const fn artifact_digest(&self) -> &PersistedRecoveryArtifactDigest {
        &self.artifact_digest
    }

    pub const fn recovery_profile(&self) -> &RecoveryProfileId {
        &self.recovery_profile
    }

    pub const fn replay_cursor(&self) -> &RedoApplicationCursor {
        &self.replay_cursor
    }

    pub const fn checkpoint_base(&self) -> &CheckpointBaseAdmission {
        &self.checkpoint_base
    }

    pub const fn inspected_records(&self) -> usize {
        self.inspected_records
    }

    pub const fn inspected_bytes(&self) -> usize {
        self.inspected_bytes
    }
}

fn reopened_checkpoint_base(
    decoded: &DecodedRecoveryRecords<'_>,
) -> Result<CheckpointBaseAdmission, ReopenedRecoveryArtifactAdmissionDenial> {
    let checkpoint = decoded
        .checkpoint()
        .ok_or(ReopenedRecoveryArtifactAdmissionDenial::MissingCheckpointManifest)?;
    let root_reference = reopened_root_reference(checkpoint)?;
    let covered_lsn_range = WalLsnRange::new(
        LogSequenceNumber::new(checkpoint.covered_lsn_start),
        LogSequenceNumber::new(checkpoint.covered_lsn_end),
    )
    .map_err(|_| ReopenedRecoveryArtifactAdmissionDenial::InvalidCheckpointCoverage)?;
    let checkpoint_id = CheckpointId::from_basis(format!(
        "reopened-checkpoint:{}:{}:{}:{}:{}",
        checkpoint.root_reference,
        checkpoint.root_generation,
        checkpoint.covered_lsn_start,
        checkpoint.covered_lsn_end,
        checkpoint.source_profile
    ));
    let trace = RecoveryCandidateDiscoveryTrace::new(
        checkpoint.source_profile.clone(),
        "reopened-checkpoint-base",
        1,
    );
    let counters = CheckpointRecoveryCounterSnapshot::new()
        .with_candidate()
        .with_locator_check()
        .with_manifest_validation()
        .with_integrity_damage_check();
    Ok(CheckpointBaseAdmission::from_reopened_artifact(
        checkpoint_id,
        root_reference,
        covered_lsn_range,
        trace,
        counters,
    ))
}

fn reopened_root_reference(
    checkpoint: &CheckpointManifestRecord,
) -> Result<worth_store_physical_format::PhysicalReference, ReopenedRecoveryArtifactAdmissionDenial>
{
    let root = PhysicalRootReference::from_raw(checkpoint.root_reference)
        .map_err(|_| ReopenedRecoveryArtifactAdmissionDenial::InvalidCheckpointRootReference)?;
    let generation = PhysicalGeneration::from_raw(checkpoint.root_generation)
        .map_err(|_| ReopenedRecoveryArtifactAdmissionDenial::InvalidCheckpointRootGeneration)?;
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .root_publication_cell(root)
        .with_root_publication_generation(generation);
    Ok(PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_root_publication(cell)
        .reference())
}

fn require_verified_report(
    report: &OfflineRecoveryVerificationReport,
) -> Result<(), ReopenedRecoveryArtifactAdmissionDenial> {
    if report.conclusion() == OfflineRecoveryVerifierConclusion::Verified {
        return Ok(());
    }
    Err(ReopenedRecoveryArtifactAdmissionDenial::VerifierConclusionMismatch)
}

fn require_matching_digest(
    report: &OfflineRecoveryVerificationReport,
    artifact_digest: &PersistedRecoveryArtifactDigest,
) -> Result<(), ReopenedRecoveryArtifactAdmissionDenial> {
    if report.artifact_digest() == artifact_digest {
        return Ok(());
    }
    Err(ReopenedRecoveryArtifactAdmissionDenial::ArtifactDigestMismatch)
}

fn require_offline_only_report(
    report: &OfflineRecoveryVerificationReport,
) -> Result<(), ReopenedRecoveryArtifactAdmissionDenial> {
    if report.live_runtime_constructions() == 0 && report.runtime_cache_reads() == 0 {
        return Ok(());
    }
    Err(ReopenedRecoveryArtifactAdmissionDenial::LiveRuntimeStateReuse)
}

fn replay_cursor_from_reopened_artifacts(
    artifacts: &PersistedRecoveryArtifacts,
) -> Result<RedoApplicationCursor, ReopenedRecoveryArtifactAdmissionDenial> {
    let page_facts = artifacts
        .records()
        .iter()
        .filter_map(|record| match record.physical_record() {
            PersistedPhysicalRecord::CheckpointPageImage(page) => {
                Some(reopened_page_fact(artifacts, page))
            }
            _ => None,
        })
        .collect::<Result<Vec<_>, _>>()?;
    if page_facts.is_empty() {
        return Err(ReopenedRecoveryArtifactAdmissionDenial::MissingCheckpointPageImage);
    }
    RedoApplicationCursor::new(page_facts).map_err(|denial| {
        ReopenedRecoveryArtifactAdmissionDenial::ReplayCursorDenied(Box::new(denial))
    })
}

fn reopened_page_fact(
    artifacts: &PersistedRecoveryArtifacts,
    page: &CheckpointPageImageRecord,
) -> Result<RedoApplicationPageFact, ReopenedRecoveryArtifactAdmissionDenial> {
    let page_id = PhysicalPageId::from_raw(page.page_id)
        .map_err(|_| ReopenedRecoveryArtifactAdmissionDenial::InvalidPhysicalPageIdentity)?;
    let page_generation = PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(reopened_artifact_segment_id()?, page_id)
        .with_page_generation(reopened_artifact_generation(page.page_generation)?);
    let classified_page_lsn = PageLsn::from_lsn(LogSequenceNumber::new(page.page_lsn));
    let redo_frontier = PageLsn::from_lsn(LogSequenceNumber::new(
        wal_frontier_for_page(artifacts, page.page_id).unwrap_or(page.page_lsn),
    ));
    let eligibility = PageRedoEligibility::from_reopened_artifact(
        BackendDurabilityProfileId::PosixFileFsyncDirFsync,
        page_generation,
        classified_page_lsn,
        redo_frontier,
    );
    let digest_state = PageRedoDigestState::new(
        page_generation,
        classified_page_lsn,
        page.physical_state_digest.clone(),
    );
    Ok(RedoApplicationPageFact::new(
        page_id,
        eligibility,
        digest_state,
    ))
}

fn wal_frontier_for_page(artifacts: &PersistedRecoveryArtifacts, page_id: u64) -> Option<u64> {
    artifacts
        .records()
        .iter()
        .filter_map(|record| match record.physical_record() {
            PersistedPhysicalRecord::WalRedoFrame(frame) if frame.page_id == page_id => {
                Some(frame.lsn)
            }
            _ => None,
        })
        .max()
}

fn reopened_artifact_segment_id(
) -> Result<PhysicalSegmentId, ReopenedRecoveryArtifactAdmissionDenial> {
    PhysicalSegmentId::from_raw(REOPENED_ARTIFACT_SEGMENT_ID)
        .map_err(|_| ReopenedRecoveryArtifactAdmissionDenial::InvalidPhysicalPageIdentity)
}

fn reopened_artifact_generation(
    value: u64,
) -> Result<PhysicalGeneration, ReopenedRecoveryArtifactAdmissionDenial> {
    PhysicalGeneration::from_raw(value)
        .map_err(|_| ReopenedRecoveryArtifactAdmissionDenial::InvalidPhysicalPageIdentity)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReopenedRecoveryArtifactAdmissionDenial {
    VerifierConclusionMismatch,
    ArtifactDigestMismatch,
    LiveRuntimeStateReuse,
    MissingCheckpointManifest,
    InvalidCheckpointRootReference,
    InvalidCheckpointRootGeneration,
    InvalidCheckpointCoverage,
    MissingCheckpointPageImage,
    InvalidPhysicalPageIdentity,
    ReplayCursorDenied(Box<RedoPlanningDenial>),
}
