use super::RecoveryPersistedRecord;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecoveryProfileId(String);

impl RecoveryProfileId {
    pub fn strict_s4() -> Self {
        Self("strict-s4-recovery".to_string())
    }

    pub fn new(value: impl Into<String>) -> Result<Self, PersistedRecoveryArtifactDenial> {
        let value = value.into();
        if value.is_empty() {
            return Err(PersistedRecoveryArtifactDenial::MissingRecoveryProfile);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedRecoveryArtifacts {
    format_version: String,
    backend_profile: String,
    recovery_profile: RecoveryProfileId,
    records: Vec<RecoveryPersistedRecord>,
}

impl PersistedRecoveryArtifacts {
    pub fn admit(
        format_version: impl Into<String>,
        backend_profile: impl Into<String>,
        recovery_profile: RecoveryProfileId,
        mut records: Vec<RecoveryPersistedRecord>,
    ) -> Result<Self, PersistedRecoveryArtifactDenial> {
        let format_version = format_version.into();
        if format_version.is_empty() {
            return Err(PersistedRecoveryArtifactDenial::MissingFormatVersion);
        }
        let backend_profile = backend_profile.into();
        if backend_profile.is_empty() {
            return Err(PersistedRecoveryArtifactDenial::MissingBackendProfile);
        }
        if records.is_empty() {
            return Err(PersistedRecoveryArtifactDenial::NoPersistedRecords);
        }
        records.sort_by(|left, right| left.record_id().cmp(right.record_id()));
        for window in records.windows(2) {
            if window[0].record_id() == window[1].record_id() {
                return Err(PersistedRecoveryArtifactDenial::DuplicateRecordId(
                    window[0].record_id().to_string(),
                ));
            }
        }
        Ok(Self {
            format_version,
            backend_profile,
            recovery_profile,
            records,
        })
    }

    pub fn format_version(&self) -> &str {
        &self.format_version
    }

    pub fn backend_profile(&self) -> &str {
        &self.backend_profile
    }

    pub const fn recovery_profile(&self) -> &RecoveryProfileId {
        &self.recovery_profile
    }

    pub fn records(&self) -> &[RecoveryPersistedRecord] {
        &self.records
    }

    pub fn total_bytes(&self) -> usize {
        self.records()
            .iter()
            .map(|record| record.bytes().len())
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistedRecoveryArtifactDenial {
    MissingFormatVersion,
    MissingBackendProfile,
    MissingRecoveryProfile,
    MissingRecordId,
    EmptyRecordBytes { record_id: String },
    DuplicateRecordId(String),
    NoPersistedRecords,
    MalformedPhysicalRecord { record_id: String },
}

pub(super) fn malformed_physical_record(record_id: &str) -> PersistedRecoveryArtifactDenial {
    PersistedRecoveryArtifactDenial::MalformedPhysicalRecord {
        record_id: record_id.to_string(),
    }
}
