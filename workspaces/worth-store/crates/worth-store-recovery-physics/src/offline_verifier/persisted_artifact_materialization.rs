use super::{
    PersistedRecoveryArtifactDenial, PersistedRecoveryArtifacts, RecoveryPersistedRecord,
    RecoveryProfileId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedRecoveryArtifactMaterialization {
    format_version: String,
    backend_profile: String,
    recovery_profile: RecoveryProfileId,
    checkpoint: CheckpointManifestMaterialization,
    wal_frame: WalRedoFrameMaterialization,
    page_image: CheckpointPageImageMaterialization,
}

impl PersistedRecoveryArtifactMaterialization {
    pub fn new(
        format_version: impl Into<String>,
        backend_profile: impl Into<String>,
        recovery_profile: RecoveryProfileId,
        checkpoint: CheckpointManifestMaterialization,
        wal_frame: WalRedoFrameMaterialization,
        page_image: CheckpointPageImageMaterialization,
    ) -> Self {
        Self {
            format_version: format_version.into(),
            backend_profile: backend_profile.into(),
            recovery_profile,
            checkpoint,
            wal_frame,
            page_image,
        }
    }

    pub fn materialize(
        self,
    ) -> Result<PersistedRecoveryArtifacts, PersistedRecoveryArtifactDenial> {
        PersistedRecoveryArtifacts::admit(
            self.format_version,
            self.backend_profile,
            self.recovery_profile,
            vec![
                self.checkpoint.materialize_record()?,
                self.wal_frame.materialize_record()?,
                self.page_image.materialize_record()?,
            ],
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointManifestMaterialization {
    record_id: String,
    root_reference: u64,
    root_generation: u64,
    covered_lsn_start: u64,
    covered_lsn_end: u64,
    source_profile: String,
    source_candidate_count: usize,
    memory_envelope_bytes: u64,
    memory_envelope_frames: u32,
    allocation_bytes: u64,
    total_store_pages: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointManifestRecoveryBasisMaterialization {
    root_reference: u64,
    root_generation: u64,
    covered_lsn_start: u64,
    covered_lsn_end: u64,
}

impl CheckpointManifestRecoveryBasisMaterialization {
    pub const fn new(
        root_reference: u64,
        root_generation: u64,
        covered_lsn_start: u64,
        covered_lsn_end: u64,
    ) -> Self {
        Self {
            root_reference,
            root_generation,
            covered_lsn_start,
            covered_lsn_end,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointManifestSourceMaterialization {
    profile: String,
    candidate_count: usize,
}

impl CheckpointManifestSourceMaterialization {
    pub fn new(profile: impl Into<String>, candidate_count: usize) -> Self {
        Self {
            profile: profile.into(),
            candidate_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointManifestBudgetMaterialization {
    memory_envelope_bytes: u64,
    memory_envelope_frames: u32,
    allocation_bytes: u64,
    total_store_pages: u64,
}

impl CheckpointManifestBudgetMaterialization {
    pub const fn new(
        memory_envelope_bytes: u64,
        memory_envelope_frames: u32,
        allocation_bytes: u64,
        total_store_pages: u64,
    ) -> Self {
        Self {
            memory_envelope_bytes,
            memory_envelope_frames,
            allocation_bytes,
            total_store_pages,
        }
    }
}

impl CheckpointManifestMaterialization {
    pub fn new(
        record_id: impl Into<String>,
        basis: CheckpointManifestRecoveryBasisMaterialization,
        source: CheckpointManifestSourceMaterialization,
        budget: CheckpointManifestBudgetMaterialization,
    ) -> Self {
        Self {
            record_id: record_id.into(),
            root_reference: basis.root_reference,
            root_generation: basis.root_generation,
            covered_lsn_start: basis.covered_lsn_start,
            covered_lsn_end: basis.covered_lsn_end,
            source_profile: source.profile,
            source_candidate_count: source.candidate_count,
            memory_envelope_bytes: budget.memory_envelope_bytes,
            memory_envelope_frames: budget.memory_envelope_frames,
            allocation_bytes: budget.allocation_bytes,
            total_store_pages: budget.total_store_pages,
        }
    }

    fn materialize_record(
        self,
    ) -> Result<RecoveryPersistedRecord, PersistedRecoveryArtifactDenial> {
        RecoveryPersistedRecord::from_persisted_bytes(
            self.record_id,
            format!(
                "checkpoint:root_reference={};root_generation={};covered_start={};covered_end={};source_profile={};source_candidates={};memory_bytes={};memory_frames={};allocation_bytes={};total_store_pages={}",
                self.root_reference,
                self.root_generation,
                self.covered_lsn_start,
                self.covered_lsn_end,
                self.source_profile,
                self.source_candidate_count,
                self.memory_envelope_bytes,
                self.memory_envelope_frames,
                self.allocation_bytes,
                self.total_store_pages
            ),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalRedoFrameMaterialization {
    record_id: String,
    lsn: u64,
    page_id: u64,
    operation_digest: String,
    idempotence_digest: String,
}

impl WalRedoFrameMaterialization {
    pub fn new(
        record_id: impl Into<String>,
        lsn: u64,
        page_id: u64,
        operation_digest: impl Into<String>,
        idempotence_digest: impl Into<String>,
    ) -> Self {
        Self {
            record_id: record_id.into(),
            lsn,
            page_id,
            operation_digest: operation_digest.into(),
            idempotence_digest: idempotence_digest.into(),
        }
    }

    fn materialize_record(
        self,
    ) -> Result<RecoveryPersistedRecord, PersistedRecoveryArtifactDenial> {
        RecoveryPersistedRecord::from_persisted_bytes(
            self.record_id,
            format!(
                "wal:lsn={};page={};op={};idem={}",
                self.lsn, self.page_id, self.operation_digest, self.idempotence_digest
            ),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointPageImageMaterialization {
    record_id: String,
    page_id: u64,
    page_generation: u64,
    page_lsn: u64,
    physical_state_digest: String,
}

impl CheckpointPageImageMaterialization {
    pub fn new(
        record_id: impl Into<String>,
        page_id: u64,
        page_generation: u64,
        page_lsn: u64,
        physical_state_digest: impl Into<String>,
    ) -> Self {
        Self {
            record_id: record_id.into(),
            page_id,
            page_generation,
            page_lsn,
            physical_state_digest: physical_state_digest.into(),
        }
    }

    fn materialize_record(
        self,
    ) -> Result<RecoveryPersistedRecord, PersistedRecoveryArtifactDenial> {
        RecoveryPersistedRecord::from_persisted_bytes(
            self.record_id,
            format!(
                "page:id={};generation={};lsn={};digest={}",
                self.page_id, self.page_generation, self.page_lsn, self.physical_state_digest
            ),
        )
    }
}
