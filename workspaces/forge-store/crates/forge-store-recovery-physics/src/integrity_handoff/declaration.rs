use super::inspection_envelope::BoundedInspectionEnvelopeEvidence;
use super::payload::{
    IntegrityHandoffCounters, IntegrityHandoffPayload, SealedIntegrityHandoffPayloadParts,
};
use super::{IntegrityHandoffDenial, IntegrityHandoffDenialKind};
use crate::{
    IntegrityDamageMap, IntegrityVettedCheckpointRecord, IntegrityVettedPageFrameRecord,
    IntegrityVettedRootManifestRecord, IntegrityVettedSegmentManifestRecord,
    IntegrityVettedWalFrame, PartialPublicationBeforeWalReplayRead,
};

#[derive(Debug, Clone, Default)]
pub struct IntegrityHandoffDeclaration {
    root_manifest: Option<IntegrityVettedRootManifestRecord>,
    segment_manifest: Option<IntegrityVettedSegmentManifestRecord>,
    page_frames: Vec<IntegrityVettedPageFrameRecord>,
    wal_frames: Vec<IntegrityVettedWalFrame>,
    checkpoint_records: Vec<IntegrityVettedCheckpointRecord>,
    damage_map: IntegrityDamageMap,
    inspection_envelope: Option<BoundedInspectionEnvelopeEvidence>,
    partial_publication_before_wal_replay_read: Option<PartialPublicationBeforeWalReplayRead>,
}

impl IntegrityHandoffDeclaration {
    pub fn root_manifest(mut self, record: IntegrityVettedRootManifestRecord) -> Self {
        self.root_manifest = Some(record);
        self
    }

    pub fn segment_manifest(mut self, record: IntegrityVettedSegmentManifestRecord) -> Self {
        self.segment_manifest = Some(record);
        self
    }

    pub fn page_frame(mut self, record: IntegrityVettedPageFrameRecord) -> Self {
        self.page_frames.push(record);
        self
    }

    pub fn wal_frame(mut self, record: IntegrityVettedWalFrame) -> Self {
        self.wal_frames.push(record);
        self
    }

    pub fn checkpoint_record(mut self, record: IntegrityVettedCheckpointRecord) -> Self {
        self.checkpoint_records.push(record);
        self
    }

    pub fn damage_map(mut self, damage_map: IntegrityDamageMap) -> Self {
        self.damage_map = damage_map;
        self
    }

    pub fn inspection_envelope(mut self, evidence: BoundedInspectionEnvelopeEvidence) -> Self {
        self.inspection_envelope = Some(evidence);
        self
    }

    pub fn partial_publication_before_wal_replay_read(
        mut self,
        replay_read: PartialPublicationBeforeWalReplayRead,
    ) -> Self {
        self.partial_publication_before_wal_replay_read = Some(replay_read);
        self
    }

    pub fn seal(self) -> Result<IntegrityHandoffPayload, IntegrityHandoffDenial> {
        let counters =
            classify_handoff_counters(&self).expect("inspection envelope already required");
        let root_manifest = self
            .root_manifest
            .ok_or_else(|| denial(IntegrityHandoffDenialKind::MissingRootManifestRecord))?;
        let segment_manifest = self
            .segment_manifest
            .ok_or_else(|| denial(IntegrityHandoffDenialKind::MissingSegmentManifestRecord))?;
        require_vetted_records(
            &self.page_frames,
            &self.wal_frames,
            &self.checkpoint_records,
        )?;
        let inspection_envelope = self
            .inspection_envelope
            .ok_or_else(|| denial(IntegrityHandoffDenialKind::MissingInspectionEnvelopeEvidence))?;
        let checksum_basis = inspection_envelope.checksum_basis().clone();
        let sealed_parts = SealedIntegrityHandoffPayloadParts {
            root_manifest,
            segment_manifest,
            page_frames: self.page_frames,
            wal_frames: self.wal_frames,
            checkpoint_records: self.checkpoint_records,
            damage_map: self.damage_map,
            checksum_basis,
            counters: IntegrityHandoffCounters::new(
                counters.vetted_record_count,
                counters.recovery_blocking_count,
                counters.quarantine_summary_count,
                counters.checked_byte_count,
                counters.checksum_execution_count,
                counters.skipped_decode_count,
            ),
            inspection_envelope,
            partial_publication_before_wal_replay_read: self
                .partial_publication_before_wal_replay_read,
        };
        Ok(IntegrityHandoffPayload::from_sealed_parts(sealed_parts))
    }
}

fn require_vetted_records(
    page_frames: &[IntegrityVettedPageFrameRecord],
    wal_frames: &[IntegrityVettedWalFrame],
    checkpoint_records: &[IntegrityVettedCheckpointRecord],
) -> Result<(), IntegrityHandoffDenial> {
    if page_frames.is_empty() {
        return Err(denial(IntegrityHandoffDenialKind::MissingPageFrameRecord));
    }
    if wal_frames.is_empty() {
        return Err(denial(IntegrityHandoffDenialKind::MissingWalFrame));
    }
    if checkpoint_records.is_empty() {
        return Err(denial(IntegrityHandoffDenialKind::MissingCheckpointRecord));
    }
    Ok(())
}

fn classify_handoff_counters(
    declaration: &IntegrityHandoffDeclaration,
) -> Option<IntegrityHandoffCounterClassification> {
    let pre_decode = declaration.inspection_envelope.as_ref()?.counters();
    Some(IntegrityHandoffCounterClassification {
        vetted_record_count: 2
            + declaration.page_frames.len() as u64
            + declaration.wal_frames.len() as u64
            + declaration.checkpoint_records.len() as u64,
        recovery_blocking_count: declaration.damage_map.recovery_blocking_findings().len() as u64,
        quarantine_summary_count: declaration.damage_map.quarantine_summaries().len() as u64,
        checked_byte_count: pre_decode.checked_byte_count(),
        checksum_execution_count: pre_decode.checksum_execution_count(),
        skipped_decode_count: pre_decode.skipped_logical_decode().skipped_count(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IntegrityHandoffCounterClassification {
    vetted_record_count: u64,
    recovery_blocking_count: u64,
    quarantine_summary_count: u64,
    checked_byte_count: u64,
    checksum_execution_count: u32,
    skipped_decode_count: u32,
}

fn denial(kind: IntegrityHandoffDenialKind) -> IntegrityHandoffDenial {
    IntegrityHandoffDenial::new(kind)
}
