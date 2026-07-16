use std::io::Write;

use worth_store_authority::ControlStoreGeneration;

use super::{ControlMediaFault, ControlMediaLocation, PhysicalOperationalControlStore};

#[test]
fn append_reopen_and_idempotent_retry_preserve_one_atomic_prefix() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let store =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path)).expect("open");
    let first = store
        .compare_exchange_append(None, "operation:opened", b"opened")
        .expect("first append");
    let replay = store
        .compare_exchange_append(None, "operation:opened", b"opened")
        .expect("idempotent replay");
    let second = store
        .compare_exchange_append(Some(first.generation()), "operation:lease", b"lease")
        .expect("second append");

    assert!(!first.idempotent_replay());
    assert!(replay.idempotent_replay());
    assert_eq!(first.generation(), replay.generation());
    assert_eq!(
        second.generation(),
        ControlStoreGeneration::from_raw(2).expect("generation")
    );

    let reopened = PhysicalOperationalControlStore::open(ControlMediaLocation::new(path))
        .expect("reopen")
        .inspect()
        .expect("inspect");
    assert!(reopened.damage().is_none());
    assert_eq!(reopened.records().len(), 2);
    assert_eq!(reopened.records()[1].payload(), b"lease");
}

#[test]
fn same_path_journal_replacement_changes_media_identity_and_invalidates_open_handles() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let displaced = directory.path().join("displaced.log");
    let store = PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path))
        .expect("original store");
    store
        .append_at_current_tail("operation:opened", b"opened")
        .expect("original append");
    let original_identity = store.identity();
    let valid_bytes = std::fs::read(&path).expect("valid journal bytes");

    std::fs::rename(&path, displaced).expect("retain the original physical file identity");
    std::fs::write(&path, valid_bytes).expect("replace journal at the configured path");
    assert!(matches!(
        store.inspect(),
        Err(ControlMediaFault::ControlMediaIdentityChanged { .. })
    ));

    let replacement = PhysicalOperationalControlStore::open(ControlMediaLocation::new(path))
        .expect("replacement is a separately identifiable control store");
    assert_ne!(replacement.identity(), original_identity);
}

#[test]
fn nonempty_journal_cannot_recreate_missing_or_corrupt_identity_metadata() {
    let directory = tempfile::tempdir().expect("temp directory");
    let missing_path = directory.path().join("missing.log");
    let missing_location = ControlMediaLocation::new(&missing_path);
    let missing = PhysicalOperationalControlStore::open(missing_location.clone())
        .expect("original missing case");
    missing
        .append_at_current_tail("missing:opened", b"opened")
        .expect("original append");
    drop(missing);
    std::fs::remove_file(missing_location.identity_path()).expect("remove identity metadata");
    assert!(matches!(
        PhysicalOperationalControlStore::open(missing_location),
        Err(ControlMediaFault::MissingControlMediaIdentity)
    ));

    let corrupt_path = directory.path().join("corrupt.log");
    let corrupt_location = ControlMediaLocation::new(&corrupt_path);
    let corrupt = PhysicalOperationalControlStore::open(corrupt_location.clone())
        .expect("original corrupt case");
    corrupt
        .append_at_current_tail("corrupt:opened", b"opened")
        .expect("original append");
    drop(corrupt);
    std::fs::write(corrupt_location.identity_path(), b"not an identity")
        .expect("corrupt identity metadata");
    assert!(matches!(
        PhysicalOperationalControlStore::open(corrupt_location),
        Err(ControlMediaFault::CorruptControlMediaIdentity)
    ));
}

#[test]
fn torn_tail_and_stale_generation_fail_closed() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let store =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path)).expect("open");
    let first = store
        .compare_exchange_append(None, "operation:opened", b"opened")
        .expect("append");
    assert!(matches!(
        store.compare_exchange_append(None, "operation:other", b"other"),
        Err(ControlMediaFault::GenerationMismatch { .. })
    ));
    assert_eq!(first.generation(), ControlStoreGeneration::initial());

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("append torn bytes");
    file.write_all(b"WCT").expect("write torn tail");
    file.sync_all().expect("sync torn tail");
    let inspection = store.inspect().expect("inspect prefix");
    assert_eq!(inspection.records().len(), 1);
    assert!(matches!(
        inspection.damage(),
        Some(ControlMediaFault::TornTail { .. })
    ));
    assert!(matches!(
        store.compare_exchange_append(Some(first.generation()), "operation:blocked", b"blocked"),
        Err(ControlMediaFault::TornTail { .. })
    ));
}

