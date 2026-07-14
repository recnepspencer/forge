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
    root: String,
    frontier_lsn: u64,
    source_profile: String,
    source_candidate_count: usize,
    memory_envelope_bytes: u64,
    memory_envelope_frames: u32,
    allocation_bytes: u64,
    total_store_pages: u64,
}

impl CheckpointManifestMaterialization {
    pub fn new(
        record_id: impl Into<String>,
        root: impl Into<String>,
        frontier_lsn: u64,
        source_profile: impl Into<String>,
        source_candidate_count: usize,
        memory_envelope_bytes: u64,
        memory_envelope_frames: u32,
        allocation_bytes: u64,
        total_store_pages: u64,
    ) -> Self {
        Self {
            record_id: record_id.into(),
            root: root.into(),
            frontier_lsn,
            source_profile: source_profile.into(),
            source_candidate_count,
            memory_envelope_bytes,
            memory_envelope_frames,
            allocation_bytes,
            total_store_pages,
        }
    }

    fn materialize_record(
        self,
    ) -> Result<RecoveryPersistedRecord, PersistedRecoveryArtifactDenial> {
        RecoveryPersistedRecord::from_persisted_bytes(
            self.record_id,
            format!(
                "checkpoint:root={};frontier={};source_profile={};source_candidates={};memory_bytes={};memory_frames={};allocation_bytes={};total_store_pages={}",
                self.root,
                self.frontier_lsn,
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
