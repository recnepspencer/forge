use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Weak};

use same_file::Handle as StableFileHandle;

const MUTATION_SEQUENCE_SWEEP_INTERVAL: usize = 64;

/// Owner-local coordination indexed by stable opened-file identity.
///
/// This serializes length-affecting mutations to the same file while leaving
/// unrelated files independent. Entries disappear when the last mutable file
/// handle closes.
#[derive(Debug, Default)]
pub(super) struct FileMutationSequences {
    entries: Mutex<HashMap<StableFileHandle, SequenceEntry>>,
    opens_since_sweep: AtomicUsize,
    next_ordinal: AtomicU64,
}

#[derive(Debug)]
struct SequenceEntry {
    ordinal: u64,
    sequence: Weak<Mutex<()>>,
}

#[derive(Debug, Clone)]
pub(super) struct FileMutationSequence {
    ordinal: u64,
    sequence: Arc<Mutex<()>>,
}

impl FileMutationSequences {
    pub(super) fn for_file(&self, file: &std::fs::File) -> std::io::Result<FileMutationSequence> {
        let identity = StableFileHandle::from_file(file.try_clone()?)?;
        let mut entries = self
            .entries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.opens_since_sweep.fetch_add(1, Ordering::Relaxed) % MUTATION_SEQUENCE_SWEEP_INTERVAL
            == MUTATION_SEQUENCE_SWEEP_INTERVAL - 1
        {
            entries.retain(|_, entry| entry.sequence.strong_count() > 0);
        }
        if let Some((ordinal, sequence)) = entries.get(&identity).and_then(|entry| {
            std::sync::Weak::upgrade(&entry.sequence).map(|sequence| (entry.ordinal, sequence))
        }) {
            return Ok(FileMutationSequence { ordinal, sequence });
        }
        let sequence = Arc::new(Mutex::new(()));
        let ordinal = self.next_ordinal.fetch_add(1, Ordering::Relaxed);
        entries.insert(
            identity,
            SequenceEntry {
                ordinal,
                sequence: Arc::downgrade(&sequence),
            },
        );
        Ok(FileMutationSequence { ordinal, sequence })
    }
}

impl FileMutationSequence {
    pub(super) fn lock(&self) -> std::sync::MutexGuard<'_, ()> {
        self.sequence
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(super) fn with_ordered_pair<T>(
        first: &Self,
        second: Option<&Self>,
        effect: impl FnOnce() -> T,
    ) -> T {
        let Some(second) = second else {
            let _first = first.lock();
            return effect();
        };
        if Arc::ptr_eq(&first.sequence, &second.sequence) {
            let _shared = first.lock();
            return effect();
        }
        if first.ordinal < second.ordinal {
            let _first = first.lock();
            let _second = second.lock();
            effect()
        } else {
            let _second = second.lock();
            let _first = first.lock();
            effect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileMutationSequences;

    #[test]
    fn stable_file_identity_localizes_coordination_without_store_wide_serialization() {
        let root = tempfile::tempdir().expect("sequence root");
        let first_path = root.path().join("first");
        let second_path = root.path().join("second");
        std::fs::write(&first_path, []).expect("first file");
        std::fs::write(&second_path, []).expect("second file");
        let first = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&first_path)
            .expect("open first");
        let first_alias = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&first_path)
            .expect("open first alias");
        let second = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&second_path)
            .expect("open second");
        let sequences = FileMutationSequences::default();
        let first_sequence = sequences.for_file(&first).expect("first sequence");
        let first_alias_sequence = sequences
            .for_file(&first_alias)
            .expect("first alias sequence");
        let second_sequence = sequences.for_file(&second).expect("second sequence");

        let _held = first_sequence.lock();
        assert!(first_alias_sequence.sequence.try_lock().is_err());
        assert!(second_sequence.sequence.try_lock().is_ok());
    }
}
