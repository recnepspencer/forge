use super::persisted_artifacts::{malformed_physical_record, PersistedRecoveryArtifactDenial};
use super::RecoveryPersistedRecordRole;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum S4PersistedPhysicalRecord {
    CheckpointManifest(S4CheckpointManifestRecord),
    WalRedoFrame(S4WalRedoFrameRecord),
    CheckpointPageImage(S4CheckpointPageImageRecord),
}

impl S4PersistedPhysicalRecord {
    pub(super) const fn role(&self) -> RecoveryPersistedRecordRole {
        match self {
            Self::CheckpointManifest(_) => RecoveryPersistedRecordRole::CheckpointManifest,
            Self::WalRedoFrame(_) => RecoveryPersistedRecordRole::WalRedoFrame,
            Self::CheckpointPageImage(_) => RecoveryPersistedRecordRole::CheckpointPageImage,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct S4CheckpointManifestRecord {
    pub(super) root: String,
    pub(super) frontier_lsn: u64,
    pub(super) source_profile: String,
    pub(super) source_candidate_count: usize,
    pub(super) memory_envelope_bytes: u64,
    pub(super) memory_envelope_frames: u32,
    pub(super) allocation_bytes: u64,
    pub(super) total_store_pages: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct S4WalRedoFrameRecord {
    pub(super) lsn: u64,
    pub(super) page_id: u64,
    pub(super) operation_digest: String,
    pub(super) idempotence_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct S4CheckpointPageImageRecord {
    pub(super) page_id: u64,
    pub(super) page_generation: u64,
    pub(super) page_lsn: u64,
    pub(super) physical_state_digest: String,
}

pub(super) fn parse_s4_physical_record(
    record_id: &str,
    bytes: &[u8],
) -> Result<S4PersistedPhysicalRecord, PersistedRecoveryArtifactDenial> {
    let text = std::str::from_utf8(bytes).map_err(|_| malformed_physical_record(record_id))?;
    let Some((role, fields)) = text.split_once(':') else {
        return Err(malformed_physical_record(record_id));
    };
    match role {
        "checkpoint" => parse_checkpoint_manifest(record_id, fields),
        "wal" => parse_wal_redo_frame(record_id, fields),
        "page" => parse_checkpoint_page_image(record_id, fields),
        _ => Err(malformed_physical_record(record_id)),
    }
}

fn parse_checkpoint_manifest(
    record_id: &str,
    fields: &str,
) -> Result<S4PersistedPhysicalRecord, PersistedRecoveryArtifactDenial> {
    Ok(S4PersistedPhysicalRecord::CheckpointManifest(
        S4CheckpointManifestRecord {
            root: field_text(record_id, fields, "root")?.to_string(),
            frontier_lsn: field_u64(record_id, fields, "frontier")?,
            source_profile: field_text(record_id, fields, "source_profile")?.to_string(),
            source_candidate_count: field_usize(record_id, fields, "source_candidates")?,
            memory_envelope_bytes: field_u64(record_id, fields, "memory_bytes")?,
            memory_envelope_frames: field_u32(record_id, fields, "memory_frames")?,
            allocation_bytes: field_u64(record_id, fields, "allocation_bytes")?,
            total_store_pages: field_u64(record_id, fields, "total_store_pages")?,
        },
    ))
}

fn parse_wal_redo_frame(
    record_id: &str,
    fields: &str,
) -> Result<S4PersistedPhysicalRecord, PersistedRecoveryArtifactDenial> {
    Ok(S4PersistedPhysicalRecord::WalRedoFrame(
        S4WalRedoFrameRecord {
            lsn: field_u64(record_id, fields, "lsn")?,
            page_id: field_u64(record_id, fields, "page")?,
            operation_digest: field_text(record_id, fields, "op")?.to_string(),
            idempotence_digest: field_text(record_id, fields, "idem")?.to_string(),
        },
    ))
}

fn parse_checkpoint_page_image(
    record_id: &str,
    fields: &str,
) -> Result<S4PersistedPhysicalRecord, PersistedRecoveryArtifactDenial> {
    Ok(S4PersistedPhysicalRecord::CheckpointPageImage(
        S4CheckpointPageImageRecord {
            page_id: field_u64(record_id, fields, "id")?,
            page_generation: field_u64(record_id, fields, "generation")?,
            page_lsn: field_u64(record_id, fields, "lsn")?,
            physical_state_digest: field_text(record_id, fields, "digest")?.to_string(),
        },
    ))
}

fn field_text<'a>(
    record_id: &str,
    fields: &'a str,
    name: &str,
) -> Result<&'a str, PersistedRecoveryArtifactDenial> {
    fields
        .split(';')
        .filter_map(|field| field.split_once('='))
        .find_map(|(key, value)| (key == name && !value.is_empty()).then_some(value))
        .ok_or_else(|| malformed_physical_record(record_id))
}

fn field_u64(
    record_id: &str,
    fields: &str,
    name: &str,
) -> Result<u64, PersistedRecoveryArtifactDenial> {
    field_text(record_id, fields, name)?
        .parse()
        .map_err(|_| malformed_physical_record(record_id))
}

fn field_u32(
    record_id: &str,
    fields: &str,
    name: &str,
) -> Result<u32, PersistedRecoveryArtifactDenial> {
    field_text(record_id, fields, name)?
        .parse()
        .map_err(|_| malformed_physical_record(record_id))
}

fn field_usize(
    record_id: &str,
    fields: &str,
    name: &str,
) -> Result<usize, PersistedRecoveryArtifactDenial> {
    field_text(record_id, fields, name)?
        .parse()
        .map_err(|_| malformed_physical_record(record_id))
}
