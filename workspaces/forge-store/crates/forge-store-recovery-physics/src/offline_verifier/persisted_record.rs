use super::persisted_artifacts::PersistedRecoveryArtifactDenial;
use super::physical_record_grammar::{parse_physical_record, PersistedPhysicalRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPersistedRecordRole {
    CheckpointManifest,
    WalRedoFrame,
    CheckpointPageImage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPersistedRecord {
    record_id: String,
    bytes: Vec<u8>,
    physical_record: PersistedPhysicalRecord,
}

impl RecoveryPersistedRecord {
    pub fn from_persisted_bytes(
        record_id: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self, PersistedRecoveryArtifactDenial> {
        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(PersistedRecoveryArtifactDenial::MissingRecordId);
        }
        let bytes = bytes.into();
        if bytes.is_empty() {
            return Err(PersistedRecoveryArtifactDenial::EmptyRecordBytes { record_id });
        }
        let physical_record = parse_physical_record(&record_id, &bytes)?;
        Ok(Self {
            record_id,
            bytes,
            physical_record,
        })
    }

    pub fn record_id(&self) -> &str {
        &self.record_id
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub const fn role(&self) -> RecoveryPersistedRecordRole {
        self.physical_record.role()
    }

    pub(super) const fn physical_record(&self) -> &PersistedPhysicalRecord {
        &self.physical_record
    }
}