#[test]
fn duplicate_transition_with_different_meaning_is_rejected() {
    let directory = tempfile::tempdir().expect("temp directory");
    let store = PhysicalOperationalControlStore::open(ControlMediaLocation::new(
        directory.path().join("control.log"),
    ))
    .expect("open");
    store
        .compare_exchange_append(None, "same-transition", b"first")
        .expect("append");
    assert!(matches!(
        store.compare_exchange_append(None, "same-transition", b"different"),
        Err(ControlMediaFault::DuplicateTransitionConflict)
    ));
}

#[test]
fn every_interrupted_second_append_reopens_to_the_first_atomic_prefix() {
    let directory = tempfile::tempdir().expect("temp directory");
    let source = directory.path().join("source.log");
    let store =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(&source)).expect("open");
    let first = store
        .compare_exchange_append(None, "operation:opened", b"opened")
        .expect("first append");
    let first_prefix = std::fs::read(&source).expect("first prefix");
    store
        .compare_exchange_append(Some(first.generation()), "operation:lease", b"lease")
        .expect("second append");
    let complete = std::fs::read(&source).expect("complete log");

    for cut in first_prefix.len() + 1..complete.len() {
        let interrupted = directory.path().join(format!("interrupted-{cut}.log"));
        write_control_image(&interrupted, &complete[..cut]);
        let inspection =
            PhysicalOperationalControlStore::open(ControlMediaLocation::new(interrupted))
                .expect("reopen")
                .inspect()
                .expect("inspect");
        assert_eq!(inspection.records().len(), 1, "cut={cut}");
        assert!(
            matches!(
                inspection.damage(),
                Some(ControlMediaFault::TornTail { .. })
            ),
            "cut={cut}"
        );
    }

    let reordered = directory.path().join("reordered.log");
    let mut bytes = complete[first_prefix.len()..].to_vec();
    bytes.extend_from_slice(&first_prefix);
    write_control_image(&reordered, &bytes);
    let inspection = PhysicalOperationalControlStore::open(ControlMediaLocation::new(reordered))
        .expect("reopen")
        .inspect()
        .expect("inspect");
    assert!(inspection.damage().is_some());
}

fn write_control_image(path: &std::path::Path, bytes: &[u8]) {
    let destination = PhysicalOperationalControlStore::open(ControlMediaLocation::new(path))
        .expect("create destination control-media identity");
    drop(destination);
    std::fs::write(path, bytes).expect("write controlled journal image");
}

#[test]
fn oversized_control_records_are_rejected_before_media_mutation() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let store =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path)).expect("open");
    let oversized = vec![0; super::durable_prefix_recovery::MAX_CONTROL_PAYLOAD_BYTES + 1];

    assert!(matches!(
        store.compare_exchange_append(None, "operation:oversized", &oversized),
        Err(ControlMediaFault::RecordTooLarge { .. })
    ));
    assert_eq!(std::fs::metadata(path).expect("metadata").len(), 0);
}

#[test]
fn bounded_summary_scans_long_control_histories_without_materializing_records() {
    let directory = tempfile::tempdir().expect("temp directory");
    let store = PhysicalOperationalControlStore::open(ControlMediaLocation::new(
        directory.path().join("control.log"),
    ))
    .expect("open");
    let mut generation = None;
    for index in 0..256 {
        let receipt = store
            .compare_exchange_append(generation, &format!("transition-{index}"), b"payload")
            .expect("append");
        assert_eq!(receipt.prefix_records_scanned(), 0);
        generation = Some(receipt.generation());
    }

    let summary = store.inspect_summary().expect("summary");
    assert!(summary.damage().is_none());
    assert_eq!(summary.record_count(), 256);
    assert_eq!(summary.last_generation(), generation);
}

