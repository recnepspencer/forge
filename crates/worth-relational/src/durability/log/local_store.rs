use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::capabilities::DurabilityRead;
use crate::durability::data::{
    DurabilityError, DurableCheckpoint, DurableCheckpointId, DurableIntegrityStatus,
    DurableSegmentId, DurableSegmentManifest, DurableStore, DurableStoreLayout,
    RecoveryFailureClass,
};
use crate::durability::log::native_file_codec::{
    read_segment_file, read_store_manifest_file, write_segment_file, write_store_manifest_file,
};
use crate::durability::log::persisted_canonical_commit::PersistedCanonicalCommit;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DurableStoreManifestFile {
    pub(crate) store: DurableStore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DurableSegmentFile {
    pub(crate) entries: Vec<PersistedCanonicalCommit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DurableCheckpointFile {
    pub(crate) checkpoint: DurableCheckpoint,
}

pub(crate) fn ensure_loaded_store(
    runtime: &(impl DurabilityRead + crate::capabilities::RuntimeConfigSource),
) -> Result<DurableStore, DurabilityError> {
    if let Some(store) = runtime.durable_store() {
        return Ok(store.clone());
    }
    let Some(layout) = runtime
        .runtime_config()
        .durability
        .policy
        .store_layout
        .clone()
    else {
        return Err(DurabilityError::new(
            RecoveryFailureClass::DurableIoFailure,
            "persisted durability mode requires a durable store layout",
        ));
    };
    load_or_initialize_store(layout)
}

pub(crate) fn load_store_from_disk(
    runtime: &impl crate::capabilities::RuntimeConfigSource,
) -> Result<DurableStore, DurabilityError> {
    let Some(layout) = runtime
        .runtime_config()
        .durability
        .policy
        .store_layout
        .clone()
    else {
        return Err(DurabilityError::new(
            RecoveryFailureClass::DurableIoFailure,
            "persisted durability mode requires a durable store layout",
        ));
    };
    load_or_initialize_store(layout)
}

pub(crate) fn persist_store_manifest(store: &DurableStore) -> Result<(), DurabilityError> {
    ensure_store_dirs(&store.layout)?;
    write_store_manifest_file(
        &manifest_path(&store.layout),
        &DurableStoreManifestFile {
            store: store.clone(),
        },
    )
}

pub(crate) fn current_segment_ids(store: Option<&DurableStore>) -> Vec<DurableSegmentId> {
    store
        .map(|store| {
            store
                .segments
                .iter()
                .map(|segment| segment.segment_id)
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn manifest_path(layout: &DurableStoreLayout) -> PathBuf {
    layout.root_path.join("manifest.worth-store")
}

pub(crate) fn segment_file_path(
    layout: &DurableStoreLayout,
    segment_id: DurableSegmentId,
) -> PathBuf {
    layout
        .root_path
        .join("segments")
        .join(format!("segment-{}.worth-segment", segment_id.0))
}

pub(crate) fn checkpoint_file_path(
    layout: &DurableStoreLayout,
    checkpoint_id: DurableCheckpointId,
) -> PathBuf {
    layout
        .root_path
        .join("checkpoints")
        .join(format!("checkpoint-{}.worth-checkpoint", checkpoint_id.0))
}

pub(crate) fn ensure_store_dirs(layout: &DurableStoreLayout) -> Result<(), DurabilityError> {
    fs::create_dir_all(layout.root_path.join("segments")).map_err(io_error)?;
    fs::create_dir_all(layout.root_path.join("checkpoints")).map_err(io_error)?;
    Ok(())
}

pub(crate) fn load_or_initialize_store(
    layout: DurableStoreLayout,
) -> Result<DurableStore, DurabilityError> {
    ensure_store_dirs(&layout)?;
    let manifest = manifest_path(&layout);
    if manifest.exists() {
        return refresh_store_segments_from_disk(read_store_manifest_file(&manifest)?.store);
    }
    let store = DurableStore {
        layout: layout.clone(),
        segments: Vec::new(),
        checkpoints: Vec::new(),
    };
    write_store_manifest_file(
        &manifest,
        &DurableStoreManifestFile {
            store: store.clone(),
        },
    )?;
    refresh_store_segments_from_disk(store)
}

fn refresh_store_segments_from_disk(
    mut store: DurableStore,
) -> Result<DurableStore, DurabilityError> {
    let segments_dir = store.layout.root_path.join("segments");
    let mut refreshed_segments = fs::read_dir(&segments_dir)
        .map_err(io_error)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "worth-segment"))
        .filter_map(|path| {
            let stem = path.file_stem()?.to_str()?;
            let suffix = stem.strip_prefix("segment-")?;
            let segment_id = suffix.parse::<u64>().ok()?;
            Some((DurableSegmentId(segment_id), path))
        })
        .map(|(segment_id, path)| {
            let existing = store
                .segments
                .iter()
                .find(|segment| segment.segment_id == segment_id)
                .cloned();
            let (first_commit_id, last_commit_id, commit_count, integrity) =
                match read_segment_inventory(&path) {
                    Ok(entries) => (
                        entries.first().map(|entry| entry.commit_id),
                        entries.last().map(|entry| entry.commit_id),
                        entries.len(),
                        DurableIntegrityStatus::Verified,
                    ),
                    Err(_) => (
                        existing
                            .as_ref()
                            .and_then(|segment| segment.first_commit_id),
                        existing.as_ref().and_then(|segment| segment.last_commit_id),
                        existing
                            .as_ref()
                            .map(|segment| segment.commit_count)
                            .unwrap_or(0),
                        DurableIntegrityStatus::Corrupt,
                    ),
                };
            DurableSegmentManifest {
                segment_id,
                path,
                first_commit_id,
                last_commit_id,
                commit_count,
                runtime_name: existing
                    .as_ref()
                    .map(|segment| segment.runtime_name.clone())
                    .unwrap_or_default(),
                profile: existing
                    .as_ref()
                    .map(|segment| segment.profile)
                    .unwrap_or(crate::config::data::RelationalRuntimeProfile::CertificationCore),
                schema_version: existing
                    .as_ref()
                    .map(|segment| segment.schema_version)
                    .unwrap_or(crate::schema::data::SchemaVersionId(0)),
                integrity,
            }
        })
        .collect::<Vec<_>>();
    refreshed_segments.sort_by_key(|segment| segment.segment_id);
    store.segments = refreshed_segments;
    Ok(store)
}

pub(crate) fn read_segment_entries_with_registry(
    path: &Path,
    registry: &crate::schema::data::RelationalSchemaRegistry,
) -> Result<Vec<crate::durability::migration::ReadmittedCanonicalCommit>, DurabilityError> {
    match read_segment_file(path) {
        Ok(file) => file
            .entries
            .into_iter()
            .map(|entry| {
                entry.readmit().map_err(|detail| {
                    DurabilityError::new(RecoveryFailureClass::CorruptSegment, detail)
                })
            })
            .collect(),
        Err(current_error) => {
            let bytes = fs::read(path).map_err(io_error)?;
            match crate::durability::migration::decode_worth_query_9_16_1_1_segment(
                &bytes, registry,
            ) {
                Ok(entries) => Ok(entries),
                Err(crate::durability::migration::LegacySegmentDecodeError::Schema(detail)) => Err(
                    DurabilityError::new(RecoveryFailureClass::SchemaMismatch, detail),
                ),
                Err(
                    crate::durability::migration::LegacySegmentDecodeError::UnsupportedLineage(
                        detail,
                    ),
                ) => Err(DurabilityError::new(
                    RecoveryFailureClass::UnsupportedLegacySemantics,
                    detail,
                )),
                Err(crate::durability::migration::LegacySegmentDecodeError::Decode) => {
                    Err(current_error)
                }
            }
        }
    }
}

pub(crate) fn segment_requires_recovery_readmission(path: &Path) -> Result<bool, DurabilityError> {
    match read_segment_file(path) {
        Ok(_) => Ok(false),
        Err(current_error) => {
            let bytes = fs::read(path).map_err(io_error)?;
            crate::durability::migration::worth_query_9_16_1_1_segment_inventory(&bytes)
                .map(|_| true)
                .map_err(|_| current_error)
        }
    }
}

fn read_segment_inventory(
    path: &Path,
) -> Result<Vec<crate::history::data::RelationalCommitReceipt>, DurabilityError> {
    match read_segment_file(path) {
        Ok(file) => Ok(file
            .entries
            .into_iter()
            .map(PersistedCanonicalCommit::into_receipt)
            .collect()),
        Err(current_error) => {
            let bytes = fs::read(path).map_err(io_error)?;
            crate::durability::migration::worth_query_9_16_1_1_segment_inventory(&bytes)
                .map_err(|_| current_error)
        }
    }
}

pub(crate) fn append_segment_entry(
    path: &Path,
    commit: &crate::history::data::PositionedCanonicalCommit,
    registry: &crate::schema::data::RelationalSchemaRegistry,
) -> Result<(), DurabilityError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    let mut entries = if path.exists() {
        read_segment_entries_with_registry(path, registry)?
            .into_iter()
            .map(|entry| {
                entry
                    .positioned()
                    .map(PersistedCanonicalCommit::from_positioned)
                    .ok_or_else(|| {
                        DurabilityError::new(
                            RecoveryFailureClass::DurableIoFailure,
                            "current append cannot extend a migration-owned segment",
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    entries.push(PersistedCanonicalCommit::from_positioned(commit));
    write_segment_file(path, &DurableSegmentFile { entries })
}

pub(crate) fn io_error(error: std::io::Error) -> DurabilityError {
    DurabilityError::new(RecoveryFailureClass::DurableIoFailure, error.to_string())
}

#[cfg(test)]
mod legacy_segment_tests {
    use super::*;
    use crate::capabilities::RuntimeConfigSource;

    #[test]
    fn native_reader_readmits_a_real_9_16_1_1_segment() {
        // Generated once by the production persisted runtime and native
        // segment writer at the 9.16.1.1 close commit 7d198923e36e9df.
        let bytes = decode_hex(include_str!(
            "../../../tests/fixtures/worth_query_9_16_1_1_native_segment.hex"
        ));
        let path = std::env::temp_dir().join(format!(
            "worth-query-9-16-1-1-segment-{}.worth-segment",
            std::process::id()
        ));
        std::fs::write(&path, bytes).expect("legacy fixture is written to the native read seam");
        let runtime = crate::tests::support::persisted_runtime_with_test_schema();
        let entries =
            read_segment_entries_with_registry(&path, &runtime.runtime_config().schema.registry)
                .expect("9.16.1.1 schema authority readmits through the runtime registry");
        std::fs::remove_file(&path).expect("legacy fixture scratch file is removed");

        assert_eq!(entries.len(), 1);
        assert!(entries[0].needs_replay_completion());
        assert_eq!(
            entries[0].position(),
            crate::publication::patch::data::PatchStreamPosition(1)
        );
        assert_eq!(entries[0].envelope().commit.commit_id.0, 1);
        assert_eq!(entries[0].envelope().commit.version_id.0, 1);
        assert_eq!(entries[0].envelope().branch_context.0, "main");
        assert_eq!(
            entries[0]
                .envelope()
                .patch
                .authoritative_record_patches
                .len(),
            1
        );
    }

    fn decode_hex(encoded: &str) -> Vec<u8> {
        let digits = encoded
            .bytes()
            .filter(|byte| !byte.is_ascii_whitespace())
            .collect::<Vec<_>>();
        assert_eq!(digits.len() % 2, 0, "fixture hex has complete bytes");
        digits
            .chunks_exact(2)
            .map(|pair| (hex_digit(pair[0]) << 4) | hex_digit(pair[1]))
            .collect()
    }

    fn hex_digit(digit: u8) -> u8 {
        match digit {
            b'0'..=b'9' => digit - b'0',
            b'a'..=b'f' => digit - b'a' + 10,
            _ => panic!("fixture contains a non-hex digit"),
        }
    }
}
