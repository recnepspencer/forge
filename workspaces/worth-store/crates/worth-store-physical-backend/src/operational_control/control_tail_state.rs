use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::sync::{Mutex, MutexGuard};

use worth_store_authority::ControlStoreGeneration;

use super::transition_receipt_index::TransitionReceiptIndex;
use super::{scan_durable_prefix, scan_durable_suffix, ControlMediaFault};

#[derive(Debug)]
pub(super) struct ControlTailState {
    pub(super) observed_bytes: u64,
    pub(super) last_generation: Option<ControlStoreGeneration>,
    pub(super) record_count: u64,
    pub(super) prefix_digest: [u8; 32],
    pub(super) last_frame_checksum: Option<[u8; 32]>,
    pub(super) transition_receipts: TransitionReceiptIndex,
    pub(super) observed_modified: std::time::SystemTime,
}

pub(super) fn synchronize_tail<'state>(
    file: &mut File,
    cached: &'state mut Option<ControlTailState>,
) -> Result<(u64, &'state mut ControlTailState), ControlMediaFault> {
    let current_metadata = file.metadata()?;
    let current_bytes = current_metadata.len();
    let current_modified = current_metadata.modified()?;
    if cached.is_none() {
        let (state, scanned) = load_tail_state(file)?;
        *cached = Some(state);
        return Ok((
            scanned,
            cached
                .as_mut()
                .ok_or(ControlMediaFault::ControlHistoryChanged)?,
        ));
    }
    let state = cached
        .as_mut()
        .ok_or(ControlMediaFault::ControlHistoryChanged)?;
    if current_bytes < state.observed_bytes {
        return Err(ControlMediaFault::ControlHistoryRewound {
            expected_bytes: state.observed_bytes,
            observed_bytes: current_bytes,
        });
    }
    if state.observed_bytes == current_bytes {
        if state.observed_modified != current_modified {
            let (reloaded, scanned) = load_tail_state(file)?;
            if reloaded.observed_bytes != state.observed_bytes
                || reloaded.last_generation != state.last_generation
                || reloaded.prefix_digest != state.prefix_digest
            {
                return Err(ControlMediaFault::ControlHistoryChanged);
            }
            *state = reloaded;
            return Ok((scanned, state));
        }
        verify_cached_tail(file, state)?;
        return Ok((0, state));
    }

    file.seek(SeekFrom::Start(state.observed_bytes))?;
    let mut suffix = TransitionReceiptIndex::empty();
    let summary = scan_durable_suffix(
        file,
        state.observed_bytes,
        state.last_generation,
        state.prefix_digest,
        state.last_frame_checksum,
        |record| {
            if state
                .transition_receipts
                .lookup(record.transition_identity())?
                .is_some()
            {
                return Err(ControlMediaFault::DuplicateTransitionConflict);
            }
            suffix.insert_record(&record)
        },
    )?;
    let after_scan = file.metadata()?;
    if after_scan.len() != current_bytes || after_scan.modified()? != current_modified {
        return Err(ControlMediaFault::ControlHistoryChanged);
    }
    state.transition_receipts.absorb(suffix)?;
    state.observed_bytes = summary.end_offset();
    state.last_generation = summary.last_generation();
    state.prefix_digest = summary.prefix_digest();
    state.last_frame_checksum = summary.last_frame_checksum();
    state.record_count = state
        .record_count
        .checked_add(summary.record_count())
        .ok_or(ControlMediaFault::GenerationExhausted)?;
    state.observed_modified = current_modified;
    Ok((summary.record_count(), state))
}

pub(super) fn load_tail_state_from_path(
    path: &std::path::Path,
) -> Result<ControlTailState, ControlMediaFault> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    load_tail_state(&mut file).map(|(state, _)| state)
}

fn load_tail_state(file: &mut File) -> Result<(ControlTailState, u64), ControlMediaFault> {
    let before = file.metadata()?;
    let before_modified = before.modified()?;
    file.seek(SeekFrom::Start(0))?;
    let mut transition_receipts = TransitionReceiptIndex::empty();
    let summary = scan_durable_prefix(file, |record| transition_receipts.insert_record(&record))?;
    let after = file.metadata()?;
    if before.len() != after.len() || before_modified != after.modified()? {
        return Err(ControlMediaFault::ControlHistoryChanged);
    }
    Ok((
        ControlTailState {
            observed_bytes: summary.end_offset(),
            last_generation: summary.last_generation(),
            record_count: summary.record_count(),
            prefix_digest: summary.prefix_digest(),
            last_frame_checksum: summary.last_frame_checksum(),
            transition_receipts,
            observed_modified: before_modified,
        },
        summary.record_count(),
    ))
}

fn verify_cached_tail(file: &mut File, state: &ControlTailState) -> Result<(), ControlMediaFault> {
    let Some(expected) = state.last_frame_checksum else {
        return if state.observed_bytes == 0 {
            Ok(())
        } else {
            Err(ControlMediaFault::ControlHistoryChanged)
        };
    };
    file.seek(SeekFrom::End(-32))?;
    let mut observed = [0; 32];
    file.read_exact(&mut observed)?;
    if observed != expected {
        return Err(ControlMediaFault::ControlHistoryChanged);
    }
    Ok(())
}

pub(super) fn lock_tail(
    tail: &Mutex<Option<ControlTailState>>,
) -> Result<MutexGuard<'_, Option<ControlTailState>>, ControlMediaFault> {
    tail.lock()
        .map_err(|_| std::io::Error::other("operational control tail state lock poisoned").into())
}
