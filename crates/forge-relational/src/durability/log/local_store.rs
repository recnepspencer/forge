use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::capabilities::DurabilityRead;
use crate::durability::data::{
    DurabilityError, DurableCheckpoint, DurableCheckpointId, DurableSegmentId, DurableStore,
    DurableStoreLayout, RecoveryFailureClass,
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
        return load_or_initialize_store(store.layout.clone());
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
        return Ok(read_json::<DurableStoreManifestFile>(&manifest)?.store);
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

pub(crate) fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), DurabilityError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    let temp_path = path.with_extension("tmp");
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
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
