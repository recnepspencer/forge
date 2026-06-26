use crate::{
    IntegrityDamageMap, IntegrityVettedCheckpointRecord, IntegrityVettedPageFrameRecord,
    IntegrityVettedRootManifestRecord, IntegrityVettedSegmentManifestRecord,
    IntegrityVettedWalFrame, S4IntegrityHandoffDenial, S4IntegrityHandoffDenialKind,
};
use forge_store_contracts::StableDigest;
use forge_store_physical_integrity::{
    ChecksumAlgorithmDeclaration, ChecksumAlgorithmId, ChecksumCoverageBasis,
    ChecksumScopeDeclaration, IntegrityCheckedFrame, IntegrityCheckedPage,
    PreDecodeAdmissionCounters,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedInspectionEnvelopeEvidence {
    resident_byte_limit: u64,
    protected_read_limit: u64,
    streaming_window_limit: u64,
    counters: PreDecodeAdmissionCounters,
    checksum_basis: S4ChecksumAlgorithmScopeBasis,
}

impl BoundedInspectionEnvelopeEvidence {
    pub fn from_checked_page(
        checked: &IntegrityCheckedPage<'_>,
        resident_byte_limit: u64,
        protected_read_limit: u64,
        streaming_window_limit: u64,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        Self::from_checked_counters(
            checked.counters(),
            checked.gate_evidence().coverage_basis(),
            resident_byte_limit,
            protected_read_limit,
            streaming_window_limit,
        )
    }

    pub fn from_checked_frame(
        checked: &IntegrityCheckedFrame<'_>,
        resident_byte_limit: u64,
        protected_read_limit: u64,
        streaming_window_limit: u64,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        Self::from_checked_counters(
            checked.counters(),
            checked.gate_evidence().coverage_basis(),
            resident_byte_limit,
            protected_read_limit,
            streaming_window_limit,
        )
    }

    pub const fn resident_byte_limit(&self) -> u64 {
        self.resident_byte_limit
    }

    pub const fn protected_read_limit(&self) -> u64 {
        self.protected_read_limit
    }

    pub const fn streaming_window_limit(&self) -> u64 {
        self.streaming_window_limit
    }

    pub const fn counters(&self) -> PreDecodeAdmissionCounters {
        self.counters
    }

    pub const fn checksum_basis(&self) -> &S4ChecksumAlgorithmScopeBasis {
        &self.checksum_basis
    }

    fn from_checked_counters(
        counters: PreDecodeAdmissionCounters,
        basis: &ChecksumCoverageBasis,
        resident_byte_limit: u64,
        protected_read_limit: u64,
        streaming_window_limit: u64,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        let checked_bytes = counters.checked_byte_count();
        if checked_bytes > resident_byte_limit
            || checked_bytes > protected_read_limit
            || checked_bytes > streaming_window_limit
        {
            return Err(S4IntegrityHandoffDenial::new(
                S4IntegrityHandoffDenialKind::InspectionEnvelopeExceeded,
            ));
        }
        Ok(Self {
            resident_byte_limit,
            protected_read_limit,
            streaming_window_limit,
            counters,
            checksum_basis: S4ChecksumAlgorithmScopeBasis::from_coverage_basis(basis)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4ChecksumAlgorithmScopeBasis {
    algorithm: ChecksumAlgorithmId,
    scope: ChecksumScopeDeclaration,
}

impl S4ChecksumAlgorithmScopeBasis {
    pub fn from_checksum_declaration(declaration: &ChecksumAlgorithmDeclaration) -> Self {
        Self::from_coverage_basis(declaration.coverage_basis())
            .expect("S.3 checksum declaration already admitted its coverage scope")
    }

    pub const fn algorithm(&self) -> ChecksumAlgorithmId {
        self.algorithm
    }

    pub const fn scope(&self) -> &ChecksumScopeDeclaration {
        &self.scope
    }

    fn from_coverage_basis(
        basis: &ChecksumCoverageBasis,
    ) -> Result<Self, S4IntegrityHandoffDenial> {
        let scope = ChecksumScopeDeclaration::for_physical_format(
            basis.physical_format_identity(),
            basis.coverage_map().clone(),
        )
        .map_err(|_| {
            S4IntegrityHandoffDenial::new(S4IntegrityHandoffDenialKind::ChecksumBasisMismatch)
        })?;
        Ok(Self {
            algorithm: basis.algorithm_id(),
            scope,
        })
    }
}

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
    raw_bytes_excluded: RawBytesExcludedFromRecoveryHandoff,
}

impl S4IntegrityHandoffPayload {
    pub fn declare() -> S4IntegrityHandoffPayloadDeclaration {
        S4IntegrityHandoffPayloadDeclaration::default()
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

    pub const fn checksum_basis(&self) -> &S4ChecksumAlgorithmScopeBasis {
        &self.checksum_basis
    }

    pub const fn counters(&self) -> S4IntegrityHandoffCounters {
        self.counters
    }

    pub const fn inspection_envelope(&self) -> &BoundedInspectionEnvelopeEvidence {
        &self.inspection_envelope
    }

    pub const fn proves_no_raw_bytes_crossed(&self) -> bool {
        matches!(self.raw_bytes_excluded, RawBytesExcludedFromRecoveryHandoff)
    }

    pub const fn claims_recovery(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Default)]
pub struct S4IntegrityHandoffPayloadDeclaration {
    root_manifest: Option<IntegrityVettedRootManifestRecord>,
    segment_manifest: Option<IntegrityVettedSegmentManifestRecord>,
    page_frames: Vec<IntegrityVettedPageFrameRecord>,
    wal_frames: Vec<IntegrityVettedWalFrame>,
    checkpoint_records: Vec<IntegrityVettedCheckpointRecord>,
    damage_map: IntegrityDamageMap,
    inspection_envelope: Option<BoundedInspectionEnvelopeEvidence>,
}

impl S4IntegrityHandoffPayloadDeclaration {
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

    pub fn seal(self) -> Result<S4IntegrityHandoffPayload, S4IntegrityHandoffDenial> {
        let root_manifest = self
            .root_manifest
            .ok_or_else(|| denial(S4IntegrityHandoffDenialKind::MissingRootManifestRecord))?;
        let segment_manifest = self
            .segment_manifest
            .ok_or_else(|| denial(S4IntegrityHandoffDenialKind::MissingSegmentManifestRecord))?;
        if self.page_frames.is_empty() {
            return Err(denial(S4IntegrityHandoffDenialKind::MissingPageFrameRecord));
        }
        if self.wal_frames.is_empty() {
            return Err(denial(S4IntegrityHandoffDenialKind::MissingWalFrame));
        }
        if self.checkpoint_records.is_empty() {
            return Err(denial(
                S4IntegrityHandoffDenialKind::MissingCheckpointRecord,
            ));
        }
        let inspection_envelope = self.inspection_envelope.ok_or_else(|| {
            denial(S4IntegrityHandoffDenialKind::MissingInspectionEnvelopeEvidence)
        })?;
        let checksum_basis = inspection_envelope.checksum_basis().clone();
        let pre_decode_counters = inspection_envelope.counters();
        let counters = S4IntegrityHandoffCounters {
            vetted_record_count: 2
                + self.page_frames.len() as u64
                + self.wal_frames.len() as u64
                + self.checkpoint_records.len() as u64,
            recovery_blocking_count: self.damage_map.recovery_blocking_findings().len() as u64,
            quarantine_summary_count: self.damage_map.quarantine_summaries().len() as u64,
            checked_byte_count: pre_decode_counters.checked_byte_count(),
            checksum_execution_count: pre_decode_counters.checksum_execution_count(),
            skipped_decode_count: pre_decode_counters.skipped_logical_decode().skipped_count(),
        };
        let identity = payload_identity(
            &root_manifest,
            &segment_manifest,
            &self.page_frames,
            &self.wal_frames,
            &self.checkpoint_records,
            &self.damage_map,
            &checksum_basis,
            counters,
            &inspection_envelope,
        );
        Ok(S4IntegrityHandoffPayload {
            identity,
            root_manifest,
            segment_manifest,
            page_frames: self.page_frames,
            wal_frames: self.wal_frames,
            checkpoint_records: self.checkpoint_records,
            damage_map: self.damage_map,
            checksum_basis,
            counters,
            inspection_envelope,
            raw_bytes_excluded: RawBytesExcludedFromRecoveryHandoff,
        })
    }
}

fn payload_identity(
    root_manifest: &IntegrityVettedRootManifestRecord,
    segment_manifest: &IntegrityVettedSegmentManifestRecord,
    page_frames: &[IntegrityVettedPageFrameRecord],
    wal_frames: &[IntegrityVettedWalFrame],
    checkpoint_records: &[IntegrityVettedCheckpointRecord],
    damage_map: &IntegrityDamageMap,
    checksum_basis: &S4ChecksumAlgorithmScopeBasis,
    counters: S4IntegrityHandoffCounters,
    inspection_envelope: &BoundedInspectionEnvelopeEvidence,
) -> StableDigest {
    StableDigest::new(format!(
        "s4-handoff:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
        root_manifest,
        segment_manifest,
        page_frames,
        wal_frames,
        checkpoint_records,
        damage_map.basis(),
        checksum_basis,
        counters,
        inspection_envelope
    ))
    .expect("S.4 integrity handoff identity basis is non-empty")
}

fn denial(kind: S4IntegrityHandoffDenialKind) -> S4IntegrityHandoffDenial {
    S4IntegrityHandoffDenial::new(kind)
}
