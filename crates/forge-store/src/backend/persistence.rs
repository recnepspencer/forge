use crate::failure::{StoreError, StoreErrorKind};
use std::path::Path;

use super::records::StoreState;

pub(super) fn load_state(path: &Path) -> Result<StoreState, StoreError> {
    if !path.exists() {
        return Ok(StoreState::default());
    }
    let raw = std::fs::read(path)?;
    let mut state: StoreState = serde_json::from_slice(&raw).map_err(store_state_decode_error)?;
    state.backfill_missing_branch_delta_layer_artifacts()?;
    super::maintenance::summaries::record_scheduler_boot_state(&mut state);
    super::maintenance::summaries::backfill_scheduler_summaries_if_missing(&mut state);
    Ok(state)
}

fn store_state_decode_error(error: serde_json::Error) -> StoreError {
    StoreError::backend_integrity(format!(
        "persisted store state failed validation during decode: {error}"
    ))
}

pub(super) fn persist_state_atomic(path: &Path, state: &StoreState) -> Result<(), StoreError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let writer = atomicwrites::AtomicFile::new(path, atomicwrites::AllowOverwrite);
    writer
        .write(|file| {
            use std::io::Write;
            let bytes = serde_json::to_vec_pretty(state).map_err(std::io::Error::other)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
            Ok(())
        })
        .map_err(|error: atomicwrites::Error<std::io::Error>| {
            StoreError::new(
                StoreErrorKind::AuthoritativeAppendAtomicityViolation,
                format!("failed to atomically persist authoritative store state: {error}"),
            )
        })
}
