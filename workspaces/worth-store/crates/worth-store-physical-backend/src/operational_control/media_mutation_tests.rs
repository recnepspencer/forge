use std::io::{Seek, SeekFrom, Write};

use super::{ControlMediaFault, ControlMediaLocation, PhysicalOperationalControlStore};

#[test]
fn cached_tail_revalidates_same_length_earlier_record_mutation() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let store = PhysicalOperationalControlStore::open(ControlMediaLocation::new(&path))
        .expect("control store");
    store
        .append_at_current_tail("first", b"first-payload")
        .expect("first append");
    store
        .append_at_current_tail("second", b"second-payload")
        .expect("second append");
    let bytes = std::fs::read(&path).expect("journal bytes");
    let payload_offset = bytes
        .windows(b"first-payload".len())
        .position(|window| window == b"first-payload")
        .expect("first payload offset");
    let original_modified = std::fs::metadata(&path)
        .expect("journal metadata")
        .modified()
        .expect("modified time");

    let mut modified = original_modified;
    for _ in 0..20 {
        let mut journal = std::fs::OpenOptions::new()
            .write(true)
            .open(&path)
            .expect("open journal for controlled damage");
        journal
            .seek(SeekFrom::Start(payload_offset as u64))
            .expect("seek first payload");
        journal.write_all(b"F").expect("damage earlier payload");
        journal.sync_all().expect("persist controlled damage");
        modified = journal
            .metadata()
            .expect("damaged metadata")
            .modified()
            .expect("damaged modified time");
        if modified != original_modified {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert_ne!(
        modified, original_modified,
        "test requires change detection"
    );
    let damaged_len = std::fs::metadata(&path).expect("damaged length").len();

    assert!(matches!(
        store.append_at_current_tail("must-not-append", b"irreversible-effect"),
        Err(ControlMediaFault::CorruptRecord { .. })
    ));
    assert_eq!(
        std::fs::metadata(path).expect("denied length").len(),
        damaged_len
    );
}
