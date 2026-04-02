use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::capabilities::DurabilityRead;
use crate::durability::data::{
    DurabilityError, DurableCheckpoint, DurableCheckpointId, DurableIntegrityStatus,
    DurableSegmentId, DurableSegmentManifest, DurableStore, DurableStoreLayout,
    RecoveryFailureClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DurableStoreManifestFile {
    pub(crate) store: DurableStore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DurableSegmentFile {
    pub(crate) entries: Vec<crate::replay::data::CanonicalCommitEnvelope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    write_json(
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
    layout.root_path.join("manifest.json")
}

pub(crate) fn segment_file_path(
    layout: &DurableStoreLayout,
    segment_id: DurableSegmentId,
) -> PathBuf {
    layout
        .root_path
        .join("segments")
        .join(format!("segment-{}.json", segment_id.0))
}

pub(crate) fn checkpoint_file_path(
    layout: &DurableStoreLayout,
    checkpoint_id: DurableCheckpointId,
) -> PathBuf {
    layout
        .root_path
        .join("checkpoints")
        .join(format!("checkpoint-{}.json", checkpoint_id.0))
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
        return refresh_store_segments_from_disk(read_json::<DurableStoreManifestFile>(&manifest)?.store);
    }
    let store = DurableStore {
        layout: layout.clone(),
        segments: Vec::new(),
        checkpoints: Vec::new(),
    };
    write_json(
        &manifest,
        &DurableStoreManifestFile {
            store: store.clone(),
        },
    )?;
    refresh_store_segments_from_disk(store)
}

fn refresh_store_segments_from_disk(mut store: DurableStore) -> Result<DurableStore, DurabilityError> {
    let segments_dir = store.layout.root_path.join("segments");
    let mut refreshed_segments = fs::read_dir(&segments_dir)
        .map_err(io_error)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
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
                match read_segment_entries(&path) {
                    Ok(entries) => (
                        entries.first().map(|entry| entry.commit.commit_id),
                        entries.last().map(|entry| entry.commit.commit_id),
                        entries.len(),
                        DurableIntegrityStatus::Verified,
                    ),
                    Err(_) => (
                        existing.as_ref().and_then(|segment| segment.first_commit_id),
                        existing.as_ref().and_then(|segment| segment.last_commit_id),
                        existing.as_ref().map(|segment| segment.commit_count).unwrap_or(0),
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

pub(crate) fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, DurabilityError> {
    let bytes = fs::read(path).map_err(io_error)?;
    serde_json::from_slice(&bytes).map_err(|error| {
        DurabilityError::new(
            RecoveryFailureClass::CorruptCheckpoint,
            format!("failed to deserialize {}: {error}", path.display()),
        )
    })
}

pub(crate) fn read_segment_entries(
    path: &Path,
) -> Result<Vec<crate::replay::data::CanonicalCommitEnvelope>, DurabilityError> {
    let bytes = fs::read(path).map_err(io_error)?;
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    if let Ok(file) = serde_json::from_slice::<DurableSegmentFile>(&bytes) {
        return Ok(file.entries);
    }

    bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            serde_json::from_slice::<crate::replay::data::CanonicalCommitEnvelope>(line).map_err(
                |error| {
                    DurabilityError::new(
                        RecoveryFailureClass::CorruptCheckpoint,
                        format!("failed to deserialize {}: {error}", path.display()),
                    )
                },
            )
        })
        .collect()
}

pub(crate) fn append_segment_entry(
    path: &Path,
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> Result<(), DurabilityError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(io_error)?;
    let bytes = serde_json::to_vec(envelope).map_err(|error| {
        DurabilityError::new(
            RecoveryFailureClass::DurableIoFailure,
            format!("failed to serialize {}: {error}", path.display()),
        )
    })?;
    file.write_all(&bytes).map_err(io_error)?;
    file.write_all(b"\n").map_err(io_error)?;
    file.flush().map_err(io_error)?;
    Ok(())
}

pub(crate) fn segment_uses_legacy_wrapper(path: &Path) -> Result<bool, DurabilityError> {
    let mut file = fs::File::open(path).map_err(io_error)?;
    let mut prefix = [0u8; 128];
    let bytes_read = file.read(&mut prefix).map_err(io_error)?;
    let prefix = std::str::from_utf8(&prefix[..bytes_read]).unwrap_or("");
    Ok(prefix.contains("\"entries\""))
}

pub(crate) fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), DurabilityError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    let temp_path = path.with_extension("tmp");
    let bytes = serde_json::to_vec(value).map_err(|error| {
        DurabilityError::new(
            RecoveryFailureClass::DurableIoFailure,
            format!("failed to serialize {}: {error}", path.display()),
        )
    })?;
    fs::write(&temp_path, bytes).map_err(io_error)?;
    fs::rename(&temp_path, path).map_err(io_error)?;
    Ok(())
}

pub(crate) fn io_error(error: std::io::Error) -> DurabilityError {
    DurabilityError::new(RecoveryFailureClass::DurableIoFailure, error.to_string())
}
