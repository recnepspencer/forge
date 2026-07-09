use super::envelope_evidence::{BoundedInspectionEnvelopeEvidence, S4ChecksumAlgorithmScopeBasis};
use crate::{
    IntegrityDamageMap, IntegrityVettedCheckpointRecord, IntegrityVettedPageFrameRecord,
    IntegrityVettedRootManifestRecord, IntegrityVettedSegmentManifestRecord,
    IntegrityVettedWalFrame, PartialPublicationBeforeWalReplayRead,
};
use worth_store_contracts::StableDigest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S4IntegrityHandoffCounters {
    vetted_record_count: u64,
    recovery_blocking_count: u64,
    quarantine_summary_count: u64,
    checked_byte_count: u64,
    checksum_execution_count: u32,
    skipped_decode_count: u32,
}

impl S4IntegrityHandoffCounters {
    pub(crate) const fn new(
        vetted_record_count: u64,
        recovery_blocking_count: u64,
        quarantine_summary_count: u64,
        checked_byte_count: u64,
        checksum_execution_count: u32,
        skipped_decode_count: u32,
    ) -> Self {
        Self {
            vetted_record_count,
            recovery_blocking_count,
            quarantine_summary_count,
            checked_byte_count,
            checksum_execution_count,
            skipped_decode_count,
        }
    }

    pub const fn vetted_record_count(self) -> u64 {
        self.vetted_record_count
    }

    pub const fn recovery_blocking_count(self) -> u64 {
        self.recovery_blocking_count
    }

    pub const fn quarantine_summary_count(self) -> u64 {
        self.quarantine_summary_count
    }

    pub const fn checked_byte_count(self) -> u64 {
        self.checked_byte_count
    }

    pub const fn checksum_execution_count(self) -> u32 {
        self.checksum_execution_count
    }

    pub const fn skipped_decode_count(self) -> u32 {
        self.skipped_decode_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawBytesExcludedFromRecoveryHandoff;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4IntegrityHandoffPayload {
    identity: StableDigest,
    root_manifest: IntegrityVettedRootManifestRecord,
    segment_manifest: IntegrityVettedSegmentManifestRecord,
    page_frames: Vec<IntegrityVettedPageFrameRecord>,
    wal_frames: Vec<IntegrityVettedWalFrame>,
    checkpoint_records: Vec<IntegrityVettedCheckpointRecord>,
    damage_map: IntegrityDamageMap,
    checksum_basis: S4ChecksumAlgorithmScopeBasis,
    counters: S4IntegrityHandoffCounters,
    inspection_envelope: BoundedInspectionEnvelopeEvidence,
    partial_publication_before_wal_replay_read: Option<PartialPublicationBeforeWalReplayRead>,
    raw_bytes_excluded: RawBytesExcludedFromRecoveryHandoff,
}

pub(crate) struct SealedS4IntegrityHandoffPayloadParts {
    pub(crate) root_manifest: IntegrityVettedRootManifestRecord,
    pub(crate) segment_manifest: IntegrityVettedSegmentManifestRecord,
    pub(crate) page_frames: Vec<IntegrityVettedPageFrameRecord>,
    pub(crate) wal_frames: Vec<IntegrityVettedWalFrame>,
    pub(crate) checkpoint_records: Vec<IntegrityVettedCheckpointRecord>,
    pub(crate) damage_map: IntegrityDamageMap,
    pub(crate) checksum_basis: S4ChecksumAlgorithmScopeBasis,
    pub(crate) counters: S4IntegrityHandoffCounters,
    pub(crate) inspection_envelope: BoundedInspectionEnvelopeEvidence,
    pub(crate) partial_publication_before_wal_replay_read:
        Option<PartialPublicationBeforeWalReplayRead>,
}

impl S4IntegrityHandoffPayload {
    pub fn declare() -> super::S4IntegrityHandoffPayloadDeclaration {
        super::S4IntegrityHandoffPayloadDeclaration::default()
    }

    pub(crate) fn from_sealed_parts(parts: SealedS4IntegrityHandoffPayloadParts) -> Self {
        let identity = payload_identity(&parts);
        Self {
            identity,
            root_manifest: parts.root_manifest,
            segment_manifest: parts.segment_manifest,
            page_frames: parts.page_frames,
            wal_frames: parts.wal_frames,
            checkpoint_records: parts.checkpoint_records,
            damage_map: parts.damage_map,
            checksum_basis: parts.checksum_basis,
            counters: parts.counters,
            inspection_envelope: parts.inspection_envelope,
            partial_publication_before_wal_replay_read: parts
                .partial_publication_before_wal_replay_read,
            raw_bytes_excluded: RawBytesExcludedFromRecoveryHandoff,
        }
    }

    pub fn identity(&self) -> &StableDigest {
        &self.identity
    }

    pub const fn root_manifest(&self) -> &IntegrityVettedRootManifestRecord {
        &self.root_manifest
    }

    pub const fn segment_manifest(&self) -> &IntegrityVettedSegmentManifestRecord {
        &self.segment_manifest
    }

    pub fn page_frames(&self) -> &[IntegrityVettedPageFrameRecord] {
        &self.page_frames
    }

    pub fn wal_frames(&self) -> &[IntegrityVettedWalFrame] {
        &self.wal_frames
    }

    pub fn checkpoint_records(&self) -> &[IntegrityVettedCheckpointRecord] {
        &self.checkpoint_records
    }

    pub const fn damage_map(&self) -> &IntegrityDamageMap {
        &self.damage_map
    }

    pub fn corruption_readmission_handoffs(
        &self,
    ) -> Vec<crate::RecoveryCorruptionReadmissionHandoff> {
        self.damage_map.build_corruption_readmission_handoffs()
    }

    pub const fn checksum_basis(&self) -> &S4ChecksumAlgorithmScopeBasis {
        &self.checksum_basis
    }

    pub const fn counters(&self) -> S4IntegrityHandoffCounters {
        self.counters
    }

    pub const fn inspection_envelope(&self) -> &BoundedInspectionEnvelopeEvidence {
        &self.inspection_envelope
    }

    pub(crate) fn partial_publication_before_wal_replay_read(
        &self,
    ) -> Option<&PartialPublicationBeforeWalReplayRead> {
        self.partial_publication_before_wal_replay_read.as_ref()
    }

    pub const fn proves_no_raw_bytes_crossed(&self) -> bool {
        matches!(self.raw_bytes_excluded, RawBytesExcludedFromRecoveryHandoff)
    }

    pub const fn claims_recovery(&self) -> bool {
        false
    }
}

fn payload_identity(parts: &SealedS4IntegrityHandoffPayloadParts) -> StableDigest {
    StableDigest::new(format!(
        "s4-handoff:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
        parts.root_manifest,
        parts.segment_manifest,
        parts.page_frames,
        parts.wal_frames,
        parts.checkpoint_records,
        parts.damage_map.basis(),
        parts.checksum_basis,
        parts.counters,
        parts.inspection_envelope,
        parts.partial_publication_before_wal_replay_read
    ))
    .expect("S.4 integrity handoff identity basis is non-empty")
}