#[test]
fn independent_handles_scan_only_unseen_suffix_records() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let first = PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path))
        .expect("open first");
    let second = PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path))
        .expect("open second");

    assert_eq!(
        first
            .append_at_current_tail("first", b"one")
            .expect("first append")
            .prefix_records_scanned(),
        0
    );
    assert_eq!(
        second
            .append_at_current_tail("second", b"two")
            .expect("second append")
            .prefix_records_scanned(),
        1
    );
    assert_eq!(
        second
            .append_at_current_tail("third", b"three")
            .expect("third append")
            .prefix_records_scanned(),
        0
    );
}

#[test]
fn idempotency_remains_exact_after_transition_receipts_spill_out_of_memory() {
    let directory = tempfile::tempdir().expect("temp directory");
    let store = PhysicalOperationalControlStore::open(ControlMediaLocation::new(
        directory.path().join("control.log"),
    ))
    .expect("open");
    let first = store
        .append_at_current_tail("oldest-transition", b"oldest-payload")
        .expect("first append");
    for index in 0..2_048 {
        store
            .append_at_current_tail(&format!("later-transition-{index}"), b"later-payload")
            .expect("bounded-index append");
    }

    let replay = store
        .append_at_current_tail("oldest-transition", b"oldest-payload")
        .expect("old transition remains idempotent");
    assert!(replay.idempotent_replay());
    assert_eq!(replay.generation(), first.generation());
    assert_eq!(replay.prefix_digest(), first.prefix_digest());
    assert_eq!(replay.prefix_records_scanned(), 0);
}

#[test]
fn duplicated_external_transition_fails_closed_without_extending_media() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let store =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path)).expect("open");
    let first = store
        .append_at_current_tail("same", b"meaning")
        .expect("first append");
    let duplicate = super::encode_record(
        first.generation().next().expect("next generation"),
        first.prefix_digest(),
        "same",
        b"meaning",
    )
    .expect("encode duplicate");
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("open raw media");
    file.write_all(&duplicate).expect("write duplicate");
    file.sync_all().expect("sync duplicate");
    let damaged_bytes = std::fs::metadata(&path).expect("metadata").len();

    assert!(matches!(
        store.append_at_current_tail("after-duplicate", b"must not append"),
        Err(ControlMediaFault::DuplicateTransitionConflict)
    ));
    assert_eq!(
        std::fs::metadata(path)
            .expect("metadata after denial")
            .len(),
        damaged_bytes
    );
}

#[test]
fn cached_tail_rejects_same_length_divergence_even_when_the_last_transition_matches() {
    let directory = tempfile::tempdir().expect("temp directory");
    let selected_path = directory.path().join("selected.log");
    let divergent_path = directory.path().join("divergent.log");
    let selected = PhysicalOperationalControlStore::open(ControlMediaLocation::new(&selected_path))
        .expect("selected store");
    let divergent =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(&divergent_path))
            .expect("divergent store");
    selected
        .append_at_current_tail("first-a", b"payload")
        .expect("selected first");
    divergent
        .append_at_current_tail("first-b", b"payload")
        .expect("divergent first");
    for store in [&selected, &divergent] {
        store
            .append_at_current_tail("shared-second", b"same")
            .expect("shared final transition");
    }
    let divergent_bytes = std::fs::read(divergent_path).expect("divergent bytes");
    assert_eq!(
        std::fs::metadata(&selected_path)
            .expect("selected length")
            .len(),
        divergent_bytes.len() as u64
    );
    std::fs::write(&selected_path, divergent_bytes).expect("same-inode divergent replacement");

    assert!(matches!(
        selected.append_at_current_tail("must-not-append", b"effect"),
        Err(ControlMediaFault::ControlHistoryChanged)
    ));
}

#[test]
fn externally_truncated_tail_fails_closed_without_repairing_by_append() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let store =
        PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path)).expect("open");
    store
        .append_at_current_tail("first", b"meaning")
        .expect("first append");
    let complete_bytes = std::fs::metadata(&path).expect("metadata").len();
    std::fs::OpenOptions::new()
        .write(true)
        .open(&path)
        .expect("open raw media")
        .set_len(complete_bytes - 1)
        .expect("truncate tail");

    assert!(matches!(
        store.append_at_current_tail("after-torn-tail", b"must not append"),
        Err(ControlMediaFault::ControlHistoryRewound { .. })
    ));
    assert_eq!(
        std::fs::metadata(path)
            .expect("metadata after denial")
            .len(),
        complete_bytes - 1
    );
}
